import { memo, useCallback } from "react";
import { useWindow } from "../hooks/useWindow";

export interface DragRegionProps {
  className: string;
}

export const DragRegion = memo(function DragRegion({ className }: DragRegionProps) {
  const { startDragging, toggleMaximize } = useWindow();

  const handleMouseDown = useCallback((event: React.MouseEvent<HTMLDivElement>) => {
    if (event.button !== 0) {
      return;
    }
    void startDragging();
  }, [startDragging]);

  const handleDoubleClick = useCallback(() => {
    void toggleMaximize();
  }, [toggleMaximize]);

  return (
    <div
      className={className}
      data-tauri-drag-region
      onDoubleClick={handleDoubleClick}
      onMouseDown={handleMouseDown}
    />
  );
});
