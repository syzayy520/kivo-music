// src/utils/log.ts

export type LogLevel = "debug" | "info" | "warn" | "error";

// 默认先开到 debug，后面可以在设置里加一个 UI 开关来调整日志等级
let currentLevel: LogLevel = "debug";

const levelWeight: Record<LogLevel, number> = {
  debug: 10,
  info: 20,
  warn: 30,
  error: 40,
};

export function setLogLevel(level: LogLevel) {
  currentLevel = level;
}

function shouldLog(level: LogLevel): boolean {
  return levelWeight[level] >= levelWeight[currentLevel];
}

function formatPrefix(module: string, level: LogLevel): string {
  return `[Kivo][${module}][${level.toUpperCase()}]`;
}

export const log = {
  debug(module: string, ...args: unknown[]) {
    if (!shouldLog("debug")) return;
    console.debug(formatPrefix(module, "debug"), ...args);
  },

  info(module: string, ...args: unknown[]) {
    if (!shouldLog("info")) return;
    console.info(formatPrefix(module, "info"), ...args);
  },

  warn(module: string, ...args: unknown[]) {
    if (!shouldLog("warn")) return;
    console.warn(formatPrefix(module, "warn"), ...args);
  },

  error(module: string, ...args: unknown[]) {
    if (!shouldLog("error")) return;
    console.error(formatPrefix(module, "error"), ...args);
  },
};
