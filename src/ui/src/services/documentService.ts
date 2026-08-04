// services/documentService.ts
//
// The only file in the frontend allowed to call Tauri commands.
// All other components call this service.
// No component imports from @tauri-apps/api directly.
//
// This boundary means if Tauri changes, only this file changes.

import { invoke } from "@tauri-apps/api/core";
import type { Document, AddDocumentInput, CommandError } from "../types/document";

// Tauri commands return Ok(T) or Err(CommandError).
// invoke() throws on Err, so we catch and re-throw with a clean message.

export async function addDocument(input: AddDocumentInput): Promise<Document> {
  try {
    const document = await invoke<Document>("add_document", { input });
    return document;
  } catch (error) {
    const commandError = error as CommandError;
    throw new Error(commandError.message ?? "Failed to add document");
  }
}

export async function listDocuments(): Promise<Document[]> {
  try {
    const documents = await invoke<Document[]>("list_documents");
    return documents;
  } catch (error) {
    const commandError = error as CommandError;
    throw new Error(commandError.message ?? "Failed to load documents");
  }
}