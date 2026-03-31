import { memo, type CSSProperties } from "react";

export interface ProgressBarProps {
  className: string;
  onChange: (value: number) => void;
  value: number;
}

export const ProgressBar = memo(function ProgressBar({ className, onChange, value }: ProgressBarProps) {
  const style = { "--range-progress": `${value}%` } as CSSProperties;
  return (
    <input
      className={className}
      max="100"
      min="0"
      onInput={(event) => onChange(Number(event.currentTarget.value))}
      step="0.1"
      style={style}
      type="range"
      value={value}
    />
  );
});
