import { memo, type ReactElement } from "react";
import { Icon } from "./Icons";

export interface NavIconProps {
  name: string;
}

export const NavIcon = memo(function NavIcon({ name }: NavIconProps) {
  const icons: Record<string, ReactElement> = {
    home: <path d="M4 10.5 12 4l8 6.5V19h-5.2v-5H9.2v5H4v-8.5Z" fill="none" stroke="currentColor" strokeLinejoin="round" strokeWidth="1.7" />,
    clock: <><circle cx="12" cy="12" r="7.5" fill="none" stroke="currentColor" strokeWidth="1.7" /><path d="M12 8v4.2l2.8 1.6" fill="none" stroke="currentColor" strokeLinecap="round" strokeWidth="1.7" /></>,
    heart: <path d="M12 19.2 5.1 12.8a4.2 4.2 0 0 1 5.9-6l1 1 1-1a4.2 4.2 0 0 1 5.9 6L12 19.2Z" fill="none" stroke="currentColor" strokeLinejoin="round" strokeWidth="1.7" />,
    queue: <path d="M6 7h12M6 12h12M6 17h8" fill="none" stroke="currentColor" strokeLinecap="round" strokeWidth="1.8" />,
    disc: <><circle cx="12" cy="12" r="7.5" fill="none" stroke="currentColor" strokeWidth="1.7" /><circle cx="12" cy="12" r="2" fill="currentColor" /></>,
    artist: <><circle cx="12" cy="8.4" r="3" fill="none" stroke="currentColor" strokeWidth="1.7" /><path d="M6.5 18c1.4-2.6 3.3-3.9 5.5-3.9s4.1 1.3 5.5 3.9" fill="none" stroke="currentColor" strokeLinecap="round" strokeWidth="1.7" /></>,
    import: <path d="M12 4v10m0 0 3.4-3.4M12 14 8.6 10.6M5 18h14" fill="none" stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.7" />,
    settings: <path d="m12 4 1.1 1.9 2.2.4-.4 2.1 1.6 1.5-1.6 1.5.4 2.1-2.2.4L12 16l-1.1-1.9-2.2-.4.4-2.1L7.5 10l1.6-1.5-.4-2.1 2.2-.4L12 4Zm0 4.8a2.2 2.2 0 1 0 0 4.4 2.2 2.2 0 0 0 0-4.4Z" fill="none" stroke="currentColor" strokeLinejoin="round" strokeWidth="1.5" />,
  };
  return <Icon className="nav-icon-svg">{icons[name]}</Icon>;
});
