//! The KatWork build profile: `effectcraft-host` built with `--no-default-features`, so without
//! `plugins`, `scripting` and `model-download`. The session must not download, load plug-ins,
//! start effect helpers or run scripts; the commands of those features are unknown and the menus
//! and Settings do not show them; documents (their expressions included) still open and render.
//! Compiled only with all three features off.
#![cfg(not(any(feature = "plugins", feature = "scripting", feature = "model-download")))]

use effectcraft_engine::EngineError;
use effectcraft_engine::project::{GroupKind, LayerId, Node};
use effectcraft_engine::time::Tick;
use serde_json::{Value, json};

fn comp() -> (effectcraft_engine::Session, u64) {
    let mut s = effectcraft_host::session();
    s.execute("comp.new", json!({"name": "K", "width": 40, "height": 20, "duration": 1})).unwrap();
    let l = s.execute("layer.newSolid", json!({"color": [1.0, 0.5, 0.0], "width": 40, "height": 20})).unwrap()["layer"].as_u64().unwrap();
    (s, l)
}

fn unknown(s: &mut effectcraft_engine::Session, id: &str, params: Value) {
    match s.execute(id, params) {
        Err(EngineError::UnknownCommand(c)) => assert_eq!(c, id),
        other => panic!("{id}: expected the unknown-command error, got {other:?}"),
    }
}

/// The plan's test, with the command names the code has: there is no `segment.model.download`;
/// the model downloads are `roto.model.download` and `face.model.download`. Every command of the
/// compiled-out plug-in runtime, effect helpers and Plug-in Library is unknown too.
#[test]
fn model_download_and_plugins_are_unknown() {
    let (mut s, _) = comp();
    unknown(&mut s, "roto.model.download", json!({"id": "mobilesam", "wait": true}));
    unknown(&mut s, "face.model.download", json!({"id": "mediapipe-face", "wait": true}));
    assert!(effectcraft_engine::commands::find("roto.model.install").is_some(), "Install from File stays");

    let dir = std::env::temp_dir().join(format!("ec-katwork-profile-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let wasm = dir.join("p.wasm");
    std::fs::write(&wasm, b"\0asm\x01\0\0\0").unwrap();
    s.plugins_dir = Some(dir.join("Plug-ins"));
    unknown(&mut s, "effect.plugins.load", json!({"path": wasm.to_string_lossy()}));
    unknown(&mut s, "effect.plugins.load", json!({"folder": dir.to_string_lossy()}));
    unknown(&mut s, "effect.plugins.install", json!({"path": wasm.to_string_lossy()}));
    for id in PLUGIN_COMMANDS {
        unknown(&mut s, id, json!({}));
    }
    unknown(&mut s, "library.refresh", json!({"catalog": "https://github.com/storytold/effectcraft/raw/main/index.json", "wait": true}));
    // Rust effect plug-ins and the built-in effects stay listed; the runtime and helpers do not.
    let info = s.execute("effect.plugins.list", json!({})).unwrap();
    assert_eq!(info["wasm"], false, "{info}");
    assert_eq!(info["helpers"], false, "no external effect helpers: {info}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// R44: the ExtendScript runner is compiled out, and with it every script command: running code
/// or script files, installing scripts and ScriptUI panels, the scripts' scheduled tasks and the
/// ScriptUI windows. (The MCP `run_script` and `script_ui` tools are hidden by the automation
/// crate; the Script Console by the UI.)
#[test]
fn script_commands_are_unknown() {
    let (mut s, _) = comp();
    assert!(s.script.is_none());
    for id in SCRIPT_COMMANDS {
        unknown(&mut s, id, json!({}));
    }
    let dir = std::env::temp_dir().join(format!("ec-katwork-script-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("s.jsx");
    std::fs::write(&file, "app.project.items.addComp('X', 8, 8, 1, 1, 25);").unwrap();
    let before = s.project.comps().count();
    unknown(&mut s, "file.runScript", json!({"path": file.to_string_lossy()}));
    unknown(&mut s, "script.run", json!({"code": "app.project.items.addComp('Y', 8, 8, 1, 1, 25)"}));
    assert_eq!(s.project.comps().count(), before, "no script ran");
    let _ = std::fs::remove_dir_all(&dir);
}

const SCRIPT_COMMANDS: &[&str] = &[
    "script.run",
    "file.runScript",
    "file.scripts.list",
    "file.installScript",
    "file.installScriptUIPanel",
    "file.uninstallScript",
    "file.runStartupScripts",
    "file.runShutdownScripts",
    "window.scriptPanel",
    "window.restoreScriptPanels",
    "script.settings.save",
    "script.runTasks",
    "scriptui.list",
    "scriptui.get",
    "scriptui.click",
    "scriptui.set",
    "scriptui.close",
    "scriptui.fileDialog",
];

const PLUGIN_COMMANDS: &[&str] = &[
    "effect.plugins.load",
    "effect.plugins.install",
    "effect.helpers.list",
    "effect.helpers.scan",
    "effect.helpers.load",
    "library.list",
    "library.missing",
    "library.refresh",
    "library.preview",
    "library.install",
    "library.update",
    "library.uninstall",
    "library.loadInstalled",
];

/// The menus and Settings of this build show none of the compiled-out features: no Scripts
/// submenu, Script Console, ScriptUI panels or Scripting Help; no plug-in, helper or Plug-in
/// Library items and no Plug-in Library settings page. Every menu item runs a known command.
#[test]
fn menus_and_settings_hide_compiled_out_features() {
    use effectcraft_engine::menus::{MenuNode, TREE, parse};
    fn walk(nodes: &[MenuNode], path: &mut Vec<String>, out: &mut Vec<String>) {
        for n in nodes {
            match n {
                MenuNode::Item(e) => {
                    assert!(effectcraft_engine::commands::find(&e.command).is_some(), "{path:?} ▸ {}: unknown command {}", e.label, e.command);
                    out.push(format!("{} ▸ {} | {} {}", path.join(" ▸ "), e.label, e.command, e.params));
                }
                MenuNode::Separator => {}
                MenuNode::Dynamic { name } => out.push(format!("{} ▸ @dynamic:{name}", path.join(" ▸ "))),
                MenuNode::Submenu { label, children } => {
                    out.push(format!("{} ▸ {label} ▸", path.join(" ▸ ")));
                    path.push(label.clone());
                    walk(children, path, out);
                    path.pop();
                }
            }
        }
    }
    for mac in [false, true] {
        let mut lines = Vec::new();
        walk(&parse(TREE, mac).unwrap(), &mut Vec::new(), &mut lines);
        assert!(lines.iter().any(|l| l.contains("help.github")), "the walk sees the menus");
        for l in &lines {
            for word in ["Script", "script", "Plug-in", "plugins.", "helpers.", "library"] {
                // "Scripting & Expressions" is the Settings page of the expressions, which stay.
                if l.contains("Scripting & Expressions") {
                    continue;
                }
                assert!(!l.contains(word), "mac {mac}: `{l}` names `{word}`");
            }
        }
    }
    let pages = effectcraft_engine::prefs::pages_json().to_string();
    assert!(pages.contains("\"roto\""), "the walk sees the pages");
    assert!(!pages.contains("\"library\""), "no Plug-in Library page");
}

/// R44: document expressions stay (they are document data, evaluated by `effectcraft-expr`).
#[test]
fn document_expressions_still_evaluate() {
    let (mut s, l) = comp();
    assert!(s.expr.is_some());
    s.execute("prop.setExpression", json!({"layer": l, "path": "transform/position", "expression": "[33, 10]"})).unwrap();
    let v = s.execute("prop.get", json!({"layer": l, "path": "transform/position"})).unwrap();
    assert!(v.get("expressionError").is_none(), "{v}");
    assert_eq!(v["evaluated"][0], json!(33.0), "{v}");
    assert_eq!(v["evaluated"][1], json!(10.0), "{v}");
}

/// A project that uses an effect from a plug-in this build cannot load opens, renders as it would
/// upstream without that plug-in, and does not panic.
#[test]
fn a_project_with_a_plug_in_effect_opens_and_renders() {
    let (mut s, l) = comp();
    s.execute("effect.apply", json!({"layer": l, "effect": "ec.blur.gaussian"})).unwrap();
    let cid = s.active_comp_id().unwrap();
    let proj = std::sync::Arc::make_mut(&mut s.project);
    let layer = proj.comp_mut(cid).unwrap().layer_mut(LayerId(l)).unwrap();
    let fx = layer.props.sub_mut("effects").unwrap().children.iter_mut().find_map(|n| match n {
        Node::Group(g) => Some(g),
        _ => None,
    });
    fx.unwrap().kind = GroupKind::Effect { effect: "org.example.katwork.plugin".into() };
    let dir = std::env::temp_dir().join(format!("ec-katwork-open-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("p.ecproj");
    s.execute("file.saveAs", json!({"path": path.to_string_lossy()})).unwrap();

    let mut t = effectcraft_host::session();
    t.execute("file.open", json!({"path": path.to_string_lossy()})).unwrap();
    let cid = *t.project.comps().next().unwrap().0;
    let px = t.render(cid, Tick::ZERO, Default::default()).get(20, 10);
    assert!(px[0] > 0.9 && (px[1] - 0.5).abs() < 0.1, "the solid renders: {px:?}");
    let _ = std::fs::remove_dir_all(&dir);
}
