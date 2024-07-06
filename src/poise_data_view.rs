use crate::{
    data::{ApplyMultiplier, WeaponPoiseDamage},
    static_data::POISE_DATA,
    weapon_select_view::WeaponSelectView,
};

#[derive(Default)]
pub struct PoiseDataView {
    pub is_open: bool,
    pub is_selected_weapon_change: bool,
    pub selected_weapon: Option<String>,
    pub selected_weapon_class: Option<String>,
    pub selected_poise_damage: Option<WeaponPoiseDamage>,
    pub weapon_select_view: WeaponSelectView,
}

impl PoiseDataView {
    pub fn new() -> Self {
        let weapon_select_view = WeaponSelectView::new();

        Self {
            is_open: true,
            is_selected_weapon_change: false,
            selected_weapon: None,
            selected_weapon_class: None,
            selected_poise_damage: None,
            weapon_select_view,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, incoming_poise_damage_multiplier: &Option<f64>) {
        ui.button("Select Weapon").clicked().then(|| {
            self.weapon_select_view.is_open = true;
        });

        if self.weapon_select_view.is_open {
            self.weapon_select_view.show(ui, "Incoming Poise Damage");
        }

        if let Some(selected_weapon) = &self.weapon_select_view.selected_weapon {
            self.selected_weapon = Some(selected_weapon.clone());
            self.weapon_select_view.selected_weapon = None;
            self.is_selected_weapon_change = true;
        }

        if let Some(weapon) = &self.selected_weapon {
            let weapon_poise_data = POISE_DATA.get(weapon).unwrap();

            match incoming_poise_damage_multiplier {
                Some(incoming_poise_damage_multiplier) => {
                    let weapon_poise_damage =
                        weapon_poise_data.apply_multiplier(*incoming_poise_damage_multiplier);

                    self.selected_poise_damage = Some(weapon_poise_damage);
                }
                None => {
                    self.selected_poise_damage = Some(weapon_poise_data.clone());
                }
            }
        }

        if let Some(selected_poise_damage) = &self.selected_poise_damage {
            selected_poise_damage.view(ui);
        }
    }
}
