use nih_plug::prelude::*;
use nih_plug_egui::egui::{self, Color32, Pos2, Response, Sense, Shape, Stroke, Ui, Vec2, Widget};
use std::f32::consts::PI;

const MIN_ANGLE: f32 = PI * 0.75;
const MAX_ANGLE: f32 = PI * 2.25;
const SWEEP: f32 = MAX_ANGLE - MIN_ANGLE;

const KNOB_BODY: Color32 = Color32::from_rgb(35, 35, 35);
const KNOB_EDGE: Color32 = Color32::from_rgb(55, 55, 55);
const KNOB_CAP_CENTER: Color32 = Color32::from_rgb(160, 160, 155);
const KNOB_CAP_EDGE: Color32 = Color32::from_rgb(90, 90, 85);
const KNOB_INDICATOR: Color32 = Color32::WHITE;
const TRACK_BG: Color32 = Color32::from_rgb(50, 50, 50);
const SILVER_CAP_CENTER: Color32 = Color32::from_rgb(200, 200, 195);
const SILVER_CAP_EDGE: Color32 = Color32::from_rgb(140, 140, 135);

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
        Self {
            param,
            setter,
            size: 38.0,
            label,
            variant: KnobVariant::Black,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: KnobVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl<P: Param> Widget for ParamKnob<'_, P> {
    fn ui(self, ui: &mut Ui) -> Response {
        // Fixed-size column for the knob + label
        let col_width = self.size + 4.0;
        let total_height = self.size + 16.0;

        let (outer_rect, _) = ui.allocate_exact_size(
            Vec2::new(col_width, total_height),
            Sense::hover(),
        );

        // Label above (centered, no wrapping)
        let label_rect = egui::Rect::from_min_size(
            outer_rect.min,
            Vec2::new(col_width, 12.0),
        );
        ui.painter().text(
            label_rect.center(),
            egui::Align2::CENTER_CENTER,
            self.label,
            egui::FontId::proportional(8.5),
            Color32::from_rgb(210, 210, 205),
        );

        // Knob area
        let knob_rect = egui::Rect::from_min_size(
            egui::pos2(
                outer_rect.center().x - self.size / 2.0,
                outer_rect.min.y + 14.0,
            ),
            Vec2::splat(self.size),
        );

        let response = ui.interact(knob_rect, ui.id().with(self.label), Sense::click_and_drag());

        // Interaction
        let normalized = self.param.modulated_normalized_value();

        if response.drag_started() {
            self.setter.begin_set_parameter(self.param);
        }
        if response.dragged() {
            let sensitivity = if ui.input(|i| i.modifiers.shift) { 0.001 } else { 0.005 };
            let delta = -response.drag_delta().y * sensitivity;
            let new = (normalized + delta).clamp(0.0, 1.0);
            self.setter.set_parameter_normalized(self.param, new);
        }
        if response.drag_stopped() {
            self.setter.end_set_parameter(self.param);
        }
        if response.double_clicked() {
            self.setter.begin_set_parameter(self.param);
            self.setter
                .set_parameter_normalized(self.param, self.param.default_normalized_value());
            self.setter.end_set_parameter(self.param);
        }

        response.clone().on_hover_text(format!("{}: {}", self.label, self.param.to_string()));

        // Paint
        let current = self.param.modulated_normalized_value();
        paint_knob(ui, knob_rect, current, self.variant);

        response
    }
}

fn paint_knob(ui: &Ui, rect: egui::Rect, normalized: f32, variant: KnobVariant) {
    let painter = ui.painter_at(rect);
    let center = rect.center();
    let outer_r = rect.width() / 2.0 - 1.0;
    let body_r = outer_r * 0.82;
    let cap_r = body_r * 0.65;
    let value_angle = MIN_ANGLE + normalized * SWEEP;

    // Track arc
    draw_arc(&painter, center, outer_r, MIN_ANGLE, MAX_ANGLE, Stroke::new(2.0, TRACK_BG), 48);

    match variant {
        KnobVariant::Black => {
            painter.circle_filled(center, body_r, KNOB_BODY);
            painter.circle_stroke(center, body_r, Stroke::new(0.5, KNOB_EDGE));
            paint_gradient_circle(&painter, center, cap_r, KNOB_CAP_CENTER, KNOB_CAP_EDGE);
            // Indicator
            let r_inner = cap_r * 0.3;
            let r_outer = cap_r * 0.85;
            let p0 = center + Vec2::new(value_angle.cos(), value_angle.sin()) * r_inner;
            let p1 = center + Vec2::new(value_angle.cos(), value_angle.sin()) * r_outer;
            painter.line_segment([p0, p1], Stroke::new(2.0, KNOB_INDICATOR));
        }
        KnobVariant::Silver => {
            paint_gradient_circle(&painter, center, body_r, SILVER_CAP_CENTER, SILVER_CAP_EDGE);
            let r_inner = body_r * 0.3;
            let r_outer = body_r * 0.85;
            let p0 = center + Vec2::new(value_angle.cos(), value_angle.sin()) * r_inner;
            let p1 = center + Vec2::new(value_angle.cos(), value_angle.sin()) * r_outer;
            painter.line_segment([p0, p1], Stroke::new(2.0, KNOB_INDICATOR));
        }
    }
}

fn draw_arc(
    painter: &egui::Painter, center: Pos2, radius: f32,
    start: f32, end: f32, stroke: Stroke, segments: usize,
) {
    let points: Vec<Pos2> = (0..=segments)
        .map(|i| {
            let t = i as f32 / segments as f32;
            let angle = start + t * (end - start);
            center + Vec2::new(angle.cos(), angle.sin()) * radius
        })
        .collect();
    painter.add(Shape::line(points, stroke));
}

fn paint_gradient_circle(
    painter: &egui::Painter, center: Pos2, radius: f32,
    center_color: Color32, edge_color: Color32,
) {
    use egui::epaint::Mesh;
    use std::sync::Arc;

    let n = 32u32;
    let mut mesh = Mesh::default();
    let highlight = center + Vec2::new(-radius * 0.15, -radius * 0.2);
    mesh.colored_vertex(highlight, center_color);
    for i in 0..=n {
        let angle = (i as f32 / n as f32) * std::f32::consts::TAU;
        mesh.colored_vertex(center + Vec2::new(angle.cos(), angle.sin()) * radius, edge_color);
    }
    for i in 0..n {
        mesh.add_triangle(0, i + 1, i + 2);
    }
    painter.add(Shape::mesh(Arc::new(mesh)));
    painter.circle_stroke(center, radius, Stroke::new(0.5, Color32::from_rgb(70, 70, 65)));
}
