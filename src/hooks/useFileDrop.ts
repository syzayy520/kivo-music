import { useEffect } from "react";
import { isAudioPath } from "../data/droppedTrack";

export interface DroppedFilePayload {
  file?: File;
  path?: string;
}

interface BrowserFile extends File {
  path?: string;
}

function pickDroppedFile(event: DragEvent) {
  const items = Array.from(event.dataTransfer?.items ?? []);
  const fromItems = items.find((item) => item.kind === "file")?.getAsFile() as BrowserFile | null;
  if (fromItems && isAudioPath(fromItems.name)) return fromItems;
  return Array.from(event.dataTransfer?.files ?? []).find((value) => isAudioPath(value.name)) as BrowserFile | undefined;
}

export function useFileDrop(onDropFile: (payload: DroppedFilePayload) => void, onResolvePath: (path: string) => void) {
  useEffect(() => {
    const onDragOver = (event: DragEvent) => event.preventDefault();
    const onDrop = (event: DragEvent) => {
      event.preventDefault();
      const file = pickDroppedFile(event);
      if (!file) return;
      onDropFile({ file, path: file.path });
      if (file.path) onResolvePath(file.path);
    };
    window.addEventListener("dragover", onDragOver);
    window.addEventListener("drop", onDrop);
    return () => {
      window.removeEventListener("dragover", onDragOver);
      window.removeEventListener("drop", onDrop);
    };
  }, [onDropFile, onResolvePath]);
}
