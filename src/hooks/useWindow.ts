import { getCurrentWindow } from "@tauri-apps/api/window";
import { useCallback } from "react";

export function useWindow() {
  const hasTauriWindow = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
  const appWindow = hasTauriWindow ? getCurrentWindow() : null;
  const startDragging = useCallback(() => {
    if (!appWindow) {
      return;
    }
    void appWindow.startDragging();
  }, [appWindow]);
  const minimize = useCallback(() => {
    if (!appWindow) {
      return;
    }
    void appWindow.minimize();
  }, [appWindow]);
  const toggleMaximize = useCallback(() => {
    if (!appWindow) {
      return;
    }
    void appWindow.toggleMaximize();
  }, [appWindow]);
  const close = useCallback(() => {
    if (!appWindow) {
      return;
    }
    void appWindow.close();
  }, [appWindow]);
  return { close, minimize, startDragging, toggleMaximize };
}
