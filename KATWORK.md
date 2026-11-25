# KatWork fork of EffectCraft

This repository is the KatWork fork of EffectCraft (https://github.com/storytold/effectcraft). KatWork builds the app's CLI and web
build from this fork. It carries the patches listed below; everything else is upstream code.

Based on EffectCraft by the ArtCraft team (https://github.com/storytold/effectcraft). This build is not an ArtCraft product.

## Fork base

- Fork base (this fork's `main` when the KatWork work started): `59aed45057561ff2ca0e3c27ab849eebbfaf71dc`
- Upstream `storytold/effectcraft` head at that time: `59aed45057561ff2ca0e3c27ab849eebbfaf71dc`

## KatWork patches

| Patch | Upstream PR |
|---|---|
| `chore(katwork): remove the ArtCraft marks` | not proposed upstream (the brand terms let a fork that proposes changes keep the marks; this one is a distributed modified version) |
| `feat(katwork): model-download feature, on by default` | draft, not opened (in the fork PR body) |
| `feat(katwork): plugins feature, on by default` | draft, not opened (in the fork PR body) |
| `feat(katwork): scripting feature, on by default` | draft, not opened (in the fork PR body) |
| `feat(katwork): self-update feature, on by default` | draft, not opened (in the fork PR body) |

## The KatWork build

KatWork builds the CLI (`effectcraft-cli`) and the web build (`apps/effectcraft-web`, `cargo xtask web`) with
`--no-default-features`. Each gate below is a Cargo feature that is on by default, so an upstream build is unchanged:

- `model-download` (in `effectcraft-engine`): `roto.model.download` and `face.model.download`, which run the system `curl`.
  Off: both commands are unknown; Install from File stays, and Settings shows no Download button.
- `plugins` (in `effectcraft-host`, forwarding the engine's `plugins`): the WebAssembly effect plug-in runtime
  (`effectcraft-plugin`, wasmi), the external effect helpers (`effectcraft-helper-host`, which start helper programs as child
  processes) and the Plug-in Library's https downloads. Off: the session has no plug-in loader and no helpers, so
  `effect.plugins.load`, `library.*` installs and `effect.helpers.*` answer that they are not available in this build (the
  engine's existing answer); a project that uses a plug-in effect opens and renders without it.
- `scripting` (in `effectcraft-host`; R44): the ExtendScript runner `effectcraft-script` (`script.run`, File ▸ Scripts, the MCP
  `run_script` tool, the Script Console). Off: they answer "JavaScript scripting is not available in this build" (the engine's
  existing answer). Document expressions (`effectcraft-expr`, boa) stay: they are document data, evaluated with their own
  limits, and have no file or socket access.
- `self-update` (in `effectcraft-ui-egui`): the release checker and self-updater `craft-update` and its HTTPS client (`ureq`,
  `rustls`), which the CLI links through the UI crate (the `bench` UI operations). Off: Check for Updates says it is not
  available in this build.

`scripts/katwork-profile.sh` (CI job `katwork-profile`) checks that these crates and engine features are not in the KatWork
build graphs, with a control that each is in the upstream build, and runs `crates/host/tests/katwork_profile.rs` with the
features off.

## The marks

The ArtCraft name, wordmark and logos are trademarks of the ArtCraft Team and are not covered by the app's licence. This fork
removes them, as the brand terms require of a distributed modified version: `docs/brand/` is deleted; links to the ArtCraft
website and its community (Discord), and taglines that present the build as an ArtCraft product, are removed; help links and
`app.links`-style results point to the upstream repository (https://github.com/storytold/effectcraft); README.md, NOTICE and the
About window carry the "Based on" line above as plain text. No new logo is drawn: where a mark was shown, the app's name is shown
as plain text.

`scripts/check-no-marks.sh` (CI job `no-marks` in `.github/workflows/katwork.yml`) fails when a tracked file is a copy of a
fork-base brand file (`scripts/katwork-marks/hashes`), when a line names the marks or the ArtCraft website, when a line names
ArtCraft and is not in `scripts/katwork-marks/allow`, or when an image below is missing from this file.

### What still names ArtCraft, and why (`scripts/katwork-marks/allow`)

- Copyright notices (`LICENSE-MIT`, `NOTICE`, README): required by the licences.
- Application ids `ai.storyteller.effectcraft` and installer paths and folders (`Programs\ArtCraft\...`): kept unchanged; they are
  identifiers, not marks, and changing them would break upgrades of existing installs.
- The nightly channel's messages, comments and tests that name ArtCraft Launcher (`build_flavor.rs`, `channel.rs`,
  `profile_root.rs`, `updates_ui.rs`, `main.rs`, the external-helpers ADR) and the Flatpak permission for the launcher's version
  folder (`xdg-data/ArtCraft/versions`): they name a separate program and its folders, the upstream nightly builds' updater;
  KatWork builds no nightly.
- Asset provenance records (`assets/app-icon/*.attribution`, `LICENSE.txt`, `README.md`, `ATTRIBUTION.md` rows, the favicon's
  sidecar): they say the app icon was drawn by the project owner in the ArtCraft image tool. They record where an asset comes
  from, which the asset rules require; they do not present the build as an ArtCraft product.
- The upstream release workflow's secret name (`release.yml`): upstream CI configuration, unchanged; KatWork does not run it.
- The "Based on" line inside code (`crates/ui-egui/src/panels/dialogs.rs`, shown in the About window).

## Image assets of the fork base

Removed (the brand files, deleted with `docs/brand/`):

- `docs/brand/LICENSE-brand.txt`: removed
- `docs/brand/artcraft-logo-white.png`: removed
- `docs/brand/artcraft-logo-white.png.attribution`: removed
- `docs/brand/artcraft-logo-white.svg`: removed
- `docs/brand/artcraft-logo-white.svg.attribution`: removed
- `docs/brand/artcraft-logo.png`: removed
- `docs/brand/artcraft-logo.png.attribution`: removed
- `docs/brand/artcraft-logo.svg`: removed
- `docs/brand/artcraft-logo.svg.attribution`: removed
- `docs/brand/artcraft-mark-black.png`: removed
- `docs/brand/artcraft-mark-black.png.attribution`: removed
- `docs/brand/artcraft-mark-black.svg`: removed
- `docs/brand/artcraft-mark-black.svg.attribution`: removed
- `docs/brand/artcraft-mark.png`: removed
- `docs/brand/artcraft-mark.png.attribution`: removed
- `docs/brand/artcraft-mark.svg`: removed
- `docs/brand/artcraft-mark.svg.attribution`: removed

Every other image of the fork base, each kept:

- `apps/effectcraft-web/web/favicon.svg`: kept: original artwork (the app icon), licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/effectcraft-1024.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/effectcraft-macos-512.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/effectcraft-nightly-1024.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/effectcraft-nightly-macos-512.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/effectcraft-nightly.icns`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/effectcraft-nightly.ico`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/effectcraft-nightly.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/effectcraft-small.svg`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/effectcraft.icns`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/effectcraft.ico`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/effectcraft.svg`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/128x128/apps/ai.storyteller.effectcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/16x16/apps/ai.storyteller.effectcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/24x24/apps/ai.storyteller.effectcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/256x256/apps/ai.storyteller.effectcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/32x32/apps/ai.storyteller.effectcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/48x48/apps/ai.storyteller.effectcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/512x512/apps/ai.storyteller.effectcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/64x64/apps/ai.storyteller.effectcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/scalable/apps/ai.storyteller.effectcraft.svg`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `crates/svg/tests/fixtures/basic-shapes.svg`: kept: test fixture (viewed: shapes, no ArtCraft mark)
- `crates/svg/tests/fixtures/paths-gradients.svg`: kept: test fixture (viewed: shapes, no ArtCraft mark)
- `docs/images/effectcraft-3d.png`: kept, edited: screenshot of the upstream build; it showed the ArtCraft mark at the left of the Tools bar and the Discord button at its right, both painted over with the bar's background (no other change)
- `docs/images/effectcraft-effects.png`: kept, edited: screenshot of the upstream build; it showed the ArtCraft mark at the left of the Tools bar and the Discord button at its right, both painted over with the bar's background (no other change)
- `docs/images/effectcraft-graph-editor.png`: kept, edited: screenshot of the upstream build; it showed the ArtCraft mark at the left of the Tools bar and the Discord button at its right, both painted over with the bar's background (no other change)
- `docs/images/effectcraft-hero.png`: kept, edited: screenshot of the upstream build; it showed the ArtCraft mark at the left of the Tools bar and the Discord button at its right, both painted over with the bar's background (no other change)
- `docs/images/effectcraft-render-queue.png`: kept, edited: screenshot of the upstream build; it showed the ArtCraft mark at the left of the Tools bar and the Discord button at its right, both painted over with the bar's background (no other change)
- `packaging/macos/dmg/background.svg`: kept: macOS disk-image background, original artwork (the app name and a keyframe drawing; viewed: no ArtCraft mark)
