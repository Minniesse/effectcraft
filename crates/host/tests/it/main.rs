//! The crate's integration tests, one binary (each module is a former `tests/<name>.rs`).

mod background_presets;
mod cryptomatte_pick;
mod determinism;
mod exr_layers;
mod footage_reload;
mod gif_animates;
#[cfg(feature = "plugins")]
mod library;
mod memory_budget;
mod pick_whip;
#[cfg(feature = "plugins")]
mod plugins;
mod relink;
mod tga_import;
mod timeline_interop;
mod vector_import;
mod wav_markers;
mod webm_metadata;

mod repros;
