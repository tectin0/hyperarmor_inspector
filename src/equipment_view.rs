use egui::Slider;

use crate::{
    data::Attacks,
    static_data::{
        BULLGOAT_MULTIPLIER, COLOSSAL_POISE_DAMAGE_MULTIPLIER, INNATE_WEAPON_POISE,
        POISE_DAMAGE_MULTIPLIER, POISE_DATA,
    },
    weapon_select_view::WeaponSelectView,
};

#[derive(Default)]
pub struct EquipmentView {
    pub is_open: bool,
    selected_weapon: Option<String>,
    selected_weapon_class: Option<String>,
    selected_attack: Option<Attacks>,
    weapon_hyperarmor: Option<f64>,
    pub incoming_poise_damage_multiplier: Option<f64>,
    pub is_changed_incoming_poise_damage_multiplier: bool,
    pub armor_poise: u16,
    is_armor_poise_changed: bool,
    pub hyperarmor: Option<f64>,
    is_bullgoat_equipped: bool,
    is_bullgoat_equipped_changed: bool,
    is_weapon_or_attack_changed: bool,
    weapon_select_view: WeaponSelectView,
}

impl EquipmentView {
    pub fn new() -> Self {
        Self {
            is_open: true,
            selected_weapon: None,
            selected_weapon_class: None,
            selected_attack: None,
            weapon_hyperarmor: None,
            incoming_poise_damage_multiplier: Some(1.0),
            is_changed_incoming_poise_damage_multiplier: false,
            armor_poise: 0,
            is_armor_poise_changed: false,
            hyperarmor: None,
            is_bullgoat_equipped: false,
            is_bullgoat_equipped_changed: false,
            is_weapon_or_attack_changed: false,
            weapon_select_view: WeaponSelectView::new(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.is_changed_incoming_poise_damage_multiplier = false;

        ui.horizontal(|ui| {
            ui.label("Selected Weapon: ");

            ui.label(match &self.selected_weapon {
                Some(weapon) => weapon,
                None => "None",
            });

            ui.button("Change").clicked().then(|| {
                self.weapon_select_view.is_open = true;
            });
        });

        ui.horizontal(|ui| {
            ui.label("Select Attack: ");

            self.is_weapon_or_attack_changed = Attacks::combobox(ui, &mut self.selected_attack);
        });

        if self.is_weapon_or_attack_changed
            && self.selected_weapon.is_some()
            && self.selected_attack.is_some()
        {
            let weapon = self.selected_weapon.as_ref().unwrap();
            let weapon_class = &POISE_DATA.get(weapon).unwrap().class;
            self.selected_weapon_class = Some(weapon_class.clone());

            let attack = self.selected_attack.as_ref().unwrap();

            let innate_weapon_poise = *INNATE_WEAPON_POISE.get(weapon).unwrap();

            let hyper_armor_multiplier = self
                .selected_attack
                .as_ref()
                .unwrap()
                .get_hyper_armour_multiplier();

            let weapon_hyperarmor = weapon_hyperarmor_from_weapon_and_attack(
                innate_weapon_poise,
                hyper_armor_multiplier,
                weapon_class,
                weapon,
                attack,
            );

            self.weapon_hyperarmor = Some(weapon_hyperarmor);
        }

        if (self.is_weapon_or_attack_changed
            || self.is_bullgoat_equipped_changed
            || self.is_armor_poise_changed)
            && self.selected_weapon.is_some()
            && self.selected_attack.is_some()
            && self.selected_weapon_class.is_some()
        {
            let weapon_hyperarmor = self.weapon_hyperarmor.unwrap_or_default();
            let weapon_class = self.selected_weapon_class.as_ref().unwrap();

            match weapon_hyperarmor as u16 > 0 {
                true => {
                    let base_multiplier = match weapon_class.contains("Colossal") {
                        true => COLOSSAL_POISE_DAMAGE_MULTIPLIER,
                        false => POISE_DAMAGE_MULTIPLIER,
                    };

                    let incoming_poise_damage_multiplier = match self.is_bullgoat_equipped {
                        true => base_multiplier * (1.0 - BULLGOAT_MULTIPLIER),
                        false => base_multiplier,
                    };

                    self.incoming_poise_damage_multiplier = Some(incoming_poise_damage_multiplier);
                    self.hyperarmor = Some(self.armor_poise as f64 + weapon_hyperarmor);
                }
                false => {
                    self.incoming_poise_damage_multiplier = match self.is_bullgoat_equipped {
                        true => Some(1.0 - BULLGOAT_MULTIPLIER),
                        false => Some(1.0),
                    };
                    self.hyperarmor = Some(0.0);
                }
            }

            self.is_changed_incoming_poise_damage_multiplier = true;

            self.is_bullgoat_equipped_changed = false;
            self.is_weapon_or_attack_changed = false;
        } else if self.is_bullgoat_equipped_changed {
            self.incoming_poise_damage_multiplier = match self.is_bullgoat_equipped {
                true => Some(1.0 - BULLGOAT_MULTIPLIER),
                false => Some(1.0),
            };

            self.is_changed_incoming_poise_damage_multiplier = true;
        }

        ui.label(format!(
            "Weapon Hyperarmor: {}",
            self.weapon_hyperarmor.map(|x| x.to_string())
                .unwrap_or_default()
        ));

        ui.add(
            Slider::from_get_set(0.0..=100.0, |value| {
                if let Some(value) = value {
                    self.armor_poise = value as u16;
                }
                self.armor_poise as f64
            })
            .text("Armor Poise"),
        )
        .changed()
        .then(|| {
            self.is_armor_poise_changed = true;
        });

        ui.label(format!(
            "Hyperarmor: {}",
            self.hyperarmor.map(|x| x.to_string())
                .unwrap_or_default()
        ));

        ui.checkbox(&mut self.is_bullgoat_equipped, "Bullgoat Equipped")
            .clicked()
            .then(|| {
                self.is_bullgoat_equipped_changed = true;
            });

        ui.label(format!(
            "Incoming Poise Damage Multiplier: {}",
            self.incoming_poise_damage_multiplier.map(|x| x.to_string())
                .unwrap_or_default()
        ));

        if self.weapon_select_view.is_open {
            self.weapon_select_view.show(ui, "Equipped Weapon");
        }

        if let Some(selected_weapon) = &self.weapon_select_view.selected_weapon {
            self.selected_weapon = Some(selected_weapon.clone());
            self.weapon_select_view.selected_weapon = None;
            self.weapon_select_view.is_open = false;
            self.is_weapon_or_attack_changed = true;
        }
    }
}

pub fn weapon_hyperarmor_from_weapon_and_attack(
    innate_weapon_poise: u16,
    hyper_armor_multiplier: f64,
    weapon_class: &String,
    weapon: &String,
    attack: &Attacks,
) -> f64 {
    if innate_weapon_poise > 77 {
        innate_weapon_poise as f64 * hyper_armor_multiplier
    } else if (52..=77).contains(&innate_weapon_poise) {
        if weapon_class == "Great Katana" {
            if weapon == "Rakshasa's Great Katana" {
                innate_weapon_poise as f64 * hyper_armor_multiplier
            } else {
                0.0
            }
        } else if weapon_class == "Hammer" {
            match attack.to_string().contains("Two Handed") || attack.to_string().contains("R2") {
                true => innate_weapon_poise as f64 * hyper_armor_multiplier,
                false => 0.0,
            }
        } else {
            match attack.to_string().contains("Two Handed") {
                true => innate_weapon_poise as f64 * hyper_armor_multiplier,
                false => 0.0,
            }
        }
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weapon_hyperarmor_from_weapon_and_attack() {
        let weapon = "Dagger".to_string();

        let weapon_class = "Dagger".to_string();

        let innate_weapon_poise = 11;
        let hyper_armor_multiplier = 0.75;

        let attack = Attacks::TwoHandedR1Running;

        let weapon_hyperarmor = weapon_hyperarmor_from_weapon_and_attack(
            innate_weapon_poise,
            hyper_armor_multiplier,
            &weapon_class,
            &weapon,
            &attack,
        );

        assert_eq!(weapon_hyperarmor as u16, 0);

        let weapon = "Claymore".to_string();

        let weapon_class = "Greatsword".to_string();

        let innate_weapon_poise = 52;
        let hyper_armor_multiplier = 0.75;

        let attack = Attacks::TwoHandedR1Running;

        let weapon_hyperarmor = weapon_hyperarmor_from_weapon_and_attack(
            innate_weapon_poise,
            hyper_armor_multiplier,
            &weapon_class,
            &weapon,
            &attack,
        );

        assert_eq!(weapon_hyperarmor as u16, 39);
    }
}
