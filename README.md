# AI Game Maker - Tauri 5.2.0

AI-assisted Godot 4 game creation desktop app built with Tauri 2.

Features:
- Godot 4 2D/3D starter projects
- Ollama-native AI bridge through Rust/Tauri
- autonomous Generate -> Validate -> Repair -> Retest loop
- persistent project context
- ComfyUI and Blender diagnostics/pipeline hooks
- plugin manifest support
- secure project file operations
- ZIP export
- Windows CI build producing MSI/NSIS installers

Development: npm install, npm run test, npm run build.
Windows installer: use .github/workflows/windows-tauri.yml.