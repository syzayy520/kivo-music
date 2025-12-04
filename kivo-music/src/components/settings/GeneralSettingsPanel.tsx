import React from "react";

export const GeneralSettingsPanel: React.FC = () => {
  return (
    <div
      style={{
        padding: 16,
        display: "flex",
        flexDirection: "column",
        gap: 12,
      }}
    >
      <h2 style={{ margin: 0, fontSize: 18 }}>常规设置</h2>
      <p style={{ margin: "4px 0 0", color: "#888", fontSize: 13 }}>
        目前还没有专门的常规设置项。以后可以把启动行为、界面语言、主题等搬到这里。
      </p>
    </div>
  );
};
