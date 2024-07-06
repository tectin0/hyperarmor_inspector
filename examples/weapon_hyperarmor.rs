use hyperarmor_inspector::{
    data::Attacks,
    equipment_view::weapon_hyperarmor_from_weapon_and_attack,
    static_data::{INNATE_WEAPON_POISE, POISE_DATA},
};

fn main() {
    let weapon = "Claymore".to_string();

    let weapon_class = POISE_DATA.get(&weapon).unwrap().class.clone();

    let innate_weapon_poise = *INNATE_WEAPON_POISE.get(&weapon).unwrap();
    let hyper_armor_multiplier = Attacks::TwoHandedR2Charged(0).get_hyper_armour_multiplier();

    let attack = Attacks::TwoHandedR2Charged(0);

    let weapon_hyperarmor = weapon_hyperarmor_from_weapon_and_attack(
        innate_weapon_poise,
        hyper_armor_multiplier,
        &weapon_class,
        &weapon,
        &attack,
    );

    dbg!(
        weapon_hyperarmor,
        weapon_class,
        weapon,
        attack,
        innate_weapon_poise,
        hyper_armor_multiplier
    );
}
