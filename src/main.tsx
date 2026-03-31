import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles/globals.css";
import "./styles/shell.css";
import "./styles/sidebar.css";
import "./styles/sidebar-nav.css";
import "./styles/toolbar.css";
import "./styles/hero.css";
import "./styles/hero-meta.css";
import "./styles/library.css";
import "./styles/library-cards.css";
import "./styles/queue.css";
import "./styles/dock.css";
import "./styles/dock-controls.css";
import "./styles/range.css";
import "./styles/scrollbar.css";
import "./styles/search-results.css";
import "./styles/responsive.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
