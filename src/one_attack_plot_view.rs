use std::collections::{BTreeMap, HashMap};

use egui::{Layout, Rect, Slider, Vec2};
use egui_extras::{Size, StripBuilder};
use egui_plot::{
    CoordinatesFormatter, Line, Plot, PlotItem, PlotPoint, PlotPoints, PlotResponse, Points,
};
use itertools::Itertools;

use crate::{
    data::Attacks,
    static_data::{POISE_DATA, WEAPON_CLASSES},
};

#[derive(Default)]
struct PlotConfig {
    point_radius: f32,
}

#[derive(Default)]
pub struct OneAttackPlotView {
    pub is_open: bool,
    weapon_class_ids: HashMap<egui::Id, String>,
    selected_attack: Option<Attacks>,
    is_attack_changed: bool,
    poise_damage_values_for_attack_by_class: BTreeMap<String, Vec<(String, f64)>>,
    hovered_weapon: Option<String>,
    hovered_weapon_class: Option<String>,
    plot_config: PlotConfig,
}

impl OneAttackPlotView {
    pub fn new() -> Self {
        let selected_attack = Attacks::OneHandedR1Chain(0);

        let poise_damage_values_for_attack_by_class =
            POISE_DATA.get_poise_damage_values_for_attack_by_class(&selected_attack, &Some(1.0));

        let weapon_class_ids = WEAPON_CLASSES
            .keys()
            .cloned()
            .into_iter()
            .map(|class| (egui::Id::new(class.clone()), class))
            .collect::<HashMap<egui::Id, String>>();

        Self {
            is_open: false,
            weapon_class_ids,
            selected_attack: Some(selected_attack),
            is_attack_changed: false,
            poise_damage_values_for_attack_by_class,
            hovered_weapon: None,
            hovered_weapon_class: None,
            plot_config: PlotConfig {
                point_radius: 3.0,
                ..Default::default()
            },
        }
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        is_changed_incoming_poise_damage_multiplier: &bool,
        incoming_poise_damage_multiplier: &Option<f64>,
        hyperarmor: &Option<f64>,
        armor_poise: &u16,
    ) {
        egui::Window::new("One Attack Plot")
            .id("One Attack Plot Window".into())
            .resizable(true)
            .title_bar(true)
            .open(&mut self.is_open)
            .show(ui.ctx(), |ui| {
                let available_rect = ui.available_rect_before_wrap();

                if self.is_attack_changed || *is_changed_incoming_poise_damage_multiplier {
                    self.poise_damage_values_for_attack_by_class = POISE_DATA
                        .get_poise_damage_values_for_attack_by_class(
                            &self.selected_attack.as_ref().unwrap(),
                            incoming_poise_damage_multiplier,
                        );
                }

                if let Some(selected_attack) = &self.selected_attack {
                    StripBuilder::new(ui)
                        .size(Size::remainder())
                        .size(Size::relative(0.2))
                        .horizontal(|mut strip| {
                            strip.cell(|ui| {
                                let plot = Plot::new("One Attack Plot");

                                let PlotResponse {
                                    response,
                                    inner:
                                        (
                                            screen_pos,
                                            pointer_coordinate,
                                            pointer_coordinate_drag_delta,
                                            bounds,
                                            hovered,
                                        ),
                                    hovered_plot_item,
                                    ..
                                } = plot.show(ui, |plot_ui| {
                                    let mut max_x_length = 0usize;

                                    for (weapon_class, poise_damage_values) in
                                        self.poise_damage_values_for_attack_by_class.iter()
                                    {
                                        let points = poise_damage_values.iter().enumerate().map(
                                            |(i, (weapon, poise_damage))| [i as f64, *poise_damage],
                                        );

                                        max_x_length = max_x_length.max(points.len());

                                        let points = points.collect::<PlotPoints>();

                                        let points = Points::new(points)
                                            .id(weapon_class.clone().into())
                                            .radius(self.plot_config.point_radius);

                                        plot_ui.points(points);
                                    }

                                    plot_ui.line(
                                        Line::new(vec![
                                            [0.0, *armor_poise as f64],
                                            [max_x_length as f64, *armor_poise as f64],
                                        ])
                                        .color(egui::Color32::from_rgb(255, 0, 0)),
                                    );

                                    plot_ui.line(
                                        Line::new(vec![
                                            [0.0, hyperarmor.unwrap_or(0.0)],
                                            [max_x_length as f64, hyperarmor.unwrap_or(0.0)],
                                        ])
                                        .color(egui::Color32::from_rgb(0, 255, 0)),
                                    );

                                    (
                                        plot_ui.screen_from_plot(PlotPoint::new(0.0, 0.0)),
                                        plot_ui.pointer_coordinate(),
                                        plot_ui.pointer_coordinate_drag_delta(),
                                        plot_ui.plot_bounds(),
                                        plot_ui.response().hovered(),
                                    )
                                });

                                if let Some(hovered_plot_item) = hovered_plot_item {
                                    self.hovered_weapon_class = Some(
                                        self.weapon_class_ids
                                            .get(&hovered_plot_item)
                                            .unwrap()
                                            .clone(),
                                    );

                                    let weapon_index = pointer_coordinate.unwrap().x as usize;

                                    let weapon = WEAPON_CLASSES
                                        .get(self.hovered_weapon_class.as_ref().unwrap())
                                        .and_then(|weapons| weapons.get(weapon_index))
                                        .unwrap_or(&"".to_string())
                                        .clone();

                                    self.hovered_weapon = Some(weapon);
                                }
                            });
                            strip.cell(|ui| {
                                ui.vertical(|ui| {
                                    self.is_attack_changed =
                                        Attacks::combobox(ui, &mut self.selected_attack);

                                    ui.add(
                                        Slider::from_get_set(0.0..=20.0, |value| {
                                            if let Some(value) = value {
                                                self.plot_config.point_radius = value as f32;
                                            }
                                            self.plot_config.point_radius as f64
                                        })
                                        .text("Point Radius"),
                                    );

                                    ui.separator();

                                    if let Some(hovered_weapon) = &self.hovered_weapon {
                                        ui.label(format!("Hovered Weapon: {}", hovered_weapon));
                                    }
                                });
                            })
                        });
                }
            });
    }
}
