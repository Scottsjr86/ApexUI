# Apex GUI High-Level Project Overview

## Project Name

**Apex GUI**

## Project Position

Apex GUI is a long-term MIT-licensed Rust GUI framework project built to deliver commercial-grade, cross-platform, self-rendered desktop software without vendor lock-in, licensing traps, or enterprise-tax nonsense.

Apex GUI is not a cute developer toy, not a theme pack, not a thin egui skin, and not a proof-of-concept playground.

Apex GUI is an aggressive, production-minded GUI framework aimed at the same class of visual polish, interaction depth, and professional application capability users expect from commercial ecosystems such as Qt, Adobe-style creative tools, Autodesk-style workspaces, pro asset managers, inspectors, timelines, editors, control panels, and enterprise desktop software.

The starting engine may be egui/eframe, but the long-term goal is that application authors build against Apex concepts, Apex widgets, Apex interaction rules, Apex styling, Apex shell systems, Apex data/view systems, and Apex documentation. By the time the full roadmap matures, users should not feel like they are writing egui applications. They should feel like they are writing Apex applications.

---

## Core Mission

Build a polished, commercially usable, MIT-licensed Rust GUI framework that gives developers a clean, fast, beautiful, cross-platform toolkit for serious applications.

The project must prioritize:

- professional visual identity
- fast development loop
- strong Rust-native ergonomics
- cross-platform desktop behavior
- self-rendered controls and surfaces
- custom interaction law
- deep widget inventory
- commercial-grade polish
- durable documentation
- testable behavior at every stage
- no vendor lock-in
- no license ambush
- no dependency roulette that makes commercial use sketchy

Apex GUI should become the toolkit a Rust developer reaches for when they want to ship a real desktop app that looks expensive, feels responsive, and does not require paying for a commercial GUI stack or wrapping their app in a bloated web shell.

---

## Licensing Intent

Apex GUI should target a permissive license, preferably **MIT**, unless a later review proves a stronger dual-license strategy is useful.

The default posture is:

- developers can use Apex GUI commercially
- developers can ship closed-source applications with Apex GUI
- developers can modify Apex GUI
- developers can redistribute Apex GUI
- developers should not need to ask permission, pay runtime fees, or expose their application source code

Dependency selection must respect this goal. Any dependency that risks commercial clarity must be treated as a serious architectural concern.

---

## Relationship to egui / eframe

Apex GUI may begin on top of egui/eframe because egui already provides valuable cross-platform runtime machinery, immediate-mode ergonomics, rendering integration, event handling, and deployment momentum.

However, Apex GUI is not defined by egui.

The correct relationship is:

```text
Apex owns the framework experience.
egui/eframe may provide the runtime substrate until Apex outgrows it.
```

Apex should wrap, constrain, replace, and eventually hide direct egui usage behind Apex APIs.

Application authors should primarily interact with:

- `ApexApp`
- `ApexFrame`
- `ApexUi`
- `ApexResponse`
- Apex widgets
- Apex shell primitives
- Apex layout systems
- Apex theme tokens
- Apex interaction policies
- Apex command systems
- Apex model/view systems

Raw egui may exist as an escape hatch during early stages, but the public direction is Apex-first.

---

## Fork Policy

Apex GUI starts with a **no-fork-by-default** rule.

Forking egui/eframe is allowed only when a real Apex requirement cannot be met cleanly through public APIs, extension layers, custom widgets, renderer callbacks, or upstream-friendly patches.

This is not a cheap-skate workaround policy. The project must not avoid hard work just to avoid a fork.

The rule is:

```text
Do not fork for convenience.
Do not hack around real engine limits forever.
Fork only when Apex law needs engine law.
```

Acceptable reasons to fork later may include:

- core focus behavior blocks Apex interaction rules
- core response semantics prevent Apex widgets from behaving consistently
- layout internals prevent commercial-grade layout systems
- text editing internals block professional input/editor widgets
- rendering internals block required visuals or performance
- accessibility internals block serious semantic support
- platform integration cannot meet Apex requirements through public APIs
- engine-level metadata is required for devtools, testing, or inspection

Before forking, the team should attempt one or more of:

- Apex wrapper layer
- custom widget implementation
- egui public extension points
- `egui-wgpu` custom rendering path
- external support crate
- upstream contribution or issue
- isolated experimental branch

If a fork becomes necessary, it should be surgical, documented, and justified by locked behavior, not vibes.

---

## Project Style

Apex GUI is built in vertical or logical slices. Each phase should produce something usable, visible, testable, and closer to the final framework.

The project does not complete phases through proofs, demos, or screenshots alone.

A phase is not complete until behavior is locked.

A phase should follow this rhythm:

1. build the smallest useful vertical slice
2. test it manually and programmatically
3. refine interaction and visuals until they meet Apex standard
4. lock behavior with tests
5. document the public API Rust-style
6. write official release/closure docs
7. add CI only when the phase is truly sealed
8. move to the next phase

No CI gates, doc guards, or process armor should be added prematurely to build plans. CI belongs to phase closure, official releases, or sealed behavior lanes, not early exploration.

Apex should stay usable and testable the whole way.

---

## Completion Law

A phase is complete only when all of the following are true:

- the feature is usable in an example app
- behavior is deterministic enough to test
- key interactions are locked with tests
- visual states are reviewed and stabilized
- public API is named deliberately
- user-facing Rust docs are written
- closure notes describe what is now law
- CI or release checks exist only after behavior is sealed
- no temporary workaround is silently accepted as final architecture

A phase is not complete because:

- it compiles
- a proof worked
- a screenshot looked good once
- an example demo ran
- a workaround exists
- a private app can barely use it

The standard is production pressure, not prototype applause.

---

## Design Philosophy

Apex GUI should feel fast, sharp, and professional.

The visual direction should be clean, modern, confident, and customizable without becoming generic. It should support dense professional tools as well as polished consumer-facing applications.

The project must avoid the common traps:

- bubbly default-widget soup
- toy-toolkit spacing
- weak hover/focus states
- bad text input
- awkward menus
- fragile docking
- unstyled inspector panels
- laggy asset grids
- inaccessible custom controls
- demo-only widgets
- one-off painter hacks
- app-specific code pretending to be framework code

Apex should make serious app surfaces feel natural:

- top bars
- side rails
- inspectors
- dock areas
- timelines
- asset walls
- property editors
- command palettes
- modal layers
- toast layers
- status bars
- virtual lists
- tables
- trees
- forms
- media viewers
- workspace layouts

Apex should look like it belongs in paid commercial software.

---

## Architecture Direction

The ideal architecture is layered:

```text
Application
  ↓
Apex public framework API
  ↓
Apex shell / widgets / interaction / theme / model-view
  ↓
Apex egui adapter or future Apex runtime adapter
  ↓
Renderer / windowing / platform layer
```

Early stages may use egui/eframe directly underneath.

Long-term, Apex should own enough abstraction that egui becomes replaceable or invisible.

The framework should be structured around clear crates or modules:

```text
apex-core
apex-ui
apex-widgets
apex-shell
apex-layout
apex-style
apex-interaction
apex-command
apex-model
apex-render
apex-access
apex-platform
apex-devtools
apex-examples
```

Early development may start as a single crate or small workspace. The architecture should not be over-sharded before patterns stabilize.

---

## Public API Direction

Apex should expose a framework-level API instead of making developers write raw egui everywhere.

Target shape:

```rust
use apex_gui::prelude::*;

struct MyApp;

impl ApexApp for MyApp {
    fn update(&mut self, frame: &mut ApexFrame) {
        frame.shell("My App", |ui| {
            ui.top_bar(|ui| {
                ui.primary_action("Import");
                ui.quiet_action("Settings");
            });

            ui.stage(|ui| {
                ui.panel("Catalogs", |ui| {
                    ui.search_box("Search catalogs");
                });
            });
        });
    }
}
```

Raw egui access may remain available as an explicit escape hatch:

```rust
ui.raw_egui(|ui| {
    ui.label("temporary escape hatch");
});
```

The escape hatch should not become the main road.

---

## Interaction Ownership

Apex GUI must own interaction behavior, not just visuals.

Apex should define rules for:

- hover behavior
- pressed behavior
- focus behavior
- keyboard navigation
- command routing
- shortcut handling
- selection models
- drag and drop
- modal blocking
- danger confirmation
- tooltip timing
- context menus
- long-running task feedback
- disabled states
- validation states
- empty states
- loading states

This requires an explicit interaction policy layer:

```rust
pub struct ApexInteractionPolicy {
    pub focus: FocusPolicy,
    pub selection: SelectionPolicy,
    pub shortcuts: ShortcutPolicy,
    pub tooltips: TooltipPolicy,
    pub drag_drop: DragDropPolicy,
    pub danger_actions: DangerActionPolicy,
}
```

Apex must avoid merely repainting egui widgets. Apex widgets must encode Apex behavior.

---

## Rendering Direction

Apex should render its own controls and surfaces through custom widgets, custom paint commands, and later custom renderer paths where needed.

Early path:

- use egui painter for normal widgets
- use texture management for images and icons
- use custom widget rendering for Apex controls
- use renderer callbacks for heavy surfaces when needed

Later path:

- Apex-specific render abstraction
- custom GPU surfaces
- thumbnail atlases
- advanced image viewers
- custom timeline/canvas renderers
- optional backend abstraction if egui no longer fits

Rendering quality targets:

- crisp HiDPI output
- clean typography
- consistent spacing
- polished focus states
- smooth hover/selection visuals
- fast asset grids
- stable texture behavior
- professional dark and light themes
- optional density modes

Apex should be self-rendered and visually distinct.

---

## Accessibility Direction

Commercial-grade GUI means accessibility cannot be an afterthought.

Apex should support:

- keyboard navigation
- visible focus states
- semantic roles for custom widgets
- labels and descriptions
- accessible tables/lists/trees where possible
- screen reader support through available backend systems
- test tools for accessibility labels and traversal

Early accessibility should focus on Apex widgets. Later accessibility may require deeper engine integration if public APIs block correct semantic behavior.

---

## Documentation Standard

Apex documentation should be Rust-native and professional.

Each sealed public API should include:

- Rustdoc examples
- crate-level overview docs
- widget usage examples
- design notes where useful
- interaction rules
- accessibility notes
- performance notes for heavy controls
- migration notes when APIs change

Docs should be written after behavior stabilizes, not before exploration hardens into law.

Official release docs should describe:

- what the phase added
- what is now stable
- what behavior is locked
- what remains experimental
- what examples demonstrate the feature
- what tests protect the feature

No doc guards should be added just to satisfy process during active phase hacking.

---

## Testing Standard

Apex must stay usable and testable through every phase.

Testing should include:

- unit tests for pure logic
- interaction tests for widget behavior
- layout tests where deterministic
- screenshot/golden tests when stable enough
- example apps for manual operator review
- performance probes for heavy widgets
- accessibility tests for labels and traversal where possible

Tests should lock behavior, not prototype fantasies.

Apex should treat examples as living test surfaces, not marketing fluff.

---

## CI Policy

CI should not be sprayed onto active build plans before behavior exists.

CI belongs to:

- sealed phase closure
- official release lanes
- stable public API protection
- behavior that is ready to defend

No phase should be called complete until it has appropriate checks. But early exploratory slices should not be strangled with premature process armor.

The rule:

```text
Build first.
Lock behavior.
Rustdoc it.
Seal it.
Then CI it.
```

---

## High-Level Phase Roadmap

The detailed checklists will be created later. This section only defines the high-level phase map.

### Phase 0: Project Law and Skeleton

Create the initial Apex GUI workspace, license posture, architecture skeleton, naming law, public crate boundary, example app, and development rhythm.

Goal: a clean foundation that can compile, run, and host future slices without pretending to be finished.

Completion target: project skeleton is usable, examples run, public naming direction exists, and early docs describe the mission without locking unfinished APIs.

---

### Phase 1: Apex Facade

Create the first Apex-facing app API over egui/eframe.

Core concepts:

- `ApexApp`
- `ApexFrame`
- `ApexUi`
- `ApexResponse`
- `ApexContext`
- raw egui escape hatch

Goal: users can build a tiny app through Apex APIs without directly implementing `eframe::App`.

Completion target: a real example app runs through Apex facade, core behavior is tested, and public API docs exist for the sealed facade pieces.

---

### Phase 2: Design Tokens and Theme System

Build Apex visual law.

Core concepts:

- color tokens
- spacing tokens
- radius tokens
- stroke tokens
- typography tokens
- density modes
- dark/light themes
- theme application layer

Goal: Apex has its own visual identity and theme system independent from raw egui visuals.

Completion target: example app visibly uses Apex styling, tokens are documented, behavior is locked, and theme changes are testable.

---

### Phase 3: Core Widget Arsenal

Build the first serious Apex widgets.

Initial targets:

- buttons
- icon buttons
- cards
- panels
- chips
- section headers
- search box
- basic text field wrapper
- toggles / checkboxes
- progress / loading indicators

Goal: normal app screens can be built with Apex widgets instead of raw egui widgets.

Completion target: each widget has stable behavior, states, docs, examples, and tests.

---

### Phase 4: Interaction Policy Layer

Make Apex interaction law explicit.

Core targets:

- hover policy
- focus policy
- keyboard navigation
- selection policy
- command activation
- tooltip policy
- danger action confirmation
- disabled/loading/error states

Goal: Apex owns behavior, not just paint.

Completion target: interaction policy is visible in examples, tested in widgets, and documented as framework law.

---

### Phase 5: App Shell and Workspace

Build the commercial app frame.

Core targets:

- top bar
- menu region
- side rail
- main stage
- inspector pane
- status bar
- split panes
- workspace persistence
- modal/toast layers

Goal: Apex can host serious desktop applications with a polished professional shell.

Completion target: example app feels like a real product shell, layout behavior is locked, and shell APIs are documented.

---

### Phase 6: Command System and Shortcuts

Build the action backbone.

Core targets:

- command registry
- shortcut registry
- command palette
- menu integration
- toolbar integration
- context action integration
- enabled/disabled command states
- dangerous command handling

Goal: actions are declared once and surfaced consistently across UI.

Completion target: example app uses commands everywhere instead of one-off callbacks.

---

### Phase 7: Model/View and Data Controls

Build data-heavy commercial controls.

Core targets:

- selection model
- list model
- table model
- tree model
- virtual list
- virtual table
- sort/filter adapters
- cell delegates/renderers

Goal: Apex can handle large structured data without app-specific widget hacks.

Completion target: examples demonstrate tables, trees, lists, selection, sorting, filtering, and virtualization.

---

### Phase 8: Asset Wall and Media Surfaces

Build the flagship visual surface.

Core targets:

- virtual asset wall
- tile rendering
- thumbnail cache
- texture lifecycle
- selection overlays
- loading/failure states
- image preview
- metadata badges
- drag/drop hooks

Goal: Apex can power creative and catalog applications with thousands of visual assets.

Completion target: asset wall is fast, testable, documented, and usable in a serious app example.

---

### Phase 9: Docking, Panels, and Pro Workspace Tools

Build power-user workspace behavior.

Core targets:

- dock areas
- floating panels
- panel tabs
- split/merge behavior
- layout persistence
- inspector/property editor patterns
- workspace presets

Goal: Apex supports Autodesk/Adobe-style pro workspace structures.

Completion target: docking and panel behavior is stable enough for real applications.

---

### Phase 10: Advanced Rendering and GPU Surfaces

Push beyond normal widget painting.

Core targets:

- custom render surfaces
- renderer callback integration
- thumbnail atlas acceleration
- timeline/canvas rendering
- image viewport acceleration
- paint/performance instrumentation

Goal: Apex can render heavy custom visual surfaces without choking or looking generic.

Completion target: at least one heavy surface uses advanced rendering with locked behavior and documented extension points.

---

### Phase 11: Forms, Validation, and Settings Systems

Build enterprise-grade form behavior.

Core targets:

- forms
- validators
- dirty tracking
- error summaries
- field groups
- settings panels
- preference pages
- reset/apply/cancel flows

Goal: Apex can build serious configuration and data-entry surfaces.

Completion target: examples show robust form flows with validation and documented state behavior.

---

### Phase 12: Accessibility and Keyboard-First Hardening

Harden accessibility and keyboard behavior across Apex.

Core targets:

- accessibility roles
- labels/descriptions
- traversal tests
- keyboard-only operation
- focus visibility
- accessible table/list patterns
- reduced motion mode

Goal: Apex custom widgets are not just pretty pixels. They are usable controls.

Completion target: core widgets and shell surfaces meet the project accessibility standard.

---

### Phase 13: Devtools and Inspection

Build tools that make Apex pleasant to use and debug.

Core targets:

- widget gallery
- theme editor
- layout inspector
- paint inspector
- command registry viewer
- interaction state viewer
- performance HUD
- accessibility inspector

Goal: Apex becomes easy to develop with and easy to debug.

Completion target: devtools are available as an example app or optional feature crate.

---

### Phase 14: Packaging, Templates, and User Adoption

Make Apex easy to start with.

Core targets:

- starter template
- example applications
- crate-level docs
- tutorial docs
- migration guides
- release notes
- versioning policy
- contribution policy

Goal: outside developers can use Apex without reading the source code like ancient ruins.

Completion target: new users can create an Apex app quickly and understand the framework model.

---

### Phase 15: Engine Surgery Review

Review whether egui/eframe still fits.

This phase decides whether Apex continues as a framework over egui, maintains surgical patches, upstreams changes, or begins deeper engine ownership.

Review targets:

- focus system
- response model
- layout limitations
- text editing limits
- renderer limitations
- accessibility limitations
- platform integration limits
- performance limits
- public API leakage

Goal: make a sober, evidence-based decision on forking or replacing engine internals.

Completion target: written technical decision record backed by locked behavior requirements and real limitations, not taste alone.

---

## Long-Term Success Definition

Apex GUI succeeds when a Rust developer can build and ship a polished cross-platform desktop application using Apex without feeling trapped between cheap-looking open-source widgets and expensive commercial frameworks.

Apex should eventually be able to support applications in the class of:

- creative asset managers
- catalog systems
- inspection tools
- engineering workspaces
- media tools
- data-heavy dashboards
- local-first productivity apps
- pro configuration software
- scientific/technical control rooms
- commercial desktop utilities

The framework should feel:

- fast
- polished
- stable
- documented
- customizable
- commercially clean
- Rust-native
- serious

The final goal is not to hide that Apex began on egui.

The final goal is that nobody using Apex has to care.

---

## Prime Directive

Apex GUI is built to ship serious software.

No toy defaults.
No fake completion.
No licensing fog.
No process theater before behavior exists.
No forking for ego.
No avoiding engine work when Apex law demands it.

Build the slice.
Make it usable.
Make it beautiful.
Lock the behavior.
Document it like a pro.
Seal it.
Move forward.
