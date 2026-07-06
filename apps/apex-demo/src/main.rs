use apex_ui::prelude::*;

mod demo_state;

use demo_state::DemoState;

#[derive(Default)]
struct DemoApp {
    state: DemoState,
}

impl ApexApp for DemoApp {
    fn update(&mut self, frame: &mut ApexFrame<'_>) {
        frame.shell("Apex Demo", |ui| {
            ui.top_bar(|ui| {
                if let Some(action) = ui
                    .primary_action("Create", ApexAction::labeled("create", "Create"))
                    .take_action()
                {
                    self.state.apply_action(&action);
                }

                if let Some(action) = ui
                    .quiet_action("Reset", ApexAction::labeled("reset", "Reset"))
                    .take_action()
                {
                    self.state.apply_action(&action);
                }

                ui.disabled_quiet_action("Settings", ApexAction::labeled("settings", "Settings"));
            });

            ui.panel("Core Forge", |ui| {
                ui.section_header("Apex facade is online");
                ui.card(|ui| {
                    ui.body_text(
                        "This surface is built through Apex APIs; egui stays inside the adapter.",
                    );

                    if let Some(action) = ui
                        .primary_action("Increment", ApexAction::named("increment"))
                        .take_action()
                    {
                        self.state.apply_action(&action);
                    }

                    let last_action = self.state.last_action().unwrap_or("none");
                    ui.status_chip(format!("Count: {}", self.state.counter()));
                    ui.status_chip(format!("Last action: {last_action}"));
                });
            });

            ui.status_bar(|ui| {
                ui.status_chip("Starter shell online");
                ui.status_text("Apex APIs drive the demo path");
            });
        });
    }
}

fn main() -> ApexRunResult {
    apex_ui::run_native("Apex Demo", DemoApp::default())
}
