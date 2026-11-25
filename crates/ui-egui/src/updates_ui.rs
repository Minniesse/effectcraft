//! Software updates, background check at startup, and startup prompt modal. Native builds with the
//! `self-update` feature (default on) check and install releases through craft-update; the web
//! build and builds without the feature answer Check for Updates with a message instead.

use crate::EffectcraftApp;

#[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
use craft_update::dialog::{CheckResultDialog, CheckResultResponse};
#[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
use craft_update::updater::{Action, Status, Updater};

// Without the updater every field has its default (the web build, and native builds without `self-update`).
#[cfg_attr(any(target_arch = "wasm32", not(feature = "self-update")), derive(Default))]
pub(crate) struct Runtime {
    #[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
    pub(crate) updater: Updater,
    /// The desktop app turns the check at start on; tests and the web build leave it off.
    #[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
    startup_check: bool,
    #[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
    check_requested: bool,
    pub(crate) manual_window_open: bool,
    #[cfg(any(target_arch = "wasm32", not(feature = "self-update")))]
    pub(crate) status_message: Option<String>,
}

#[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
impl Default for Runtime {
    fn default() -> Self {
        Self { updater: Updater::new(&crate::UPDATE_APP, env!("CARGO_PKG_VERSION")), startup_check: false, check_requested: false, manual_window_open: false }
    }
}

impl EffectcraftApp {
    /// Whether the app asks for a newer release when it starts (off until the desktop app turns it on).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_startup_update_check(&mut self, on: bool) {
        #[cfg(feature = "self-update")]
        {
            self.updates_rt.startup_check = on;
        }
        #[cfg(not(feature = "self-update"))]
        let _ = on;
    }
}

/// Restart to Update. With unsaved changes the result dialog closes and the window is asked to
/// close, so the unsaved-changes prompt opens. Cancel there leaves the update ready. An open
/// startup prompt keeps its own Restart to Update and save-first note, so the window stays open.
#[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
pub(crate) fn restart_to_update(rt: &mut Runtime, ctx: &egui::Context, clean: bool) {
    if clean {
        rt.updater.restart_to_update();
        return;
    }
    rt.manual_window_open = false;
    rt.updater.dismiss_status();
    if rt.updater.prompt_open() {
        return;
    }
    // Without a prompt this only takes the close a finished installation queued, so a save after
    // Cancel does not close the app.
    let _ = rt.updater.show_if(ctx, true);
    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
}

pub(crate) fn check(app: &mut EffectcraftApp) {
    // A Nightly session is replaced by ArtCraft Launcher, never by the stable release feed: a
    // "restart to update" there would install stable over the nightly build.
    if !crate::channel::stable_services_allowed() {
        app.ui.status = "Updates come with the ArtCraft Launcher's Nightly channel.".into();
        return;
    }
    #[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
    {
        app.updates_rt.check_requested = true;
        app.updates_rt.manual_window_open = true;
    }
    #[cfg(any(target_arch = "wasm32", not(feature = "self-update")))]
    {
        app.updates_rt.manual_window_open = true;
        let message =
            if cfg!(target_arch = "wasm32") { "Update check is not supported in the web browser." } else { "Update check is not available in this build." };
        app.updates_rt.status_message = Some(crate::i18n::tr(message).to_string());
    }
}

pub(crate) fn tick(app: &mut EffectcraftApp, ctx: &egui::Context) {
    #[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
    {
        let rt = &mut app.updates_rt;
        if std::mem::take(&mut rt.check_requested) {
            rt.updater.check_now(ctx);
        } else if rt.startup_check {
            rt.updater.check_at_start(ctx);
        }
        rt.updater.tick(ctx);
    }
    #[cfg(any(target_arch = "wasm32", not(feature = "self-update")))]
    let _ = (app, ctx);
}

/// Whether the check result dialog shows this frame. It answers Help ▸ Check for Updates until the
/// requested check ends, and it shows whenever the status is not idle: at the first start after an
/// update, the check at start leaves `Updated` or `InstallFailed`. A check that finds a release
/// leaves the status idle and the startup prompt takes over, so the dialog closes then.
#[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
pub(crate) fn result_window_open(open: bool, status: &Status, check_requested: bool) -> bool {
    !matches!(status, Status::Idle) || (open && check_requested)
}

pub(crate) fn show(app: &mut EffectcraftApp, ctx: &egui::Context) {
    #[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
    {
        // The restart helper is scheduled before the app closes, so no update starts over unsaved work.
        let clean = !app.session.is_dirty();
        match app.updates_rt.updater.show_if(ctx, clean) {
            Action::OpenUrl(url) => ctx.open_url(egui::OpenUrl::new_tab(url)),
            Action::Close => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
            Action::None => {}
        }
        let rt = &mut app.updates_rt;
        rt.manual_window_open = result_window_open(rt.manual_window_open, rt.updater.status(), rt.check_requested);
        if rt.manual_window_open {
            match CheckResultDialog::new(&crate::UPDATE_APP, env!("CARGO_PKG_VERSION")).show(ctx, rt.updater.status()) {
                CheckResultResponse::None => {}
                CheckResultResponse::Close => {
                    rt.manual_window_open = false;
                    rt.updater.dismiss_status();
                }
                CheckResultResponse::CheckAgain => rt.updater.check_now(ctx),
                CheckResultResponse::RestartToUpdate => restart_to_update(rt, ctx, clean),
            }
        }
    }

    #[cfg(any(target_arch = "wasm32", not(feature = "self-update")))]
    if app.updates_rt.manual_window_open {
        let mut open = true;
        let mut close = false;
        egui::Window::new(crate::i18n::tr("Check for Updates"))
            .open(&mut open)
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                if let Some(msg) = &app.updates_rt.status_message {
                    ui.label(msg);
                }
                ui.add_space(8.0);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(crate::i18n::tr("Close")).clicked() {
                        close = true;
                    }
                });
            });
        if !open || close {
            app.updates_rt.manual_window_open = false;
            app.updates_rt.status_message = None;
        }
    }
}
