import { useCallback, useEffect, useRef, useState } from "react";

interface ScrollMetrics {
  thumbHeight: number;
  thumbTop: number;
  visible: boolean;
}

export function useWorkspaceScroll() {
  const workspaceRef = useRef<HTMLDivElement>(null);
  const [metrics, setMetrics] = useState<ScrollMetrics>({ thumbHeight: 0, thumbTop: 0, visible: false });

  const update = useCallback(() => {
    const node = workspaceRef.current;
    if (!node) return;
    const { clientHeight, scrollHeight, scrollTop } = node;
    if (scrollHeight <= clientHeight + 1) {
      setMetrics({ thumbHeight: 0, thumbTop: 0, visible: false });
      return;
    }
    const thumbHeight = Math.max(56, (clientHeight / scrollHeight) * clientHeight);
    const maxTop = clientHeight - thumbHeight;
    const thumbTop = maxTop <= 0 ? 0 : (scrollTop / (scrollHeight - clientHeight)) * maxTop;
    setMetrics({ thumbHeight, thumbTop, visible: true });
  }, []);

  useEffect(() => {
    const node = workspaceRef.current;
    if (!node) return;
    const frame = requestAnimationFrame(update);
    const handleScroll = () => update();
    const resizeObserver = new ResizeObserver(handleScroll);
    const mutationObserver = new MutationObserver(handleScroll);
    node.addEventListener("scroll", handleScroll, { passive: true });
    resizeObserver.observe(node);
    Array.from(node.children).forEach((child) => resizeObserver.observe(child));
    mutationObserver.observe(node, { childList: true, subtree: true });
    window.addEventListener("resize", handleScroll);
    return () => {
      cancelAnimationFrame(frame);
      node.removeEventListener("scroll", handleScroll);
      resizeObserver.disconnect();
      mutationObserver.disconnect();
      window.removeEventListener("resize", handleScroll);
    };
  }, [update]);

  return { metrics, workspaceRef };
}
