const groups = ['General', 'Library', 'Playback', 'Lyrics', 'Audio Quality', 'Search', 'Appearance', 'Accessibility', 'Shortcuts', 'About']

export function SettingsPreview() {
  return (
    <section className="km-settings" id="settings">
      <p>Settings Foundation</p>
      <h2>Grouped for long-term player, library, lyrics, quality, and accessibility control.</h2>
      <div>
        {groups.map((group) => <span key={group}>{group}</span>)}
      </div>
    </section>
  )
}
