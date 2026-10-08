import { invoke } from '@tauri-apps/api/core';
import './style.css';

const $ = (s) => document.querySelector(s);
const app = $('#app');
let project = null;
const state = { logs: [] };
const escapeHtml = (s) => String(s).replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
function log(msg, kind = '') {
  state.logs.unshift({ msg: String(msg), kind });
  renderLogs();
}
function renderLogs() {
  const e = $('#logs');
  if (e) e.innerHTML = state.logs.slice(0, 30).map(x => '<div class="log ' + escapeHtml(x.kind) + '">' + escapeHtml(x.msg) + '</div>').join('');
}
async function status() {
  try { return await invoke('environment_status'); }
  catch (e) { return { os: '?', godot: null, ollama: false, ollama_model: null, comfyui: false, project_count: 0, error: String(e) }; }
}
async function create() {
  const button = $('#create');
  button.disabled = true;
  $('#busy').textContent = 'Erstelle Spielprojekt – lokale KI kann bis zu 3 Minuten brauchen …';
  const name = $('#name').value.trim() || 'MyGame';
  const prompt = $('#prompt').value.trim();
  const mode = $('#mode').value;
  const model = $('#model').value.trim();
  try {
    const result = await invoke('create_project', { name, prompt, mode, model });
    project = result.path;
    localStorage.setItem('agm-active-project', project);
    log(result.message, result.ai_used ? 'ok' : '');
    log('Projekt gespeichert unter: ' + project, 'ok');
    const validation = await invoke('validate_project', { path: project });
    if (validation.ok) log('Dateien und Szenenverknüpfung geprüft: OK', 'ok');
    else log('Validierung: ' + validation.errors.join(', '), 'error');
    const check = await invoke('godot_check', { path: project });
    if (check.available && check.ok) log('Godot-Headless-Prüfung bestanden.', 'ok');
    else if (!check.available) log('Godot nicht gefunden – automatische Engine-Prüfung übersprungen.', '');
    else log('Godot-Prüfung meldet Fehler: ' + (check.details || check.code || 'unbekannter Fehler'), 'error');
    $('#project-path').textContent = project;
    $('#open-note').textContent = 'Öffne diesen Ordner in Godot 4 über Projektmanager → Importieren → project.godot.';
  } catch (e) {
    log(String(e), 'error');
  } finally {
    $('#busy').textContent = '';
    button.disabled = false;
  }
}
async function load() {
  project = localStorage.getItem('agm-active-project');
  const s = await status();
  $('#status').textContent = (s.ollama ? 'Ollama ✓ ' + (s.ollama_model || '') : 'Ollama nicht bereit') + ' | ' + (s.godot ? 'Godot ✓' : 'Godot nicht gefunden') + ' | Projekte: ' + s.project_count;
  if (project) $('#project-path').textContent = project;
  if (s.ollama_model && !$('#model').value) $('#model').value = s.ollama_model;
  log(s.ollama ? 'Lokale KI erreichbar.' : 'Ohne Ollama erstellt die App einen spielbaren Starter. Für echte KI-Generierung muss Ollama mit einem Modell laufen.');
}
function render() {
  app.innerHTML = '<aside><h1>◈ AI Game Maker</h1><small>Tauri 2 · Godot 4</small><nav><button class="active">Studio</button></nav><div class="side">Lokale KI-Generierung · Projektprüfung · keine API-Kosten</div></aside><main><header><div><small>AI GAME STUDIO</small><h2>Baue dein Spiel mit KI</h2><p>Erstellt ein Godot-Projekt mit lokal generiertem GDScript – oder einem spielbaren Starter, wenn keine KI läuft.</p></div><button id="create" class="primary">Spiel erstellen</button></header><section class="cards"><div><b id="status">System wird geprüft …</b><span>Lokale Dienste</span></div><div><b>Ollama</b><span>Lokale KI, wenn installiert</span></div><div><b>Godot 4</b><span>Engine-Prüfung, wenn installiert</span></div><div><b>Kein API-Key</b><span>Lokale Generierung</span></div></section><section class="panel"><h3>Neues Spiel</h3><label>Projektname<input id="name" value="My Horror Game" maxlength="64"></label><label>Spielbeschreibung<textarea id="prompt" placeholder="Beschreibe dein Spiel …">Erstelle ein atmosphärisches 3D-Horrorspiel in einer verlassenen Forschungsstation. Kaltes Licht, unheimliche Räume, Erkundung und ein klares Ziel. Baue eine spielbare erste Szene mit Bewegung und Interaktionen.</textarea></label><div class="fields"><label>Spieltyp<select id="mode"><option>3D</option><option>2D</option></select></label><label>Ollama-Modell<input id="model" value="qwen2.5-coder:7b" placeholder="z. B. qwen2.5-coder:7b"></label></div><div id="busy" role="status"></div><p class="hint">Modellname muss exakt zu einem lokal installierten Ollama-Modell passen. Die App lädt keine Modelle automatisch herunter.</p></section><section class="panel"><h3>Ergebnis</h3><div id="project-path" class="path">Noch kein Projekt erstellt.</div><p id="open-note" class="hint">Nach der Erstellung erscheint hier der Projektordner.</p></section><section class="panel"><h3>Prüfprotokoll</h3><div id="logs"></div></section></main>';
  $('#create').onclick = create;
  load();
  renderLogs();
}
render();
