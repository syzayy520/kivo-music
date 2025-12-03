// src/pages/SettingsPage.tsx
import React, { useEffect, useState } from 'react';
import { open } from '@tauri-apps/plugin-dialog';
import {
  clearCoverCache,
  getCoverCacheStats,
  migrateCoverCache,
  CoverCacheStats,
} from '../persistence/CoverCache';
import {
  getEffectiveCoverCacheDir,
  loadSettings,
  saveSettings,
  KivoSettings,
} from '../persistence/SettingsPersistence';

const Section: React.FC<{ title: string; children: React.ReactNode }> = ({
  title,
  children,
}) => (
  <section style={{ marginBottom: 24 }}>
    <h2 style={{ fontSize: 18, marginBottom: 8 }}>{title}</h2>
    <div
      style={{
        padding: 12,
        borderRadius: 8,
        border: '1px solid rgba(255,255,255,0.1)',
        backgroundColor: 'rgba(0,0,0,0.15)',
      }}
    >
      {children}
    </div>
  </section>
);

export default function SettingsPage() {
  const [settings, setSettings] = useState<KivoSettings>({});
  const [effectiveDir, setEffectiveDir] = useState<string>('');
  const [stats, setStats] = useState<CoverCacheStats | null>(null);
  const [busy, setBusy] = useState(false);

  async function refreshAll() {
    const [s, dir, stat] = await Promise.all([
      loadSettings(),
      getEffectiveCoverCacheDir(),
      getCoverCacheStats(),
    ]);
    setSettings(s);
    setEffectiveDir(dir);
    setStats(stat);
  }

  useEffect(() => {
    void refreshAll();
  }, []);

  async function handleChooseCoverDir() {
    if (busy) return;
    setBusy(true);

    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: '选择封面缓存目录',
      });

      let picked: string | null = null;
      if (typeof selected === 'string') {
        picked = selected;
      } else if (Array.isArray(selected) && selected.length > 0) {
        picked = selected[0] as string;
      }

      if (!picked) return;

      const oldDir = await getEffectiveCoverCacheDir();
      if (picked === oldDir) {
        return;
      }

      const shouldMigrate = window.confirm(
        [
          '检测到你修改了封面缓存目录。',
          '',
          '是否将当前封面缓存一起迁移到新目录？',
          '',
          '推荐选择【是】，这样已经设置好的封面不会丢失；',
          '选择【否】则从空缓存开始，新封面会写到新目录。',
        ].join('\n'),
      );

      // 先更新设置，让 getEffectiveCoverCacheDir 指向新目录
      const newSettings: KivoSettings = {
        ...settings,
        coverCacheDir: picked,
      };
      await saveSettings(newSettings);
      setSettings(newSettings);
      setEffectiveDir(picked);

      if (shouldMigrate) {
        await migrateCoverCache(oldDir, picked);
      }

      const newStats = await getCoverCacheStats();
      setStats(newStats);
    } catch (error) {
      console.error('[SettingsPage] 选择封面目录失败:', error);
      alert('选择封面缓存目录时发生错误，请检查控制台日志。');
    } finally {
      setBusy(false);
    }
  }

  async function handleClearCache() {
    if (busy) return;

    const ok = window.confirm(
      [
        '确定要清空所有封面缓存吗？',
        '',
        '这不会删除你的音乐或 kivo-library.json，',
        '只是删掉封面缓存，下次会重新按规则加载 / 选择封面。',
      ].join('\n'),
    );

    if (!ok) return;

    setBusy(true);
    try {
      await clearCoverCache();
      const newStats = await getCoverCacheStats();
      setStats(newStats);
    } catch (error) {
      console.error('[SettingsPage] 清空封面缓存失败:', error);
      alert('清空封面缓存失败，请检查控制台日志。');
    } finally {
      setBusy(false);
    }
  }

  return (
    <div
      style={{
        padding: 16,
        display: 'flex',
        flexDirection: 'column',
        gap: 16,
        color: '#fff',
      }}
    >
      <h1 style={{ fontSize: 22, marginBottom: 8 }}>设置</h1>

      <Section title="封面缓存目录">
        <div
          style={{
            display: 'flex',
            flexDirection: 'column',
            gap: 8,
          }}
        >
          <div>
            <div style={{ opacity: 0.8, fontSize: 13, marginBottom: 4 }}>
              当前有效目录（实际正在使用的目录）：
            </div>
            <code
              style={{
                fontSize: 13,
                padding: 6,
                borderRadius: 4,
                backgroundColor: 'rgba(0,0,0,0.3)',
                wordBreak: 'break-all',
              }}
            >
              {effectiveDir || '加载中…'}
            </code>
          </div>

          <button
            type="button"
            disabled={busy}
            onClick={handleChooseCoverDir}
            style={{
              alignSelf: 'flex-start',
              marginTop: 4,
              padding: '6px 12px',
              borderRadius: 6,
              border: 'none',
              cursor: busy ? 'default' : 'pointer',
              backgroundColor: busy ? '#666' : '#2b7cff',
              color: '#fff',
            }}
          >
            {busy ? '处理中…' : '选择封面缓存目录…'}
          </button>

          <div style={{ opacity: 0.7, fontSize: 12, marginTop: 4 }}>
            默认会使用系统 AppData 下的
            <code style={{ marginLeft: 4, marginRight: 4 }}>
              /com.administrator.kivo-music/covers
            </code>
            ，你可以把缓存迁移到 D/E 盘等更大的磁盘。
          </div>
        </div>
      </Section>

      <Section title="封面缓存统计 & 管理">
        {stats ? (
          <div
            style={{
              display: 'flex',
              flexDirection: 'column',
              gap: 8,
              fontSize: 13,
            }}
          >
            <div>缓存文件数：{stats.fileCount}</div>
            <div>缓存总大小：{stats.humanReadableSize}</div>
            <div>Track 封面索引条数：{stats.trackEntries}</div>
            <div>文件夹封面索引条数：{stats.folderEntries}</div>

            <button
              type="button"
              disabled={busy}
              onClick={handleClearCache}
              style={{
                alignSelf: 'flex-start',
                marginTop: 8,
                padding: '6px 12px',
                borderRadius: 6,
                border: 'none',
                cursor: busy ? 'default' : 'pointer',
                backgroundColor: busy ? '#666' : '#c0392b',
                color: '#fff',
              }}
            >
              {busy ? '处理中…' : '清空封面缓存'}
            </button>
          </div>
        ) : (
          <div style={{ fontSize: 13 }}>正在加载缓存统计信息…</div>
        )}
      </Section>

      <Section title="调试说明（给后来接手的人看的小注释）">
        <ul style={{ margin: 0, paddingLeft: 18, fontSize: 12, opacity: 0.8 }}>
          <li>
            封面缓存目录和设置保存在{' '}
            <code>AppData/com.administrator.kivo-music</code> 下面；
          </li>
          <li>
            <code>covers.json</code> 记录“track ➜ 封面缓存文件路径”；
          </li>
          <li>
            <code>folder-covers.json</code>{' '}
            记录“文件夹 ➜ 封面扫描结果（包括没有封面）”，
            用来避免对不存在的 <code>cover.jpg</code> 重复发请求导致一堆
            500；
          </li>
          <li>
            播放页 / 列表页只要改用{' '}
            <code>resolveCoverPathForTrack(track)</code>{' '}
            拿封面，就可以统一走这套缓存逻辑。
          </li>
        </ul>
      </Section>
    </div>
  );
}
