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

// Mirrors the Rust CommandError struct
export interface CommandError {
  message: string;
}