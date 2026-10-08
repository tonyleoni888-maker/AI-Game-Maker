import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const read = p => fs.readFileSync(path.join(root, p), 'utf8');

test('Tauri project files exist', () => {
  for (const f of ['package.json','index.html','src/main.js','src/style.css','src-tauri/Cargo.toml','src-tauri/src/main.rs','src-tauri/tauri.conf.json']) assert.ok(fs.existsSync(path.join(root, f)), 'Missing ' + f);
});
test('Tauri 2 config and aligned version', () => {
  const p = JSON.parse(read('package.json'));
  const c = JSON.parse(read('src-tauri/tauri.conf.json'));
  const cargo = read('src-tauri/Cargo.toml');
  assert.ok(p.devDependencies['@tauri-apps/cli']);
  assert.equal(c.productName, 'AI Game Maker');
  assert.equal(c.identifier, 'com.aigamemaker.studio');
  assert.equal(c.version, p.version);
  assert.match(cargo, new RegExp('version="' + p.version + '"'));
});
test('backend commands and local AI integration', () => {
  const r = read('src-tauri/src/main.rs');
  for (const x of ['environment_status','create_project','validate_project','godot_check','list_plugins','comfyui_status','ollama_generate','/api/generate','default_script']) assert.ok(r.includes(x), 'Missing ' + x);
});
test('project generation binds script to scene and checks it', () => {
  const r = read('src-tauri/src/main.rs');
  assert.match(r, /res:\/\/scripts\/main\.gd/);
  assert.match(r, /GDScript braucht eine extends-Deklaration/);
  assert.match(r, /AI Game Maker/);
});
test('frontend passes model and uses returned project path', () => {
  const s = read('src/main.js');
  assert.match(s, /model/);
  assert.match(s, /result\.path/);
  assert.match(s, /validate_project/);
  assert.match(s, /godot_check/);
  assert.match(s, /escapeHtml/);
});
test('no Electron dependency', () => assert.doesNotMatch(read('package.json'), /electron/i));
test('Windows workflow builds and uploads installers', () => {
  const w = read('.github/workflows/windows-tauri.yml');
  assert.match(w, /windows-latest/);
  assert.match(w, /npm run build/);
  assert.match(w, /upload-artifact@v4/);
});
