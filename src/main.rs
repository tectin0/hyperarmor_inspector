// #![windows_subsystem = "windows"]

use eframe::{egui, NativeOptions};
use egui::ViewportBuilder;
use hyperarmor_inspector::*;

fn main() -> eframe::Result {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .with_module_level("hyperarmor_inspector", log::LevelFilter::Debug)
        .init()
        .unwrap();

    log::info!("Starting Hyperarmor Inspector");

    let native_options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };

    match eframe::run_native(
        "Minimum Poise Calculator",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    ) {
        Ok(_) => {
            log::info!("Shutting down Hyperarmor Inspector");
            Ok(())
        }
        Err(e) => {
            log::error!("Error: {}", e);
            Err(e)
        }
    }
}

#[derive(Default)]
struct App {
    poise_data_view: poise_data_view::PoiseDataView,
    equipment_view: equipment_view::EquipmentView,
    one_attack_plot_view: one_attack_plot_view::OneAttackPlotView,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            poise_data_view: poise_data_view::PoiseDataView::new(),
            equipment_view: equipment_view::EquipmentView::new(),
            one_attack_plot_view: one_attack_plot_view::OneAttackPlotView::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_label(self.equipment_view.is_open, "Equipment")
                    .clicked()
                    .then(|| {
                        self.equipment_view.is_open = !self.equipment_view.is_open;
                    });

                ui.selectable_label(self.poise_data_view.is_open, "Poise Data")
                    .clicked()
                    .then(|| {
                        self.poise_data_view.is_open = !self.poise_data_view.is_open;
                    });

                ui.selectable_label(self.one_attack_plot_view.is_open, "One Attack Plot")
                    .clicked()
                    .then(|| {
                        self.one_attack_plot_view.is_open = !self.one_attack_plot_view.is_open;
                    });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.equipment_view.is_open {
                self.equipment_view.show(ui);
            }

            if self.poise_data_view.is_open {
                self.poise_data_view
                    .show(ui, &self.equipment_view.incoming_poise_damage_multiplier);
            }

            if self.one_attack_plot_view.is_open {
                self.one_attack_plot_view.show(
                    ui,
                    &self
                        .equipment_view
                        .is_changed_incoming_poise_damage_multiplier,
                    &self.equipment_view.incoming_poise_damage_multiplier,
                    &self.equipment_view.hyperarmor,
                    &self.equipment_view.armor_poise,
                );
            }
        });
    }
}
