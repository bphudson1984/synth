use nih_plug::prelude::*;
use nih_plug_egui::egui::{self, Color32, Pos2, Response, Sense, Shape, Stroke, Ui, Vec2, Widget};
use std::f32::consts::{PI, TAU};
use std::sync::Arc;

const MIN_ANGLE: f32 = PI * 0.75;
const MAX_ANGLE: f32 = PI * 2.25;
const SWEEP: f32 = MAX_ANGLE - MIN_ANGLE;

#[derive(Clone, Copy, PartialEq)]
pub enum KnobVariant {
    Black,
    Silver,
}

pub struct ParamKnob<'a, P: Param> {
    param: &'a P,
    setter: &'a ParamSetter<'a>,
    size: f32,
    label: &'a str,
    variant: KnobVariant,
}

impl<'a, P: Param> ParamKnob<'a, P> {
    pub fn new(param: &'a P, setter: &'a ParamSetter<'a>, label: &'a str) -> Self {
        Self { param, setter, size: 38.0, label, variant: KnobVariant::Black }
    }
    pub fn size(mut self, size: f32) -> Self { self.size = size; self }
    pub fn variant(mut self, variant: KnobVariant) -> Self { self.variant = variant; self }
}

impl<P: Param> Widget for ParamKnob<'_, P> {
    fn ui(self, ui: &mut Ui) -> Response {
        let col_width = self.size + 6.0;
        let total_height = self.size + 18.0;
        let (outer_rect, _) = ui.allocate_exact_size(
            Vec2::new(col_width, total_height), Sense::hover(),
        );

        // Label
        ui.painter().text(
            egui::pos2(outer_rect.center().x, outer_rect.min.y + 6.0),
            egui::Align2::CENTER_CENTER,
            self.label,
            egui::FontId::proportional(8.5),
            Color32::from_rgb(200, 198, 190),
        );

        // Knob area
        let knob_rect = egui::Rect::from_min_size(
            egui::pos2(outer_rect.center().x - self.size / 2.0, outer_rect.min.y + 14.0),
            Vec2::splat(self.size),
        );

        let response = ui.interact(knob_rect, ui.id().with(self.label), Sense::click_and_drag());

        // Interaction
        if response.drag_started() { self.setter.begin_set_parameter(self.param); }
        if response.dragged() {
            let s = if ui.input(|i| i.modifiers.shift) { 0.001 } else { 0.005 };
            let d = -response.drag_delta().y * s;
            let n = (self.param.modulated_normalized_value() + d).clamp(0.0, 1.0);
            self.setter.set_parameter_normalized(self.param, n);
        }
        if response.drag_stopped() { self.setter.end_set_parameter(self.param); }
        if response.double_clicked() {
            self.setter.begin_set_parameter(self.param);
            self.setter.set_parameter_normalized(self.param, self.param.default_normalized_value());
            self.setter.end_set_parameter(self.param);
        }
        response.clone().on_hover_text(format!("{}: {}", self.label, self.param.to_string()));

        let val = self.param.modulated_normalized_value();
        paint_knob(ui, knob_rect, val, self.variant);
        response
    }
}

fn paint_knob(ui: &Ui, rect: egui::Rect, normalized: f32, variant: KnobVariant) {
    let painter = ui.painter_at(rect);
    let center = rect.center();
    let r = rect.width() / 2.0 - 1.0;
    let angle = MIN_ANGLE + normalized * SWEEP;

    // Shadow beneath knob
    painter.circle_filled(
        center + Vec2::new(1.0, 2.0),
        r * 0.88,
        Color32::from_rgba_premultiplied(0, 0, 0, 50),
    );

    // Track arc (subtle background arc showing the range)
    draw_arc(&painter, center, r, MIN_ANGLE, MAX_ANGLE,
        Stroke::new(2.0, Color32::from_rgb(40, 40, 38)), 48);

    // Value indicator arc (from min to current position)
    if normalized > 0.01 {
        let val_color = match variant {
            KnobVariant::Black => Color32::from_rgb(140, 135, 125),
            KnobVariant::Silver => Color32::from_rgb(180, 175, 165),
        };
        draw_arc(&painter, center, r, MIN_ANGLE, angle,
            Stroke::new(2.0, val_color), (48.0 * normalized) as usize + 2);
    }

    match variant {
        KnobVariant::Black => {
            let body_r = r * 0.82;
            let cap_r = body_r * 0.68;

            // Knob body — dark with subtle edge bevel
            paint_radial_gradient(&painter, center, body_r,
                Color32::from_rgb(42, 42, 40),   // center
                Color32::from_rgb(25, 25, 23),    // edge
                Vec2::new(-body_r * 0.1, -body_r * 0.15), // highlight offset
            );
            // Edge ring
            painter.circle_stroke(center, body_r, Stroke::new(0.8, Color32::from_rgb(60, 58, 55)));

            // Silver cap — metallic with highlight
            paint_radial_gradient(&painter, center, cap_r,
                Color32::from_rgb(185, 183, 178),  // bright center (highlight)
                Color32::from_rgb(100, 98, 93),    // darker edge
                Vec2::new(-cap_r * 0.2, -cap_r * 0.25), // light source offset
            );
            // Cap edge ring
            painter.circle_stroke(center, cap_r, Stroke::new(0.6, Color32::from_rgb(75, 73, 68)));

            // Inner shadow on cap edge (gives depth)
            painter.circle_stroke(
                center, cap_r - 1.0,
                Stroke::new(0.5, Color32::from_rgba_premultiplied(255, 255, 255, 20)),
            );

            // Indicator line
            let p0 = center + Vec2::new(angle.cos(), angle.sin()) * (cap_r * 0.25);
            let p1 = center + Vec2::new(angle.cos(), angle.sin()) * (cap_r * 0.9);
            painter.line_segment([p0, p1], Stroke::new(2.0, Color32::WHITE));
            // Indicator glow
            painter.line_segment([p0, p1],
                Stroke::new(4.0, Color32::from_rgba_premultiplied(255, 255, 255, 25)));
        }
        KnobVariant::Silver => {
            let body_r = r * 0.82;

            // Full silver knob body
            paint_radial_gradient(&painter, center, body_r,
                Color32::from_rgb(210, 208, 200),
                Color32::from_rgb(130, 128, 120),
                Vec2::new(-body_r * 0.2, -body_r * 0.25),
            );
            painter.circle_stroke(center, body_r, Stroke::new(0.6, Color32::from_rgb(90, 88, 82)));

            // Indicator
            let p0 = center + Vec2::new(angle.cos(), angle.sin()) * (body_r * 0.25);
            let p1 = center + Vec2::new(angle.cos(), angle.sin()) * (body_r * 0.88);
            painter.line_segment([p0, p1], Stroke::new(2.0, Color32::from_rgb(50, 48, 45)));
        }
    }
}

fn draw_arc(
    painter: &egui::Painter, center: Pos2, radius: f32,
    start: f32, end: f32, stroke: Stroke, segments: usize,
) {
    if segments < 2 { return; }
    let points: Vec<Pos2> = (0..=segments)
        .map(|i| {
            let t = i as f32 / segments as f32;
            let a = start + t * (end - start);
            center + Vec2::new(a.cos(), a.sin()) * radius
        })
        .collect();
    painter.add(Shape::line(points, stroke));
}

/// Radial gradient circle with offset highlight
fn paint_radial_gradient(
    painter: &egui::Painter, center: Pos2, radius: f32,
    center_color: Color32, edge_color: Color32,
    highlight_offset: Vec2,
) {
    use egui::epaint::Mesh;

    let n = 36u32;
    let mut mesh = Mesh::default();

    // Center vertex — the highlight point
    mesh.colored_vertex(center + highlight_offset, center_color);

    // Outer vertices
    for i in 0..=n {
        let a = (i as f32 / n as f32) * TAU;
        mesh.colored_vertex(center + Vec2::new(a.cos(), a.sin()) * radius, edge_color);
    }

    // Triangle fan
    for i in 0..n {
        mesh.add_triangle(0, i + 1, i + 2);
    }

    painter.add(Shape::mesh(Arc::new(mesh)));
}
