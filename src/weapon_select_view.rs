use egui::Layout;

use crate::static_data::{WEAPONS, WEAPON_CLASSES};

#[derive(Default)]
pub struct WeaponSelectView {
    pub is_open: bool,
    pub selected_weapon: Option<String>,
    pub selected_weapon_class: Option<String>,
    id: String,
}

impl WeaponSelectView {
    pub fn new() -> Self {
        let random_id: u8 = rand::random();

        Self {
            is_open: false,
            selected_weapon: None,
            selected_weapon_class: None,
            id: format!("weapon_select_view_{}", random_id),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, opened_context: &str) {
        egui::Window::new(format!("{} Weapon Select", opened_context))
            .title_bar(true)
            .open(&mut self.is_open)
            .id(self.id.clone().into())
            .show(ui.ctx(), |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.selectable_value(&mut self.selected_weapon_class, None, "All");

                    for (weapon_class, weapons) in WEAPON_CLASSES.iter() {
                        ui.selectable_value(
                            &mut self.selected_weapon_class,
                            Some(weapon_class.clone()),
                            weapon_class,
                        );
                    }
                });

                ui.separator();

                ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                    ui.vertical(|ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            if let Some(selected_weapon_class) = &self.selected_weapon_class {
                                for weapon in WEAPON_CLASSES.get(selected_weapon_class).unwrap() {
                                    ui.selectable_value(
                                        &mut self.selected_weapon,
                                        Some(weapon.clone()),
                                        weapon,
                                    );
                                }
                            } else {
                                for weapon in WEAPONS.iter() {
                                    ui.selectable_value(
                                        &mut self.selected_weapon,
                                        Some(weapon.clone()),
                                        weapon,
                                    );
                                }
                            }
                        });
                    });
                });
            });
    }
}
