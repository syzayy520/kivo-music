import { memo } from "react";
import type { SearchHit } from "../data/libraryApi";
import { DragRegion } from "./DragRegion";
import { Icon } from "./Icons";
import { SearchResults } from "./SearchResults";

export interface SearchToolbarProps {
  loading: boolean;
  onQueryChange: (value: string) => void;
  query: string;
  results: SearchHit[];
}

export const SearchToolbar = memo(function SearchToolbar({ loading, onQueryChange, query, results }: SearchToolbarProps) {
  return (
    <header className="toolbar">
      <DragRegion className="toolbar-drag" />
      <div className="toolbar-actions">
        <label className="search">
          <Icon className="search-icon"><circle cx="11" cy="11" r="5.5" fill="none" stroke="currentColor" strokeWidth="1.7" /><path d="m15.2 15.2 4.1 4.1" fill="none" stroke="currentColor" strokeLinecap="round" strokeWidth="1.7" /></Icon>
          <input onChange={(event) => onQueryChange(event.target.value)} placeholder="搜索歌曲、专辑、歌手" value={query} />
          <SearchResults loading={loading} query={query} results={results} />
        </label>
        <button className="toolbar-btn toolbar-cmd" type="button"><span aria-hidden="true" className="toolbar-cmd-glyph">{"\u2318"}</span></button>
      </div>
    </header>
  );
});
