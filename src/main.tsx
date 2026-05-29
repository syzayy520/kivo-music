import React from 'react'
import { createRoot } from 'react-dom/client'
import { App } from './app/App'
import './shared/styles/tokens.css'
import './shared/styles/app.css'

const rootElement = document.getElementById('root')

if (rootElement) {
  createRoot(rootElement).render(
    <React.StrictMode>
      <App />
    </React.StrictMode>,
  )
}
