import { memo } from "react";
import { useWindow } from "../hooks/useWindow";

export const WindowControls = memo(function WindowControls() {
  const { close, minimize, toggleMaximize } = useWindow();
  return (
    <div className="traffic-controls">
      <button aria-label="Close" className="traffic-btn traffic-close" onClick={close} type="button" />
      <button aria-label="Minimize" className="traffic-btn traffic-min" onClick={minimize} type="button" />
      <button aria-label="Maximize" className="traffic-btn traffic-max" onClick={toggleMaximize} type="button" />
    </div>
  );
});
