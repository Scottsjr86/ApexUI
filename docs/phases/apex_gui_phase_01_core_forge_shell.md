# Apex GUI Phase 01: Core Forge Shell

**Status:** in progress - starter shell accepted; interaction state locks active
**Lane:** Apex GUI, Rust, MIT, egui/eframe-powered until engine surgery is earned
**Phase rule:** no docs-first work. This phase starts with source, runnable behavior, tests, and operator proof.

## Mission

Build the first usable Apex GUI vertical slice: a Rust workspace with a real `apex-ui` crate, a runnable `apex-demo` app, an Apex-owned facade over egui/eframe, a first theme/token system, a first shell, and the first real Apex widgets.

This phase is not a proof. It is not a theme demo. It is not a screenshot toy. It is the first working slice of Apex as a toolkit/framework someone could actually start using to build a small Rust desktop app.

The phase closes only when a developer can use Apex APIs to launch a window, render an Apex shell, use Apex controls, receive Apex responses/actions, and build/run/test the demo without touching raw egui from app code.

```text
Apex App Code
  -> ApexApp / ApexFrame / ApexUi
  -> Apex widgets / shell / theme / interaction policy
  -> Apex egui adapter internals
  -> egui / eframe runtime
```

## Phase Position

This is the first build phase.

There is no docs-only Phase 0. The header and law docs already exist as planning inputs. From here forward, every phase begins with working code.

## Operating Inputs

Use these as the standing rules when this phase is dropped into a fresh chat:

- `apex_gui_workflow_header.md` controls patch workflow, newest-base truth sync, handoff style, and completion guard.
- `apex_gui_law.md` controls durable laws, boundaries, module shape, comment/Rustdoc expectations, no fake completion, and no cheap workarounds.
- This file controls the active build slice only.

If a repo tar is supplied, the newest tar is the only source authority. Older tars, old chat memory, older extracted trees, and stale summaries are not implementation truth.

## Non-Negotiable Direction

Apex GUI is not an egui theme.

Apex GUI is an opinionated Rust GUI framework/toolkit layer that currently uses egui/eframe as runtime steel while Apex owns the public API, widget vocabulary, interaction model, shell shape, theme tokens, responses, and developer experience.

Do not fork egui in this phase.

Do not avoid real work with cheap wrappers either. A wrapper that just renames `ui.button()` is not Apex. Apex must start owning behavior, state language, theme tokens, responses, and shell structure from the first slice.

## Fork Policy For This Phase

Forking is forbidden in Phase 01.

Allowed:

- use `eframe` for app runner/runtime
- use `egui` internally inside Apex adapter/modules
- use egui `Painter` internally for custom widget drawing
- expose a deliberately named raw escape hatch only if needed for demo survival

Forbidden:

- forking egui/eframe
- patching egui internals
- app/demo code calling raw egui widgets as the primary UI path
- treating raw egui behavior as Apex behavior without wrapping and naming the policy
- custom `wgpu` callbacks unless a specific blocked rendering need is proven inside this phase

## No Docs-First Rule

Do not spend this phase writing user docs, README polish, API guides, generated docs, website content, or CI docs.

During active build:

- source first
- runnable demo first
- local tests first
- semantic goldens after behavior is accepted
- comments only where needed to explain non-obvious source decisions

After phase close, and only after operator acceptance, a seal pass may add proper Rustdoc, examples, README notes, release notes, and optional CI/release checks if the operator explicitly wants them.

## Target Repo Shape

Use this shape unless the repo reality forces a smaller equivalent:

```text
Cargo.toml
crates/
  apex-ui/
    Cargo.toml
    src/
      lib.rs
      app.rs
      frame.rs
      ui.rs
      response.rs
      action.rs
      intent.rs
      prelude.rs
      egui_bridge/
        mod.rs
        runtime.rs
        style.rs
      theme/
        mod.rs
        tokens.rs
        palette.rs
        density.rs
      shell/
        mod.rs
        app_shell.rs
        top_bar.rs
        status_bar.rs
      widgets/
        mod.rs
        button.rs
        panel.rs
        card.rs
        status_chip.rs
        section_header.rs
      testing/
        mod.rs
        semantic_snapshot.rs
apps/
  apex-demo/
    Cargo.toml
    src/
      main.rs
tests/
  goldens/
    semantic/
    behavior/
```

If the initial patch must be smaller, preserve the boundaries:

```text
apex-ui owns framework APIs.
apex-demo consumes Apex APIs.
egui stays inside Apex internals.
```

## Public API Target

By the end of Phase 01, app code should look like Apex, not egui.

Example target shape:

```rust
use apex_ui::prelude::*;

struct DemoApp {
    counter: usize,
}

impl ApexApp for DemoApp {
    fn update(&mut self, frame: &mut ApexFrame) {
        frame.shell("Apex Demo", |ui| {
            ui.top_bar(|ui| {
                ui.primary_action("Create").on_click(ApexAction::named("create"));
                ui.quiet_action("Settings").on_click(ApexAction::named("settings"));
            });

            ui.panel("Counter", |ui| {
                ui.section_header("First real Apex widget slice");

                if ui.primary_button("Increment").activated() {
                    self.counter += 1;
                }

                ui.status_chip(format!("Count: {}", self.counter));
            });
        });
    }
}
```

The exact names may change during source work. The law is that the demo must use Apex concepts, not raw egui widget calls for the main path.


## Current Adjustment Notes

- 2026-07-06: Operator accepted the first runnable starter shell. Phase 01 checkboxes may now be checked only where the real source, local green result, and operator pass agree.
- 2026-07-06: `ApexUI_phase_1_base_2.tar` carried workspace/app files but omitted `crates/apex-ui` even though `Cargo.toml` referenced it. The next patch restores the missing crate source and continues from the accepted starter shell instead of requiring patch hand edits.
- 2026-07-06: Demo status copy must avoid phase/proof language in normal UI. The status chip is corrected to product-facing starter-shell copy before locking semantic goldens.
- 2026-07-06: `ApexUI_phase_1_base_3.tar` is confirmed caught up and green. Next source slice makes the disabled Settings action visible in the demo shell and locks that semantic surface; operator disabled-state pass is pending for this patch.
- 2026-07-06: Operator confirmed the disabled Settings action is green in `ApexUI_phase_1_base_4.tar`. The next source slice promotes response state priority into Apex-owned `ApexInteractionState` language and locks that behavior with a semantic golden.

## Must Build In This Phase

### 1. Workspace And Runnable Demo

- [x] Rust workspace exists or is updated without unrelated cleanup.
- [x] `crates/apex-ui` exists as a library crate.
- [x] `apps/apex-demo` exists as a runnable demo binary.
- [x] Demo launches through `cargo run -p apex-demo`.
- [x] Demo window uses eframe internally through Apex-owned run/app wiring.
- [x] Demo app code imports `apex_ui::prelude::*` or equivalent.
- [x] Demo app code does not build its main interface from raw `egui::Ui` calls.
- [x] Workspace builds with `cargo build`.

### 2. Apex App Facade

- [x] `ApexApp` exists as the app-facing trait.
- [x] `ApexFrame` exists as the frame/update context.
- [x] `ApexUi` exists as the app-facing UI wrapper.
- [x] `ApexResponse` exists as the app-facing response wrapper.
- [x] `ApexAction` or equivalent exists for named action/command output.
- [x] Public API hides egui types from ordinary app code where practical.
- [x] Any raw egui escape hatch is explicit, isolated, and not used for the main demo path.
- [x] Public prelude exports the intended Phase 01 API.

### 3. Theme And Tokens

- [x] `ApexTheme` exists.
- [x] Semantic color tokens exist.
- [x] Spacing tokens exist.
- [x] Radius tokens exist.
- [x] Typography/text-style tokens exist or a narrow starter equivalent exists.
- [x] Density mode exists or is explicitly deferred inside source comments/handoff.
- [x] Theme is applied through Apex runtime, not scattered through demo code.
- [x] No hardcoded widget color soup appears in app/demo code.
- [x] Apex widgets read theme/tokens from Apex-owned state or context.

### 4. First Widget Set

- [x] `primary_button` exists and returns an `ApexResponse`.
- [x] `quiet_button` or secondary/quiet action equivalent exists.
- [x] Button behavior distinguishes default, hovered, pressed/active, focused, and disabled where egui exposes enough state.
- [x] `panel` or `ApexPanel` exists.
- [x] `card` or simple raised surface exists.
- [x] `status_chip` exists.
- [x] `section_header` or equivalent small hierarchy primitive exists.
- [x] Widgets are custom Apex widgets/wrappers with Apex tokens, not raw egui defaults renamed.
- [x] Widget modules stay small and focused.

### 5. First Shell

- [x] `frame.shell(...)` or equivalent exists.
- [x] Top bar area exists.
- [x] Main content area exists.
- [x] Bottom/status strip exists or a phase-approved starter equivalent exists.
- [x] Shell does not fake a full commercial app.
- [x] Shell demonstrates real Apex composition: top action area, body panel/card, status output.
- [x] Demo remains clean at normal desktop sizes.
- [x] Demo does not show debug phase labels, raw egui labels, raw internal IDs, or proof copy as normal UI.

### 6. Interaction Ownership

- [x] `activated()` or equivalent Apex response method exists.
- [x] Click/activation logic is named in Apex terms.
- [x] Disabled behavior is represented in Apex terms where present.
- [x] Action emission is represented in Apex terms where present.
- [x] Button action handling does not require app code to inspect raw egui response details for normal use.
- [x] Keyboard/focus behavior is not claimed beyond what is actually implemented.

### 7. Tests And Behavior Locks

Tests are added after source behavior exists. Goldens are added after the operator accepts the visible feel.

- [x] Unit tests cover theme token defaults or construction.
- [x] Unit tests cover `ApexAction`/response behavior where practical.
- [x] Unit tests cover semantic snapshot output where practical.
- [x] Demo build is verified locally.
- [x] Semantic golden exists for the Phase 01 demo shell once operator accepts the UI shape.
- [x] Behavior golden or snapshot exists for the basic counter/action path once accepted.
- [x] Golden churn is not used to hide unstable behavior.

### 8. Handoff And Patch Discipline

- [x] `.handoff` records the active phase as Apex GUI Phase 01.
- [x] `.handoff` records newest tar/base used.
- [x] `.handoff` records changed files and exact verification commands.
- [x] `.handoff` records what is real, what is deferred, and what must not be claimed.
- [x] `.handoff` records operator manual checks for the demo app.
- [x] Patch is generated only from the current source after reassessment.
- [x] Patch passes `git apply --check <patch-file>` against the newest base when a patch artifact is produced.

## Avoid

A checked box here means the forbidden shortcut was verified absent.

- [x] Do not fork egui/eframe in Phase 01.
- [x] Do not create a theme-only crate and call it a framework.
- [x] Do not make app/demo code depend on raw egui as the normal path.
- [x] Do not create a fake commercial demo with fake workflows.
- [x] Do not add docs/README/Rustdoc polish before the source behavior is usable.
- [x] Do not add CI.
- [x] Do not add guard scripts.
- [x] Do not add doc guards, phase guards, preservation bots, workflow gates, or policy theater.
- [x] Do not add custom renderer/wgpu code just to look advanced.
- [x] Do not add paid packages, unlicensed fonts, brand-clone visuals, or unclear assets.
- [x] Do not scatter raw colors/spacing across widgets.
- [x] Do not build one giant `lib.rs` landfill.
- [x] Do not write huge comment walls during unstable source exploration.
- [x] Do not mark the phase done because it compiles while the demo still uses raw egui for primary behavior.
- [x] Do not mark the phase done because a screenshot looks okay.
- [x] Do not mark the phase done because tests pass while the operator has not used the real demo.

## Verification Commands

Run the exact commands that match the repo after reassessment. Expected Phase 01 commands:

```bash
cargo fmt --check
```

```bash
cargo build
```

```bash
cargo test
```

```bash
cargo run -p apex-demo
```

If package names differ, report the real commands in `.handoff` and in the patch response.

Do not add CI commands. These are local verification commands only.

## Manual Operator Checks

The operator must launch the real demo app and confirm:

- [x] Demo opens as a native eframe window.
- [x] Demo looks like Apex starter UI, not stock egui defaults.
- [x] Top bar/main panel/status strip or accepted starter shell areas are visible.
- [x] Primary button increments or triggers a real visible state change.
- [x] Status chip or status area reflects the real state change.
- [x] Window resize does not immediately trash the layout.
- [x] No fake product workflow is shown.
- [x] No raw egui demo clutter is visible.
- [x] App code for the demo is readable as Apex API usage.
- [x] Disabled Settings action is visibly disabled and does not change Count or Last action.

## Close Gate

Phase 01 closes only when all of these are true:

- [x] `apex-ui` crate exists and builds.
- [x] `apex-demo` app exists and launches.
- [x] App/demo code uses Apex facade and Apex widgets for the primary UI path.
- [x] Apex facade owns app/frame/ui/response/action vocabulary.
- [x] Theme/tokens are used by widgets.
- [x] First widget set works through real interactions.
- [x] First shell works in a real window.
- [x] Local tests pass.
- [x] Semantic/behavior goldens are added after operator acceptance.
- [x] Operator has used the real demo and accepted the Phase 01 feel.
- [x] No fork was introduced.
- [x] No CI was added.
- [x] No guard theater was added.
- [x] No docs-first work was used as completion proof.

## Post-Close Seal Only

After the operator explicitly closes Phase 01, a separate seal patch may add:

- [ ] Rustdoc for public Phase 01 APIs.
- [ ] A short README/API quickstart for the closed Phase 01 surface.
- [ ] Example comments showing intended usage.
- [ ] Comment/header cleanup to match the law doc.
- [ ] Optional local release-check script if requested.
- [ ] Optional CI only if the operator explicitly promotes the closed phase into an official release lane.

The post-close seal must not add new behavior. If behavior changes are needed, Phase 01 is not closed yet.

## Before Starting Phase 02

- [ ] Phase 01 close gate is complete.
- [ ] Any post-close docs/Rustdoc/CI seal requested by the operator is complete.
- [ ] `.handoff` records Phase 01 as closed.
- [ ] `.handoff` records the exact next recommended Phase 02 target.
- [ ] Newest green tar/base is produced by the operator.

## Recommended Phase 02 Target

Do not start this until Phase 01 is closed.

Likely next phase:

```text
Apex GUI Phase 02: Interaction Policy, Command Registry, And Real App Shell Hardening
```

Expected focus:

- command registry
- shortcut map
- modal/popover/toast layer
- stronger focus/disabled/confirmation behavior
- shell persistence starter
- more serious widget states
- no fork unless Phase 01 exposes a hard wall
