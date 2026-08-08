// App.tsx
//
// Application root.
//
// Responsibilities:
//   - Own the document list state
//   - Own the editing state (which document is being edited)
//   - Load documents on startup
//   - Pass data down to DocumentList
//   - Pass callbacks down to AddDocumentForm and EditDocumentForm
//
// This component coordinates. It does not render business UI directly.

import { useState, useEffect } from "react";
import type { Document } from "./types/document";
import { listDocuments, deleteDocument } from "./services/documentService";
import { AddDocumentForm } from "./components/AddDocumentForm";
import { EditDocumentForm } from "./components/EditDocumentForm";
import { DocumentList } from "./components/DocumentList";

export default function App() {
  const [documents, setDocuments] = useState<Document[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [loadError, setLoadError] = useState<string | null>(null);
  const [deleteError, setDeleteError] = useState<string | null>(null);
  const [editingDocument, setEditingDocument] = useState<Document | null>(null);

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
      // Backend confirmed deletion. Now remove from local state.
      setDocuments((prev) => prev.filter((doc) => doc.id !== id));
    } catch (err) {
      // Deletion failed. Document remains in state. Surface the error.
      setDeleteError(
        err instanceof Error ? err.message : "Failed to delete document."
      );
    }
  }

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
            <h2 className="text-base font-semibold text-gray-900">
              Vault
            </h2>
            <span className="text-xs text-gray-400">
              {documents.length}{" "}
              {documents.length === 1 ? "document" : "documents"}
            </span>
          </div>

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
            documents={documents}
            isLoading={isLoading}
            onEditDocument={handleEditDocument}
            onDeleteDocument={handleDeleteDocument}
          />
        </section>
      </main>
    </div>
  );
}
