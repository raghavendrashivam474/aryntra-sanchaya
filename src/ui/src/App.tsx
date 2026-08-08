// App.tsx
//
// Application root.
//
// Responsibilities:
//   - Own the document list state
//   - Own the editing state (which document is being edited)
//   - Own search and filter state
//   - Load documents on startup
//   - Derive filteredDocuments from search query and category selection
//   - Pass filtered data down to DocumentList
//   - Pass callbacks down to AddDocumentForm and EditDocumentForm
//
// This component coordinates. It does not render business UI directly.

import { useState, useEffect, useMemo } from "react";
import type { Document, DocumentCategory } from "./types/document";
import { DOCUMENT_CATEGORIES, CATEGORY_LABELS } from "./types/document";
import { listDocuments, deleteDocument } from "./services/documentService";
import { AddDocumentForm } from "./components/AddDocumentForm";
import { EditDocumentForm } from "./components/EditDocumentForm";
import { DocumentList } from "./components/DocumentList";

export default function App() {
  // ---------------------------------------------------------------------------
  // Document state — canonical collection from backend
  // ---------------------------------------------------------------------------
  const [documents, setDocuments] = useState<Document[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [loadError, setLoadError] = useState<string | null>(null);
  const [deleteError, setDeleteError] = useState<string | null>(null);
  const [editingDocument, setEditingDocument] = useState<Document | null>(null);

  // ---------------------------------------------------------------------------
  // Search and filter state
  // ---------------------------------------------------------------------------
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedCategory, setSelectedCategory] =
    useState<DocumentCategory | "all">("all");

  // ---------------------------------------------------------------------------
  // Derived filtered documents
  //
  // Not stored as mutable state — computed from source documents.
  // Recomputes whenever documents, searchQuery, or selectedCategory changes.
  // ---------------------------------------------------------------------------
  const filteredDocuments = useMemo(() => {
    const trimmed = searchQuery.trim().toLowerCase();

    return documents.filter((doc) => {
      // Category filter — skip if "all"
      const categoryMatch =
        selectedCategory === "all" || doc.category === selectedCategory;

      // Search filter — skip if query is empty
      const searchMatch =
        trimmed === "" ||
        doc.title.toLowerCase().includes(trimmed) ||
        (doc.issuer?.toLowerCase().includes(trimmed) ?? false) ||
        (doc.description?.toLowerCase().includes(trimmed) ?? false);

      return categoryMatch && searchMatch;
    });
  }, [documents, searchQuery, selectedCategory]);

  // ---------------------------------------------------------------------------
  // Derived state flags
  // ---------------------------------------------------------------------------

  // True only when the vault itself contains no documents at all.
  const vaultIsEmpty = documents.length === 0;

  // True when filters are active and produced no results,
  // but the vault is not actually empty.
  const filtersAreActive =
    searchQuery.trim() !== "" || selectedCategory !== "all";

  // ---------------------------------------------------------------------------
  // Lifecycle
  // ---------------------------------------------------------------------------
  useEffect(() => {
    loadDocuments();
  }, []);

  async function loadDocuments() {
    setIsLoading(true);
    setLoadError(null);

    try {
      const docs = await listDocuments();
      setDocuments(docs);
    } catch (err) {
      setLoadError(
        err instanceof Error ? err.message : "Failed to load documents."
      );
    } finally {
      setIsLoading(false);
    }
  }

  // ---------------------------------------------------------------------------
  // CRUD callbacks
  // ---------------------------------------------------------------------------
  function handleDocumentAdded(document: Document) {
    setDocuments((prev) => [document, ...prev]);
  }

  function handleEditDocument(document: Document) {
    setDeleteError(null);
    setEditingDocument(document);
  }

  function handleDocumentUpdated(updated: Document) {
    setDocuments((prev) =>
      prev.map((doc) => (doc.id === updated.id ? updated : doc))
    );
    setEditingDocument(null);
  }

  function handleCancelEdit() {
    setEditingDocument(null);
  }

  async function handleDeleteDocument(id: string) {
    setDeleteError(null);

    try {
      await deleteDocument(id);
      setDocuments((prev) => prev.filter((doc) => doc.id !== id));
    } catch (err) {
      setDeleteError(
        err instanceof Error ? err.message : "Failed to delete document."
      );
    }
  }

  // ---------------------------------------------------------------------------
  // Filter controls
  // ---------------------------------------------------------------------------
  function handleClearFilters() {
    setSearchQuery("");
    setSelectedCategory("all");
  }

  // ---------------------------------------------------------------------------
  // Render
  // ---------------------------------------------------------------------------
  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <header className="bg-white border-b border-gray-200 px-6 py-4">
        <div className="max-w-3xl mx-auto">
          <h1 className="text-xl font-semibold text-gray-900">
            Aryntra Sanchaya
          </h1>
          <p className="text-sm text-gray-500 mt-0.5">
            Your personal document vault
          </p>
        </div>
      </header>

      {/* Main */}
      <main className="max-w-3xl mx-auto px-6 py-8 space-y-8">

        {/* Edit form replaces Add form while editing */}
        {editingDocument ? (
          <EditDocumentForm
            document={editingDocument}
            onDocumentUpdated={handleDocumentUpdated}
            onCancel={handleCancelEdit}
          />
        ) : (
          <AddDocumentForm onDocumentAdded={handleDocumentAdded} />
        )}

        {/* Document List */}
        <section>
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-base font-semibold text-gray-900">Vault</h2>
            <span className="text-xs text-gray-400">
              {filtersAreActive
                ? `${filteredDocuments.length} of ${documents.length} ${
                    documents.length === 1 ? "document" : "documents"
                  }`
                : `${documents.length} ${
                    documents.length === 1 ? "document" : "documents"
                  }`}
            </span>
          </div>

          {/* Search and filter controls — hidden while vault is empty */}
          {!vaultIsEmpty && !isLoading && (
            <div className="flex flex-col sm:flex-row gap-2 mb-4">
              {/* Search input */}
              <div className="relative flex-1">
                <span className="absolute inset-y-0 left-3 flex items-center text-gray-400 pointer-events-none text-sm">
                  🔍
                </span>
                <input
                  type="text"
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  placeholder="Search documents..."
                  className="w-full pl-9 pr-3 py-2 text-sm border border-gray-200 rounded-lg bg-white text-gray-900 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-indigo-300 focus:border-indigo-400"
                />
              </div>

              {/* Category selector */}
              <select
                value={selectedCategory}
                onChange={(e) =>
                  setSelectedCategory(
                    e.target.value as DocumentCategory | "all"
                  )
                }
                className="sm:w-44 px-3 py-2 text-sm border border-gray-200 rounded-lg bg-white text-gray-700 focus:outline-none focus:ring-2 focus:ring-indigo-300 focus:border-indigo-400"
              >
                <option value="all">All Categories</option>
                {DOCUMENT_CATEGORIES.map((cat) => (
                  <option key={cat} value={cat}>
                    {CATEGORY_LABELS[cat]}
                  </option>
                ))}
              </select>

              {/* Clear button — only visible when filters are active */}
              {filtersAreActive && (
                <button
                  onClick={handleClearFilters}
                  className="sm:w-auto px-4 py-2 text-sm font-medium text-gray-600 bg-white border border-gray-200 rounded-lg hover:bg-gray-50 hover:text-gray-900 transition-colors"
                >
                  Clear
                </button>
              )}
            </div>
          )}

          {/* Load error */}
          {loadError && (
            <div className="bg-red-50 border border-red-200 text-red-700 text-sm rounded px-4 py-3 mb-4">
              {loadError}
            </div>
          )}

          {/* Delete error */}
          {deleteError && (
            <div className="bg-red-50 border border-red-200 text-red-700 text-sm rounded px-4 py-3 mb-4">
              {deleteError}
            </div>
          )}

          <DocumentList
            documents={filteredDocuments}
            isLoading={isLoading}
            vaultIsEmpty={vaultIsEmpty}
            filtersAreActive={filtersAreActive}
            onEditDocument={handleEditDocument}
            onDeleteDocument={handleDeleteDocument}
          />
        </section>
      </main>
    </div>
  );
}
