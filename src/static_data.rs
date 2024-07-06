use std::{
    collections::{BTreeMap, HashSet},
    ops::Deref,
    sync::LazyLock,
};

use crate::{
    data::{load_data, Attacks, WeaponPoiseDamage, POISE_DATA_FILE},
    download,
};

pub struct PoiseData(pub BTreeMap<String, WeaponPoiseDamage>);

impl Deref for PoiseData {
    type Target = BTreeMap<String, WeaponPoiseDamage>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PoiseData {
    pub fn get_poise_damage_values_for_attack(&self, attack: &Attacks) -> Vec<f64> {
        self.iter()
            .filter_map(
                |(_, poise_data)| match poise_data.get_poise_damage_by_attack(attack) {
                    Some(poise_damage_values) => Some(
                        poise_damage_values
                            .0
                            .iter()
                            .map(|&value| value as f64)
                            .sum::<f64>(),
                    ),
                    None => None,
                },
            )
            .collect()
    }

    pub fn get_poise_damage_values_for_attack_by_class(
        &self,
        attack: &Attacks,
        multiplier: &Option<f64>,
    ) -> BTreeMap<String, Vec<(String, f64)>> {
        let mut poise_damage_values_for_attack_per_class =
            BTreeMap::<String, Vec<(String, f64)>>::new();

        for (weapon_class, weapons) in WEAPON_CLASSES.iter() {
            for weapon in weapons {
                match self.get(weapon).and_then(|x| match multiplier {
                    None => x.get_poise_damage_by_attack(attack).cloned(),
                    Some(multiplier) => {
                        x.get_poise_damage_by_attack_w_multiplier(attack, *multiplier)
                    }
                }) {
                    Some(poise_damage_for_attack) => {
                        poise_damage_values_for_attack_per_class
                            .entry(weapon_class.clone())
                            .or_insert_with(Vec::new)
                            .push((
                                weapon.clone(),
                                poise_damage_for_attack.0.iter().sum::<u16>() as f64,
                            ));
                    }
                    None => {
                        log::warn!(
                            "Weapon {} does not have poise damage for attack {:?}",
                            weapon,
                            attack
                        );
                    }
                };
            }
        }

        poise_damage_values_for_attack_per_class
    }
}

pub static POISE_DATA: LazyLock<PoiseData> =
    LazyLock::new(
        || match std::path::Path::exists(std::path::Path::new(POISE_DATA_FILE)) {
            true => PoiseData(load_data()),
            false => {
                download::download_poise_data();
                PoiseData(load_data())
            }
        },
    );

pub static WEAPONS: LazyLock<Vec<String>> = LazyLock::new(|| POISE_DATA.keys().cloned().collect());

pub static WEAPON_CLASSES: LazyLock<BTreeMap<String, Vec<String>>> = LazyLock::new(|| {
    let mut weapon_classes = BTreeMap::<String, Vec<String>>::new();

    for weapon in WEAPONS.iter() {
        let weapon_class = POISE_DATA.get::<String>(weapon).unwrap().class.clone();

        weapon_classes
            .entry(weapon_class)
            .or_default()
            .push(weapon.clone());
    }

    for (_, weapons) in &mut weapon_classes {
        weapons.sort();
    }

    weapon_classes
});

pub const BULLGOAT_MULTIPLIER: f64 = 0.25;

pub const COLOSSAL_POISE_DAMAGE_MULTIPLIER: f64 = 0.5625;
pub const POISE_DAMAGE_MULTIPLIER: f64 = 0.8125;

pub const INNATE_WEAPON_POISE: LazyLock<BTreeMap<String, u16>> = LazyLock::new(|| {
    // https://www.reddit.com/r/EldenRingPVP/comments/1dl2j8n/elden_ring_shadow_of_the_erdtree_112_hyper_armour/

    let iwp_classes = [
        ("Colossal Weapon", 99),
        ("Colossal Sword", 90),
        ("Great Hammer", 77),
        ("Longhaft Axe", 70),
        ("Greatsword", 59),
        ("Curved Greatsword", 59),
        ("Greataxe", 59),
        ("Great Spear", 59),
        ("Heavy Thrusting Sword", 59),
        ("Hammer", 52),
        ("Flail", 52),
        ("Halberd", 52),
        ("Straight Sword", 15),
        ("Curved Sword", 15),
        ("Katana", 15),
        ("Twinblade", 15),
        ("Axe", 15),
        ("Spear", 15),
        ("Fist", 15),
        ("Reaper", 15),
        ("Thrusting Sword", 14),
        ("Whip", 14),
        ("Dagger", 11),
        ("Claw", 11),
        ("Rakshasa's Great Katana", 77),
        ("Great Katana", 52),
        ("Light Greatsword", 30),
        ("Thrusting Shield", 27),
        ("Bloodfiend's Sacred Spear", 15),
        ("Backhand Blade", 15),
        ("Hand-to-Hand", 15),
        ("Beast Claw", 14),
        ("Perfume Bottle", 14),
        ("Throwing Blade", 11),
    ]
    .into_iter()
    .map(|(weapon, poise)| (weapon.to_string(), poise as u16))
    .collect::<BTreeMap<_, _>>();

    let mut iwp_data = BTreeMap::<String, u16>::new();

    let mut iwp_classes_used = HashSet::new();

    for weapon in WEAPONS.iter() {
        let weapon_class = &POISE_DATA.get::<String>(weapon).unwrap().class;

        if iwp_classes.contains_key(weapon) {
            iwp_data.insert(weapon.clone(), iwp_classes[weapon]);
            iwp_classes_used.insert(weapon.clone());
        } else if iwp_classes.contains_key(weapon_class) {
            iwp_data.insert(weapon.clone(), iwp_classes[weapon_class]);
            iwp_classes_used.insert(weapon_class.clone());
        } else {
            iwp_data.insert(weapon.clone(), 0);
        }
    }

    if iwp_classes.len() != iwp_classes_used.len() {
        let mut iwp_classes_not_used = iwp_classes
            .keys()
            .cloned()
            .collect::<HashSet<String>>()
            .difference(&iwp_classes_used)
            .cloned()
            .collect::<Vec<String>>();

        iwp_classes_not_used.sort();

        panic!(
            "The following weapon classes are not used: {:?}",
            iwp_classes_not_used
        );
    }

    iwp_data
});
