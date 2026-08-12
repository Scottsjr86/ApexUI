# Apex GUI Law

**Status:** draft law document for review  
**Lane:** Apex GUI, commercial-grade Rust GUI framework built on egui/eframe until engine surgery is earned  
**Purpose:** define the non-negotiable engineering, architecture, source, documentation, completion, and verification laws for Apex GUI.

This document is the law layer. It is not the patch workflow header. It is not a phase plan. It does not close features by itself.

A future fresh chat should receive:

```text
1. Apex GUI workflow header
2. This Apex GUI Law doc
3. The active phase plan
4. The newest green repo tar/base
```

The header explains how work moves. This law doc explains what work is allowed to be.

---

# 0. Mission Law

Apex GUI is not a cute dev tool, not a theme pack, and not a polished egui demo.

Apex GUI exists to become a commercial-grade, MIT-friendly Rust GUI framework for serious cross-platform desktop software.

The target class is:

```text
Adobe-grade creator tools
Autodesk-grade technical tools
Qt-grade commercial desktop apps
enterprise workstation tools
asset managers
catalog/workflow software
professional inspection/editing surfaces
local-first power-user applications
```

Apex GUI should be sexy, fast, durable, documented, and legally clean enough that someone can build and sell a real product with it.

Apex may use egui/eframe as the engine substrate while the framework is growing, but the long-term goal is that users think in Apex concepts, Apex components, Apex interaction policy, Apex layout, Apex shell, and Apex docs rather than raw egui.

---

# 1. Commercial-Grade Law

Apex must be judged against real desktop software, not against hobby demos.

A feature is not complete because it renders. A feature is complete when it is usable, understandable, testable, documented, and safe enough that a paying user could rely on it in the scoped release.

Apex must prioritize:

- clean visual language
- fast interaction
- predictable behavior
- stable keyboard/mouse flow
- accessible state where practical
- honest unsupported states
- strong defaults
- safe destructive actions
- good docs
- strong examples
- local tests and goldens
- commercial licensing hygiene

Apex must not hide missing engineering under styling.

---

# 2. Egui Relationship Law

Apex begins as an egui/eframe-powered framework, not as a fork.

Default posture:

```text
framework first
wrapper first
custom widget layer first
custom shell first
custom interaction policy first
custom render surfaces where needed
fork last
```

Allowed above egui:

- ApexApp facade
- ApexFrame wrapper
- ApexUi wrapper
- ApexResponse wrapper
- ApexTheme and tokens
- ApexInteractionPolicy
- ApexCommand system
- Apex widget library
- Apex shell/docking/pane system
- Apex model/view layer
- Apex devtools
- custom painter widgets
- custom egui-wgpu surfaces
- custom texture/thumbnail systems

Forking is allowed only when a real Apex law cannot be implemented cleanly above egui.

Forking is not allowed for ego, cosmetics, impatience, or because a wrapper feels less glamorous.

---

# 3. No Cheap Workaround Law

No forking unless earned. No cheap workarounds to avoid real engineering.

If egui blocks an Apex requirement, the patch author must choose one of these honest paths:

1. implement cleanly above egui
2. build a proper Apex abstraction around the limitation
3. create a focused experiment branch to prove the needed engine change
4. fork surgically when the limitation is real and documented
5. mark the requirement blocked with exact reason and operator approval

Forbidden workaround patterns:

- fake state that only looks right
- duplicate UI paths with different truth
- brittle timing hacks
- visual overlays that pretend behavior exists
- swallowing errors
- hiding broken actions behind vague disabled copy
- adding docs instead of source behavior
- adding guards instead of tests
- adding CI theater instead of local proof
- keeping old code alive because deleting it is inconvenient
- avoiding needed refactors until the source becomes landfill

If the right fix is hard, do the hard fix in the smallest honest slice.

---

# 4. One Truth Law

Every product behavior has exactly one source of truth.

Apex owns UI truth:

- layout state
- transient interaction state
- selection presentation
- focus state
- hover/pressed visual state
- view filters
- open panels/drawers
- local UI preferences where scoped
- command availability presentation
- friendly copy

The product/app using Apex owns product truth:

- project/catalog data
- persisted files
- destructive actions
- business behavior
- asset metadata
- model data
- external integrations
- job/process truth
- durable settings outside Apex UI scope

Apex may define typed UI contracts, adapters, models, and view snapshots. Apex must not pretend to own application data that belongs to the host app.

If a value decides real product behavior, it must come from the approved product seam, not from an Apex visual component.

---

# 5. Spine Boundary Law

Apex must maintain a clean spine between app/product logic and UI presentation.

Preferred path:

```text
App product state
  -> typed Apex snapshot/model
  -> Apex session/frame/context
  -> ApexUi/component
  -> user interaction
  -> typed Apex action/command/intent
  -> app handler/session/product logic
```

Forbidden paths:

```text
widget -> random product mutation
widget -> direct filesystem/business action
theme -> behavior decision
layout module -> product state change
renderer -> product state change
example/demo code -> framework law
raw egui escape hatch -> hidden Apex state fork
```

Escape hatches are allowed, but they must be explicit and named. Raw egui access must not become the normal path for Apex components.

---

# 6. Tiny Module Law

Apex source must be split before files become mixed-purpose junk drawers.

Every module should have a small job, a clear boundary, and obvious neighbors.

Prefer many focused modules over one giant file that knows too much.

Good module shapes:

```text
apex_app       = app facade and frame lifecycle
apex_core      = ids, geometry, color, events, state primitives
apex_theme     = tokens, typography, density, semantic visuals
apex_input     = shortcuts, focus helpers, pointer/keyboard policy
apex_widgets   = component library
apex_shell     = top bar, dock, panes, status, workspace
apex_model     = table/tree/list model-view contracts
apex_render    = custom surfaces, image/cache/render hooks
apex_access    = accessibility mapping helpers
apex_devtools  = inspectors, gallery, debug views
apex_examples  = examples that teach real usage
```

A module is too large when:

- it mixes rendering, interaction, model state, and product logic
- it has unrelated widgets packed together
- it requires scrolling through unrelated concepts to edit one feature
- comments are needed to explain why unrelated things live together
- tests cannot target it cleanly
- a user would not know where to look from the module name

Split early enough that the source stays boring.

---

# 7. Comment Header Law

Apex source should become heavily documented once behavior is real.

Build first. Prove behavior. Lock goldens. Then document the hell out of it.

The desired final style is standard-library energy:

```text
large useful module/page headers
clear user-facing explanations
warning notes where misuse is easy
small examples where helpful
source code kept compact and readable
comments that explain why, not noise that repeats what
```

After a module's behavior is locked, aim for approximately:

```text
3 parts explanation / headers / notes
1 part source
```

This is a target for mature, public-facing modules, not an excuse to delay building.

During early source discovery:

- comments may be lighter
- APIs may move
- modules may be rough
- docs may be TODO-light

After goldens and behavior locks:

- public items get proper Rust docs
- module headers explain purpose and boundaries
- examples show normal use
- warnings explain traps
- comments capture design intent
- confusing source gets refactored, not merely explained

Do not bury bad design under green comment fog.

---

# 8. Rustdoc Pro Law

When a phase is complete enough to seal, Apex must be documented like a serious Rust crate.

Required for public-facing APIs after phase closure:

- `//!` module docs
- `///` item docs
- examples for core user flows
- panic/error notes where relevant
- feature flag notes where relevant
- platform notes where relevant
- safety notes where relevant
- accessibility notes where relevant
- performance notes where relevant
- migration notes for changed public APIs

Docs must teach users how to use Apex, not narrate implementation history.

Bad docs:

```text
This function creates a button.
```

Good docs:

```text
Creates an Apex primary action button using the current interaction policy.
Use this for the dominant action in a panel or modal. Do not use more than
one primary action in the same local action row unless the actions are truly
equal choices.
```

Rustdoc is part of the product.

---

# 9. Source First, Docs Second Law

No docs-only closure after the planning/setup phase.

Implementation order for real feature phases:

1. source contract
2. source behavior
3. operator/app feel pass where visible
4. tests
5. goldens
6. docs/rustdoc hardening
7. release notes/closure spec

Docs can record decisions before code, but docs cannot close a feature after work begins.

A checkbox is not checked because the doc says the thing exists. It is checked because the app/source/tests/goldens prove the thing exists.

---

# 10. Completion Law

No proof-as-completion. No deferring. No “good enough.”

A phase checkbox can be checked only when the scoped behavior is real at that point of release quality.

A feature is complete only when:

- the scoped user can fully use it
- the UI does not lie
- unsupported states are honest
- errors are recoverable where practical
- source behavior is real
- tests cover the behavior that matters
- goldens lock accepted states where appropriate
- docs/rustdocs are updated after behavior stabilizes
- operator has used the real app for visible behavior
- release-blocking rough edges are fixed or explicitly scoped out

If a user could not reasonably use the feature in a release at that point, the box stays unchecked.

Screenshots alone do not complete a feature.

Tests alone do not complete a visible feature.

Docs alone do not complete a feature.

Compile success alone does not complete a feature.

A demo that works only on happy path does not complete a feature.

---

# 11. Operator Reality Law

Visible GUI behavior must be judged in the real app by the operator.

The operator owns visual acceptance:

- feel
- density
- layout comfort
- interaction rhythm
- menu placement
- shell shape
- clutter tolerance
- final visual direction

The implementation can propose, but operator approval locks the accepted visible direction.

After operator acceptance, future patches must not undo that direction because of old docs, stale screenshots, previous phase names, inherited templates, or assistant memory.

Accepted visual decisions must be recorded durably in the active phase plan or `.handoff`.

---

# 12. Golden Law

Goldens lock accepted truth.

Use goldens for:

- semantic state
- user-facing copy
- layout contracts
- behavior transitions
- visual baselines after polish

Prefer stable semantic/copy/layout/behavior goldens during normal feature development. Use pixel/visual goldens once commercial visual polish is intentionally being locked.

Golden churn without intended source behavior change is a bug smell.

Goldens are not decoration. They are closure steel.

---

# 13. No CI Law

This lane does not add CI until the operator explicitly changes that law.

Do not add:

- GitHub Actions
- CI runner configs
- badges
- CI-only docs
- preservation bots
- fake gatekeeper workflows
- release automation theater

Use local verification:

- build commands
- test commands
- golden review commands
- examples
- manual operator checks
- package/release checks when scoped

Never report CI status for this lane.

---

# 14. No Guard Theater Law

Do not replace engineering proof with guard files.

Forbidden unless explicitly requested:

- doc guard scripts
- phase guard scripts
- policy gate scripts
- sentinel files that pretend to enforce architecture
- workflow blockers that do not build, test, run, or inspect real behavior

Allowed local helpers:

- build scripts
- run scripts
- test scripts
- golden update/review scripts
- example launchers
- profiling helpers
- screenshot capture helpers

A helper is valid only if it directly helps build, run, test, inspect, or profile the real app/framework.

---

# 15. Patch Scope Law

Every patch must have a bounded purpose.

A patch should normally touch one slice:

- one widget family
- one shell region
- one interaction policy area
- one model/view primitive
- one rendering primitive
- one docs/rustdoc hardening pass
- one test/golden lock pass

Avoid mixed patches that combine unrelated cleanup, redesign, behavior changes, docs rewrites, dependency changes, and test changes without an explicit reason.

A patch can include supporting files, but the mission must be obvious.

---

# 16. User Honesty Law

Apex UI must tell the truth in product language.

Controls must be:

- real
- disabled with clear reason
- hidden until supported
- or explicitly marked as a demo/example in demo code only

The framework and examples must avoid:

- fake success
- fake progress
- fake data in real app flows
- fake unavailable states
- debug labels in user flows
- raw panic/error dumps as normal UI
- internal command IDs as labels
- backend jargon in user-facing copy

If progress is unknown, show indeterminate work state. Do not fake percentages.

If a destructive action exists, make it explicit, opt-in, and confirmable.

---

# 17. Usability Before Polish Law

Basic usability is not polish.

These are blockers before visual polish can claim completion:

- clipped controls
- unreachable controls
- broken resize behavior
- unreadable contrast
- hidden primary actions
- inconsistent loading/empty/error states
- lost selection after normal navigation
- fake placeholders inside wired workflows
- UI freezes in expected workflows
- confusing destructive actions
- keyboard focus that disappears
- menus that hide required commands
- dev/debug clutter in user flows

Commercial polish cannot cover basic GUI defects.

---

# 18. Commercial Polish Last Law

Commercial polish happens after behavior works.

Polish may refine:

- typography
- spacing
- density
- color system
- surface hierarchy
- icons
- app identity
- transitions
- empty states
- error states
- screenshots/goldens
- docs presentation

Polish may not:

- hide unfinished behavior
- rename unsupported behavior into existence
- fake premium quality with animation over broken workflows
- introduce brand-copying
- add unlicensed assets
- add paid dependencies without approval
- sneak in new product scope

---

# 19. Performance Law

Apex must assume serious workloads.

The framework should avoid patterns that collapse at scale:

- unbounded textures
- unbounded decoded images
- full reloads every frame
- expensive formatting every frame
- hidden O(n) work on every UI tick
- blocking the UI thread for product work
- unbounded background jobs
- repaint storms
- layout thrash
- synchronous disk/network work in widgets

Commercial-grade UI must stay responsive under real data.

Framework APIs should encourage:

- virtualization
- bounded caches
- incremental snapshots
- visible/near-visible loading
- background work with UI-safe state updates
- predictable invalidation
- profiling hooks
- performance examples

---

# 20. Accessibility And Keyboard Law

Apex must not treat accessibility and keyboard navigation as decorative extras.

For mature components, define:

- focus behavior
- keyboard activation
- tab order expectations
- visible focus states
- labels/names where practical
- disabled reason where practical
- selection announcements where practical
- reduced-motion behavior where practical

A component is not commercial-grade if it only works for mouse-click happy path.

Accessibility quality may phase in, but public docs must be honest about what is supported.

---

# 21. Licensing Law

Apex is intended to be clean for commercial use.

Default license target:

```text
MIT unless operator explicitly changes it
```

Do not add dependencies, fonts, icons, assets, generated art, templates, sample data, or third-party controls that compromise commercial use.

Every new dependency should be checked for:

- license
- transitive risk where practical
- platform impact
- binary/package impact
- paid/commercial restrictions
- attribution requirements

No paid GUI control packs, proprietary assets, restricted fonts, or brand-copying visual assets without explicit operator approval.

---

# 22. Public API Stability Law

Apex can move fast before public release, but must become disciplined as phases seal.

Before a component is public/stable:

- APIs may change
- names may improve
- modules may split
- examples may be rewritten

After a component is sealed:

- breaking changes require a migration note
- rustdocs must be updated
- examples must compile
- tests/goldens must reflect the new behavior
- stale names must not linger beside new names

Do not preserve bad APIs out of fear. Do not break sealed APIs casually.

---

# 23. Example Integrity Law

Examples are product-facing documentation.

Examples must:

- compile
- show real Apex patterns
- avoid fake framework claims
- use clean names
- demonstrate recommended API style
- avoid raw egui unless teaching escape hatches
- show errors/disabled/empty states where relevant

A bad example teaches users to build bad apps.

---

# 24. Devtools Law

Apex should grow devtools because commercial frameworks win through feedback loops.

Allowed devtools:

- widget gallery
- theme editor
- layout inspector
- style inspector
- command registry viewer
- interaction inspector
- accessibility inspector
- performance HUD
- texture/cache stats
- golden review tools
- example launcher

Devtools must not become required runtime behavior for normal users.

---

# 25. Phase Plan Contract Law

Each working phase plan must follow the active workflow style:

- mission statement
- scope
- must-complete checklist
- avoid checklist
- verification checklist
- close gate
- before-next-phase gate
- pointer to laws and workflow header
- exact build/test/run expectations when known
- no CI language unless operator changes law
- no guard theater

Phase checkboxes are not suggestions. They are closure contracts.

Blocked items do not count as complete unless the operator explicitly narrows the phase.

---

# 26. Handoff Law

Every meaningful patch or phase transition must leave a durable handoff.

The handoff must record:

- newest base used
- active phase
- scope
- what changed
- what is real
- what is placeholder
- what is unsupported
- commands run
- commands still required
- tests/goldens status
- operator manual checks
- dependency/license impact
- next exact action
- no-CI/no-guard status

Chat-only decisions are not durable enough.

---

# 27. No Memory-Only Source Law

Never patch from memory.

Before editing:

- inspect the newest repo/base
- inspect files to be changed
- inspect active workflow header
- inspect this law doc
- inspect active phase plan
- inspect `.handoff` if present

Do not rely on remembered line numbers, remembered source shape, old tar bases, old assistant summaries, or stale screenshots.

Newest green base wins.

---

# 28. Release Slice Law

Every phase should leave the project more usable, not merely more theoretical.

A vertical, horizontal, or pie slice is acceptable if it produces real locked value.

Good slices:

- a usable shell
- a real button family with docs/tests/examples
- a real command system primitive
- a real theme token engine
- a real virtual list primitive
- a real asset wall prototype using Apex laws
- a real model/view table slice
- a real devtools gallery slice

Bad slices:

- giant architecture with no usable path
- proof-only renderer demo
- fake UI screenshots
- docs that claim future behavior
- half a dozen partial systems with no completed surface

---

# 29. Fork Trigger Law

Forking can be considered only when all are true:

- the requirement is part of Apex's commercial-grade mission
- the requirement cannot be implemented cleanly above egui
- a minimal experiment demonstrates the limitation
- the expected fork surface is identified
- the maintenance cost is accepted
- the operator approves the fork path
- the active phase plan records why the fork is necessary

Possible earned fork targets:

- response semantics
- focus traversal
- event routing
- style resolution
- text editing/IME
- accessibility tree internals
- epaint primitives
- render pipeline
- layout internals
- introspection/debug metadata

Forks must be surgical. A fork is not a bonfire.

---

# 30. Repo Law Summary

If only one section survives into someone's skull, make it this:

```text
Apex is not an egui theme.
Apex is an MIT-targeted commercial Rust GUI framework.
Build real usable slices.
No fake completion.
No docs-only closure.
No CI yet.
No guard theater.
No fork unless earned.
No cheap hacks to avoid hard work.
Tiny modules.
Heavy useful comments after behavior is locked.
Rustdoc like a pro crate.
Operator approval for visible UX.
Goldens lock accepted behavior.
Newest green base wins.
```
