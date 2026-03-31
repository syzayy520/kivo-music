import { memo } from "react";

export interface WorkspaceScrollbarProps {
  thumbHeight: number;
  thumbTop: number;
  visible: boolean;
}

export const WorkspaceScrollbar = memo(function WorkspaceScrollbar(props: WorkspaceScrollbarProps) {
  const { thumbHeight, thumbTop, visible } = props;
  if (!visible) return null;
  return (
    <div aria-hidden="true" className="workspace-scrollbar">
      <div className="workspace-thumb" style={{ height: `${thumbHeight}px`, transform: `translateY(${thumbTop}px)` }} />
    </div>
  );
});
