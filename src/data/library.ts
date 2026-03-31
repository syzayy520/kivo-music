export interface NavItem {
  icon: string;
  label: string;
}

export interface Track {
  id: string;
  title: string;
  artist: string;
  album: string;
  duration: string;
  path?: string;
  tone?: string;
}

export interface AlbumCard {
  title: string;
  artist: string;
  tone?: string;
}

export interface MixCard {
  title: string;
  meta: string;
  tone?: string;
}

export const browseItems: NavItem[] = [
  { icon: "home", label: "\u9996\u9875" },
  { icon: "clock", label: "\u6700\u8fd1\u64ad\u653e" },
  { icon: "heart", label: "\u6211\u559c\u6b22\u7684\u97f3\u4e50" },
  { icon: "queue", label: "\u64ad\u653e\u5217\u8868" },
];

export const libraryItems: NavItem[] = [
  { icon: "disc", label: "\u4e13\u8f91" },
  { icon: "artist", label: "\u827a\u672f\u5bb6" },
  { icon: "import", label: "\u672c\u5730\u5bfc\u5165" },
  { icon: "settings", label: "\u8bbe\u7f6e" },
];

export const queue: Track[] = [
  { id: "late-night-walk", title: "Late Night Walk", artist: "Luna Harbor", album: "Quiet Tapes Vol.2", duration: "3:57", tone: "linear-gradient(135deg,#d9c3ae 0%,#efe4d7 44%,#a57d5f 100%)" },
  { id: "last-color", title: "When the Windows Hold the Last Color of Sunset", artist: "June Vale", album: "Last Color of Sunset", duration: "4:12", tone: "linear-gradient(135deg,#dfc9b7 0%,#ead7ca 42%,#8a654c 100%)" },
  { id: "soft-corners", title: "Soft Corners", artist: "Aster Field", album: "Soft Corners", duration: "3:21", tone: "linear-gradient(135deg,#d8c1aa 0%,#f1e5d7 46%,#9d7456 100%)" },
];

export const recentAlbums: AlbumCard[] = [
  { title: "All the Small Warm Lights in the City at Dusk", artist: "North Avenue", tone: "linear-gradient(145deg,#e8d7c7,#b99576 58%,#8b6349)" },
  { title: "Paper Moon", artist: "Haru Sato", tone: "linear-gradient(145deg,#ead9ca,#9a765c 60%,#76523e)" },
  { title: "Cloud Notes", artist: "Aster Field", tone: "linear-gradient(145deg,#e5d1bf,#8b694f 60%,#694a37)" },
  { title: "Slow Cinema", artist: "Evening Motel", tone: "linear-gradient(145deg,#ebd8c8,#9f7b60 60%,#7a5944)" },
];

export const frequentMixes: MixCard[] = [
  { title: "\u6668\u95f4\u8f7b\u76c8\u901a\u52e4\u7cbe\u9009", meta: "24 \u9996\u6b4c \u00b7 \u67d4\u548c\u8282\u594f", tone: "linear-gradient(135deg,#e7d3c0,#ab8666)" },
  { title: "\u4f4e\u9971\u548c\u591c\u665a\u6c1b\u56f4", meta: "18 \u9996\u6b4c \u00b7 \u5b89\u9759\u4e0d\u62a2\u8033", tone: "linear-gradient(135deg,#ead6c4,#957055)" },
  { title: "\u4e0b\u96e8\u5929\u7684\u6728\u5409\u4ed6", meta: "31 \u9996\u6b4c \u00b7 \u6e29\u548c\u8d28\u611f", tone: "linear-gradient(135deg,#ddc6b3,#805d45)" },
];
