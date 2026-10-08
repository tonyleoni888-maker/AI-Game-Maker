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
test('Tauri 2 config', () => { const p=JSON.parse(read('package.json')); const c=JSON.parse(read('src-tauri/tauri.conf.json')); assert.ok(p.devDependencies['@tauri-apps/cli']); assert.equal(c.productName,'AI Game Maker'); assert.equal(c.identifier,'com.aigamemaker.studio'); });
test('core commands', () => { const r=read('src-tauri/src/main.rs'); for (const x of ['environment_status','create_project','validate_project','godot_check','list_plugins','comfyui_status']) assert.match(r,new RegExp(x)); });
test('self healing loop', () => { const s=read('src/main.js'); assert.match(s,/i<=5/); assert.match(s,/validate_project/); assert.match(s,/godot_check/); });
test('no electron', () => assert.doesNotMatch(read('package.json'),/electron/i));
test('windows workflow', () => { const w=read('.github/workflows/windows-tauri.yml'); assert.match(w,/windows-latest/); assert.match(w,/npm run build/); assert.match(w,/upload-artifact@v4/); });
