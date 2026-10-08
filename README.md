# AI Game Maker 5.3.0

AI Game Maker is a Tauri 2 desktop app that creates starter projects for Godot 4.

## Current functionality

- Creates a Godot 4 project with a scene and GDScript.
- Connects to a locally running Ollama server at `http://127.0.0.1:11434` and sends the game description to the selected local model.
- If Ollama is unavailable or returns unsuitable code, creates a built-in 2D or 3D playable starter instead.
- Checks required files and the scene-to-script link.
- If Godot 4 is installed and on PATH, runs a headless editor check.
- Saves projects in `AI-Game-Maker-Projects` in the current user's home folder.

## Important limitations

This version is an initial playable-project generator, not a finished AAA game generator. It does not automatically repair code, create a full asset library, implement plugin installation, or guarantee that arbitrary AI-generated GDScript runs correctly. Godot's headless check is the real syntax/import check when Godot is installed. Ollama and a compatible model must be installed locally for AI generation; the app does not download them for you.

## Build

Requirements: Node.js, Rust, and the platform-specific Tauri prerequisites. Windows requires the Microsoft C++ Build Tools and WebView2. See the [official Tauri prerequisites](https://v2.tauri.app/start/prerequisites/).

```sh
npm install
npm test
npm run build
```

## Windows installer

Every push to `main` triggers the Windows workflow. When it succeeds, open the [GitHub Actions runs](https://github.com/tonyleoni888-maker/AI-Game-Maker/actions/workflows/windows-tauri.yml) and download the `AI-Game-Maker-Windows` artifact from the successful run.
