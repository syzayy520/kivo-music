import { memo } from "react";
import type { SearchHit } from "../data/libraryApi";

export interface SearchResultsProps {
  loading: boolean;
  query: string;
  results: SearchHit[];
}

export const SearchResults = memo(function SearchResults({ loading, query, results }: SearchResultsProps) {
  if (!query.trim()) return null;
  return (
    <div className="search-results">
      {loading ? (
        <div className="search-state">搜索中…</div>
      ) : results.length > 0 ? (
        results.map((result) => (
          <div className="search-result" key={result.id}>
            <span className="title-ellipsis">{result.title}</span>
            <span className="sub-ellipsis">{result.path}</span>
          </div>
        ))
      ) : (
        <div className="search-state">没有找到匹配项</div>
      )}
    </div>
  );
});
