// src/components/common/KivoButton.tsx
import React from "react";
import { useKivoTheme } from "../../styles/ThemeContext";

type ButtonVariant = "primary" | "secondary" | "ghost" | "danger";
type ButtonSize = "sm" | "md";

export interface KivoButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: ButtonVariant;
  size?: ButtonSize;
  iconLeft?: React.ReactNode;
  iconRight?: React.ReactNode;
}

/**
 * 通用按钮组件
 *
 * - variant: primary / secondary / ghost / danger
 * - size: sm / md
 * - iconLeft / iconRight: 左右图标插槽
 *
 * 这里刻意只使用 backgroundColor，不使用 background 简写，
 * 以避免 React 关于 background/backgroundColor 的警告。
 */
export const KivoButton: React.FC<KivoButtonProps> = ({
  variant = "primary",
  size = "md",
  iconLeft,
  iconRight,
  children,
  style,
  onMouseEnter,
  onMouseLeave,
  ...rest
}) => {
  const { theme } = useKivoTheme();
  const { colors, radius, spacing } = theme;

  const paddingX = size === "sm" ? spacing.md : spacing.lg;
  const paddingY = size === "sm" ? spacing.xs : spacing.sm;
  const fontSize = size === "sm" ? 12 : 14;

  let baseBg: string;
  let hoverBg: string;
  let borderColor: string;
  let hoverBorderColor: string;
  let textColor: string;

  switch (variant) {
    case "primary":
      baseBg = colors.primary;
      hoverBg = "#1d4ed8";
      borderColor = colors.primary;
      hoverBorderColor = "#1d4ed8";
      textColor = colors.textOnPrimary;
      break;
    case "secondary":
      baseBg = "rgba(15, 23, 42, 0.25)";
      hoverBg = "rgba(15, 23, 42, 0.5)";
      borderColor = colors.borderSubtle;
      hoverBorderColor = colors.borderStrong;
      textColor = colors.textOnDark;
      break;
    case "ghost":
      baseBg = "transparent";
      hoverBg = "rgba(15, 23, 42, 0.06)";
      borderColor = "transparent";
      hoverBorderColor = "rgba(148, 163, 184, 0.5)";
      textColor = colors.textOnDark;
      break;
    case "danger":
      baseBg = "rgba(248, 113, 113, 0.10)";
      hoverBg = "rgba(248, 113, 113, 0.18)";
      borderColor = colors.danger;
      hoverBorderColor = colors.danger;
      textColor = colors.danger;
      break;
    default:
      baseBg = colors.primary;
      hoverBg = "#1d4ed8";
      borderColor = colors.primary;
      hoverBorderColor = "#1d4ed8";
      textColor = colors.textOnPrimary;
  }

  const [isHover, setIsHover] = React.useState(false);

  const resolvedBg = !rest.disabled && isHover ? hoverBg : baseBg;
  const resolvedBorder =
    !rest.disabled && isHover ? hoverBorderColor : borderColor;

  const handleMouseEnter: React.MouseEventHandler<HTMLButtonElement> = (e) => {
    setIsHover(true);
    onMouseEnter?.(e);
  };

  const handleMouseLeave: React.MouseEventHandler<HTMLButtonElement> = (e) => {
    setIsHover(false);
    onMouseLeave?.(e);
  };

  return (
    <button
      {...rest}
      type={rest.type ?? "button"}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      style={{
        padding: `${paddingY}px ${paddingX}px`,
        borderWidth: 1,
        borderStyle: "solid",
        borderColor: resolvedBorder,
        borderRadius: radius.pill,
        backgroundColor: resolvedBg, // ✅ 只用 backgroundColor
        color: textColor,
        fontSize,
        fontWeight: 500,
        display: "inline-flex",
        alignItems: "center",
        gap: 6,
        cursor: rest.disabled ? "not-allowed" : "pointer",
        opacity: rest.disabled ? 0.5 : 1,
        whiteSpace: "nowrap",
        transition:
          "background-color 120ms ease-out, border-color 120ms ease-out, transform 80ms ease-out",
        transform: isHover && !rest.disabled ? "translateY(-0.5px)" : "none",
        ...style,
      }}
    >
      {iconLeft && <span style={{ display: "inline-flex" }}>{iconLeft}</span>}
      <span>{children}</span>
      {iconRight && <span style={{ display: "inline-flex" }}>{iconRight}</span>}
    </button>
  );
};

export default KivoButton;
