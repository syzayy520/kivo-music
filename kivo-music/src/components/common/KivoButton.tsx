// src/components/common/KivoButton.tsx
import React from "react";
import { kivoTheme } from "../../styles/theme";

type ButtonVariant = "primary" | "secondary" | "ghost" | "danger";
type ButtonSize = "sm" | "md";

export interface KivoButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: ButtonVariant;
  size?: ButtonSize;
  iconLeft?: React.ReactNode;
  iconRight?: React.ReactNode;
}

export const KivoButton: React.FC<KivoButtonProps> = ({
  variant = "primary",
  size = "md",
  iconLeft,
  iconRight,
  children,
  style,
  ...rest
}) => {
  const { colors, radius, spacing } = kivoTheme;

  const basePaddingX = size === "sm" ? spacing.md : spacing.lg;
  const basePaddingY = size === "sm" ? spacing.xs : spacing.sm;
  const fontSize = size === "sm" ? 12 : 14;

  let background: string | undefined;
  let borderColor: string | undefined;
  let color = colors.textOnPrimary;
  let hoverBackground: string | undefined;
  let hoverBorderColor: string | undefined;

  switch (variant) {
    case "primary":
      background = colors.primary;
      borderColor = "transparent";
      hoverBackground = "#1d4ed8";
      hoverBorderColor = borderColor;
      break;
    case "secondary":
      background = "rgba(15, 23, 42, 0.8)";
      borderColor = colors.borderSubtle;
      color = colors.textOnDark;
      hoverBackground = "rgba(15, 23, 42, 1)";
      hoverBorderColor = colors.borderStrong;
      break;
    case "ghost":
      background = "transparent";
      borderColor = colors.borderSubtle;
      color = colors.textOnDark;
      hoverBackground = "rgba(15, 23, 42, 0.5)";
      hoverBorderColor = colors.borderStrong;
      break;
    case "danger":
      background = "rgba(248, 113, 113, 0.1)";
      borderColor = colors.danger;
      color = colors.danger;
      hoverBackground = "rgba(248, 113, 113, 0.2)";
      hoverBorderColor = colors.danger;
      break;
  }

  const [isHover, setIsHover] = React.useState(false);

  const resolvedBackground = isHover && hoverBackground ? hoverBackground : background;
  const resolvedBorderColor = isHover && hoverBorderColor ? hoverBorderColor : borderColor;

  return (
    <button
      {...rest}
      onMouseEnter={(e) => {
        setIsHover(true);
        rest.onMouseEnter?.(e);
      }}
      onMouseLeave={(e) => {
        setIsHover(false);
        rest.onMouseLeave?.(e);
      }}
      style={{
        display: "inline-flex",
        alignItems: "center",
        justifyContent: "center",
        paddingLeft: basePaddingX,
        paddingRight: basePaddingX,
        paddingTop: basePaddingY,
        paddingBottom: basePaddingY,
        gap: 6,
        fontSize,
        fontWeight: 500,
        borderWidth: 1,
        borderStyle: "solid",
        borderColor: resolvedBorderColor,
        borderRadius: radius.pill,
        background: resolvedBackground,
        color,
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
