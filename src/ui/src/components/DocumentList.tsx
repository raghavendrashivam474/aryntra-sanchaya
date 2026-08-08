// components/DocumentList.tsx
//
// Displays documents stored in the vault.
//
// Responsibilities:
//   - Render a list of documents
//   - Handle loading state
//   - Handle vault-empty state (no documents exist at all)
//   - Handle no-results state (filters produced no matches)
//   - Format dates for display
//   - Surface Edit and Delete actions for each document
//   - Show inline confirmation before destructive deletion
//
// This component receives data and callbacks as props.
// It does not fetch data itself.
// App.tsx owns the data and passes it down.
//
// Two distinct empty states:
//   vaultIsEmpty      — the vault contains no documents
//   filtersAreActive  — documents exist but the current search/filter
//                       produced no matches

import { useState } from "react";
import type { Document } from "../types/document";
import { CATEGORY_LABELS } from "../types/document";

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

function formatDate(dateString: string | null): string {
  if (!dateString) return "\u2014";
  const date = new Date(dateString);
  return date.toLocaleDateString(undefined, {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
}

function isExpired(dateString: string | null): boolean {
  if (!dateString) return false;
  return new Date(dateString) < new Date();
}

function isExpiringSoon(dateString: string | null): boolean {
  if (!dateString) return false;
  const expiry = new Date(dateString);
  const now = new Date();
  const ninetyDays = 90 * 24 * 60 * 60 * 1000;
  return expiry > now && expiry.getTime() - now.getTime() < ninetyDays;
}

export function DocumentList({
  documents,
  isLoading,
  vaultIsEmpty,
  filtersAreActive,
  onEditDocument,
  onDeleteDocument,
}: Props) {
  // Track which document is awaiting delete confirmation.
  // null means no confirmation is active.
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

  // Loading state
  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-16 text-gray-400 text-sm">
        Loading documents...
      </div>
    );
  }

  // Vault genuinely contains no documents
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

  // Filters are active but no documents matched
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
      {documents.map((doc) => (
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

          {/* Dates row */}
          <div className="flex gap-6 pt-1">
            <div>
              <p className="text-xs text-gray-400">Issue Date</p>
              <p className="text-xs text-gray-700">
                {formatDate(doc.issue_date)}
              </p>
            </div>

            <div>
              <p className="text-xs text-gray-400">Expiry Date</p>
              <p
                className={`text-xs font-medium ${
                  isExpired(doc.expiry_date)
                    ? "text-red-600"
                    : isExpiringSoon(doc.expiry_date)
                    ? "text-yellow-600"
                    : "text-gray-700"
                }`}
              >
                {formatDate(doc.expiry_date)}
                {isExpired(doc.expiry_date) && " \u2014 Expired"}
                {isExpiringSoon(doc.expiry_date) && " \u2014 Expiring Soon"}
              </p>
            </div>
          </div>
        </div>
      ))}
    </div>
  );
}
