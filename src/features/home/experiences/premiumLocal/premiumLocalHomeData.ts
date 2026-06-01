export type PremiumLocalFeature = {
  title: string
  description: string
}

export type PremiumLocalShelfItem = {
  id: string
  title: string
  subtitle: string
  meta: string
}

export const premiumLocalFeatures: PremiumLocalFeature[] = [
  {
    title: 'Native-First Playback Core',
    description:
      '专注本地音频链路，围绕稳定、低延迟和可验证边界推进自研核心。',
  },
  {
    title: 'Strict Local Library',
    description:
      '不接外部资源搜索与下载能力，只服务你自己的本地音乐资产管理。',
  },
  {
    title: 'Diagnostic-Driven Delivery',
    description: '每个阶段都提供模型与测试快照，确保可回溯、可维护、可持续演进。',
  },
]

export const premiumLocalShelves: PremiumLocalShelfItem[] = [
  {
    id: 'recent-import',
    title: '最近导入',
    subtitle: '本地扫描结果',
    meta: '126 首 · 14 张专辑',
  },
  {
    id: 'focus-work',
    title: '专注工作流',
    subtitle: '高保真本地播放',
    meta: '42 首 · 6 小时',
  },
  {
    id: 'night-drive',
    title: '夜间驾驶',
    subtitle: '离线收藏',
    meta: '58 首 · 4 小时 20 分',
  },
]

