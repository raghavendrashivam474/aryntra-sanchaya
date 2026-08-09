// services/documentService.ts
//
// The only file in the frontend allowed to call Tauri commands.
// All other components call this service.
// No component imports from @tauri-apps/api directly.

import { invoke } from "@tauri-apps/api/core";
import type {
  Document,
  AddDocumentInput,
  UpdateDocumentInput,
  Attachment,
  CommandError,
} from "../types/document";

// ---------------------------------------------------------------------------
// Document operations
// ---------------------------------------------------------------------------

export async function addDocument(input: AddDocumentInput): Promise<Document> {
  try {
    return await invoke<Document>("add_document", { input });
  } catch (error) {
    const commandError = error as CommandError;
    throw new Error(commandError.message ?? "Failed to add document");
  }
}

export async function listDocuments(): Promise<Document[]> {
  try {
    return await invoke<Document[]>("list_documents");
  } catch (error) {
    const commandError = error as CommandError;
    throw new Error(commandError.message ?? "Failed to load documents");
  }
}

export async function updateDocument(
  input: UpdateDocumentInput
): Promise<Document> {
  try {
    return await invoke<Document>("update_document", { input });
  } catch (error) {
    const commandError = error as CommandError;
    throw new Error(commandError.message ?? "Failed to update document");
  }
}

export async function deleteDocument(id: string): Promise<void> {
  try {
    await invoke<void>("delete_document", { id });
  } catch (error) {
    const commandError = error as CommandError;
    throw new Error(commandError.message ?? "Failed to delete document");
  }
}

// ---------------------------------------------------------------------------
// Attachment operations (v0.7.0)
// ---------------------------------------------------------------------------

export async function attachDocumentFile(
  documentId: string,
  sourcePath: string,
  originalFilename: string
): Promise<Attachment> {
  try {
    return await invoke<Attachment>("attach_document_file", {
      documentId,
      sourcePath,
      originalFilename,
    });
  } catch (error) {
    const commandError = error as CommandError;
    throw new Error(commandError.message ?? "Failed to attach file");
  }
}

export async function getDocumentAttachment(
  documentId: string
): Promise<Attachment | null> {
  try {
    return await invoke<Attachment | null>("get_document_attachment", {
      documentId,
    });
  } catch (error) {
    const commandError = error as CommandError;
    throw new Error(commandError.message ?? "Failed to get attachment");
  }
}

export async function removeDocumentAttachment(
  documentId: string
): Promise<void> {
  try {
    await invoke<void>("remove_document_attachment", { documentId });
  } catch (error) {
    const commandError = error as CommandError;
    throw new Error(commandError.message ?? "Failed to remove attachment");
  }
}

export async function openDocumentAttachment(
  documentId: string
): Promise<void> {
  try {
    await invoke<void>("open_document_attachment", { documentId });
  } catch (error) {
    const commandError = error as CommandError;
    throw new Error(commandError.message ?? "Failed to open attachment");
  }
}