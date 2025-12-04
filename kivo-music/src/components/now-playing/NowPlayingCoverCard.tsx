// src/components/now-playing/NowPlayingCoverCard.tsx
import React from "react";
import { useNowPlayingCover } from "../../hooks/useNowPlayingCover";

interface NowPlayingCoverCardProps {
  track: any | null;
  playlist: any[];
  currentIndex: number;
}

const boxStyle: React.CSSProperties = {
  borderRadius: 24,
  padding: 16,
  background: "#ffffff",
  boxShadow: "0 10px 30px rgba(15,23,42,0.08)",
  display: "flex",
  flexDirection: "column",
  gap: 12,
  height: "100%",
};

const buttonStyle: React.CSSProperties = {
  padding: "6px 10px",
  fontSize: 12,
  borderRadius: 9999,
  border: "1px solid #e5e7eb",
  background: "#111827",
  color: "#f9fafb",
  cursor: "pointer",
  whiteSpace: "nowrap",
};

const disabledButtonStyle: React.CSSProperties = {
  ...buttonStyle,
  background: "#6b7280",
  borderColor: "#9ca3af",
  cursor: "not-allowed",
};

const NowPlayingCoverCard: React.FC<NowPlayingCoverCardProps> = ({
  track,
  playlist,
  currentIndex,
}) => {
  const { coverSrc, isLoading, hasError, pickCover } = useNowPlayingCover(
    track,
    playlist,
    currentIndex,
  );

  const anyTrack = track as any;
  const title =
    anyTrack?.title ?? anyTrack?.name ?? anyTrack?.fileName ?? "未选择曲目";

  const canPickCover = !!track && !isLoading;

  return (
    <div style={boxStyle}>
      {/* 标题 + 按钮行 */}
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          gap: 8,
        }}
      >
        <div style={{ fontSize: 13, fontWeight: 600 }}>封面</div>
        <button
          type="button"
          style={canPickCover ? buttonStyle : disabledButtonStyle}
          disabled={!canPickCover}
          onClick={() => {
            void pickCover();
          }}
        >
          选择封面图片…
        </button>
      </div>

      {/* 封面区域 */}
      <div
        style={{
          flex: 1,
          minHeight: 0,
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
        }}
      >
        <div
          style={{
            width: "100%",
            maxWidth: 360,
            aspectRatio: "1 / 1",
            borderRadius: 24,
            overflow: "hidden",
            background: "#e5e7eb",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
          }}
        >
          {coverSrc ? (
            <img
              src={coverSrc}
              alt={title}
              style={{
                width: "100%",
                height: "100%",
                objectFit: "cover",
                display: "block",
              }}
            />
          ) : (
            <span style={{ fontSize: 12, color: "#6b7280" }}>
              当前曲目暂无封面
            </span>
          )}
        </div>
      </div>

      {/* 说明文案 */}
      <div style={{ fontSize: 11, color: "#6b7280", lineHeight: 1.5 }}>
        封面会从曲目自带路径或缓存目录加载。点击
        <strong>「选择封面图片…」</strong> 后：
        图片会复制到封面缓存目录，并记录在 <code>covers.json</code> 中。
        即使原始图片删除，只要封面缓存未清空，仍可恢复显示。
      </div>

      {/* 状态提示 */}
      {isLoading && (
        <div style={{ fontSize: 11, color: "#9ca3af" }}>正在处理封面…</div>
      )}
      {hasError && (
        <div style={{ fontSize: 11, color: "#b91c1c" }}>
          读取或设置封面时发生错误，请稍后再试。
        </div>
      )}
    </div>
  );
};

export default NowPlayingCoverCard;
