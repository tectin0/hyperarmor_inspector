use std::collections::{BTreeMap, HashMap};

use egui::{Layout, Slider};
use egui_extras::{Size, StripBuilder};
use egui_plot::{Line, Plot, PlotPoint, PlotPoints, PlotResponse, Points};

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
    selected_weapon_classes: BTreeMap<String, bool>,
    hovered_weapon: Option<String>,
    hovered_weapon_class: Option<String>,
    plot_config: PlotConfig,
    rect: Option<egui::Rect>,
}

impl OneAttackPlotView {
    pub fn new() -> Self {
        let selected_attack = Attacks::OneHandedR1Chain(0);

        let poise_damage_values_for_attack_by_class =
            POISE_DATA.get_poise_damage_values_for_attack_by_class(&selected_attack, &Some(1.0));

        let weapon_class_ids = WEAPON_CLASSES
            .keys()
            .cloned()
            .map(|class| (egui::Id::new(class.clone()), class))
            .collect::<HashMap<egui::Id, String>>();

        Self {
            is_open: false,
            weapon_class_ids,
            selected_attack: Some(selected_attack),
            is_attack_changed: false,
            poise_damage_values_for_attack_by_class,
            selected_weapon_classes: WEAPON_CLASSES
                .keys()
                .cloned()
                .map(|class| (class, true))
                .collect(),
            hovered_weapon: None,
            hovered_weapon_class: None,
            plot_config: PlotConfig {
                point_radius: 3.0,
                ..Default::default()
            },
            ..Default::default()
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
        const INITIAL_WINDOW_SIZE: [f32; 2] = [600.0, 400.0];

        let window = egui::Window::new("One Attack Plot")
            .id("One Attack Plot Window".into())
            .resizable(true)
            .title_bar(true)
            .default_size(INITIAL_WINDOW_SIZE)
            .open(&mut self.is_open)
            .show(ui.ctx(), |ui| {
                if self.rect.is_none() {
                    return;
                }

                if self.is_attack_changed || *is_changed_incoming_poise_damage_multiplier {
                    self.poise_damage_values_for_attack_by_class = POISE_DATA
                        .get_poise_damage_values_for_attack_by_class(
                            self.selected_attack.as_ref().unwrap(),
                            incoming_poise_damage_multiplier,
                        );
                }

                if let Some(_selected_attack) = &self.selected_attack {
                    StripBuilder::new(ui)
                        .cell_layout(Layout::top_down(egui::Align::Min))
                        .size(Size::relative(0.7))
                        .size(Size::initial(200.0))
                        .horizontal(|mut strip| {
                            strip.cell(|ui| {
                                let plot = Plot::new("One Attack Plot");

                                let PlotResponse {
                                    response: _,
                                    inner:
                                        (
                                            _screen_pos,
                                            pointer_coordinate,
                                            _pointer_coordinate_drag_delta,
                                            _bounds,
                                            _hovered,
                                        ),
                                    hovered_plot_item,
                                    ..
                                } = plot.show(ui, |plot_ui| {
                                    let mut max_x_length = 0usize;

                                    for (weapon_class, poise_damage_values) in
                                        self.poise_damage_values_for_attack_by_class.iter()
                                    {
                                        if !self.selected_weapon_classes.get(weapon_class).unwrap()
                                        {
                                            continue;
                                        }

                                        let points = poise_damage_values.iter().enumerate().map(
                                            |(i, (_weapon, poise_damage))| {
                                                [i as f64, *poise_damage]
                                            },
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

                                    ui.separator();

                                    let mut unselect_all_other = None;

                                    ui.horizontal_wrapped(|ui| {
                                        for (weapon_class, is_selected) in
                                            self.selected_weapon_classes.iter_mut()
                                        {
                                            ui.selectable_label(*is_selected, weapon_class)
                                                .clicked()
                                                .then(|| {
                                                    *is_selected = !*is_selected;

                                                    if ui.ctx().input(|i| i.modifiers.ctrl) {
                                                        unselect_all_other =
                                                            Some(weapon_class.clone());
                                                    }
                                                });
                                        }
                                    });

                                    if let Some(unselect_all_other) = unselect_all_other {
                                        for (weapon_class, is_selected) in
                                            self.selected_weapon_classes.iter_mut()
                                        {
                                            if *weapon_class != unselect_all_other {
                                                *is_selected = false;
                                            }

                                            if *weapon_class == unselect_all_other {
                                                *is_selected = true;
                                            }
                                        }
                                    }

                                    ui.horizontal(|ui| {
                                        ui.button("Deselect All").clicked().then(|| {
                                            for is_selected in
                                                self.selected_weapon_classes.values_mut()
                                            {
                                                *is_selected = false;
                                            }
                                        });

                                        ui.button("Select All").clicked().then(|| {
                                            for is_selected in
                                                self.selected_weapon_classes.values_mut()
                                            {
                                                *is_selected = true;
                                            }
                                        });
                                    });
                                });
                            })
                        });
                }
            })
            .unwrap();

        self.rect = Some(window.response.interact_rect);
    }
}
