// components/DocumentList.tsx
//
// Displays all documents stored in the vault.
//
// Responsibilities:
//   - Render a list of documents
//   - Handle empty state
//   - Handle loading state
//   - Format dates for display
//
// This component receives data as props.
// It does not fetch data itself.
// App.tsx owns the data and passes it down.

import type { Document } from "../types/document";
import { CATEGORY_LABELS } from "../types/document";

interface Props {
  documents: Document[];
  isLoading: boolean;
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
  if (!dateString) return "—";
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

export function DocumentList({ documents, isLoading }: Props) {
  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-16 text-gray-400 text-sm">
        Loading documents...
      </div>
    );
  }

  if (documents.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center py-16 text-center space-y-2">
        <p className="text-gray-500 text-sm">No documents yet.</p>
        <p className="text-gray-400 text-xs">
          Add your first document using the form above.
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
            <span
              className={`shrink-0 text-xs font-medium px-2 py-0.5 rounded-full ${
                CATEGORY_COLORS[doc.category] ?? CATEGORY_COLORS.other
              }`}
            >
              {CATEGORY_LABELS[doc.category] ?? doc.category}
            </span>
          </div>

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
                {isExpired(doc.expiry_date) && " — Expired"}
                {isExpiringSoon(doc.expiry_date) && " — Expiring Soon"}
              </p>
            </div>
          </div>
        </div>
      ))}
    </div>
  );
}