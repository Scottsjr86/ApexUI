# Apex GUI Steel Map Quick Reference

> Cleaned and reformatted from the original steel list.  
> Purpose: a fast field guide for deciding what Apex can own on top of egui/eframe, what requires engine surgery, and where the long-term commercial-grade attack surface lives.

---

## 0. North Star

Apex GUI is **not** an egui theme.

Apex GUI is an opinionated Rust app framework powered by egui/eframe until Apex laws require engine surgery.

It owns:

- Widget vocabulary
- Interaction model
- App shell
- Theme system
- Command layer
- Pro-tool controls
- Visual polish
- Commercial-grade application patterns

egui/eframe remains the runtime only as long as it does not block Apex law.

> **Field note:** The goal is not “cute dev tool.” The target is Rust-native commercial GUI energy: Adobe/Autodesk/Qt-class polish without licensing traps.

---

## 1. Core Strategy

```text
Framework first.
Fork last.
No cheap workarounds.
No fork-for-ego.
No duct-tape avoidance of real engine work.
```

Build Apex as a hard framework layer first:

```text
user app
  -> Apex public API
  -> Apex widgets / shell / interaction policies
  -> egui adapter layer
  -> eframe runtime
```

Fork only when egui’s core behavior physically blocks Apex behavior.

---

## 2. The Cutoff Line

### Can Be Done Without Forking

These can be built as Apex framework/components on top of egui:

- Custom visual identity
- Custom widgets
- Custom app shell
- Custom panels, cards, top bars
- Custom asset walls
- Custom buttons, inputs, tabs
- Custom response wrappers
- Keyboard shortcut layer
- Command palette
- Modal/toast/popover system
- Docking/split panes
- Virtual lists/grids
- Theme tokens
- Painter-based custom rendering
- wgpu callbacks
- Image cache
- Thumbnail atlas
- Platform file-dialog wrappers
- App framework facade

> **Field note:** This is enough to make Apex feel like Apex, not “egui with lipstick.”

### Requires Fork or Deep Internal Patch

Fork when you need to change:

- Core event routing
- Core focus rules
- Widget response semantics
- Ui-level layout behavior
- Text editing internals
- IME internals
- Accessibility tree generation
- Paint pipeline architecture
- Shape/epaint primitives
- Font system internals
- Memory/cache lifecycle
- Multi-pass layout assumptions
- Animation frame lifecycle
- Global style-resolution model

> **Field note:** This is the reactor room. Open it only when the wrapper layer cannot enforce Apex law cleanly.

---

## 3. Fast Decision Matrix

| Area | No Fork Path | Fork Only If |
|---|---|---|
| Branding | Full Apex control | Almost never |
| Theme tokens | Full Apex control | Need selector/style engine in core |
| Buttons/cards/panels | Full Apex control | Need all egui widgets to obey Apex behavior |
| Asset wall | Full Apex control | Texture/paint/input internals block performance |
| Docking | Mostly Apex control | Need native/multi-window/deep popup control |
| Menus | Mostly Apex control | Need global native menu or input priority rewrite |
| Keyboard shortcuts | Full app control | Need core event priority changes |
| Focus | Apex widgets can own most | Need universal focus traversal rewrite |
| Layout | Strong wrapper control | Need size policies/multi-pass constraints |
| Text input | Polished wrappers | Need editor-grade IME/selection/rich text |
| Rendering | Strong with Painter/callbacks | Need new primitives/compositing/cache |
| Accessibility | Good for Apex widgets | Need internal tree/role rewrite |
| Dev inspector | Good external tooling | Need internal reflection hooks |
| Performance | Strong app-level control | Need dirty regions/paint/layout cache |
| Platform | Mostly external wrappers | Need DnD/IME/window behavior inside eframe |

---

# 4. Attack Surface by Area

Each area has two lanes:

- **Above egui:** what Apex can build without forking.
- **Fork trigger:** what would justify engine surgery.

---

## 4.1 Public API and Framework Facade

### Goal

Make users feel like they are using Apex, not raw egui.

### Above egui

Build:

- `ApexApp`
- `ApexFrame`
- `ApexUi`
- `ApexResponse`
- `ApexContext`
- `ApexTheme`
- `ApexInteractionPolicy`
- `ApexCommand`
- `ApexAction`

Example target API:

```rust
impl ApexApp for MyApp {
    fn update(&mut self, frame: &mut ApexFrame) {
        frame.shell("Vault", |ui| {
            ui.top_bar(|ui| {
                ui.primary_action("Import");
                ui.quiet_action("Settings");
            });

            ui.stage(|ui| {
                ui.asset_wall("assets", &mut self.assets);
            });
        });
    }
}
```

### Fork trigger

Fork only if you must:

- Replace `egui::Ui` semantics
- Change `Response` internals
- Change widget ID lifecycle
- Change memory storage model
- Change frame update phases
- Add first-class app/action routing inside the engine

> **Note:** Do not fork just for a prettier API. Wrap it.

---

## 4.2 Theme and Styling System

### Goal

Create a real design system, not a color swap.

### Above egui

Build token-driven styling:

```rust
pub struct ApexTheme {
    pub colors: ApexColors,
    pub spacing: ApexSpacing,
    pub radius: ApexRadius,
    pub strokes: ApexStrokes,
    pub typography: ApexTypography,
    pub shadows: ApexShadows,
    pub motion: ApexMotion,
}
```

Add:

- Semantic colors
- Density modes
- Compact/comfy layout modes
- Focus ring tokens
- Danger/action/quiet button tokens
- Surface elevation tokens
- Tile state tokens
- Code/editor colors
- Status colors
- Dark/light presets
- Theme serialization
- Theme editor panel
- Live theme reload

### Fork trigger

Fork only if Apex needs:

- Style selector engine
- Class-based styling
- Pseudo-states like `:hover`, `:pressed`, `:selected`
- Style inheritance
- Style invalidation cache
- Per-widget style classes baked into egui
- CSS-ish selector resolution
- Qt-like subcontrol styling

> **Note:** Without fork: strong design system. With fork: true styling engine.

---

## 4.3 Widget Library

### Goal

Own the commercial-grade surface area users actually touch.

### Above egui

Build:

- `ApexButton`
- `ApexIconButton`
- `ApexSplitButton`
- `ApexToggle`
- `ApexCheckbox`
- `ApexRadio`
- `ApexSlider`
- `ApexComboBox`
- `ApexSearchBox`
- `ApexTextField`
- `ApexTextArea`
- `ApexCard`
- `ApexPanel`
- `ApexTabBar`
- `ApexTree`
- `ApexTable`
- `ApexVirtualGrid`
- `ApexAssetWall`
- `ApexInspector`
- `ApexPropertyEditor`
- `ApexColorPicker`
- `ApexToast`
- `ApexModal`
- `ApexCommandPalette`
- `ApexBreadcrumb`
- `ApexStatusChip`
- `ApexProgress`
- `ApexSpinner`
- `ApexTimeline`
- `ApexLogConsole`

### Fork trigger

Fork only if you need:

- Core `Widget` trait expansion
- Custom `Response` fields engine-wide
- Built-in animation state
- Built-in accessibility metadata per widget
- Built-in validation states
- Built-in hover/focus lifecycle
- Built-in widget class names

> **Note:** If it can be expressed as an `egui::Widget`, do not fork.

---

## 4.4 Interaction Model

### Goal

Stop being “skin egui.” Own behavior.

### Above egui

Create:

```rust
pub struct ApexInteractionPolicy {
    pub click: ClickPolicy,
    pub focus: FocusPolicy,
    pub keyboard_nav: KeyboardNavPolicy,
    pub selection: SelectionPolicy,
    pub drag: DragPolicy,
    pub tooltip: TooltipPolicy,
    pub danger: DangerActionPolicy,
}
```

Control:

- Hover delay
- Tooltip delay
- Pressed visuals
- Long press
- Double click
- Selection rules
- Multi-select behavior
- Danger confirmation
- Keyboard shortcut routing
- Command dispatch
- Context menu behavior
- Drag/drop behavior
- Focus ring visibility

### Fork trigger

Fork only if you need to rewrite:

- Pointer capture internals
- Focus manager
- Keyboard traversal internals
- `Response` generation
- Event bubbling/capturing model
- Drag/drop dispatch
- Modal layer blocking
- Popup/input priority

> **Note:** Above egui controls Apex widgets. Fork controls the whole universe, including third-party egui widgets.

---

## 4.5 Layout Engine

### Goal

Move from “arranged widgets” to commercial app composition.

### Above egui

Wrap layout primitives:

- `ApexRow`
- `ApexColumn`
- `ApexGrid`
- `ApexSplit`
- `ApexDock`
- `ApexStack`
- `ApexOverlay`
- `ApexStage`
- `ApexInspectorLayout`
- `ApexResponsivePanel`

Add:

- Density presets
- Minimum touch targets
- Golden-ratio panel presets
- Dock/split persistence
- Layout serialization
- Responsive breakpoints
- Virtualized scrolling regions

### Fork trigger

Fork only if Apex needs:

- Multi-pass layout
- Constraint solver
- Flexbox/Grid engine integration
- Intrinsic measurement
- Qt-like size policies
- Layout invalidation
- Retained layout cache
- Subpixel layout
- Baseline alignment
- Deeper right-to-left layout support

> **Note:** Without fork: good app layouts. With fork: true commercial retained-layout engine.

---

## 4.6 Rendering and Paint Pipeline

### Goal

Make Apex visually expensive without becoming dependency soup.

### Above egui

Use:

- `egui::Painter`
- Custom meshes
- Custom shapes
- Texture atlases
- Image cache
- Offscreen generated textures
- `egui-wgpu` callbacks
- Shader surfaces

Build:

- `ApexSurface` renderer
- Shadow approximations
- Gradient primitives
- Blur fallback
- Image viewport
- Thumbnail atlas
- Canvas
- Timeline renderer
- Graph renderer

### Fork trigger

Fork only if Apex needs:

- New `epaint` primitive set
- Better rounded-rect tessellation
- Real shadow primitives
- Real blur primitives
- Gradient primitives
- Clip stack rewrite
- Layer compositing
- Blend modes
- Render pass architecture changes
- Text atlas behavior changes
- Texture management changes
- GPU cache invalidation
- Dirty region rendering

> **Note:** Painter/callbacks are enough for a lot. Fork only for new engine-level rendering laws.

---

## 4.7 Animation and Motion

### Goal

Motion should feel deliberate, not carnival-grade.

### Above egui

Build animation helpers:

- Hover lerp
- Open/close transitions
- Toast slide
- Modal fade
- Selection pulse
- Loading skeleton
- Spinner
- Progress tween
- Tab underline animation
- Panel reveal

### Fork trigger

Fork only if Apex needs:

- Frame scheduler rewrite
- Animation graph
- Built-in transitions
- Layout animation
- State transition lifecycle
- Retained animation cache
- Animation invalidation
- Reduced-motion global policy inside core

> **Note:** Without fork: polished motion. With fork: full animation engine.

---

## 4.8 Text Rendering and Editing

### Goal

Avoid the glyph swamp until Apex actually needs to swim in it.

### Above egui

Wrap existing text controls:

- `ApexTextField`
- `ApexSearchBox`
- `ApexCodeInput`
- `ApexValidatedInput`
- `ApexPasswordInput`
- `ApexEditableLabel`

Add:

- Validation states
- Icons
- Clear buttons
- Placeholder behavior
- Error text
- Search chips
- Command palette input
- Custom focus visuals

### Fork trigger

Fork only if Apex needs:

- Text edit internals
- Cursor movement law
- Selection model
- IME composition changes
- Text shaping/cache changes
- Font fallback changes
- Rich text editing
- Multi-cursor editing
- Code editor primitives
- Bidi edge cases
- Clipboard behavior changes
- Undo/redo stack
- Input method abstraction

> **Note:** Text is a dragon pit. Wrap first, fork late.

---

## 4.9 Accessibility

### Goal

Commercial-grade means usable, navigable, and testable.

### Above egui

For Apex widgets:

- Label everything
- Use proper widget roles
- Support keyboard navigation
- Show focus rings
- Provide screen-reader names
- Support status announcements
- Test accessible labels

Build:

- `ApexAccessible`
- `ApexRole`
- `ApexLabelPolicy`
- `ApexFocusTraversal`
- `ApexA11yTestHarness`

### Fork trigger

Fork only if Apex needs:

- Accessibility tree generation rewrite
- Role mapping internals
- Focus announcements
- Custom widget semantic hooks
- Screen-reader event timing
- Accessible action metadata
- Tree diffing
- Selection announcements
- Table/tree semantics

> **Note:** Above egui gives accessible Apex widgets. Fork gives framework-wide semantics.

---

## 4.10 Docking, Panes, Windows, and Shell

### Goal

Apex should build serious workspaces, not toy panels.

### Above egui

Build:

- `ApexShell`
- `ApexTopBar`
- `ApexVaultMenu`
- `ApexSideRail`
- `ApexStatusBar`
- `ApexDockArea`
- `ApexSplitPane`
- `ApexInspectorPane`
- `ApexFloatingPanel`
- `ApexToolWindow`
- `ApexWorkspace`

Add persistence:

- Save layout
- Restore layout
- Named panes
- Dock/undock
- Split ratios
- Workspace presets
- Keyboard focus per pane

### Fork trigger

Fork only if Apex needs:

- Native multi-window integration
- Viewport lifecycle rewrite
- Global z-order model
- Popup/floating layer internals
- Modal blocking semantics
- Window drag/resize hit testing
- Custom titlebar deep integration

> **Note:** Above egui can make a pro shell. Fork only for deep windowing law.

---

## 4.11 Menus, Popovers, Modals, and Commands

### Goal

Commands become first-class, not random button callbacks.

### Above egui

Build command system:

```rust
pub struct ApexCommand {
    pub id: CommandId,
    pub label: String,
    pub shortcut: Option<ApexShortcut>,
    pub enabled: bool,
    pub dangerous: bool,
}
```

Wire:

- Menu bar
- Context menu
- Command palette
- Toolbar
- Shortcut dispatcher
- Right-click actions
- Danger confirmations
- Recent files
- Quick switcher

### Fork trigger

Fork only if Apex needs:

- Input priority rewrite
- Modal event blocking internals
- Global command routing inside core
- Shortcut resolution internals
- Popup positioning internals
- Menu navigation model rewrite
- Native menu bridge

> **Note:** Most command UX can live in Apex. Fork only for engine-wide priority law.

---

## 4.12 Tables, Model/View, and Data Binding

### Goal

Apex needs Qt-class data-heavy app capability.

### Above egui

Build:

- `ApexModel`
- `ApexTable`
- `ApexTree`
- `ApexList`
- `ApexVirtualList`
- `ApexVirtualGrid`
- `ApexSortModel`
- `ApexFilterModel`
- `ApexSelectionModel`
- `ApexDelegate`

Example:

```rust
pub trait ApexTableModel {
    fn row_count(&self) -> usize;
    fn column_count(&self) -> usize;
    fn cell(&self, row: usize, col: usize) -> ApexCell;
}
```

Add:

- Sorting
- Filtering
- Selection
- Virtualization
- Column resizing
- Frozen columns
- Row details
- Inline editors
- Delegates/renderers

### Fork trigger

Fork only if Apex needs:

- Retained item views
- Incremental diffing
- Built-in model invalidation
- Selection/focus integration inside core
- Accessible table semantics inside core
- Keyboard traversal internals

> **Note:** Above egui can create strong Apex model/view. Fork only if model/view becomes toolkit law.

---

## 4.13 Forms and Validation

### Goal

Enterprise apps need boring forms that never embarrass you.

### Above egui

Build:

- `ApexForm`
- `ApexField`
- `ApexValidator`
- `ApexFormState`
- `ApexDirtyTracker`
- `ApexErrorSummary`

Add:

- Required markers
- Inline errors
- Warning states
- Success states
- Submit lockout
- Dirty state
- Reset
- Field groups
- Wizard pages

### Fork trigger

Almost never.

Possible fork only if validation must be baked into `Response` or widget memory.

> **Note:** Forms belong in Apex, not the engine.

---

## 4.14 Drag and Drop

### Goal

Drag/drop must feel native enough to trust, especially for asset-heavy apps.

### Above egui

Build:

- `ApexDragSource`
- `ApexDropZone`
- `ApexDragPayload`
- `ApexDropPreview`
- `ApexFileDrop`
- `ApexReorderableList`
- `ApexAssetDropZone`

Support:

- Internal drag/drop
- File drop
- Visual previews
- Tile reordering
- Drag-to-panel
- Drag-to-tag

### Fork trigger

Fork only if Apex needs:

- Platform DnD integration rewrite
- Wayland/X11 behavior fixes
- Drop target dispatch changes
- Drag hover routing changes
- Pointer capture internals
- Multi-window drag/drop
- OS drag image support

> **Note:** Above egui is enough for in-app drag/drop. Native DnD may eventually demand surgery.

---

## 4.15 Image, Asset Wall, and Media

### Goal

This is the throne room for Apex’s visual credibility.

### Above egui

Build:

- `ApexAssetWall`
- `ApexAssetTile`
- `ApexThumbnailCache`
- `ApexTextureAtlas`
- `ApexLazyGrid`
- `ApexVisibleRange`
- `ApexSelectionOverlay`
- `ApexMetadataBadges`
- `ApexPreviewPane`
- `ApexImageViewer`

Must have:

- Virtualization
- Async thumbnail generation
- Texture eviction
- Smooth scroll
- Stable selection
- Keyboard navigation
- Drag/drop ingest
- Folder/file badges
- Loading skeletons
- Failed thumbnail fallback
- Batch selection
- Range select

### Fork trigger

Fork only if:

- egui texture lifecycle blocks performance
- Clip/paint ordering blocks wall rendering
- Input dispatch cannot support selection law
- Viewport/render callback integration gets ugly

> **Note:** Start no-fork. Use custom wgpu callback later for atlas-backed wall if needed.

---

## 4.16 Performance Engine

### Goal

Commercial GUI means performance is a feature, not a wish.

### Above egui

Build:

- `ApexFrameProfiler`
- `ApexPaintStats`
- `ApexTextureStats`
- `ApexUiTrace`
- `ApexLayoutTrace`
- `ApexWidgetBench`
- `ApexScreenshotBench`

Optimize:

- Thumbnail caching
- Visible range computation
- Texture atlas reuse
- Batched painter calls
- Avoid cloning huge data
- Avoid expensive formatting every frame
- Incremental search indexes
- Lazy metadata loading
- Debounced file watching

### Fork trigger

Fork only if Apex needs:

- Dirty region rendering
- Retained paint cache
- Partial repaint
- Incremental tessellation
- Shape cache lifecycle
- Font atlas eviction
- Paint command diffing
- Layout cache

> **Note:** Above egui handles app performance. Fork handles engine performance.

---

## 4.17 Developer Tooling

### Goal

Apex needs tools that make users trust the framework.

### Above egui

Build:

- Apex Showcase app
- Theme editor
- Widget gallery
- Layout debugger
- Style inspector
- Interaction inspector
- Command registry viewer
- Accessibility inspector
- Screenshot golden tests
- Performance HUD
- Live reload for theme JSON/TOML

### Fork trigger

Fork only if Apex needs:

- Built-in egui inspector expansion
- Widget tree metadata
- Stable widget reflection
- Layout tree reflection
- Paint tree reflection
- Debug hooks inside egui core

> **Note:** Great tooling can mostly be external. Fork only for deep reflection.

---

## 4.18 Testing and Quality Gates

### Goal

No proof-only closures. Behavior gets locked.

### Above egui

Build:

- Golden screenshot tests
- Interaction tests
- Keyboard navigation tests
- Theme regression tests
- Accessibility label tests
- Asset wall performance tests
- Layout persistence tests

Snapshot:

- Button states
- Panel layouts
- Asset tiles
- Menus
- Modals
- Forms
- Tables
- Dark/light themes
- High-DPI scaling

### Fork trigger

Fork only if Apex needs:

- Core response testing
- Event replay harness
- Layout replay harness
- Paint replay harness
- Deterministic frame tests
- Internal memory diff tests

> **Note:** Tests lock behavior. CI waits until the phase is officially sealed.

---

## 4.19 Platform Integration

### Goal

Cross-platform should feel intentional, not “works on my compositor.”

### Above egui

Use helper crates behind Apex wrappers for:

- File dialogs
- Clipboard
- Notifications
- System tray
- Open file/location
- Native theme detection
- Recent files
- Config dirs
- Window icons
- Multi-window helpers

Build:

- `ApexFilePicker`
- `ApexClipboard`
- `ApexNotify`
- `ApexRecentFiles`
- `ApexWindowState`

### Fork trigger

Fork only if Apex needs:

- Native menu bar
- Native titlebar fusion
- Platform DnD rewrite
- IME behavior rewrite
- Per-monitor DPI internals
- Multi-window event routing
- Platform accessibility adapters

> **Note:** Most platform work can stay outside the fork. Don’t open the engine unless the platform layer blocks Apex behavior.

---

## 4.20 Plugin and Extension System

### Goal

Apex should grow without everything becoming core.

### Above egui

Build plugin hooks:

```rust
pub trait ApexPlugin {
    fn name(&self) -> &'static str;
    fn register(&self, registry: &mut ApexRegistry);
    fn update(&mut self, ctx: &mut ApexPluginCtx);
}
```

Support:

- Custom widgets
- Custom commands
- Custom panels
- Custom inspectors
- Custom themes
- Custom asset renderers
- Custom importers

### Fork trigger

Fork only if Apex needs:

- Core plugin lifecycle
- Widget registry inside engine
- Style class registry inside engine
- Paint primitive registry inside engine
- Layout node registry inside engine

> **Note:** Plugin architecture belongs in Apex first.

---

# 5. Fork Attack Map

If Apex eventually maintains an `apex-egui` branch, these are the high-value areas to investigate.

## 5.1 `egui` Core

Targets:

- `Context`
- `Ui`
- `Response`
- `Memory`
- `Id`
- `LayerId`
- `Area`
- `Popup`

Purpose:

- Own interaction lifecycle
- Own focus model
- Own modal behavior
- Own response semantics
- Own debug metadata

Risk: **High**  
Power: **High**

---

## 5.2 `epaint`

Targets:

- `Shape`
- `Mesh`
- `TextShape`
- `Primitive`
- `Clipping`
- `Tessellation`

Purpose:

- Better shadows
- Gradients
- Blur
- Compositing
- Paint caching
- Advanced vector primitives

Risk: **Medium/High**  
Power: **High**

---

## 5.3 `egui-wgpu`

Targets:

- Renderer callbacks
- Texture manager
- Paint jobs
- Render pass setup
- Shader pipeline
- Surface configuration

Purpose:

- Custom GPU panels
- Atlas rendering
- Offscreen passes
- Blur/shadow passes
- Advanced effects

Risk: **Medium**  
Power: **High**

---

## 5.4 Text Edit Internals

Targets:

- `TextEdit`
- Text cursor state
- Selection state
- Galley/layout
- IME path
- Font cache

Purpose:

- Professional text input
- Code-editor-ish controls
- Better IME
- Custom caret/selection behavior
- Rich text editing

Risk: **High**  
Power: **High**

> **Warning:** Text is a goblin fortress. Enter late, with tests.

---

## 5.5 Layout Internals

Targets:

- `Layout`
- `Ui` sizing
- Available rect
- Child `Ui` allocation
- Scroll areas
- Grids

Purpose:

- Better size policies
- Multi-pass layout
- Intrinsic measurement
- Dock/constraint layouts

Risk: **High**  
Power: **High**

---

## 5.6 Accessibility Internals

Targets:

- AccessKit mapping
- Widget roles
- Label relationships
- Focus updates
- Announcements

Purpose:

- Screen-reader-grade custom widgets
- Tables/trees
- Asset wall accessibility
- Command palette accessibility

Risk: **Medium/High**  
Power: **Commercially important**

---

# 6. Commercial-Grade Feature Inventory

## Core UI

- Button
- Split button
- Icon button
- Toggle
- Checkbox
- Radio
- Slider
- Combo box
- Search box
- Text field
- Text area
- Password field
- Number input
- Color picker
- Date/time picker
- Progress
- Spinner
- Status chip
- Badge
- Breadcrumb
- Tabs
- Accordion
- Tree
- Table
- List
- Virtual grid
- Asset wall
- Image viewer
- Property inspector
- Log console
- Timeline

## Shell

- Top bar
- Menu bar
- Left rail
- Right inspector
- Bottom status bar
- Dock area
- Split panes
- Workspace presets
- Floating tool panels
- Modal layer
- Toast layer
- Command palette
- Settings/preferences surface

## App Systems

- Command registry
- Shortcut registry
- Action routing
- Theme registry
- Plugin registry
- Model/view layer
- Selection manager
- Undo/redo command stack
- Notification bus
- Job/task monitor
- File watcher integration
- Recent files
- Layout persistence

## Polish

- Smooth hover states
- Pressed states
- Focus rings
- Animated selection
- Loading skeletons
- Empty states
- Error states
- Tooltips
- Contextual help
- Onboarding panels
- Keyboard-first navigation
- High-DPI correctness
- Reduced-motion mode

## Dev Tools

- Widget gallery
- Theme editor
- Layout inspector
- Paint inspector
- Accessibility inspector
- Performance HUD
- Screenshot goldens
- Interaction replay
- Example apps
- Template generator
- Docs site

---

# 7. Sane Build Order

## Stage 1: Hard Wrapper, No Fork

Build:

- `ApexApp`
- `ApexFrame`
- `ApexUi`
- `ApexResponse`
- `ApexTheme`
- `ApexInteractionPolicy`

Goal:

> Users can build apps without touching raw egui.

---

## Stage 2: Component Arsenal

Build:

- Buttons
- Cards
- Panels
- Tabs
- Search
- Inputs
- Chips
- Toolbars
- Modals
- Toasts
- Command palette

Goal:

> Apex has a visible identity.

---

## Stage 3: Pro App Shell

Build:

- Dock
- Split panes
- Inspector
- Status bar
- Workspace layout persistence

Goal:

> Apex can build serious desktop tools.

---

## Stage 4: Data/Model Layer

Build:

- `ApexModel`
- `ApexTable`
- `ApexTree`
- `ApexVirtualGrid`
- `SelectionModel`
- `SortModel`
- `FilterModel`

Goal:

> Apex can build data-heavy apps.

---

## Stage 5: Performance Visuals

Build:

- Thumbnail atlas
- Image viewer
- Custom wgpu panels
- Paint stats
- Texture stats

Goal:

> Apex handles thousands of assets without choking.

---

## Stage 6: Dev Tools

Build:

- Widget gallery
- Theme editor
- Inspector
- Golden tests
- Template app
- Docs

Goal:

> Apex is usable by people who are not you.

---

## Stage 7: Surgical Fork, Only If Earned

Possible fork targets:

- `Response`
- Focus
- Style resolution
- `TextEdit`
- `epaint`
- Accessibility
- Layout

Goal:

> Apex laws become engine laws where wrappers are not enough.

---

# 8. Fork Protocol

Maintain a branch:

```text
apex-egui-experiments
```

Use it only for probes like:

- Can Apex rewrite focus cleanly?
- Can Apex add style classes cleanly?
- Can Apex add paint primitives cleanly?
- Can Apex expose better widget metadata?
- Can Apex improve text edit without making the engine brittle?

When a probe proves valuable:

1. Upstream it to egui if possible.
2. Hide it behind the Apex adapter if possible.
3. Maintain a small fork patch only if necessary.

> **Field note:** The fork is black-ops, not the main supply road.

---

# 9. Repo Line

Put this near the top of the project docs:

```text
Apex UI is not an egui theme.

Apex UI is an opinionated Rust app framework powered by egui/eframe.
It owns the widget vocabulary, interaction model, app shell, theme system,
command layer, and pro-tool controls.

egui remains the runtime until Apex laws require engine surgery.
```

---

# 10. Source Links from Original Notes

- [Qt Widgets documentation][1]
- [egui GitHub repository][2]
- [eframe docs.rs][3]
- [egui style docs.rs][4]
- [Qt Style Sheets reference][5]
- [egui README custom painting/accessibility context][6]
- [egui-wgpu docs.rs][7]

[1]: https://doc.qt.io/qt-6/qtwidgets-index.html
[2]: https://github.com/emilk/egui
[3]: https://docs.rs/eframe/latest/eframe/
[4]: https://docs.rs/egui/latest/egui/style/
[5]: https://doc.qt.io/qt-6/stylesheet-reference.html
[6]: https://github.com/emilk/egui/blob/main/README.md
[7]: https://docs.rs/egui-wgpu/latest/egui_wgpu/
