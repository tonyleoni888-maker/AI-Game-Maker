use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf, process::Command, time::Duration};

#[derive(Serialize)]
struct Status { os:String, arch:String, godot:Option<String>, ollama:bool, ollama_model:Option<String>, comfyui:bool, project_count:usize }
#[derive(Deserialize)]
struct OllamaResponse { response:Option<String> }
fn home()->PathBuf { env::var_os("USERPROFILE").or_else(||env::var_os("HOME")).map(PathBuf::from).unwrap_or_else(||PathBuf::from(".")) }
fn root()->PathBuf { home().join("AI-Game-Maker-Projects") }
fn cmd(n:&str)->Option<String> {
 let c=if cfg!(windows){Command::new("where").arg(n).output()}else{Command::new("sh").args(["-lc",&format!("command -v {n}")]).output()};
 c.ok().filter(|x|x.status.success()).and_then(|x|String::from_utf8(x.stdout).ok()).and_then(|s|s.lines().next().map(str::trim).map(str::to_string)).filter(|x|!x.is_empty())
}
async fn ollama_generate(prompt:&str,model:&str)->Result<String,String>{
 let client=reqwest::Client::builder().timeout(Duration::from_secs(180)).build().map_err(|e|e.to_string())?;
 let response=client.post("http://127.0.0.1:11434/api/generate").json(&serde_json::json!({"model":model,"prompt":prompt,"stream":false,"options":{"temperature":0.25}})).send().await.map_err(|e|format!("Ollama nicht erreichbar: {e}"))?;
 if !response.status().is_success(){return Err(format!("Ollama HTTP {}",response.status()))}
 let data:OllamaResponse=response.json().await.map_err(|e|e.to_string())?;
 data.response.filter(|s|!s.trim().is_empty()).ok_or_else(||"Ollama hat keinen Code zurückgegeben".into())
}
fn clean(s:&str)->String{s.chars().map(|c|if c.is_ascii_alphanumeric()||c=='-'||c=='_'{c}else{'_'}).collect::<String>().trim_matches('_').chars().take(64).collect()}
fn default_script(node:&str)->String{
 if node=="Node3D" { return r#"extends Node3D
var player: CharacterBody3D
var camera: Camera3D
func _ready() -> void:
    var world := WorldEnvironment.new()
    var env := Environment.new()
    env.background_mode = Environment.BG_SKY
    env.ambient_light_source = Environment.AMBIENT_SOURCE_COLOR
    env.ambient_light_color = Color(0.28, 0.32, 0.42)
    env.ambient_light_energy = 0.8
    world.environment = env
    add_child(world)
    var sun := DirectionalLight3D.new()
    sun.rotation_degrees = Vector3(-50, -25, 0)
    add_child(sun)
    _box("Floor", Vector3(0, -0.5, 0), Vector3(40, 1, 40), Color(0.12, 0.16, 0.2))
    for i in range(12):
        _box("Pillar_%d" % i, Vector3(float((i % 4) * 5 - 8), 1, float((i / 4) * 5 - 8)), Vector3(1.2, 2, 1.2), Color(0.2, 0.25, 0.32))
    player = CharacterBody3D.new()
    var collision := CollisionShape3D.new()
    var capsule := CapsuleShape3D.new()
    capsule.height = 1.8
    collision.shape = capsule
    collision.position.y = 0.9
    player.add_child(collision)
    camera = Camera3D.new()
    camera.position.y = 1.55
    camera.current = true
    player.add_child(camera)
    player.position = Vector3(0, 0.1, 8)
    add_child(player)
    Input.mouse_mode = Input.MOUSE_MODE_CAPTURED
func _box(label: String, pos: Vector3, size: Vector3, color: Color) -> void:
    var body := StaticBody3D.new()
    body.name = label
    body.position = pos
    var mesh_node := MeshInstance3D.new()
    var mesh := BoxMesh.new()
    mesh.size = size
    mesh_node.mesh = mesh
    var mat := StandardMaterial3D.new()
    mat.albedo_color = color
    mesh_node.material_override = mat
    body.add_child(mesh_node)
    var shape := CollisionShape3D.new()
    var box := BoxShape3D.new()
    box.size = size
    shape.shape = box
    body.add_child(shape)
    add_child(body)
func _unhandled_input(event: InputEvent) -> void:
    if event is InputEventMouseMotion and Input.mouse_mode == Input.MOUSE_MODE_CAPTURED:
        player.rotate_y(-event.relative.x * 0.002)
        camera.rotate_x(-event.relative.y * 0.002)
        camera.rotation.x = clamp(camera.rotation.x, -1.4, 1.4)
    if event.is_action_pressed("ui_cancel"):
        Input.mouse_mode = Input.MOUSE_MODE_VISIBLE
    if event is InputEventMouseButton and event.pressed:
        Input.mouse_mode = Input.MOUSE_MODE_CAPTURED
func _physics_process(delta: float) -> void:
    if player == null: return
    var keys := Vector2(Input.get_action_strength("ui_right") - Input.get_action_strength("ui_left"), Input.get_action_strength("ui_down") - Input.get_action_strength("ui_up"))
    var dir := (player.transform.basis * Vector3(keys.x, 0, keys.y)).normalized()
    player.velocity.x = dir.x * 5.0
    player.velocity.z = dir.z * 5.0
    if not player.is_on_floor(): player.velocity.y -= 18.0 * delta
    elif Input.is_key_pressed(KEY_SPACE): player.velocity.y = 6.0
    player.move_and_slide()
"#.to_string(); }
 r#"extends Node2D
var player: CharacterBody2D
func _ready() -> void:
    var bg := ColorRect.new()
    bg.color = Color(0.035, 0.055, 0.085)
    bg.set_anchors_and_offsets_preset(Control.PRESET_FULL_RECT)
    add_child(bg)
    player = CharacterBody2D.new()
    player.position = Vector2(400, 280)
    var avatar := ColorRect.new()
    avatar.color = Color(0.42, 0.35, 1.0)
    avatar.size = Vector2(36, 48)
    avatar.position = Vector2(-18, -24)
    player.add_child(avatar)
    add_child(player)
    for i in range(8):
        var block := ColorRect.new()
        block.color = Color(0.12, 0.2, 0.28)
        block.size = Vector2(80, 40)
        block.position = Vector2(70 + i * 95, 400 + sin(float(i)) * 40)
        add_child(block)
func _process(delta: float) -> void:
    player.position += Input.get_vector("ui_left", "ui_right", "ui_up", "ui_down") * 260.0 * delta
    player.position.x = clamp(player.position.x, 18.0, 782.0)
    player.position.y = clamp(player.position.y, 24.0, 576.0)
"#.to_string()
}
#[tauri::command]
async fn environment_status()->Result<Status,String>{
 fs::create_dir_all(root()).map_err(|e|e.to_string())?;
 let godot=cmd("godot").or_else(||cmd("godot4"));
 let count=fs::read_dir(root()).map(|r|r.filter_map(Result::ok).filter(|e|e.path().is_dir()).count()).unwrap_or(0);
 let client=reqwest::Client::builder().timeout(Duration::from_secs(2)).build().map_err(|e|e.to_string())?;
 let response=client.get("http://127.0.0.1:11434/api/tags").send().await.ok();
 let models:Vec<String>=if let Some(resp)=response { if resp.status().is_success(){let data:serde_json::Value=resp.json().await.unwrap_or_default();data["models"].as_array().map(|a|a.iter().filter_map(|m|m["name"].as_str().map(String::from)).collect()).unwrap_or_default()}else{vec![]} }else{vec![]};
 Ok(Status{os:env::consts::OS.into(),arch:env::consts::ARCH.into(),godot,ollama:!models.is_empty(),ollama_model:models.first().cloned(),comfyui:std::net::TcpStream::connect("127.0.0.1:8188").is_ok(),project_count:count})
}
#[tauri::command]
async fn create_project(name:String,prompt:String,mode:String,model:String)->Result<serde_json::Value,String>{
 let n=clean(&name);
 if n.is_empty(){return Err("Bitte gib einen Projektnamen ein.".into())}
 if prompt.trim().is_empty(){return Err("Bitte beschreibe dein Spiel.".into())}
 let r=root().join(&n);
 fs::create_dir_all(r.join("scripts")).map_err(|e|e.to_string())?;
 let node=if mode=="3D"{"Node3D"}else{"Node2D"};
 let scene=format!("[gd_scene load_steps=2 format=3]\n\n[ext_resource type=\"Script\" path=\"res://scripts/main.gd\" id=\"1_script\"]\n\n[node name=\"{n}\" type=\"{node}\"]\nscript = ExtResource(\"1_script\")\n");
 fs::write(r.join("project.godot"),format!("[application]\nconfig/name=\"{n}\"\nrun/main_scene=\"res://main.tscn\"\n\n[display]\nwindow/size/viewport_width=800\nwindow/size/viewport_height=600\n\n[rendering]\nrenderer/rendering_method=\"gl_compatibility\"\n")).map_err(|e|e.to_string())?;
 fs::write(r.join("main.tscn"),scene).map_err(|e|e.to_string())?;
 let selected=if model.trim().is_empty(){"qwen2.5-coder:7b"}else{model.trim()};
 let task=format!("You are an expert Godot 4 game developer. Return only complete valid GDScript source, no markdown or explanations. It is attached to a scene root of type {node} and must start with extends {node}. Build a polished playable prototype for this request: {prompt}. Use Godot 4 APIs only, create required nodes/materials in code, no external assets/plugins, self-contained and robust.");
 let (script,ai_used,message)=match ollama_generate(&task,selected).await {
  Ok(raw)=>{let code=raw.lines().filter(|line|!line.trim().starts_with(char::from(96))).collect::<Vec<_>>().join("\n").trim().to_string();if code.starts_with(&format!("extends {node}")){(code,true,format!("Lokale KI ({selected}) hat den Spielcode generiert."))}else{(default_script(node),false,"KI-Antwort war kein passendes Godot-Skript; Starter verwendet.".into())}},
  Err(err)=>(default_script(node),false,format!("Ollama nicht verfügbar ({err}). Spielbarer Starter erstellt; Ollama starten und ein Modell laden, um KI-Code zu erzeugen."))
 };
 fs::write(r.join("scripts/main.gd"),script).map_err(|e|e.to_string())?;
 fs::write(r.join("README.txt"),format!("AI Game Maker\nName: {n}\nMode: {mode}\nPrompt: {prompt}\nAI generated: {ai_used}\n\nOpen project.godot with Godot 4. Controls: arrows/WASD; mouse look in 3D; Space jump; Esc release mouse.\n")).map_err(|e|e.to_string())?;
 Ok(serde_json::json!({"path":r.to_string_lossy(),"ai_used":ai_used,"message":message,"model":selected}))
}
#[tauri::command]
fn validate_project(path:String)->Result<serde_json::Value,String>{
 let p=PathBuf::from(path);let mut errors:Vec<String>=["project.godot","main.tscn","scripts/main.gd"].iter().filter(|f|!p.join(f).is_file()).map(|f|format!("Fehlt: {f}")).collect();
 let script=fs::read_to_string(p.join("scripts/main.gd")).unwrap_or_default();let scene=fs::read_to_string(p.join("main.tscn")).unwrap_or_default();
 if !scene.contains("res://scripts/main.gd"){errors.push("Skript ist nicht in main.tscn eingebunden".into())}
 if !script.starts_with("extends "){errors.push("GDScript braucht eine extends-Deklaration".into())}
 Ok(serde_json::json!({"ok":errors.is_empty(),"errors":errors,"warnings":[]}))
}
#[tauri::command]
fn godot_check(path:String)->Result<serde_json::Value,String>{
 let Some(g)=cmd("godot").or_else(||cmd("godot4"))else{return Ok(serde_json::json!({"ok":false,"available":false,"message":"Godot 4 nicht installiert oder nicht im PATH."}))};
 let output=Command::new(g).args(["--headless","--path",&path,"--editor","--quit"]).output().map_err(|e|e.to_string())?;
 Ok(serde_json::json!({"ok":output.status.success(),"available":true,"code":output.status.code(),"details":String::from_utf8_lossy(&output.stderr).chars().take(2000).collect::<String>()}))
}
#[tauri::command]fn list_plugins()->Result<Vec<String>,String>{Ok(vec![])}
#[tauri::command]fn comfyui_status()->Result<bool,String>{Ok(std::net::TcpStream::connect("127.0.0.1:8188").is_ok())}
fn main(){tauri::Builder::default().invoke_handler(tauri::generate_handler![environment_status,create_project,validate_project,godot_check,list_plugins,comfyui_status]).run(tauri::generate_context!()).expect("error while running tauri application");}
