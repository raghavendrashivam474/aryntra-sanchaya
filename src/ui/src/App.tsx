// App.tsx
//
// Application root.
//
// Responsibilities:
//   - Own the document list state
//   - Load documents on startup
//   - Pass data down to DocumentList
//   - Pass callbacks down to AddDocumentForm
//
// This component coordinates. It does not render business UI directly.

import { useState, useEffect } from "react";
import type { Document } from "./types/document";
import { listDocuments } from "./services/documentService";
import { AddDocumentForm } from "./components/AddDocumentForm";
import { DocumentList } from "./components/DocumentList";

export default function App() {
  const [documents, setDocuments] = useState<Document[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [loadError, setLoadError] = useState<string | null>(null);

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

        {/* Add Document Form */}
        <AddDocumentForm onDocumentAdded={handleDocumentAdded} />

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

          <DocumentList documents={documents} isLoading={isLoading} />
        </section>
      </main>
    </div>
  );
}