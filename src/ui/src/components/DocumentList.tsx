// components/DocumentList.tsx
//
// Displays documents stored in the vault.
//
// Responsibilities:
//   - Render a list of documents
//   - Handle loading state
//   - Handle vault-empty state (no documents exist at all)
//   - Handle no-results state (filters produced no matches)
//   - Display expiry status badge and human-readable relative time
//   - Surface Edit and Delete actions for each document
//   - Show inline confirmation before destructive deletion
//
// Expiry classification uses getExpiryStatus() and getDaysUntilExpiry()
// from types/document.ts. The 30-day threshold lives there, not here.

import { useState } from "react";
import type { Document, ExpiryStatus } from "../types/document";
import {
  CATEGORY_LABELS,
  getExpiryStatus,
  getDaysUntilExpiry,
  formatExpiryDate,
} from "../types/document";

interface Props {
  documents: Document[];
  isLoading: boolean;
  vaultIsEmpty: boolean;
  filtersAreActive: boolean;
  onEditDocument: (document: Document) => void;
  onDeleteDocument: (id: string) => void;
}

const CATEGORY_COLORS: Record<string, string> = {
  identity: "bg-blue-100 text-blue-700",
  education: "bg-green-100 text-green-700",
  financial: "bg-yellow-100 text-yellow-700",
  medical: "bg-red-100 text-red-700",
  legal: "bg-purple-100 text-purple-700",
  employment: "bg-orange-100 text-orange-700",
  travel: "bg-teal-100 text-teal-700",
  other: "bg-gray-100 text-gray-700",
};

const EXPIRY_BADGE_COLORS: Record<ExpiryStatus, string> = {
  expired: "bg-red-100 text-red-700 border-red-200",
  expiring_soon: "bg-yellow-100 text-yellow-700 border-yellow-200",
  valid: "bg-green-100 text-green-700 border-green-200",
  no_expiry: "bg-gray-100 text-gray-500 border-gray-200",
};

const EXPIRY_BADGE_LABELS: Record<ExpiryStatus, string> = {
  expired: "Expired",
  expiring_soon: "Expiring Soon",
  valid: "Valid",
  no_expiry: "No Expiry",
};

function formatRelativeDays(days: number | null): string | null {
  if (days === null) return null;
  if (days < 0) {
    const abs = Math.abs(days);
    return abs === 1 ? "1 day ago" : `${abs} days ago`;
  }
  if (days === 0) return "today";
  if (days === 1) return "in 1 day";
  return `in ${days} days`;
}

export function DocumentList({
  documents,
  isLoading,
  vaultIsEmpty,
  filtersAreActive,
  onEditDocument,
  onDeleteDocument,
}: Props) {
  const [confirmingDeleteId, setConfirmingDeleteId] = useState<string | null>(
    null
  );

  function handleDeleteClick(id: string) {
    setConfirmingDeleteId(id);
  }

  function handleCancelDelete() {
    setConfirmingDeleteId(null);
  }

  function handleConfirmDelete(id: string) {
    setConfirmingDeleteId(null);
    onDeleteDocument(id);
  }

  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-16 text-gray-400 text-sm">
        Loading documents...
      </div>
    );
  }

  if (vaultIsEmpty) {
    return (
      <div className="flex flex-col items-center justify-center py-16 text-center space-y-2">
        <p className="text-gray-500 text-sm">No documents yet.</p>
        <p className="text-gray-400 text-xs">
          Add your first document using the form above.
        </p>
      </div>
    );
  }

  if (filtersAreActive && documents.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center py-16 text-center space-y-2">
        <p className="text-gray-500 text-sm">No matching documents.</p>
        <p className="text-gray-400 text-xs">
          Try a different search or filter.
        </p>
      </div>
    );
  }

  return (
    <div className="space-y-3">
      {documents.map((doc) => {
        const now = new Date();
        const status = getExpiryStatus(doc.expiry_date, now);
        const days = getDaysUntilExpiry(doc.expiry_date, now);
        const relative = formatRelativeDays(days);

        return (
          <div
            key={doc.id}
            className="bg-white border border-gray-200 rounded-lg p-4 space-y-2"
          >
            {/* Header row */}
            <div className="flex items-start justify-between gap-4">
              <h3 className="text-sm font-semibold text-gray-900 leading-tight">
                {doc.title}
              </h3>
              <div className="flex items-center gap-2 shrink-0">
                <span
                  className={`text-xs font-medium px-2 py-0.5 rounded-full ${
                    CATEGORY_COLORS[doc.category] ?? CATEGORY_COLORS.other
                  }`}
                >
                  {CATEGORY_LABELS[doc.category] ?? doc.category}
                </span>
                <button
                  onClick={() => onEditDocument(doc)}
                  className="text-xs text-indigo-600 hover:text-indigo-800 font-medium"
                >
                  Edit
                </button>
                <button
                  onClick={() => handleDeleteClick(doc.id)}
                  className="text-xs text-red-500 hover:text-red-700 font-medium"
                >
                  Delete
                </button>
              </div>
            </div>

            {/* Inline delete confirmation */}
            {confirmingDeleteId === doc.id && (
              <div className="bg-red-50 border border-red-200 rounded p-3 space-y-2">
                <p className="text-xs text-red-700 font-medium">
                  Delete &ldquo;{doc.title}&rdquo;? This cannot be undone.
                </p>
                <div className="flex gap-2">
                  <button
                    onClick={handleCancelDelete}
                    className="text-xs px-3 py-1 rounded border border-gray-300 bg-white text-gray-700 hover:bg-gray-50 font-medium"
                  >
                    Cancel
                  </button>
                  <button
                    onClick={() => handleConfirmDelete(doc.id)}
                    className="text-xs px-3 py-1 rounded bg-red-600 text-white hover:bg-red-700 font-medium"
                  >
                    Delete
                  </button>
                </div>
              </div>
            )}

            {/* Issuer */}
            {doc.issuer && (
              <p className="text-xs text-gray-500">Issued by {doc.issuer}</p>
            )}

            {/* Description */}
            {doc.description && (
              <p className="text-xs text-gray-600">{doc.description}</p>
            )}

            {/* Expiry row */}
            <div className="flex items-center gap-3 pt-1">
              {/* Expiry date */}
              <div>
                <p className="text-xs text-gray-400">Expires</p>
                <p className="text-xs text-gray-700">
                  {doc.expiry_date ? formatExpiryDate(doc.expiry_date) : "\u2014"}
                </p>
              </div>

              {/* Expiry status badge */}
              <span
                className={`text-xs font-medium px-2 py-0.5 rounded border ${
                  EXPIRY_BADGE_COLORS[status]
                }`}
              >
                {EXPIRY_BADGE_LABELS[status]}
                {relative && status !== "no_expiry" && ` \u2014 ${relative}`}
              </span>
            </div>
          </div>
        );
      })}
    </div>
  );
}