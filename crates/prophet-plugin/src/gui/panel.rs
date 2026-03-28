use nih_plug::prelude::*;
use nih_plug_egui::egui::{self, Color32, Stroke, Vec2};
use std::sync::Arc;

use super::knob::{KnobVariant, ParamKnob};
use super::switch::ParamSwitch;
use crate::ProphetParams;
use crate::presets;

const PANEL_BG: Color32 = Color32::from_rgb(25, 25, 25);
const SECTION_DIVIDER: Color32 = Color32::from_rgb(55, 55, 50);
const SECTION_LABEL: Color32 = Color32::from_rgb(210, 210, 205);
const BRAND_COLOR: Color32 = Color32::from_rgb(190, 180, 160);
const WOOD_MID: Color32 = Color32::from_rgb(110, 70, 40);
const WOOD_DARK: Color32 = Color32::from_rgb(85, 55, 30);
const WOOD_LIGHT: Color32 = Color32::from_rgb(140, 95, 55);

pub fn draw_panel(ctx: &egui::Context, setter: &ParamSetter, params: &Arc<ProphetParams>) {
    // Wood cheeks
    egui::SidePanel::left("wood_l")
        .exact_width(24.0)
        .resizable(false)
        .frame(egui::Frame::new().fill(WOOD_MID).inner_margin(0))
        .show(ctx, |ui| paint_wood(ui));

    egui::SidePanel::right("wood_r")
        .exact_width(24.0)
        .resizable(false)
        .frame(egui::Frame::new().fill(WOOD_MID).inner_margin(0))
        .show(ctx, |ui| paint_wood(ui));

    // Main panel
    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(PANEL_BG).inner_margin(egui::Margin::symmetric(8, 6)))
        .show(ctx, |ui| {
            // Header with brand and preset selector
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("SEQUENTIAL CIRCUITS")
                        .size(9.0)
                        .color(BRAND_COLOR),
                );
                ui.label(
                    egui::RichText::new("PROPHET-5")
                        .size(13.0)
                        .color(Color32::WHITE)
                        .strong(),
                );

                ui.add_space(30.0);

                // Preset dropdown — organized by category
                let factory = presets::factory_presets();
                let categories = presets::categories();

                // Get current preset name from UI state (or "Init Saw")
                let current_name = ui.ctx().data_mut(|d| {
                    d.get_temp_mut_or_insert_with::<String>(
                        egui::Id::new("current_preset"),
                        || "Init Saw".to_string(),
                    ).clone()
                });

                ui.label(
                    egui::RichText::new("PROGRAM")
                        .size(8.0)
                        .color(Color32::from_rgb(150, 150, 145)),
                );

                let combo = egui::ComboBox::from_id_salt("preset_selector")
                    .selected_text(
                        egui::RichText::new(&current_name)
                            .size(10.0)
                            .color(Color32::from_rgb(220, 210, 190)),
                    )
                    .width(180.0);

                combo.show_ui(ui, |ui| {
                    ui.style_mut().visuals.widgets.inactive.bg_fill = Color32::from_rgb(35, 33, 30);
                    ui.style_mut().visuals.widgets.hovered.bg_fill = Color32::from_rgb(55, 50, 42);

                    for &cat in categories {
                        // Category header
                        ui.label(
                            egui::RichText::new(cat)
                                .size(9.0)
                                .color(Color32::from_rgb(180, 160, 120))
                                .strong(),
                        );
                        ui.separator();

                        // Presets in this category
                        for preset in &factory {
                            if preset.category != cat {
                                continue;
                            }
                            let is_selected = current_name == preset.name;
                            let label = egui::RichText::new(preset.name)
                                .size(10.0)
                                .color(if is_selected {
                                    Color32::from_rgb(255, 220, 150)
                                } else {
                                    Color32::from_rgb(200, 195, 185)
                                });

                            if ui.selectable_label(is_selected, label).clicked() {
                                presets::apply_preset(preset, params, setter);
                                ui.ctx().data_mut(|d| {
                                    d.insert_temp(
                                        egui::Id::new("current_preset"),
                                        preset.name.to_string(),
                                    );
                                });
                            }
                        }

                        ui.add_space(4.0);
                    }
                });
            });
            ui.add_space(4.0);

            // Thin separator line
            let rect = ui.available_rect_before_wrap();
            ui.painter().hline(
                rect.x_range(),
                rect.top(),
                Stroke::new(0.5, Color32::from_gray(60)),
            );
            ui.add_space(6.0);

            // All sections in a single horizontal row
            // Using allocate_ui with fixed widths for each section
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(0.0, 0.0);

                section(ui, "POLY MOD", 100.0, |ui, _w| {
                    row(ui, |ui| {
                        ui.add(ParamKnob::new(&params.poly_mod_filt_env, setter, "FILT ENV").size(34.0));
                        ui.add(ParamKnob::new(&params.poly_mod_osc_b, setter, "OSC B").size(34.0));
                    });
                    row(ui, |ui| {
                        ui.add(ParamSwitch::new(&params.poly_mod_freq_a, setter, "FREQ A"));
                        ui.add(ParamSwitch::new(&params.poly_mod_pw_a, setter, "PW A"));
                        ui.add(ParamSwitch::new(&params.poly_mod_filter, setter, "FILT"));
                    });
                });

                divider(ui);

                section(ui, "LFO", 75.0, |ui, _w| {
                    row(ui, |ui| {
                        ui.add(ParamKnob::new(&params.lfo_freq, setter, "FREQ").size(34.0));
                    });
                    row(ui, |ui| {
                        ui.add(ParamSwitch::new(&params.lfo_tri, setter, "TRI"));
                        ui.add(ParamSwitch::new(&params.lfo_saw, setter, "SAW"));
                        ui.add(ParamSwitch::new(&params.lfo_square, setter, "SQR"));
                    });
                });

                divider(ui);

                section(ui, "WHEEL MOD", 105.0, |ui, _w| {
                    row(ui, |ui| {
                        ui.add(ParamKnob::new(&params.wheel_mod_mix, setter, "MIX").size(34.0));
                    });
                    row(ui, |ui| {
                        ui.add(ParamSwitch::new(&params.wheel_mod_freq_a, setter, "F.A"));
                        ui.add(ParamSwitch::new(&params.wheel_mod_freq_b, setter, "F.B"));
                        ui.add(ParamSwitch::new(&params.wheel_mod_filter, setter, "FILT"));
                    });
                    row(ui, |ui| {
                        ui.add(ParamSwitch::new(&params.wheel_mod_pw_a, setter, "PW.A"));
                        ui.add(ParamSwitch::new(&params.wheel_mod_pw_b, setter, "PW.B"));
                    });
                });

                divider(ui);

                section(ui, "OSCILLATOR A", 120.0, |ui, _w| {
                    row(ui, |ui| {
                        ui.add(ParamSwitch::new(&params.osc_a_saw, setter, "SAW"));
                        ui.add(ParamSwitch::new(&params.osc_a_pulse, setter, "PULSE"));
                        ui.add(ParamSwitch::new(&params.sync, setter, "SYNC"));
                    });
                    row(ui, |ui| {
                        ui.add(ParamKnob::new(&params.osc_a_pw, setter, "PW").size(34.0));
                    });
                });

                divider(ui);

                section(ui, "OSCILLATOR B", 130.0, |ui, _w| {
                    row(ui, |ui| {
                        ui.add(ParamSwitch::new(&params.osc_b_saw, setter, "SAW"));
                        ui.add(ParamSwitch::new(&params.osc_b_tri, setter, "TRI"));
                        ui.add(ParamSwitch::new(&params.osc_b_pulse, setter, "PULSE"));
                    });
                    row(ui, |ui| {
                        ui.add(ParamKnob::new(&params.osc_b_pw, setter, "PW").size(34.0));
                    });
                });

                divider(ui);

                section(ui, "MIXER", 130.0, |ui, _w| {
                    row(ui, |ui| {
                        ui.add(ParamKnob::new(&params.osc_a_level, setter, "OSC A").size(34.0));
                        ui.add(ParamKnob::new(&params.osc_b_level, setter, "OSC B").size(34.0));
                        ui.add(ParamKnob::new(&params.noise_level, setter, "NOISE").size(34.0));
                    });
                });

                divider(ui);

                section(ui, "FILTER", 105.0, |ui, _w| {
                    row(ui, |ui| {
                        ui.add(ParamKnob::new(&params.filter_cutoff, setter, "CUTOFF").size(38.0));
                        ui.add(ParamKnob::new(&params.filter_resonance, setter, "RES").size(38.0));
                    });
                    row(ui, |ui| {
                        ui.add(ParamKnob::new(&params.filter_env_amount, setter, "ENV AMT").size(34.0));
                    });
                });

                divider(ui);

                section(ui, "FILTER ENVELOPE", 170.0, |ui, _w| {
                    row(ui, |ui| {
                        ui.add(ParamKnob::new(&params.filter_attack, setter, "A").size(34.0));
                        ui.add(ParamKnob::new(&params.filter_decay, setter, "D").size(34.0));
                        ui.add(ParamKnob::new(&params.filter_sustain, setter, "S").size(34.0));
                        ui.add(ParamKnob::new(&params.filter_release, setter, "R").size(34.0));
                    });
                });

                divider(ui);

                section(ui, "AMPLIFIER", 170.0, |ui, _w| {
                    row(ui, |ui| {
                        ui.add(ParamKnob::new(&params.amp_attack, setter, "A").size(34.0));
                        ui.add(ParamKnob::new(&params.amp_decay, setter, "D").size(34.0));
                        ui.add(ParamKnob::new(&params.amp_sustain, setter, "S").size(34.0));
                        ui.add(ParamKnob::new(&params.amp_release, setter, "R").size(34.0));
                    });
                });

                divider(ui);

                section(ui, "MASTER", 120.0, |ui, _w| {
                    row(ui, |ui| {
                        ui.add(
                            ParamKnob::new(&params.volume, setter, "VOL")
                                .size(42.0)
                                .variant(KnobVariant::Silver),
                        );
                        ui.add(ParamKnob::new(&params.glide_rate, setter, "GLIDE").size(34.0));
                        ui.add(ParamKnob::new(&params.drift, setter, "DRIFT").size(34.0));
                    });
                    row(ui, |ui| {
                        ui.add(ParamSwitch::new(&params.glide_on, setter, "GLIDE"));
                        ui.add(ParamSwitch::new(&params.unison, setter, "UNISN"));
                    });
                });
            });
        });
}

/// Draw a named section with a fixed width.
fn section(
    ui: &mut egui::Ui,
    title: &str,
    width: f32,
    content: impl FnOnce(&mut egui::Ui, f32),
) {
    ui.allocate_ui(Vec2::new(width, ui.available_height()), |ui| {
        ui.push_id(title, |ui| {
        ui.set_min_width(width);
        ui.vertical(|ui| {
            // Section title — painted directly, no wrapping
            ui.painter().text(
                egui::pos2(ui.available_rect_before_wrap().center().x, ui.cursor().top() + 6.0),
                egui::Align2::CENTER_CENTER,
                title,
                egui::FontId::proportional(9.0),
                SECTION_LABEL,
            );
            ui.add_space(14.0);

            content(ui, width);
        });
        });
    });
}

/// Horizontal row of controls, centered.
fn row(ui: &mut egui::Ui, content: impl FnOnce(&mut egui::Ui)) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(2.0, 2.0);
        content(ui);
    });
}

/// Vertical divider between sections.
fn divider(ui: &mut egui::Ui) {
    let rect = ui.available_rect_before_wrap();
    ui.painter().vline(
        rect.left() + 3.0,
        rect.y_range(),
        Stroke::new(0.5, SECTION_DIVIDER),
    );
    ui.add_space(7.0);
}

fn paint_wood(ui: &mut egui::Ui) {
    let rect = ui.available_rect_before_wrap();
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 0.0, WOOD_MID);

    // Grain lines
    for i in 0..6 {
        let t = i as f32 / 6.0;
        let x = rect.left() + t * rect.width() + (i as f32 * 2.3).sin() * 1.5;
        let color = if i % 2 == 0 { WOOD_DARK } else { WOOD_LIGHT };
        painter.vline(
            x,
            rect.y_range(),
            Stroke::new(1.0, Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 35)),
        );
    }

    // Edge shadow
    painter.vline(rect.left(), rect.y_range(), Stroke::new(1.0, Color32::from_rgb(55, 35, 18)));
    painter.vline(rect.right() - 0.5, rect.y_range(), Stroke::new(1.0, Color32::from_rgb(55, 35, 18)));
}
