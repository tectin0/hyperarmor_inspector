use std::{
    collections::BTreeMap,
    default,
    fmt::{Display, Formatter},
};

use convert_case::Casing;
use eframe::App;
use egui::{ComboBox, InnerResponse, Layout};
use egui_extras::{Column, TableBuilder};
use itertools::Itertools;
use strum::{EnumIter, IntoEnumIterator};

pub const POISE_DATA_FILE: &str = "poise_data.csv";

pub type PoiseData = BTreeMap<String, WeaponPoiseDamage>;

pub type PoiseDamage = u16;

#[derive(Default, Debug, Clone, PartialEq, EnumIter)]
pub enum CriticalSize {
    #[default]
    Default,
    Small,
    Large,
}

#[derive(Debug, Default, PartialEq, EnumIter, Clone)]
pub enum Attacks {
    #[default]
    None,
    OneHandedR1Chain(u8),
    OneHandedR1Running,
    OneHandedR1Rolling,
    OneHandedR1Backstep,
    OneHandedR1Jumping,
    OneHandedR1GuardCounter,
    OneHandedR2Chain(u8),
    OneHandedR2Charged(u8),
    OneHandedR2Running,
    OneHandedR2Jumping,
    OneHandedR2Feint(u8),
    TwoHandedR1Chain(u8),
    TwoHandedR1Running,
    TwoHandedR1Rolling,
    TwoHandedR1Backstep,
    TwoHandedR1Jumping,
    TwoHandedR1GuardCounter,
    TwoHandedR2Chain(u8),
    TwoHandedR2Charged(u8),
    TwoHandedR2Running,
    TwoHandedR2Jumping,
    TwoHandedR2Feint(u8),
    PairedL1Chain(u8),
    PairedL1Running,
    PairedL1Rolling,
    PairedL1Backstep,
    PairedL1Jumping,
    OffHandR1Chain(u8),
    Backstab(CriticalSize),
    Riposte(CriticalSize),
    Shieldpoke,
}

impl Display for Attacks {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!("{:?}", self)
                .as_str()
                .to_case(convert_case::Case::Title)
                .replace("R 1", "R1")
                .replace("R 2", "R2")
                .replace("L 1", "L1")
        )
    }
}

impl Attacks {
    pub fn get_hyper_armour_multiplier(&self) -> f64 {
        // https://www.reddit.com/r/EldenRingPVP/comments/1dl2j8n/elden_ring_shadow_of_the_erdtree_112_hyper_armour/
        match self {
            Attacks::None => 0.0,
            Attacks::OneHandedR1Chain(_) => 1.0,
            Attacks::OneHandedR1Running => 0.75,
            Attacks::OneHandedR1Rolling => 0.75,
            Attacks::OneHandedR1Backstep => 0.75,
            Attacks::OneHandedR1Jumping => 0.75,
            Attacks::OneHandedR1GuardCounter => 0.5,
            Attacks::OneHandedR2Chain(_) => 1.0,
            Attacks::OneHandedR2Charged(_) => 2.0,
            Attacks::OneHandedR2Running => 1.0,
            Attacks::OneHandedR2Jumping => 1.0,
            Attacks::OneHandedR2Feint(_) => 1.0,
            Attacks::TwoHandedR1Chain(_) => 1.0,
            Attacks::TwoHandedR1Running => 0.75,
            Attacks::TwoHandedR1Rolling => 0.75,
            Attacks::TwoHandedR1Backstep => 0.75,
            Attacks::TwoHandedR1Jumping => 0.75,
            Attacks::TwoHandedR1GuardCounter => 0.5,
            Attacks::TwoHandedR2Chain(_) => 1.0,
            Attacks::TwoHandedR2Charged(_) => 2.0,
            Attacks::TwoHandedR2Running => 1.0,
            Attacks::TwoHandedR2Jumping => 1.0,
            Attacks::TwoHandedR2Feint(_) => 1.0,
            // TODO: not sure
            Attacks::PairedL1Chain(_) => 1.0,
            Attacks::PairedL1Running => 1.0,
            Attacks::PairedL1Rolling => 1.0,
            Attacks::PairedL1Backstep => 1.0,
            Attacks::PairedL1Jumping => 1.0,
            Attacks::OffHandR1Chain(_) => 1.0,
            Attacks::Backstab(_) => 1.0,
            Attacks::Riposte(_) => 1.0,
            Attacks::Shieldpoke => 1.0,
        }
    }

    pub fn combobox(ui: &mut egui::Ui, selected_attack: &mut Option<Attacks>) -> bool {
        let mut has_attack_changed = false;

        egui::ComboBox::from_label("")
            .selected_text(format!(
                "{}",
                selected_attack.as_ref().unwrap_or(&Attacks::None)
            ))
            .show_ui(ui, |ui| {
                for attack in Attacks::iter() {
                    let attack_str = format!("{}", attack);

                    ui.selectable_value(selected_attack, Some(attack), format!("{}", attack_str))
                        .clicked()
                        .then(|| {
                            has_attack_changed = true;
                        });
                }
            });

        has_attack_changed
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PoiseDamageValues(pub Vec<PoiseDamage>);

impl ApplyMultiplier for PoiseDamageValues {
    fn apply_multiplier(&self, multiplier: f64) -> Self {
        Self(
            self.0
                .iter()
                .map(|n| (*n as f64 * multiplier) as PoiseDamage)
                .collect(),
        )
    }
}

impl Display for PoiseDamageValues {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let values = self.0.iter().map(|n| n.to_string()).join(" + ");

        write!(f, "{}", values)
    }
}

impl From<Vec<PoiseDamage>> for PoiseDamageValues {
    fn from(v: Vec<PoiseDamage>) -> Self {
        Self(v)
    }
}

impl From<&str> for PoiseDamageValues {
    fn from(s: &str) -> Self {
        let values = poise_string_to_numbers(s);

        Self(values)
    }
}

#[derive(Debug, Clone)]
pub struct WeaponPoiseDamage {
    pub name: String,
    pub class: String,
    pub one_handed: Grip,
    pub two_handed: Grip,
    pub paired: Strength,
    pub offhand: Chain,
    pub riposte: Size,
    pub backstab: Size,
    pub shieldpoke: PoiseDamageValues,
    pub poise_damage_multiplier: f64,
}

impl ApplyMultiplier for WeaponPoiseDamage {
    fn apply_multiplier(&self, multiplier: f64) -> Self {
        Self {
            name: self.name.clone(),
            class: self.class.clone(),
            one_handed: self.one_handed.apply_multiplier(multiplier),
            two_handed: self.two_handed.apply_multiplier(multiplier),
            paired: self.paired.apply_multiplier(multiplier),
            offhand: self.offhand.apply_multiplier(multiplier),
            riposte: self.riposte.apply_multiplier(multiplier),
            backstab: self.backstab.apply_multiplier(multiplier),
            shieldpoke: self.shieldpoke.apply_multiplier(multiplier),
            poise_damage_multiplier: multiplier,
        }
    }
}

impl WeaponPoiseDamage {
    pub fn get_poise_damage_by_attack(&self, attack: &Attacks) -> Option<&PoiseDamageValues> {
        match attack {
            Attacks::None => None,
            Attacks::OneHandedR1Chain(n) => Some(self.one_handed.r1.chain.get(*n as usize)?),
            Attacks::OneHandedR1Running => Some(&self.one_handed.r1.running),
            Attacks::OneHandedR1Rolling => Some(&self.one_handed.r1.rolling),
            Attacks::OneHandedR1Backstep => Some(&self.one_handed.r1.backstep),
            Attacks::OneHandedR1Jumping => Some(&self.one_handed.r1.jumping),
            Attacks::OneHandedR1GuardCounter => Some(&self.one_handed.r1.guard_counter),
            Attacks::OneHandedR2Chain(n) => Some(self.one_handed.r2.chain.get(*n as usize)?),
            Attacks::OneHandedR2Charged(n) => Some(self.one_handed.r2.charged.get(*n as usize)?),
            Attacks::OneHandedR2Running => Some(&self.one_handed.r2.running),
            Attacks::OneHandedR2Jumping => Some(&self.one_handed.r2.jumping),
            Attacks::OneHandedR2Feint(n) => Some(self.one_handed.r2.feint.get(*n as usize)?),
            Attacks::TwoHandedR1Chain(n) => Some(self.two_handed.r1.chain.get(*n as usize)?),
            Attacks::TwoHandedR1Running => Some(&self.two_handed.r1.running),
            Attacks::TwoHandedR1Rolling => Some(&self.two_handed.r1.rolling),
            Attacks::TwoHandedR1Backstep => Some(&self.two_handed.r1.backstep),
            Attacks::TwoHandedR1Jumping => Some(&self.two_handed.r1.jumping),
            Attacks::TwoHandedR1GuardCounter => Some(&self.two_handed.r1.guard_counter),
            Attacks::TwoHandedR2Chain(n) => Some(self.two_handed.r2.chain.get(*n as usize)?),
            Attacks::TwoHandedR2Charged(n) => Some(self.two_handed.r2.charged.get(*n as usize)?),
            Attacks::TwoHandedR2Running => Some(&self.two_handed.r2.running),
            Attacks::TwoHandedR2Jumping => Some(&self.two_handed.r2.jumping),
            Attacks::TwoHandedR2Feint(n) => Some(self.two_handed.r2.feint.get(*n as usize)?),
            Attacks::PairedL1Chain(n) => Some(self.paired.chain.get(*n as usize)?),
            Attacks::PairedL1Running => Some(&self.paired.running),
            Attacks::PairedL1Rolling => Some(&self.paired.rolling),
            Attacks::PairedL1Backstep => Some(&self.paired.backstep),
            Attacks::PairedL1Jumping => Some(&self.paired.jumping),
            Attacks::OffHandR1Chain(n) => Some(self.offhand.get(*n as usize)?),
            Attacks::Backstab(size) => match size {
                CriticalSize::Default => Some(&self.backstab.default),
                CriticalSize::Small => Some(&self.backstab.small),
                CriticalSize::Large => Some(&self.backstab.large),
            },
            Attacks::Riposte(size) => match size {
                CriticalSize::Default => Some(&self.riposte.default),
                CriticalSize::Small => Some(&self.riposte.small),
                CriticalSize::Large => Some(&self.riposte.large),
            },
            Attacks::Shieldpoke => Some(&self.shieldpoke),
        }
    }

    pub fn get_poise_damage_by_attack_w_multiplier(
        &self,
        attack: &Attacks,
        multiplier: f64,
    ) -> Option<PoiseDamageValues> {
        Some(PoiseDamageValues(
            self.get_poise_damage_by_attack(&attack)?
                .0
                .iter()
                .map(|n| (*n as f64 * multiplier) as PoiseDamage)
                .collect(),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct Grip {
    pub r1: Strength,
    pub r2: Strength,
}

impl ApplyMultiplier for Grip {
    fn apply_multiplier(&self, multiplier: f64) -> Self {
        Self {
            r1: self.r1.apply_multiplier(multiplier),
            r2: self.r2.apply_multiplier(multiplier),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Strength {
    pub chain: Chain,
    pub charged: Chain,
    pub running: PoiseDamageValues,
    pub rolling: PoiseDamageValues,
    pub backstep: PoiseDamageValues,
    pub jumping: PoiseDamageValues,
    pub guard_counter: PoiseDamageValues,
    pub feint: Chain,
}

impl ApplyMultiplier for Strength {
    fn apply_multiplier(&self, multiplier: f64) -> Self {
        Self {
            chain: self.chain.apply_multiplier(multiplier),
            charged: self.charged.apply_multiplier(multiplier),
            running: self.running.apply_multiplier(multiplier),
            rolling: self.rolling.apply_multiplier(multiplier),
            backstep: self.backstep.apply_multiplier(multiplier),
            jumping: self.jumping.apply_multiplier(multiplier),
            guard_counter: self.guard_counter.apply_multiplier(multiplier),
            feint: self.feint.apply_multiplier(multiplier),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Chain {
    pub one: PoiseDamageValues,
    pub two: PoiseDamageValues,
    pub three: PoiseDamageValues,
    pub four: PoiseDamageValues,
    pub five: PoiseDamageValues,
    pub six: PoiseDamageValues,
}

impl Chain {
    pub fn get(&self, n: usize) -> Option<&PoiseDamageValues> {
        match n {
            0 => Some(&self.one),
            1 => Some(&self.two),
            2 => Some(&self.three),
            3 => Some(&self.four),
            4 => Some(&self.five),
            5 => Some(&self.six),
            _ => None,
        }
    }
}

impl ApplyMultiplier for Chain {
    fn apply_multiplier(&self, multiplier: f64) -> Self {
        Self {
            one: self.one.apply_multiplier(multiplier),
            two: self.two.apply_multiplier(multiplier),
            three: self.three.apply_multiplier(multiplier),
            four: self.four.apply_multiplier(multiplier),
            five: self.five.apply_multiplier(multiplier),
            six: self.six.apply_multiplier(multiplier),
        }
    }
}

impl Display for Chain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let one = self.one.to_string();
        let two = self.two.to_string();
        let three = self.three.to_string();
        let four = self.four.to_string();
        let five = self.five.to_string();
        let six = self.six.to_string();

        let mut string = String::new();

        if !one.is_empty() {
            string.push_str(&one);
        }

        if !two.is_empty() {
            string.push_str(" ⏵ ");
            string.push_str(&two);
        }

        if !three.is_empty() {
            string.push_str(" ⏵ ");
            string.push_str(&three);
        }

        if !four.is_empty() {
            string.push_str(" ⏵ ");
            string.push_str(&four);
        }

        if !five.is_empty() {
            string.push_str(" ⏵ ");
            string.push_str(&five);
        }

        if !six.is_empty() {
            string.push_str(" ⏵ ");
            string.push_str(&six);
        }

        write!(f, "{}", string)
    }
}

impl From<Chain> for String {
    fn from(val: Chain) -> Self {
        val.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Size {
    pub default: PoiseDamageValues,
    pub small: PoiseDamageValues,
    pub large: PoiseDamageValues,
}

impl ApplyMultiplier for Size {
    fn apply_multiplier(&self, multiplier: f64) -> Self {
        Self {
            default: self.default.apply_multiplier(multiplier),
            small: self.small.apply_multiplier(multiplier),
            large: self.large.apply_multiplier(multiplier),
        }
    }
}

pub fn load_data() -> PoiseData {
    let mut rdr = csv::Reader::from_path(POISE_DATA_FILE).unwrap();

    let mut data = PoiseData::new();

    let records_iter = rdr.records();

    #[cfg(test)]
    let records_iter = records_iter.skip(5).take(1);

    for record in records_iter {
        let record = record.unwrap();

        let class = record.get(0).unwrap().to_string();
        let name = record.get(1).unwrap().to_string();

        if name.is_empty() || class.is_empty() {
            continue;
        }

        let one_handed_r1_1 = record.get(2).unwrap().into();
        let one_handed_r1_2 = record.get(3).unwrap().into();
        let one_handed_r1_3 = record.get(4).unwrap().into();
        let one_handed_r1_4 = record.get(5).unwrap().into();
        let one_handed_r1_5 = record.get(6).unwrap().into();
        let one_handed_r1_6 = record.get(7).unwrap().into();

        let one_handed_r1_chain = Chain {
            one: one_handed_r1_1,
            two: one_handed_r1_2,
            three: one_handed_r1_3,
            four: one_handed_r1_4,
            five: one_handed_r1_5,
            six: one_handed_r1_6,
        };

        let one_handed_r2_1 = record.get(8).unwrap().into();
        let one_handed_r2_2 = record.get(9).unwrap().into();

        let one_handed_r2_chain = Chain {
            one: one_handed_r2_1,
            two: one_handed_r2_2,
            ..Default::default()
        };

        let one_handed_charged_r2_1 = record.get(10).unwrap().into();
        let one_handed_charged_r2_2 = record.get(11).unwrap().into();

        let one_handed_charged_r2_chain = Chain {
            one: one_handed_charged_r2_1,
            two: one_handed_charged_r2_2,
            ..Default::default()
        };

        let one_handed_running_r1 = record.get(12).unwrap().into();
        let one_handed_running_r2 = record.get(13).unwrap().into();

        let one_handed_rolling_r1 = record.get(14).unwrap().into();

        let one_handed_backstep_r1 = record.get(15).unwrap().into();

        let one_handed_jumping_r1 = record.get(16).unwrap().into();
        let one_handed_jumping_r2 = record.get(17).unwrap().into();

        let one_handed_guard_counter = record.get(18).unwrap().into();

        let two_handed_r1_1 = record.get(19).unwrap().into();
        let two_handed_r1_2 = record.get(20).unwrap().into();
        let two_handed_r1_3 = record.get(21).unwrap().into();
        let two_handed_r1_4 = record.get(22).unwrap().into();
        let two_handed_r1_5 = record.get(23).unwrap().into();
        let two_handed_r1_6 = record.get(24).unwrap().into();

        let two_handed_r1_chain = Chain {
            one: two_handed_r1_1,
            two: two_handed_r1_2,
            three: two_handed_r1_3,
            four: two_handed_r1_4,
            five: two_handed_r1_5,
            six: two_handed_r1_6,
        };

        let two_handed_r2_1 = record.get(25).unwrap().into();
        let two_handed_r2_2 = record.get(26).unwrap().into();

        let two_handed_r2_chain = Chain {
            one: two_handed_r2_1,
            two: two_handed_r2_2,
            ..Default::default()
        };

        let two_handed_charged_r2_1 = record.get(27).unwrap().into();
        let two_handed_charged_r2_2 = record.get(28).unwrap().into();

        let two_handed_charged_r2_chain = Chain {
            one: two_handed_charged_r2_1,
            two: two_handed_charged_r2_2,
            ..Default::default()
        };

        let two_handed_running_r1 = record.get(29).unwrap().into();
        let two_handed_running_r2 = record.get(30).unwrap().into();

        let two_handed_rolling_r1 = record.get(31).unwrap().into();

        let two_handed_backstep_r1 = record.get(32).unwrap().into();

        let two_handed_jumping_r1 = record.get(33).unwrap().into();
        let two_handed_jumping_r2 = record.get(34).unwrap().into();

        let two_handed_guard_counter = record.get(35).unwrap().into();

        let off_hand_r1_1 = record.get(36).unwrap().into();
        let off_hand_r1_2 = record.get(37).unwrap().into();
        let off_hand_r1_3 = record.get(38).unwrap().into();
        let off_hand_r1_4 = record.get(39).unwrap().into();
        let off_hand_r1_5 = record.get(40).unwrap().into();
        let off_hand_r1_6 = record.get(41).unwrap().into();

        let off_hand_r1_chain = Chain {
            one: off_hand_r1_1,
            two: off_hand_r1_2,
            three: off_hand_r1_3,
            four: off_hand_r1_4,
            five: off_hand_r1_5,
            six: off_hand_r1_6,
        };

        let _bs_whiff: PoiseDamageValues = record.get(42).unwrap().into();

        let backstab_default = record.get(43).unwrap().into();
        let riposte_default = record.get(44).unwrap().into();

        let backstab_small = record.get(45).unwrap().into();
        let riposte_small = record.get(46).unwrap().into();

        let backstab_large = PoiseDamageValues::default();
        let riposte_large = record.get(47).unwrap().into();

        let shieldpoke = record.get(48).unwrap().into();

        let one_handed_feint_1 = record.get(49).unwrap().into();
        let one_handed_feint_2 = record.get(50).unwrap().into();

        let one_handed_feint_r2_chain = Chain {
            one: one_handed_feint_1,
            two: one_handed_feint_2,
            ..Default::default()
        };

        let two_handed_feint_1 = record.get(51).unwrap().into();
        let two_handed_feint_2 = record.get(52).unwrap().into();

        let two_handed_feint_chain = Chain {
            one: two_handed_feint_1,
            two: two_handed_feint_2,
            ..Default::default()
        };

        let paired_l1_1 = record.get(53).unwrap().into();
        let paired_l1_2 = record.get(54).unwrap().into();
        let paired_l1_3 = record.get(55).unwrap().into();
        let paired_l1_4 = record.get(56).unwrap().into();
        let paired_l1_5 = record.get(57).unwrap().into();
        let paired_l1_6 = record.get(58).unwrap().into();

        let paired_l1_chain = Chain {
            one: paired_l1_1,
            two: paired_l1_2,
            three: paired_l1_3,
            four: paired_l1_4,
            five: paired_l1_5,
            six: paired_l1_6,
        };

        let paired_running_l1 = record.get(59).unwrap().into();

        let paired_rolling_l1 = record.get(60).unwrap().into();

        let paired_backstep_l1 = record.get(61).unwrap().into();

        let paired_jumping_l1 = record.get(62).unwrap().into();

        let one_handed_r1 = Strength {
            chain: one_handed_r1_chain,
            charged: Chain::default(),
            running: one_handed_running_r1,
            rolling: one_handed_rolling_r1,
            backstep: one_handed_backstep_r1,
            jumping: one_handed_jumping_r1,
            guard_counter: one_handed_guard_counter,
            feint: Chain::default(),
        };

        let one_handed_r2 = Strength {
            chain: one_handed_r2_chain,
            charged: one_handed_charged_r2_chain,
            running: one_handed_running_r2,
            rolling: PoiseDamageValues::default(),
            backstep: PoiseDamageValues::default(),
            jumping: one_handed_jumping_r2,
            guard_counter: PoiseDamageValues::default(),
            feint: one_handed_feint_r2_chain,
        };

        let two_handed_r1 = Strength {
            chain: two_handed_r1_chain,
            charged: Chain::default(),
            running: two_handed_running_r1,
            rolling: two_handed_rolling_r1,
            backstep: two_handed_backstep_r1,
            jumping: two_handed_jumping_r1,
            guard_counter: two_handed_guard_counter,
            feint: Chain::default(),
        };

        let two_handed_r2 = Strength {
            chain: two_handed_r2_chain,
            charged: two_handed_charged_r2_chain,
            running: two_handed_running_r2,
            rolling: PoiseDamageValues::default(),
            backstep: PoiseDamageValues::default(),
            jumping: two_handed_jumping_r2,
            guard_counter: PoiseDamageValues::default(),
            feint: two_handed_feint_chain,
        };

        let paired_l1 = Strength {
            chain: paired_l1_chain,
            charged: Chain::default(),
            running: paired_running_l1,
            rolling: paired_rolling_l1,
            backstep: paired_backstep_l1,
            jumping: paired_jumping_l1,
            guard_counter: PoiseDamageValues::default(),
            feint: Chain::default(),
        };

        let one_handed = Grip {
            r1: one_handed_r1,
            r2: one_handed_r2,
        };

        let two_handed = Grip {
            r1: two_handed_r1,
            r2: two_handed_r2,
        };

        let paired = paired_l1;

        let offhand = off_hand_r1_chain;

        let riposte = Size {
            default: riposte_default,
            small: riposte_small,
            large: riposte_large,
        };

        let backstab = Size {
            default: backstab_default,
            small: backstab_small,
            large: backstab_large,
        };

        let shieldpoke = shieldpoke;

        let key = name.clone();

        let poise_damage_multiplier = 1.0;

        let weapon = WeaponPoiseDamage {
            name,
            class,
            one_handed,
            two_handed,
            paired,
            offhand,
            riposte,
            backstab,
            shieldpoke,
            poise_damage_multiplier,
        };

        data.insert(key, weapon);
    }

    data.into_iter()
        .sorted_by(|(a, _), (b, _)| a.cmp(b))
        .collect()
}

#[cfg(test)]
mod test_load_data {

    #[test]
    fn test() {
        let data = super::load_data();

        dbg!(&data);

        assert_eq!(data["Dagger"].one_handed.r1.chain.one, vec![40].into());
        assert_eq!(
            data["Dagger"].paired.chain.five,
            Vec::<super::PoiseDamage>::new().into()
        );
    }
}

// Example
// "10" to [10]
// "10 + 10" to [10, 10]
pub fn poise_string_to_numbers(s: &str) -> Vec<PoiseDamage> {
    if s.trim().is_empty() {
        return vec![];
    }

    s.split("+")
        .map(|s| match s.trim().parse::<f32>() {
            Ok(f) => f as PoiseDamage,
            Err(error) => {
                log::error!(
                    "Error parsing PoiseDamage: {} with error: {}. Defaulting to 0",
                    s,
                    error
                );
                0
            }
        })
        .collect()
}

#[cfg(test)]
mod test_poise_string_to_numbers {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(poise_string_to_numbers("10"), vec![10]);
        assert_eq!(poise_string_to_numbers("10 + 10"), vec![10, 10]);
        assert_eq!(poise_string_to_numbers("302.5 + 605"), vec![302, 605]);
        assert_eq!(poise_string_to_numbers(""), Vec::<PoiseDamage>::new());
        assert_eq!(poise_string_to_numbers(" "), Vec::<PoiseDamage>::new());
    }
}

mod ui {
    use super::*;

    impl WeaponPoiseDamage {
        pub fn view(&self, ui: &mut egui::Ui) {
            ui.vertical(|ui| {
                // ui.label(format!("Name: {}", self.name));
                // ui.label(format!("Class: {}", self.class));

                TableBuilder::new(ui)
                    .column(Column::auto().resizable(true)) // Attack Type
                    .column(Column::auto().resizable(true)) // OneHanded
                    .column(Column::auto().resizable(true)) // TwoHanded
                    .cell_layout(Layout::centered_and_justified(egui::Direction::TopDown))
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.label("Attack Type");
                        });
                        header.col(|ui| {
                            ui.label("One Handed");
                        });
                        header.col(|ui| {
                            ui.label("Two Handed");
                        });
                    })
                    .body(|body| {
                        let rows = [
                            [
                                "R1 Chain".to_string(),
                                self.one_handed.r1.chain.to_string(),
                                self.two_handed.r1.chain.to_string(),
                            ],
                            [
                                "R1 Running".to_string(),
                                self.one_handed.r1.running.to_string(),
                                self.two_handed.r1.running.to_string(),
                            ],
                            [
                                "R1 Rolling".to_string(),
                                self.one_handed.r1.rolling.to_string(),
                                self.two_handed.r1.rolling.to_string(),
                            ],
                            [
                                "R1 Backstep".to_string(),
                                self.one_handed.r1.backstep.to_string(),
                                self.two_handed.r1.backstep.to_string(),
                            ],
                            [
                                "R1 Jumping".to_string(),
                                self.one_handed.r1.jumping.to_string(),
                                self.two_handed.r1.jumping.to_string(),
                            ],
                            [
                                "R1 Guard Counter".to_string(),
                                self.one_handed.r1.guard_counter.to_string(),
                                self.two_handed.r1.guard_counter.to_string(),
                            ],
                            [
                                "R2 Chain".to_string(),
                                self.one_handed.r2.chain.to_string(),
                                self.two_handed.r2.chain.to_string(),
                            ],
                            [
                                "R2 Charged".to_string(),
                                self.one_handed.r2.charged.to_string(),
                                self.two_handed.r2.charged.to_string(),
                            ],
                            [
                                "R2 Running".to_string(),
                                self.one_handed.r2.running.to_string(),
                                self.two_handed.r2.running.to_string(),
                            ],
                            [
                                "R2 Jumping".to_string(),
                                self.one_handed.r2.jumping.to_string(),
                                self.two_handed.r2.jumping.to_string(),
                            ],
                            [
                                "R2 Feint".to_string(),
                                self.one_handed.r2.feint.to_string(),
                                self.two_handed.r2.feint.to_string(),
                            ],
                        ];

                        let total_rows = rows.len();

                        body.rows(30.0, total_rows, |mut row| {
                            let row_index = row.index();

                            row.col(|ui| {
                                ui.label(&rows[row_index][0]);
                            });

                            row.col(|ui| {
                                ui.label(&rows[row_index][1]);
                            });

                            row.col(|ui| {
                                ui.label(&rows[row_index][2]);
                            });
                        });
                    });
            });
        }
    }
}

pub trait ApplyMultiplier {
    fn apply_multiplier(&self, multiplier: f64) -> Self;
}
