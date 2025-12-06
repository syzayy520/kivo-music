// src/i18n/locales/zh-CN.ts
/**
 * 简体中文文案。
 */
export const zhCNMessages: Record<string, string> = {
  // Mini 播放器
  "miniPlayer.button.exit": "返回完整模式",
  "miniPlayer.cover.placeholder": "暂无封面",
  "miniPlayer.header.subtitle.hasTrack": "当前曲目",
  "miniPlayer.header.subtitle.noTrack": "暂无曲目",
  "miniPlayer.header.title": "Kivo Music · Mini",
  "miniPlayer.volume.label": "音量",

  // 导航
  "nav.library.label": "资料库",
  "nav.library.description": "浏览本地音乐、按歌曲/专辑/艺人查看",
  "nav.playlist.label": "播放列表 & 智能列表",
  "nav.playlist.description": "管理播放列表和智能列表，基于播放行为自动更新。",
  "nav.nowPlaying.label": "正在播放",
  "nav.nowPlaying.description": "聚焦当前播放内容视图。",
  "nav.settings.label": "设置",
  "nav.settings.description": "封面缓存、日志、实验功能",

  // Library · Header
  "library.header.title": "本地音乐资料库",
  "library.header.subtitle.line1": "按照本地曲库整理你的音乐。",
  "library.header.subtitle.line2":
    "支持按歌曲 / 专辑 / 艺人视图切换，支持搜索、排序和播放队列。",
  "library.header.viewMode.tracks": "按歌曲",
  "library.header.viewMode.albums": "按专辑",
  "library.header.viewMode.artists": "按艺人",
  "library.header.searchPlaceholder": "搜索歌曲 / 艺人 / 专辑 / 路径…",
  "library.header.sort.default": "默认顺序",
  "library.header.sort.title": "标题",
  "library.header.sort.artist": "艺人",
  "library.header.sort.album": "专辑",
  "library.header.sort.recent": "最近播放",
  "library.header.actions.playAll": "播放全部",
  "library.header.actions.shufflePlay": "随机播放",
  "library.header.actions.importLocalMusic": "+ 导入本地音乐",
  "library.header.actions.clearLibrary": "清空资料库",
  "library.header.extra.totalTracks": "共 {count} 首歌曲",

  // Library · 排序条
  "library.sortBar.option.none": "默认顺序",
  "library.sortBar.option.title": "标题",
  "library.sortBar.option.artist": "艺人",
  "library.sortBar.hint.doubleClickToPlay":
    "双击任意一行即可播放，使用当前筛选结果作为播放队列。",

  // Playlists · Header
  "playlists.header.title": "播放列表 & 智能列表",
  "playlists.header.description":
    "当前共有 {count} 首歌曲，支持根据播放队列、最近播放、常常播放等维度查看。",
  "playlists.header.smartList.hint":
    "智能列表会根据你的播放行为自动更新，无需手动维护。",
  "playlists.header.nowPlaying.badge": "正在播放",
  "playlists.header.nowPlaying.title.fallback": "当前没有正在播放的歌曲",
  "playlists.header.nowPlaying.artist.fallback": "未知艺人",
  "playlists.header.clearQueueButton": "清空当前播放队列",

  // Settings · General
  "settings.general.title": "通用",
  "settings.general.theme.cardTitle": "主题",
  "settings.general.theme.cardDescription":
    "在浅色 / 深色 / 跟随系统之间切换。主题仅影响界面外观，不会影响播放行为。",
  "settings.general.theme.system": "跟随系统",
  "settings.general.theme.light": "浅色",
  "settings.general.theme.dark": "深色",
  "settings.general.language.cardTitle": "界面语言",
  "settings.general.language.cardDescription":
    "默认跟随系统语言。你也可以手动指定语言，用于多国语言体验测试。",
  "settings.general.language.system": "跟随系统",
  "settings.general.language.zhCN": "简体中文",
  "settings.general.language.enUS": "English（英文）",

  // Now Playing · Header
  "nowPlaying.header.title": "正在播放",
  "nowPlaying.header.subtitle.line1":
    "聚焦当前播放视图，显示大尺寸封面、曲目信息和播放队列。",
  "nowPlaying.header.subtitle.line2":
    "当前播放列表来源于你的操作（资料库 / 播放列表 / 智能列表等）。",
  "nowPlaying.header.queueEmpty":
    "当前播放队列为空。可以在“资料库”或“播放列表”中选择歌曲开始播放。",
  "nowPlaying.header.note":
    "本页面只负责展示，不再额外创建音频元素，所有播放行为仍由底部的播放器控制条统一管理。",

  // 正在播放 · 信息面板
  "nowPlaying.info.fallbackTitle": "未选择曲目",
  "nowPlaying.info.fallbackArtist": "未知艺人",
  "nowPlaying.info.description.line1":
    "当前页面与底部播放器、Mini 模式共享同一播放器状态。",
  "nowPlaying.info.description.line2":
    "在「资料库 / 播放列表」中操作播放/切歌，都会实时反映到这里。",
  "nowPlaying.info.statusLabel": "状态：",
  "nowPlaying.info.status.playing": "正在播放",
  "nowPlaying.info.status.paused": "已暂停",
  "nowPlaying.info.status.noTrack": "未选择曲目",

  // 正在播放 · 可视化面板（示例）
  "nowPlaying.visualizer.title": "音频可视化",
  "nowPlaying.visualizer.status.enabled": "已开启",
  "nowPlaying.visualizer.status.disabled": "已关闭",

  // 播放器 · 底部控制条
  "player.bar.fallbackTitle": "未选择曲目",
  "player.bar.fallbackArtist": "未知艺人",
  "player.bar.tooltip.previous": "上一首",
  "player.bar.tooltip.play": "播放",
  "player.bar.tooltip.pause": "暂停",
  "player.bar.tooltip.next": "下一首",
  "player.bar.volume.label": "音量",

  // 侧边栏 & Footer
  "sidebar.section.title": "LIBRARY & PLAYBACK",
  "sidebar.kivo.tagline.line1": "Kivo Music",
  "sidebar.kivo.tagline.line2": "本地 · 高性能 · Apple 灵感",
  "sidebar.nav.library.title": "Library",
  "sidebar.nav.library.description":
    "浏览本地音乐，按歌曲 / 专辑 / 艺人查看。",
  "sidebar.nav.playlists.title": "Playlists",
  "sidebar.nav.playlists.description":
    "播放列表与智能列表，按播放行为自动更新。",
  "sidebar.nav.nowPlaying.title": "Now Playing",
  "sidebar.nav.nowPlaying.description":
    "大封面视图和详细信息展示。",
  "sidebar.nav.settings.title": "Settings",
  "sidebar.nav.settings.description":
    "封面缓存、日志和实验功能。",
  "sidebar.miniPlayer.badge.experimental": "实验中",
  "sidebar.miniPlayer.title": "Mini 播放器",
  "sidebar.section.libraryAndPlayback": "资料库 & 播放",

  // Library · 分组视图（专辑 / 艺人）
  "library.groups.unknownArtistLabel": "未知艺人",
  "library.groups.albums.empty": "当前筛选条件下没有专辑。",
  "library.groups.albums.card.trackCount": "{count} 首歌曲",
  "library.groups.albums.card.favoriteCount": " · {count} 首已标记喜欢",
  "library.groups.artists.empty": "当前筛选条件下没有艺人。",
  "library.groups.artists.header.artist": "艺人",
  "library.groups.artists.header.trackCount": "歌曲数量",
  "library.groups.artists.header.albumCount": "专辑数量",

  // Library · 按歌曲视图（虚拟列表）
    "library.tracks.fallbackTitle": "未知标题",
  "library.tracks.fallbackArtist": "未知艺人",
  "library.tracks.fallbackAlbum": "未分专辑",

  "library.tracks.favorite.tooltip.on": "取消喜欢",
  "library.tracks.favorite.tooltip.off": "标记为喜欢",

  "library.tracks.emptyMessage":
    "当前资料库为空，请先导入本地音乐文件。",

  // Library · 曲目右键菜单
  "library.contextMenu.play": "播放此歌曲",
  "library.contextMenu.playNext": "设为下一首播放",
  "library.contextMenu.appendToQueue": "添加到当前队列尾部",
  "library.contextMenu.openInFolder": "打开文件所在文件夹",

  // Library · 表头（资料库表格）
  "library.table.header.title": "标题",
  "library.table.header.artist": "艺人",
  "library.table.header.album": "专辑",
  "library.table.header.duration": "时长",
  "library.table.header.playCount": "播放次数",
  "library.table.header.lastPlayed": "最近播放",
  "library.table.header.actions": "操作",

  // Playlists · 标签（顶部选项卡）
  "playlist.tabs.queue": "当前队列",
  "playlist.tabs.recentlyAdded": "最近添加",
  "playlist.tabs.recentlyPlayed": "最近播放",
  "playlist.tabs.mostPlayed": "常常播放",
  "playlist.tabs.favorites": "喜欢的歌曲",
    // 兼容旧写法：playlist.table.actions.*
  "playlist.table.actions.playNext": "设为下一首播放",
  "playlist.table.actions.appendToQueue": "加入队列",


  // 侧边栏 · Mini 播放器卡片
  "sidebar.miniPlayer.description": "Mini 模式：更紧凑的小窗播放器。",

  // 导航 · Playlists 复数形式（预留）
  "nav.playlists.label": "播放列表",
  "nav.playlists.description":
    "播放列表与智能列表，基于播放行为自动更新。",

  // Playlists · 表格视图（队列 / 智能列表通用）
  "playlists.table.header.title": "标题",
  "playlists.table.header.artist": "艺人",
  "playlists.table.header.album": "专辑",
  "playlists.table.header.playCount": "播放次数",
  "playlists.table.header.lastPlayed": "最近播放",
  "playlists.table.header.favorite": "喜欢",
  "playlists.table.header.actions": "操作",

  "playlists.table.row.action.playNext": "设为下一首播放",
  "playlists.table.row.action.appendToQueue": "添加到当前队列尾部",

  "playlists.table.empty.currentTab": "当前列表暂无歌曲",

  // Settings · 页面骨架 & Tab
  "settings.page.title": "设置",
  "settings.page.sidebar.sectionLabel": "设置分类",

  "settings.tabs.general": "常规",
  "settings.tabs.coverCache": "封面缓存",
  "settings.tabs.developer": "开发者",

  // Settings · General · 主题
  "settings.general.theme.title": "主题",
  "settings.general.theme.subtitle":
    "在浅色 / 深色 / 跟随系统之间切换。主题仅影响界面外观，不会影响播放行为。",
  "settings.general.theme.currentLabel": "当前主题：",
  "settings.general.theme.description":
    "切换主题不会影响播放行为，仅影响界面外观。",
  "settings.general.theme.currentValue.default": "Kivo 默认主题",

  // Settings · General · 语言
  "settings.general.language.title": "界面语言",
  "settings.general.language.subtitle":
    "默认跟随系统语言，你也可以手动指定语言，用于多国语言体验测试。",
  "settings.general.language.currentLabel": "当前语言：",
  "settings.general.language.note":
    "语言切换会立即生效。大部分界面文案已接入多语言配置，少量实验功能仍在逐步补全。",

  // Settings · 封面缓存 · 目录信息
  "settings.cache.directory.title": "封面缓存目录",
  "settings.cache.directory.currentLabel":
    "当前有效目录（实际正在使用的目录）：",
  "settings.cache.directory.loading": "加载中…",
  "settings.cache.directory.chooseButton": "选择封面缓存目录…",
  "settings.cache.directory.help":
    "默认会使用系统 AppData 下的封面缓存目录，你可以把缓存迁移到 D/E 盘等更大的磁盘。当前默认路径：",

  // Settings · 封面缓存 · 统计与操作
  "settings.cache.stats.title": "封面缓存统计与管理",
  "settings.cache.stats.loading": "正在加载缓存统计信息…",
  "settings.cache.stats.fileCountLabel": "缓存文件数：",
  "settings.cache.stats.sizeLabel": "缓存总大小：",
  "settings.cache.stats.trackEntriesLabel": "Track 封面索引条数：",
  "settings.cache.stats.folderEntriesLabel": "文件夹封面索引条数：",
  "settings.cache.stats.action.processing": "正在处理…",
  "settings.cache.stats.action.refresh": "刷新统计",
  "settings.cache.stats.action.repairIndex": "修复封面索引",
  "settings.cache.stats.action.clearCache": "清空封面缓存",

  // Settings · 封面缓存 · 消息提示
  "settings.cache.message.refreshSuccess": "封面缓存统计已刷新。",
  "settings.cache.message.refreshFailure":
    "刷新封面缓存统计失败，请检查控制台日志。",
  "settings.cache.message.dirUnchanged": "封面缓存目录没有变化。",
  "settings.cache.message.dirUpdatedWithMigrate":
    "封面缓存目录已更新，并已尝试迁移旧缓存。",
  "settings.cache.message.dirUpdatedWithoutMigrate":
    "封面缓存目录已更新，但未迁移旧缓存。",
  "settings.cache.message.chooseDirError":
    "选择封面缓存目录时发生错误，请检查控制台日志。",
  "settings.cache.message.clearSuccess": "封面缓存已完全清空。",
  "settings.cache.message.clearFailure":
    "清空封面缓存失败，请检查控制台日志。",
  "settings.cache.message.repairCompleted": "封面索引自检完成：",
  "settings.cache.message.repairTrackSummary":
    "Track 封面索引检查 {coverChecked} 条，移除 {coverRemoved} 条失效记录；",
  "settings.cache.message.repairFolderSummary":
    "文件夹封面索引检查 {folderChecked} 条，移除 {folderRemoved} 条失效记录。",
  "settings.cache.message.repairFailure":
    "修复封面索引失败，请检查控制台日志。",

  // Settings · 封面缓存 · 各类确认弹窗
  "settings.cache.dialog.chooseDirTitle": "选择封面缓存目录",
  "settings.cache.dialog.changeDirConfirm":
    "检测到你修改了封面缓存目录。\n\n是否将当前封面缓存一起迁移到新目录？\n\n推荐选择【是】，这样已经设置好的封面不会丢失；\n选择【否】则从空缓存开始，新封面会写到新目录。",
  "settings.cache.dialog.clearConfirm":
    "确定要清空所有封面缓存吗？\n\n这不会删除你的音乐或 kivo-library.json，\n只是删掉封面缓存，下次会重新按规则加载 / 选择封面。",
  "settings.cache.dialog.repairConfirm":
    "此操作会扫描封面索引（covers.json / folder-covers.json），并自动移除那些指向“已经不存在的文件”的记录。\n\n不会删除任何新的封面文件，也不会影响音乐库，只是做索引自检 / 清理。\n\n确定要继续吗？",

  // Settings · 封面缓存 · 给后来接手人的调试说明
  "settings.cache.debug.title": "调试说明（给后来接手的人看的小注释）",
  "settings.cache.debug.item1.prefix": "封面缓存目录和设置保存在 ",
  "settings.cache.debug.item1.suffix": " 下面；",
  "settings.cache.debug.item2": " 记录“track ➜ 封面缓存文件路径”；",
  "settings.cache.debug.item3.prefix":
    " 记录“文件夹 ➜ 封面扫描结果（包括没有封面）”，用来避免对不存在的 ",
  "settings.cache.debug.item3.suffix": " 重复发请求；",
  "settings.cache.debug.item4.prefix": "播放页 / 列表页只要改用 ",
  "settings.cache.debug.item4.suffix":
    " 拿封面，就可以统一走这套缓存逻辑。",
      // Sidebar · 品牌 & 页脚
  "sidebar.brand.tagline":
    "为仍然保留本地音乐文件的人，做一款好用的播放器。",
  "sidebar.footer.line1":
    "Kivo 仍在积极开发中，欢迎提出任何反馈和建议。",
  "sidebar.footer.line2":
    "当前版本主要聚焦本地播放和轻量级统计功能。",

  // Playlists · 头部旧 key（兼容早期写法）
  "playlists.header.smartListTip":
    "智能列表会根据你的播放行为自动更新，无需手动维护。",
  "playlists.header.current.badgeNowPlaying": "正在播放",
  "playlists.header.actions.clearQueue": "清空当前播放队列",

  // Settings · 开发者设置面板
  "settings.developer.title": "开发者设置（预留）",
  "settings.developer.description.line1":
    "这些选项当前只会保存在本机 localStorage 中，暂时不会直接影响播放内核。",
  "settings.developer.description.line2":
    "以后可以在此基础上接入 log.ts、调试信息 Overlay、频谱可视化等功能。",

  "settings.developer.logLevel.label": "日志等级（预留）",
  "settings.developer.logLevel.option.debug": "Debug（最详细）",
  "settings.developer.logLevel.option.info": "Info（默认）",
  "settings.developer.logLevel.option.warn": "Warn（仅警告+错误）",
  "settings.developer.logLevel.option.error": "Error（仅错误）",
  "settings.developer.logLevel.note":
    "将来可以连接 log.ts 控制全局日志输出。",

  "settings.developer.overlay.label":
    "显示调试信息 Overlay（预留）",
  "settings.developer.overlay.note":
    "将来可以在界面上显示当前播放状态、内存占用等实时调试信息。",

  "settings.developer.visualizer.label":
    "启用 Web Audio 频谱可视化（实验性）",
  "settings.developer.visualizer.note":
    "开启后，“正在播放”页面会显示一个简单的频谱条形图，可能会略微增加 CPU 占用。",

};
