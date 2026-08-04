// components/AddDocumentForm.tsx
//
// Form for adding a new document to the vault.
//
// Responsibilities:
//   - Collect document metadata from the user
//   - Validate required fields before submission
//   - Call the document service
//   - Report success or failure to the parent
//
// This component does not know about Tauri or SQLite.
// It receives a callback and calls it when a document is added.

import { useState } from "react";
import type { AddDocumentInput, Document } from "../types/document";
import { DOCUMENT_CATEGORIES, CATEGORY_LABELS } from "../types/document";
import { addDocument } from "../services/documentService";

interface Props {
  onDocumentAdded: (document: Document) => void;
}

const EMPTY_FORM: AddDocumentInput = {
  title: "",
  category: "other",
  description: null,
  file_path: null,
  issuer: null,
  issue_date: null,
  expiry_date: null,
};

export function AddDocumentForm({ onDocumentAdded }: Props) {
  const [form, setForm] = useState<AddDocumentInput>(EMPTY_FORM);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  function handleChange(
    e: React.ChangeEvent<
      HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement
    >
  ) {
    const { name, value } = e.target;
    setForm((prev) => ({
      ...prev,
      [name]: value === "" ? null : value,
    }));
  }

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setError(null);

    if (!form.title || form.title.trim() === "") {
      setError("Title is required.");
      return;
    }

    setIsSubmitting(true);

    try {
      const document = await addDocument(form);
      onDocumentAdded(document);
      setForm(EMPTY_FORM);
    } catch (err) {
      setError(err instanceof Error ? err.message : "An error occurred.");
    } finally {
      setIsSubmitting(false);
    }
  }

  return (
    <form
      onSubmit={handleSubmit}
      className="bg-white border border-gray-200 rounded-lg p-6 space-y-4"
    >
      <h2 className="text-lg font-semibold text-gray-900">Add Document</h2>

      {/* Error */}
      {error && (
        <div className="bg-red-50 border border-red-200 text-red-700 text-sm rounded px-4 py-3">
          {error}
        </div>
      )}

      {/* Title */}
      <div className="space-y-1">
        <label
          htmlFor="title"
          className="block text-sm font-medium text-gray-700"
        >
          Title <span className="text-red-500">*</span>
        </label>
        <input
          id="title"
          name="title"
          type="text"
          value={form.title}
          onChange={handleChange}
          placeholder="e.g. Passport, Degree Certificate"
          className="w-full border border-gray-300 rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
        />
      </div>

      {/* Category */}
      <div className="space-y-1">
        <label
          htmlFor="category"
          className="block text-sm font-medium text-gray-700"
        >
          Category
        </label>
        <select
          id="category"
          name="category"
          value={form.category ?? "other"}
          onChange={handleChange}
          className="w-full border border-gray-300 rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
        >
          {DOCUMENT_CATEGORIES.map((cat) => (
            <option key={cat} value={cat}>
              {CATEGORY_LABELS[cat]}
            </option>
          ))}
        </select>
      </div>

      {/* Issuer */}
      <div className="space-y-1">
        <label
          htmlFor="issuer"
          className="block text-sm font-medium text-gray-700"
        >
          Issuing Authority
        </label>
        <input
          id="issuer"
          name="issuer"
          type="text"
          value={form.issuer ?? ""}
          onChange={handleChange}
          placeholder="e.g. Government of India"
          className="w-full border border-gray-300 rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
        />
      </div>

      {/* Dates */}
      <div className="grid grid-cols-2 gap-4">
        <div className="space-y-1">
          <label
            htmlFor="issue_date"
            className="block text-sm font-medium text-gray-700"
          >
            Issue Date
          </label>
          <input
            id="issue_date"
            name="issue_date"
            type="date"
            value={form.issue_date?.substring(0, 10) ?? ""}
            onChange={(e) => {
              const value = e.target.value;
              setForm((prev) => ({
                ...prev,
                issue_date: value ? `${value}T00:00:00Z` : null,
              }));
            }}
            className="w-full border border-gray-300 rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
          />
        </div>

        <div className="space-y-1">
          <label
            htmlFor="expiry_date"
            className="block text-sm font-medium text-gray-700"
          >
            Expiry Date
          </label>
          <input
            id="expiry_date"
            name="expiry_date"
            type="date"
            value={form.expiry_date?.substring(0, 10) ?? ""}
            onChange={(e) => {
              const value = e.target.value;
              setForm((prev) => ({
                ...prev,
                expiry_date: value ? `${value}T00:00:00Z` : null,
              }));
            }}
            className="w-full border border-gray-300 rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
          />
        </div>
      </div>

      {/* Description */}
      <div className="space-y-1">
        <label
          htmlFor="description"
          className="block text-sm font-medium text-gray-700"
        >
          Description
        </label>
        <textarea
          id="description"
          name="description"
          rows={3}
          value={form.description ?? ""}
          onChange={handleChange}
          placeholder="Optional notes about this document"
          data-gramm="false"
          data-gramm_editor="false"
          data-enable-grammarly="false"
          className="w-full border border-gray-300 rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 resize-none"
        />
      </div>

      {/* Submit */}
      <button
        type="submit"
        disabled={isSubmitting}
        className="w-full bg-indigo-600 hover:bg-indigo-700 disabled:opacity-50 text-white text-sm font-medium rounded px-4 py-2 transition-colors"
      >
        {isSubmitting ? "Saving..." : "Save Document"}
      </button>
    </form>
  );
}