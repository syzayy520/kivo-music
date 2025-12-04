// src/components/layout/PageHeader.tsx
import React from "react";
import { kivoTheme } from "../../styles/theme";

export interface PageHeaderProps {
  title: string;
  subtitle?: React.ReactNode;
  leftBottomExtra?: React.ReactNode; // 例如“共 X 首歌曲”
  centerSlot?: React.ReactNode; // 搜索 + 视图切换
  rightSlot?: React.ReactNode; // 排序 + 按钮
}

export const PageHeader: React.FC<PageHeaderProps> = ({
  title,
  subtitle,
  leftBottomExtra,
  centerSlot,
  rightSlot,
}) => {
  const { colors, radius, spacing, shadow } = kivoTheme;

  return (
    <div
      style={{
        marginBottom: spacing.lg,
      }}
    >
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          backgroundImage: colors.headerBackground,
          borderRadius: radius.xl,
          padding: spacing.xl,
          boxShadow: shadow.card,
          borderWidth: 1,
          borderStyle: "solid",
          borderColor: colors.headerBorder,
          color: colors.textOnPrimary,
        }}
      >
        {/* 顶部：标题 + 右侧操作 */}
        <div
          style={{
            display: "flex",
            alignItems: "flex-start",
            justifyContent: "space-between",
            gap: spacing.lg,
          }}
        >
          {/* 标题区 */}
          <div style={{ flex: 1, minWidth: 0 }}>
            <div
              style={{
                display: "flex",
                alignItems: "baseline",
                gap: spacing.md,
                marginBottom: spacing.sm,
              }}
            >
              <h1
                style={{
                  fontSize: 24,
                  fontWeight: 700,
                  letterSpacing: 0.4,
                  margin: 0,
                }}
              >
                {title}
              </h1>
            </div>
            {subtitle && (
              <div
                style={{
                  fontSize: 13,
                  lineHeight: 1.5,
                  color: "rgba(241, 245, 249, 0.86)",
                  maxWidth: 600,
                }}
              >
                {subtitle}
              </div>
            )}
            {leftBottomExtra && (
              <div
                style={{
                  marginTop: spacing.sm,
                  fontSize: 12,
                  color: "rgba(226, 232, 240, 0.9)",
                }}
              >
                {leftBottomExtra}
              </div>
            )}
          </div>

          {/* 右侧：操作区（按钮等） */}
          {rightSlot && (
            <div
              style={{
                display: "flex",
                flexDirection: "column",
                alignItems: "flex-end",
                gap: spacing.sm,
                minWidth: 220,
              }}
            >
              {rightSlot}
            </div>
          )}
        </div>

        {/* 下层：中间区域，一般放搜索 + 视图切换 + 排序 */}
        {centerSlot && (
          <div
            style={{
              marginTop: spacing.lg,
              display: "flex",
              alignItems: "center",
              justifyContent: "space-between",
              gap: spacing.lg,
              flexWrap: "wrap",
            }}
          >
            {centerSlot}
          </div>
        )}
      </div>
    </div>
  );
};
