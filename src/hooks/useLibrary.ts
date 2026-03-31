import { useCallback, useDeferredValue, useEffect, useState } from "react";
import { fetchLibraryCounts, scanDefaultLibrary, searchLibraryPrefix, type LibraryCounts, type SearchHit } from "../data/libraryApi";

const initialCounts: LibraryCounts = { deleted: 0, roots: 1, tracks: 1248 };

export function useLibrary() {
  const [counts, setCounts] = useState(initialCounts);
  const [isScanning, setIsScanning] = useState(false);
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchHit[]>([]);
  const [searching, setSearching] = useState(false);
  const deferredQuery = useDeferredValue(query);

  const refreshCounts = useCallback(async () => {
    setCounts(await fetchLibraryCounts());
  }, []);

  useEffect(() => {
    void refreshCounts();
  }, [refreshCounts]);

  useEffect(() => {
    let cancelled = false;
    if (!deferredQuery.trim()) {
      setResults([]);
      setSearching(false);
      return;
    }
    setSearching(true);
    void searchLibraryPrefix(deferredQuery).then((hits) => {
      if (!cancelled) {
        setResults(hits);
        setSearching(false);
      }
    });
    return () => {
      cancelled = true;
    };
  }, [deferredQuery]);

  const scanLibrary = useCallback(async () => {
    setIsScanning(true);
    try {
      await scanDefaultLibrary();
      await refreshCounts();
    } finally {
      setIsScanning(false);
    }
  }, [refreshCounts]);

  return { counts, isScanning, query, results, scanLibrary, searching, setQuery };
}
