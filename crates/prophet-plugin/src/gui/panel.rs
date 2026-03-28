use nih_plug::prelude::*;
use nih_plug_egui::egui::{self, Color32, Stroke, Vec2};
use std::sync::Arc;

use super::knob::{KnobVariant, ParamKnob};
use super::switch::ParamSwitch;
use crate::ProphetParams;
use crate::presets;

// Colors
const PANEL_BG: Color32 = Color32::from_rgb(28, 28, 27);
const SECTION_DIVIDER: Color32 = Color32::from_rgb(48, 46, 42);
const SECTION_LABEL: Color32 = Color32::from_rgb(195, 192, 182);
const BRAND_COLOR: Color32 = Color32::from_rgb(175, 165, 145);
const BRAND_BRIGHT: Color32 = Color32::from_rgb(230, 225, 210);

pub fn draw_panel(ctx: &egui::Context, setter: &ParamSetter, params: &Arc<ProphetParams>) {
    // Wood cheeks
    egui::SidePanel::left("wood_l")
        .exact_width(28.0)
        .resizable(false)
        .frame(egui::Frame::new().fill(Color32::from_rgb(100, 62, 35)).inner_margin(0))
        .show(ctx, |ui| paint_wood(ui));

    egui::SidePanel::right("wood_r")
        .exact_width(28.0)
        .resizable(false)
        .frame(egui::Frame::new().fill(Color32::from_rgb(100, 62, 35)).inner_margin(0))
        .show(ctx, |ui| paint_wood(ui));

    // Main panel
    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(PANEL_BG).inner_margin(egui::Margin::symmetric(10, 6)))
        .show(ctx, |ui| {
            // Panel texture — subtle grain
            paint_panel_texture(ui);

            // Header
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("SEQUENTIAL CIRCUITS")
                        .size(9.5)
                        .color(BRAND_COLOR),
                );
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new("PROPHET-5")
                        .size(14.0)
                        .color(BRAND_BRIGHT)
                        .strong(),
                );

                ui.add_space(30.0);

                // Preset dropdown
                draw_preset_selector(ui, setter, params);
            });
            ui.add_space(4.0);

            // Separator — thin metallic line
            let rect = ui.available_rect_before_wrap();
            let y = rect.top();
            ui.painter().hline(rect.x_range(), y, Stroke::new(0.5, Color32::from_rgb(55, 53, 48)));
            ui.painter().hline(rect.x_range(), y + 1.0, Stroke::new(0.3, Color32::from_rgb(35, 33, 30)));
            ui.add_space(8.0);

            // Sections
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(0.0, 0.0);

                section(ui, "POLY MOD", 100.0, |ui| {
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

                section(ui, "LFO", 100.0, |ui| {
                    row(ui, |ui| {
                        ui.add(ParamKnob::new(&params.lfo_freq, setter, "FREQ").size(34.0));
                        ui.add(ParamKnob::new(&params.lfo_initial_amount, setter, "AMT").size(34.0));
                    });
                    row(ui, |ui| {
                        ui.add(ParamSwitch::new(&params.lfo_tri, setter, "TRI"));
                        ui.add(ParamSwitch::new(&params.lfo_saw, setter, "SAW"));
                        ui.add(ParamSwitch::new(&params.lfo_square, setter, "SQR"));
                    });
                });

                divider(ui);

                section(ui, "WHEEL MOD", 105.0, |ui| {
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

                section(ui, "OSCILLATOR A", 115.0, |ui| {
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

                section(ui, "OSCILLATOR B", 150.0, |ui| {
                    row(ui, |ui| {
                        ui.add(ParamKnob::new(&params.osc_b_semitones, setter, "FREQ").size(34.0));
                        ui.add(ParamKnob::new(&params.osc_b_fine, setter, "FINE").size(34.0));
                        ui.add(ParamKnob::new(&params.osc_b_pw, setter, "PW").size(34.0));
                    });
                    row(ui, |ui| {
                        ui.add(ParamSwitch::new(&params.osc_b_saw, setter, "SAW"));
                        ui.add(ParamSwitch::new(&params.osc_b_tri, setter, "TRI"));
                        ui.add(ParamSwitch::new(&params.osc_b_pulse, setter, "PULSE"));
                    });
                });

                divider(ui);

                section(ui, "MIXER", 130.0, |ui| {
                    row(ui, |ui| {
                        ui.add(ParamKnob::new(&params.osc_a_level, setter, "OSC A").size(34.0));
                        ui.add(ParamKnob::new(&params.osc_b_level, setter, "OSC B").size(34.0));
                        ui.add(ParamKnob::new(&params.noise_level, setter, "NOISE").size(34.0));
                    });
                });

                divider(ui);

                section(ui, "FILTER", 130.0, |ui| {
                    row(ui, |ui| {
                        ui.add(ParamKnob::new(&params.filter_cutoff, setter, "CUTOFF").size(40.0));
                        ui.add(ParamKnob::new(&params.filter_resonance, setter, "RES").size(40.0));
                    });
                    row(ui, |ui| {
                        ui.add(ParamKnob::new(&params.filter_env_amount, setter, "ENV AMT").size(34.0));
                        ui.add(ParamKnob::new(&params.filter_drive, setter, "DRIVE").size(34.0));
                    });
                });

                divider(ui);

                section(ui, "FILTER ENVELOPE", 170.0, |ui| {
                    row(ui, |ui| {
                        ui.add(ParamKnob::new(&params.filter_attack, setter, "A").size(34.0));
                        ui.add(ParamKnob::new(&params.filter_decay, setter, "D").size(34.0));
                        ui.add(ParamKnob::new(&params.filter_sustain, setter, "S").size(34.0));
                        ui.add(ParamKnob::new(&params.filter_release, setter, "R").size(34.0));
                    });
                });

                divider(ui);

                section(ui, "AMPLIFIER", 170.0, |ui| {
                    row(ui, |ui| {
                        ui.add(ParamKnob::new(&params.amp_attack, setter, "A").size(34.0));
                        ui.add(ParamKnob::new(&params.amp_decay, setter, "D").size(34.0));
                        ui.add(ParamKnob::new(&params.amp_sustain, setter, "S").size(34.0));
                        ui.add(ParamKnob::new(&params.amp_release, setter, "R").size(34.0));
                    });
                });

                divider(ui);

                section(ui, "MASTER", 130.0, |ui| {
                    row(ui, |ui| {
                        ui.add(
                            ParamKnob::new(&params.volume, setter, "VOL")
                                .size(44.0)
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

            // Effects separator
            ui.add_space(6.0);
            let rect = ui.available_rect_before_wrap();
            ui.painter().hline(rect.x_range(), rect.top(), Stroke::new(0.5, Color32::from_rgb(55, 53, 48)));
            ui.painter().hline(rect.x_range(), rect.top() + 1.0, Stroke::new(0.3, Color32::from_rgb(35, 33, 30)));
            ui.add_space(6.0);

            // Effects row
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(0.0, 0.0);

                section(ui, "CHORUS", 130.0, |ui| {
                    row(ui, |ui| {
                        ui.add(ParamKnob::new(&params.chorus_rate, setter, "RATE").size(34.0));
                        ui.add(ParamKnob::new(&params.chorus_depth, setter, "DEPTH").size(34.0));
                        ui.add(ParamKnob::new(&params.chorus_mix, setter, "MIX").size(34.0));
                    });
                });

                divider(ui);

                section(ui, "DELAY", 170.0, |ui| {
                    row(ui, |ui| {
                        ui.add(ParamKnob::new(&params.delay_time, setter, "TIME").size(34.0));
                        ui.add(ParamKnob::new(&params.delay_feedback, setter, "FDBK").size(34.0));
                        ui.add(ParamKnob::new(&params.delay_tone, setter, "TONE").size(34.0));
                        ui.add(ParamKnob::new(&params.delay_mix, setter, "MIX").size(34.0));
                    });
                });

                divider(ui);

                section(ui, "REVERB", 130.0, |ui| {
                    row(ui, |ui| {
                        ui.add(ParamKnob::new(&params.reverb_decay, setter, "DECAY").size(34.0));
                        ui.add(ParamKnob::new(&params.reverb_damping, setter, "DAMP").size(34.0));
                        ui.add(ParamKnob::new(&params.reverb_mix, setter, "MIX").size(34.0));
                    });
                });
            });
        });
}

fn draw_preset_selector(ui: &mut egui::Ui, setter: &ParamSetter, params: &Arc<ProphetParams>) {
    let factory = presets::factory_presets();
    let categories = presets::categories();

    let current_name = ui.ctx().data_mut(|d| {
        d.get_temp_mut_or_insert_with::<String>(
            egui::Id::new("current_preset"), || "Init Saw".to_string(),
        ).clone()
    });

    ui.label(egui::RichText::new("PROGRAM").size(8.0).color(Color32::from_rgb(140, 138, 130)));

    egui::ComboBox::from_id_salt("preset_selector")
        .selected_text(egui::RichText::new(&current_name).size(10.0).color(Color32::from_rgb(210, 205, 185)))
        .width(180.0)
        .show_ui(ui, |ui| {
            ui.style_mut().visuals.widgets.inactive.bg_fill = Color32::from_rgb(35, 33, 30);
            ui.style_mut().visuals.widgets.hovered.bg_fill = Color32::from_rgb(55, 50, 42);

            for &cat in categories {
                ui.label(egui::RichText::new(cat).size(9.0).color(Color32::from_rgb(170, 150, 110)).strong());
                ui.separator();
                for preset in &factory {
                    if preset.category != cat { continue; }
                    let sel = current_name == preset.name;
                    let label = egui::RichText::new(preset.name).size(10.0).color(
                        if sel { Color32::from_rgb(255, 215, 140) } else { Color32::from_rgb(195, 190, 180) }
                    );
                    if ui.selectable_label(sel, label).clicked() {
                        presets::apply_preset(preset, params, setter);
                        ui.ctx().data_mut(|d| {
                            d.insert_temp(egui::Id::new("current_preset"), preset.name.to_string());
                        });
                    }
                }
                ui.add_space(4.0);
            }
        });
}

fn section(ui: &mut egui::Ui, title: &str, width: f32, content: impl FnOnce(&mut egui::Ui)) {
    ui.allocate_ui(Vec2::new(width, ui.available_height()), |ui| {
        ui.push_id(title, |ui| {
            ui.set_min_width(width);
            ui.vertical(|ui| {
                // Section title
                ui.painter().text(
                    egui::pos2(ui.available_rect_before_wrap().center().x, ui.cursor().top() + 5.0),
                    egui::Align2::CENTER_CENTER,
                    title,
                    egui::FontId::proportional(9.0),
                    SECTION_LABEL,
                );
                ui.add_space(14.0);
                content(ui);
            });
        });
    });
}

fn row(ui: &mut egui::Ui, content: impl FnOnce(&mut egui::Ui)) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(2.0, 2.0);
        content(ui);
    });
}

fn divider(ui: &mut egui::Ui) {
    let rect = ui.available_rect_before_wrap();
    let x = rect.left() + 3.0;
    ui.painter().vline(x, rect.y_range(), Stroke::new(0.5, SECTION_DIVIDER));
    // Highlight edge
    ui.painter().vline(x + 1.0, rect.y_range(),
        Stroke::new(0.3, Color32::from_rgba_premultiplied(255, 255, 255, 8)));
    ui.add_space(7.0);
}

fn paint_panel_texture(ui: &mut egui::Ui) {
    let rect = ui.available_rect_before_wrap();
    let painter = ui.painter();

    // Very subtle brushed-aluminum grain — barely visible
    let step = 3.0;
    let mut y = rect.top();
    let mut i = 0u32;
    while y < rect.bottom() {
        let hash = i.wrapping_mul(2654435761);
        let brightness = ((hash >> 16) & 0x7) as u8;
        // Much lower alpha — just a hint of texture
        let alpha = 1 + (brightness / 4);
        painter.hline(rect.x_range(), y,
            Stroke::new(0.3, Color32::from_rgba_premultiplied(255, 255, 255, alpha)));
        y += step;
        i += 1;
    }
}

fn paint_wood(ui: &mut egui::Ui) {
    let rect = ui.available_rect_before_wrap();
    let painter = ui.painter_at(rect);

    // Base wood gradient — darker at edges
    let mid_x = rect.center().x;
    let half_w = rect.width() / 2.0;

    // Base fill
    painter.rect_filled(rect, 0.0, Color32::from_rgb(105, 65, 35));

    // Vertical grain lines with varying thickness and color
    let grains = [
        (0.12, 1.2, Color32::from_rgb(75, 45, 22)),
        (0.28, 0.8, Color32::from_rgb(125, 80, 45)),
        (0.42, 1.5, Color32::from_rgb(70, 42, 20)),
        (0.55, 0.6, Color32::from_rgb(135, 88, 50)),
        (0.68, 1.0, Color32::from_rgb(80, 48, 25)),
        (0.82, 0.7, Color32::from_rgb(120, 75, 42)),
        (0.92, 1.3, Color32::from_rgb(68, 40, 18)),
    ];

    for &(t, width, color) in &grains {
        let x = rect.left() + t * rect.width();
        // Slight waviness
        let wave_offset = (t * 7.3).sin() * 1.5;
        painter.vline(x + wave_offset, rect.y_range(),
            Stroke::new(width, Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 45)));
    }

    // Edge shadows — darker at the borders
    painter.vline(rect.left(), rect.y_range(), Stroke::new(2.0, Color32::from_rgb(50, 30, 15)));
    painter.vline(rect.right() - 1.0, rect.y_range(), Stroke::new(2.0, Color32::from_rgb(50, 30, 15)));

    // Inner edge highlights
    painter.vline(rect.left() + 2.0, rect.y_range(),
        Stroke::new(0.5, Color32::from_rgba_premultiplied(255, 200, 150, 15)));
    painter.vline(rect.right() - 3.0, rect.y_range(),
        Stroke::new(0.5, Color32::from_rgba_premultiplied(255, 200, 150, 15)));

    // Top/bottom edges
    painter.hline(rect.x_range(), rect.top(), Stroke::new(1.0, Color32::from_rgb(45, 28, 12)));
    painter.hline(rect.x_range(), rect.bottom() - 1.0, Stroke::new(1.0, Color32::from_rgb(45, 28, 12)));
}
