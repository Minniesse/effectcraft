//! The KatWork build profile: `effectcraft-host` built with `--no-default-features`, so without
//! `plugins`, `scripting` and `model-download`. The session must not download, load plug-ins,
//! start effect helpers or run scripts, and documents (their expressions included) must still
//! open and render. Compiled only with all three features off.
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

fn not_available(s: &mut effectcraft_engine::Session, id: &str, params: Value) {
    let e = s.execute(id, params).expect_err(id).to_string();
    assert!(e.contains("not available in this build"), "{id}: {e}");
}

/// The plan's test, with the command names the code has: there is no `segment.model.download`;
/// the model downloads are `roto.model.download` and `face.model.download`, and they are unknown.
/// `effect.plugins.load` keeps the engine's existing answer for a session without a plug-in loader.
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
    not_available(&mut s, "effect.plugins.load", json!({"path": wasm.to_string_lossy()}));
    not_available(&mut s, "effect.plugins.load", json!({"folder": dir.to_string_lossy()}));
    not_available(&mut s, "effect.plugins.install", json!({"path": wasm.to_string_lossy()}));
    let info = s.execute("effect.plugins.list", json!({})).unwrap();
    assert_eq!(info["wasm"], false, "{info}");
    assert_eq!(info["helpers"], false, "no external effect helpers: {info}");
    not_available(&mut s, "effect.helpers.scan", json!({}));
    // The Plug-in Library downloads nothing (its https fetch is compiled out).
    s.plugins_dir = Some(dir.join("Plug-ins"));
    let r = s.execute("library.refresh", json!({"catalog": "https://github.com/storytold/effectcraft/raw/main/index.json", "wait": true}));
    let text = format!("{r:?} {}", s.execute("library.list", json!({})).map(|v| v.to_string()).unwrap_or_default());
    assert!(text.contains("not available in this build"), "{text}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// R44: the ExtendScript runner is compiled out. `script.run` (and the MCP `run_script` tool and
/// the Script Console, which run it) keep the engine's answer for a session with no runner.
#[test]
fn script_runner_is_not_available() {
    let (mut s, _) = comp();
    assert!(s.script.is_none());
    not_available(&mut s, "script.run", json!({"code": "app.project.numItems"}));
    let dir = std::env::temp_dir().join(format!("ec-katwork-script-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("s.jsx");
    std::fs::write(&file, "app.project.items.addComp('X', 8, 8, 1, 1, 25);").unwrap();
    let before = s.project.comps().count();
    assert!(s.execute("file.runScript", json!({"path": file.to_string_lossy()})).is_err());
    assert_eq!(s.project.comps().count(), before, "the script did not run");
    let _ = std::fs::remove_dir_all(&dir);
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
