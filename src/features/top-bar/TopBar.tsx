export function TopBar() {
  return (
    <header className="km-topbar">
      <div>
        <p>Listen Now</p>
        <h1>Your local library, beautifully alive.</h1>
      </div>
      <label className="km-search">
        <span>Search</span>
        <input aria-label="Search library" placeholder="Albums, artists, lyrics, quality" />
      </label>
      <div className="km-status">
        <small>Local Library</small>
        <strong>Lossless Ready</strong>
      </div>
    </header>
  )
}
