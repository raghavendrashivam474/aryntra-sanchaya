// types/document.ts
//
// TypeScript types that mirror the Rust domain structs.
// These types cross the Tauri boundary.
// If the Rust structs change, these must change too.

export type DocumentCategory =
  | "identity"
  | "education"
  | "financial"
  | "medical"
  | "legal"
  | "employment"
  | "travel"
  | "other";

export const DOCUMENT_CATEGORIES: DocumentCategory[] = [
  "identity",
  "education",
  "financial",
  "medical",
  "legal",
  "employment",
  "travel",
  "other",
];

export const CATEGORY_LABELS: Record<DocumentCategory, string> = {
  identity: "Identity",
  education: "Education",
  financial: "Financial",
  medical: "Medical",
  legal: "Legal",
  employment: "Employment",
  travel: "Travel",
  other: "Other",
};

// Mirrors the Rust Document struct
export interface Document {
  id: string;
  title: string;
  category: DocumentCategory;
  description: string | null;
  file_path: string | null;
  issuer: string | null;
  issue_date: string | null;
  expiry_date: string | null;
  created_at: string;
  updated_at: string;
}

// Mirrors the Rust AddDocumentInput struct
export interface AddDocumentInput {
  title: string;
  category: string;
  description: string | null;
  file_path: string | null;
  issuer: string | null;
  issue_date: string | null;
  expiry_date: string | null;
}

// Mirrors the Rust UpdateDocumentInput struct
export interface UpdateDocumentInput {
  id: string;
  title: string;
  category: string;
  description: string | null;
  file_path: string | null;
  issuer: string | null;
  issue_date: string | null;
  expiry_date: string | null;
}

// Mirrors the Rust CommandError struct
export interface CommandError {
  message: string;
}

// ---------------------------------------------------------------------------
// ExpiryStatus
// ---------------------------------------------------------------------------
//
// Mirrors the Rust ExpiryStatus enum.
//
// Classification rules (v0.6.0):
//
//   no_expiry     - expiry_date is null
//   expired       - expiry_date < now
//   expiring_soon - expiry_date >= now AND expiry_date <= now + 30 days
//   valid         - expiry_date > now + 30 days
//
// The 30-day threshold is the single source of truth for the frontend.
// No other file should redefine this number.

export type ExpiryStatus =
  | "no_expiry"
  | "expired"
  | "expiring_soon"
  | "valid";

export const EXPIRY_SOON_THRESHOLD_DAYS = 30;

export const EXPIRY_STATUS_LABELS: Record<ExpiryStatus, string> = {
  no_expiry: "No Expiry",
  expired: "Expired",
  expiring_soon: "Expiring Soon",
  valid: "Valid",
};

// All statuses in display order for the filter control.
export const EXPIRY_STATUSES: ExpiryStatus[] = [
  "expired",
  "expiring_soon",
  "valid",
  "no_expiry",
];

// ---------------------------------------------------------------------------
// getExpiryStatus
// ---------------------------------------------------------------------------
//
// Pure function. No side effects. Mirrors Document::expiry_status() in Rust.
//
// `now` is a parameter so callers control the reference time.
// Production callers pass `new Date()`.
// Tests can pass a fixed date.

export function getExpiryStatus(
  expiryDate: string | null,
  now: Date
): ExpiryStatus {
  if (!expiryDate) return "no_expiry";

  const expiry = new Date(expiryDate);
  const threshold = new Date(now);
  threshold.setDate(threshold.getDate() + EXPIRY_SOON_THRESHOLD_DAYS);

  if (expiry < now) return "expired";
  if (expiry <= threshold) return "expiring_soon";
  return "valid";
}

// ---------------------------------------------------------------------------
// getDaysUntilExpiry
// ---------------------------------------------------------------------------
//
// Returns the number of whole days between now and the expiry date.
// Negative means already expired.
// Returns null if expiry_date is null.

export function getDaysUntilExpiry(
  expiryDate: string | null,
  now: Date
): number | null {
  if (!expiryDate) return null;

  const expiry = new Date(expiryDate);
  const diffMs = expiry.getTime() - now.getTime();
  return Math.floor(diffMs / (1000 * 60 * 60 * 24));
}

// ---------------------------------------------------------------------------
// formatExpiryDate
// ---------------------------------------------------------------------------
//
// Returns a human-readable date string such as "12 Sep 2026".
// Returns an em dash for null dates.

export function formatExpiryDate(dateString: string | null): string {
  if (!dateString) return "\u2014";
  const date = new Date(dateString);
  return date.toLocaleDateString(undefined, {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
}