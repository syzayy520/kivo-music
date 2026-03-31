import { memo, type ReactNode } from "react";

interface IconProps {
  children: ReactNode;
  className?: string;
}

export const Icon = memo(function Icon({ children, className }: IconProps) {
  return <svg aria-hidden="true" className={className} viewBox="0 0 24 24">{children}</svg>;
});
