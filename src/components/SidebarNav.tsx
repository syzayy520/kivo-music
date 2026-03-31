import { memo } from "react";
import type { NavItem } from "../data/library";
import { NavIcon } from "./NavIcon";

export interface SidebarNavProps {
  activeLabel?: string;
  items: NavItem[];
  onSelect?: (label: string) => void;
  title: string;
}

export const SidebarNav = memo(function SidebarNav({ activeLabel, items, onSelect, title }: SidebarNavProps) {
  return (
    <section>
      <div className="section-title">{title}</div>
      <div className="nav-group">
        {items.map((item, index) => (
          <button className={activeLabel === item.label || (index === 0 && title === "浏览") ? "nav-item active" : "nav-item"} key={item.label} onClick={() => onSelect?.(item.label)} type="button">
            <NavIcon name={item.icon} />
            <span className="nav-label">{item.label}</span>
          </button>
        ))}
      </div>
    </section>
  );
});
