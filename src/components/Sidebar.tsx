import { memo } from "react";
import { browseItems, libraryItems } from "../data/library";
import { DragRegion } from "./DragRegion";
import { SidebarNav } from "./SidebarNav";
import { WindowControls } from "./WindowControls";

export interface SidebarProps {
  countsLabel: string;
  onLibraryAction: (label: string) => void;
}

export const Sidebar = memo(function Sidebar({ countsLabel, onLibraryAction }: SidebarProps) {
  return (
    <aside className="sidebar">
      <div className="sidebar-top">
        <WindowControls />
        <DragRegion className="sidebar-drag" />
      </div>
      <div className="brand">
        <div className="brand-mark">K</div>
        <div>
          <strong>Kivo</strong>
          <span>Desktop music player</span>
        </div>
      </div>
      <SidebarNav items={browseItems} title="浏览" />
      <SidebarNav items={libraryItems} onSelect={onLibraryAction} title="资料库" />
      <div className="sidebar-footer">
        <strong>本地资料库</strong>
        {countsLabel}
      </div>
    </aside>
  );
});
