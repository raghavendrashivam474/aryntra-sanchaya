// components/EditDocumentForm.tsx
//
// Form for editing an existing document.
//
// Responsibilities:
//   - Receive an existing Document as a prop
//   - Pre-populate all editable fields
//   - Validate required fields before submission
//   - Call the document service
//   - Report success or cancellation to the parent
//
// This component does not know about Tauri or SQLite.
// It receives callbacks and calls them on success or cancel.

import { useState } from "react";
import type { Document, UpdateDocumentInput } from "../types/document";
import { DOCUMENT_CATEGORIES, CATEGORY_LABELS } from "../types/document";
import { updateDocument } from "../services/documentService";

interface Props {
  document: Document;
  onDocumentUpdated: (document: Document) => void;
  onCancel: () => void;
}

export function EditDocumentForm({
  document,
  onDocumentUpdated,
  onCancel,
}: Props) {
  const [form, setForm] = useState<UpdateDocumentInput>({
    id: document.id,
    title: document.title,
    category: document.category,
    description: document.description,
    file_path: document.file_path,
    issuer: document.issuer,
    issue_date: document.issue_date,
    expiry_date: document.expiry_date,
  });

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
      const updated = await updateDocument(form);
      onDocumentUpdated(updated);
    } catch (err) {
      setError(err instanceof Error ? err.message : "An error occurred.");
    } finally {
      setIsSubmitting(false);
    }
  }

  return (
    <form
      onSubmit={handleSubmit}
      className="bg-white border border-indigo-200 rounded-lg p-6 space-y-4"
    >
      <div className="flex items-center justify-between">
        <h2 className="text-lg font-semibold text-gray-900">Edit Document</h2>
        <button
          type="button"
          onClick={onCancel}
          className="text-sm text-gray-500 hover:text-gray-700"
        >
          Cancel
        </button>
      </div>

      {/* Error */}
      {error && (
        <div className="bg-red-50 border border-red-200 text-red-700 text-sm rounded px-4 py-3">
          {error}
        </div>
      )}

      {/* Title */}
      <div className="space-y-1">
        <label
          htmlFor="edit-title"
          className="block text-sm font-medium text-gray-700"
        >
          Title <span className="text-red-500">*</span>
        </label>
        <input
          id="edit-title"
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
          htmlFor="edit-category"
          className="block text-sm font-medium text-gray-700"
        >
          Category
        </label>
        <select
          id="edit-category"
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
          htmlFor="edit-issuer"
          className="block text-sm font-medium text-gray-700"
        >
          Issuing Authority
        </label>
        <input
          id="edit-issuer"
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
            htmlFor="edit-issue_date"
            className="block text-sm font-medium text-gray-700"
          >
            Issue Date
          </label>
          <input
            id="edit-issue_date"
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
            htmlFor="edit-expiry_date"
            className="block text-sm font-medium text-gray-700"
          >
            Expiry Date
          </label>
          <input
            id="edit-expiry_date"
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
          htmlFor="edit-description"
          className="block text-sm font-medium text-gray-700"
        >
          Description
        </label>
        <textarea
          id="edit-description"
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
        {isSubmitting ? "Saving..." : "Save Changes"}
      </button>
    </form>
  );
}
