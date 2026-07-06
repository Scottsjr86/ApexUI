# Apex GUI Workflow Header

**Use this block at the top of a fresh chat when handing over a new repo tar, an active Law doc, and the current Phase plan.**

This header is the short operating contract. It is not the full Law doc and it is not a phase checklist. The Law doc owns permanent architecture and conduct rules. The active Phase plan owns the scoped work, checkboxes, guard rails, completion requirements, and local verification steps.

---

## 0. Drop-In Truth Sync

Before writing code, patching files, or proposing implementation, sync these facts from the current repo and docs:

```text
Project: Apex GUI
Intent: MIT Rust GUI framework / commercial-grade app toolkit
Current base: newest operator-provided green tar only
Active Law doc: <path/to/apex_gui_law.md>
Active Phase plan: <path/to/current_phase_plan.md>
Active phase: <phase name and number>
Current handoff file: .handoff, if present
Patch format: downloadable patch artifact by default
Inline diffs: only if artifact delivery fails or operator explicitly asks
CI: do not add
Guard scripts: do not add
Fork policy: no egui fork unless the active phase proves a real engine wall
```

The newest green tar is the source of reality. Chat memory, older tar bases, prior extractions, previous patch attempts, stale summaries, screenshots, and remembered line numbers are not implementation truth.

---

## 1. Mission Lock

Apex GUI is not a cute dev panel, not a theme pack, and not “polished egui.”

Apex GUI is an MIT Rust GUI framework and commercial-grade app toolkit meant to build serious cross-platform desktop software with a premium, ownable visual and interaction language.

The long-term goal is that users build through Apex APIs and Apex concepts, not raw egui. egui/eframe may remain the runtime underneath until Apex laws require engine surgery, but the product experience, component vocabulary, interaction model, shell system, theme system, and user-facing workflow belong to Apex.

Commercial target pressure:

- Adobe-class visual polish
- Autodesk-class pro-tool seriousness
- Qt-class app-structure ambition
- Rust-native MIT licensing posture
- cross-platform desktop first
- fast local development the whole way
- usable and testable at every phase

No cheap workaround should be used to avoid real engineering. No fork should be used to avoid thoughtful layering. Cut clean, prove behavior, then seal it.

---

## 2. Current-Base Rule

For every patch:

1. Use only the newest operator-provided green tar or current green working tree.
2. Retire every older base immediately when a newer base appears.
3. Inspect the real filesystem before editing.
4. Inspect the active Law doc.
5. Inspect the active Phase plan.
6. Inspect `.handoff` if it exists.
7. Inspect every file that will be changed.
8. Generate the patch from the edited real source, not from memory.
9. Verify the patch against the same newest base.

Do not stack work on stale extractions. Do not produce patches from remembered source. Do not assume old behavior still exists unless the current repo proves it.

---

## 3. Patch Scope Rule

Each patch must belong to the active phase and one focused slice.

Allowed in a patch:

- source needed by the active slice
- tests needed to lock accepted behavior
- goldens/snapshots needed to lock accepted behavior
- `.handoff` updates needed for durable next-step truth
- phase-plan updates only when source-visible truth or operator-approved direction changed

Not allowed in a patch:

- unrelated cleanup
- drive-by refactors
- CI files
- guard scripts
- policy bots
- fake enforcement docs
- placeholders pretending to be complete behavior
- test-only proof with unusable product behavior
- docs-only closure after planning/setup phases

Patch scope must be boring, sharp, and reviewable.

---

## 4. Truth Sync Patch Flow

Every behavior patch follows this order:

### 1. Reassess

Read the current repo from disk and identify:

- what is real
- what is placeholder-only
- what is unsupported
- what belongs to Apex
- what still belongs to egui/eframe or another lower layer
- what the active Law doc forbids
- what the active Phase plan requires

### 2. Source Truth First

Define or update the narrow source-facing truth for the slice:

- request type
- response type
- state snapshot
- reducer/action
- widget contract
- theme token
- layout contract
- render command
- interaction policy
- error shape
- bridge/adapter seam

No split-brain paths. No old placeholder flow beside the real path unless the phase explicitly keeps it disabled and honest.

### 3. Build the Usable Slice

Implement the visible or behavioral feature so it can be used in the real app/demo/examples for that phase.

A slice is not complete because it compiles. A slice is not complete because a screenshot looks good. A slice is not complete because a test passed. It must be usable through the intended user path.

### 4. Operator Feel Pass

For visible GUI work, the operator uses the real app or demo and nitpicks the experience. Do not lock goldens before the accepted feel exists unless the operator requested test-first scaffolding.

### 5. Lock Behavior

After acceptance, add or update the smallest useful set of:

- unit tests
- integration tests
- interaction/state tests
- semantic goldens
- copy goldens
- layout goldens
- behavior goldens
- visual goldens only when the phase calls for visual lock

### 6. Handoff

Update `.handoff` with durable next-step truth:

- active phase
- newest base used
- patch scope
- what changed
- what is now real
- what remains placeholder/unsupported
- commands run
- commands still required
- operator manual checks
- tests/goldens status
- platform/package impact when relevant
- exact next action

Chat-only instructions are not durable enough. If the operator gives a new workflow instruction, copy it into `.handoff` or the appropriate doc when it matters for future patches.

---

## 5. Completion Guard

No proofs as completion. No deferring. No “good enough.” No checkbox theater.

A checkbox may be marked complete only when the feature or requirement is release-usable for that phase scope.

Done means:

- the user path exists
- the feature can be used from the intended app/demo surface
- visible states are honest
- errors are recoverable or clearly explained
- unsupported work is hidden or disabled truthfully
- tests/goldens lock the accepted behavior where required
- the operator has used and accepted visible GUI behavior
- `.handoff` records what changed and what remains

Not done:

- compiles but cannot be used
- demo-only proof with no integrated user path
- docs say it exists but source does not
- tests pass while the UI is fake, hidden, unreachable, or incomplete
- placeholder data appears as real product behavior
- known broken resize/focus/input states are renamed as polish
- rough edge is ignored instead of fixed, scoped, or explicitly blocked

If a feature would not be credible to release at that phase boundary, do not check the box.

---

## 6. No CI / No Guard Theater

This lane uses local source inspection, local commands, tests, goldens, demos/apps, and operator approval.

Do not add:

- GitHub Actions
- CI runner files
- badges
- CI gates
- preservation bots
- doc guard scripts
- phase guard scripts
- policy-enforcement scripts
- fake blockers that pretend to replace engineering proof

Build plans and phase plans may list local verification commands and closure requirements. They must not add CI machinery.

A future official release doc may define CI/release packaging only when the operator explicitly opens that lane.

---

## 7. Fork Policy

Default stance: no fork.

Apex should first own its world through:

- Apex app facade
- Apex UI wrapper
- Apex widgets
- Apex shell
- Apex themes/tokens
- Apex interaction policies
- Apex command system
- Apex render extensions
- Apex dev tools
- Apex tests/goldens

Fork egui only if a phase proves a real engine wall, such as:

- core event routing blocks Apex interaction law
- focus traversal cannot be made correct above egui
- Response semantics cannot represent Apex behavior
- text editing/IME internals block commercial-grade input
- paint/epaint primitives block required rendering quality or performance
- accessibility tree generation blocks required semantics
- layout internals block required app structure

No cheap workarounds to avoid real work. No fork just because wrapping is less glamorous.

If a fork is proposed, the patch must state:

- what exact wall was hit
- why the wrapper layer cannot solve it cleanly
- what subsystem must change
- how the fork will stay small
- how it affects updates from upstream egui
- what tests/goldens prove the new engine law

---

## 8. Documentation Style Rule

Build first, then document hard.

During active construction:

- keep modules small
- keep names plain
- write enough comments to prevent confusion
- do not bury unfinished design behind giant docs
- do not stop building to polish docs prematurely

Once behavior is accepted and goldens/tests are laid:

- upgrade module docs aggressively
- add page/module headers
- add user-facing examples where useful
- add warnings and usage notes
- explain invariants
- explain why boundaries exist
- explain what not to do

Long-term source style target after stabilization:

```text
more green explanation than source
small modules
clear public docs
strong examples
comment headers before tricky sections
warnings beside sharp edges
no magic behavior without notes
```

Apex should eventually read like a serious Rust library: practical source, heavy guidance, and no mystery traps.

---

## 9. Required Handoff Response Shape

Every patch handoff should end with:

```text
Verification summary:
- <what passed>
- <what failed or was not run>

Build command:
- <exact command>

Launch/demo command:
- <exact command>

Test commands:
- <exact commands>

Goldens:
- <status or review path>

Operator manual checklist:
- <real app/demo checks>

Unsupported / pending:
- <honest list>

No CI:
- confirmed no CI added or relied on

No guard theater:
- confirmed no guard scripts/policy gates added or relied on

Source commit message:
- <short git commit message for source changes>

Patch artifact:
- <download link>
```

Commit messages must name only what hit source steel: behavior, corrections, goldens, tests, and accepted UI adjustments. Do not put workflow, patch mechanics, or documentation-process chatter in source commit messages unless that is the actual changed surface.

Do not print inline unified diffs by default. Print them only when the artifact link fails, the operator asks for inline diffs, or artifact delivery is not possible.

---

## 10. Phase Relationship

This header travels with every fresh chat.

The Law doc defines permanent rules.
The active Phase plan defines current work.
The repo/tar defines current truth.
`.handoff` defines the next durable action.

When starting a fresh chat, the operator should provide:

```text
1. newest green repo tar
2. this Workflow Header
3. Apex GUI Law doc
4. current active Phase plan
5. any current .handoff if not inside the tar
```

The assistant/patch author must then reassess the repo and start from the active phase, not from memory.
