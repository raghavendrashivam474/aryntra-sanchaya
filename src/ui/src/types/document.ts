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
// Attachment (v0.7.0)
// ---------------------------------------------------------------------------
//
// Mirrors the Rust Attachment struct from domain/attachment.rs.

export interface Attachment {
  id: string;
  document_id: string;
  original_filename: string;
  mime_type: string;
  size_bytes: number;
  stored_filename: string;
  created_at: string;
}

// Supported MIME types for attachment validation.
export const SUPPORTED_MIME_TYPES = [
  "application/pdf",
  "image/jpeg",
  "image/png",
  "image/webp",
] as const;

// File picker filter extensions.
export const SUPPORTED_EXTENSIONS = ["pdf", "jpg", "jpeg", "png", "webp"];

// Human-readable label for a MIME type.
export function attachmentTypeLabel(mimeType: string): string {
  switch (mimeType) {
    case "application/pdf":
      return "PDF";
    case "image/jpeg":
      return "JPG";
    case "image/png":
      return "PNG";
    case "image/webp":
      return "WebP";
    default:
      return "File";
  }
}

// Format bytes into a human-readable size string.
export function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

// ---------------------------------------------------------------------------
// ExpiryStatus
// ---------------------------------------------------------------------------

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

export const EXPIRY_STATUSES: ExpiryStatus[] = [
  "expired",
  "expiring_soon",
  "valid",
  "no_expiry",
];

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

export function getDaysUntilExpiry(
  expiryDate: string | null,
  now: Date
): number | null {
  if (!expiryDate) return null;

  const expiry = new Date(expiryDate);
  const diffMs = expiry.getTime() - now.getTime();
  return Math.floor(diffMs / (1000 * 60 * 60 * 24));
}

export function formatExpiryDate(dateString: string | null): string {
  if (!dateString) return "\u2014";
  const date = new Date(dateString);
  return date.toLocaleDateString(undefined, {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
}