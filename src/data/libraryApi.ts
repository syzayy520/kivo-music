import { invoke } from "@tauri-apps/api/core";

export interface LibraryCounts {
  deleted: number;
  roots: number;
  tracks: number;
}

export interface SearchHit {
  id: number;
  path: string;
  title: string;
}

const fallbackCounts: LibraryCounts = { deleted: 0, roots: 1, tracks: 1248 };

export async function fetchLibraryCounts() {
  try {
    return await invoke<LibraryCounts>("library_counts");
  } catch {
    return fallbackCounts;
  }
}

export async function searchLibraryPrefix(query: string) {
  if (!query.trim()) {
    return [];
  }
  try {
    return await invoke<SearchHit[]>("library_search_prefix", { limit: 5, q: query.trim() });
  } catch {
    return [];
  }
}

export async function scanDefaultLibrary() {
  return invoke("library_scan_default");
}
