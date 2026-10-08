# AI Game Maker Web Studio 6.0

A browser-first web app for creating Godot 4 project files with an OpenAI-compatible chat-completions API. It can run as a static site; no Tauri desktop shell is required.

## Features
- Responsive browser UI, prompt-based project generation, editable API endpoint and model ID.
- API key stays in page memory and is not written to localStorage.
- JSON response parsing, safe relative-path checks, duplicate-path checks, required-file checks, and one automatic repair request when structural checks fail.
- Offline starter project when you want to test the interface without an API key.
- Export generated text files as a ZIP.

## Limitations
This is a browser-based project generator, not a frontier foundation model. It uses the model and account configured by the user; it cannot guarantee GPT-6 Astra-level capability. The browser cannot run the Godot engine or truthfully claim engine tests passed. Import exported projects into Godot 4 for actual parsing and runtime testing. Very large generations are limited by the model context window, browser memory, provider rate limits, and practical token budgets. Billions of source-code lines are not a feasible single generation target; use modular, incremental project generation instead.

## Run locally
Requires Node.js 20 or newer.

    npm install
    npm test
    npm run dev

## Build the static website
    npm run build:web

The deploy workflow publishes the static site to GitHub Pages on pushes to main after tests/build pass. The Web CI workflow runs the tests and production build on pushes and pull requests. In repository Settings → Pages, set the source to GitHub Actions if it is not already configured. API requests are sent directly from the user's browser to their chosen provider; that provider must permit browser CORS for its API. Never enter a key on a website you do not trust.
