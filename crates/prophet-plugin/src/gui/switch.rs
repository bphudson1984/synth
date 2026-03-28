use nih_plug::prelude::*;
use nih_plug_egui::egui::{self, Color32, Response, Sense, Stroke, Ui, Vec2, Widget};

pub struct ParamSwitch<'a, P: Param> {
    param: &'a P,
    setter: &'a ParamSetter<'a>,
    label: &'a str,
}

impl<'a, P: Param> ParamSwitch<'a, P> {
    pub fn new(param: &'a P, setter: &'a ParamSetter<'a>, label: &'a str) -> Self {
        Self { param, setter, label }
    }
}

impl<P: Param> Widget for ParamSwitch<'_, P> {
    fn ui(self, ui: &mut Ui) -> Response {
        let col_width = 32.0;
        let total_height = 44.0;
        let (outer_rect, _) = ui.allocate_exact_size(
            Vec2::new(col_width, total_height), Sense::hover(),
        );

        let is_on = self.param.modulated_normalized_value() > 0.5;
        let painter = ui.painter();

        // LED at top — multi-layer glow for realism
        let led_center = egui::pos2(outer_rect.center().x, outer_rect.min.y + 5.0);
        let led_r = 3.0;

        if is_on {
            // Outer glow (large, faint)
            painter.circle_filled(led_center, led_r * 3.0,
                Color32::from_rgba_premultiplied(255, 30, 20, 15));
            // Mid glow
            painter.circle_filled(led_center, led_r * 2.0,
                Color32::from_rgba_premultiplied(255, 35, 25, 35));
            // Inner glow
            painter.circle_filled(led_center, led_r * 1.3,
                Color32::from_rgba_premultiplied(255, 50, 35, 80));
            // LED body
            painter.circle_filled(led_center, led_r, Color32::from_rgb(255, 45, 30));
            // Hot center spot
            painter.circle_filled(
                led_center + Vec2::new(-0.5, -0.5), led_r * 0.4,
                Color32::from_rgb(255, 180, 150));
        } else {
            // Off LED — dark red with subtle specular
            painter.circle_filled(led_center, led_r, Color32::from_rgb(55, 18, 15));
            painter.circle_filled(
                led_center + Vec2::new(-0.5, -0.5), led_r * 0.3,
                Color32::from_rgb(70, 25, 20));
        }
        painter.circle_stroke(led_center, led_r, Stroke::new(0.5, Color32::from_rgb(35, 12, 10)));

        // Switch button — raised 3D appearance
        let sw_w = 24.0;
        let sw_h = 14.0;
        let switch_rect = egui::Rect::from_center_size(
            egui::pos2(outer_rect.center().x, outer_rect.min.y + 21.0),
            Vec2::new(sw_w, sw_h),
        );

        let response = ui.interact(switch_rect, ui.id().with(self.label), Sense::click());

        if response.clicked() {
            self.setter.begin_set_parameter(self.param);
            self.setter.set_parameter_normalized(self.param, if is_on { 0.0 } else { 1.0 });
            self.setter.end_set_parameter(self.param);
        }

        let rounding = egui::CornerRadius::same(2);

        // Button shadow
        let shadow_rect = switch_rect.translate(Vec2::new(0.5, 1.5));
        painter.rect_filled(shadow_rect, rounding, Color32::from_rgba_premultiplied(0, 0, 0, 40));

        if is_on {
            // Pressed state — darker, inset look
            painter.rect_filled(switch_rect, rounding, Color32::from_rgb(38, 38, 36));
            // Top edge shadow (inset)
            painter.hline(switch_rect.x_range(), switch_rect.top() + 1.0,
                Stroke::new(0.5, Color32::from_rgb(25, 25, 23)));
        } else {
            // Raised state — lighter top, darker bottom
            painter.rect_filled(switch_rect, rounding, Color32::from_rgb(52, 52, 48));
            // Top highlight
            painter.hline(switch_rect.x_range(), switch_rect.top() + 1.0,
                Stroke::new(0.5, Color32::from_rgb(68, 68, 62)));
            // Bottom shadow
            painter.hline(switch_rect.x_range(), switch_rect.bottom() - 1.0,
                Stroke::new(0.5, Color32::from_rgb(35, 35, 32)));
        }
        painter.rect_stroke(switch_rect, rounding,
            Stroke::new(0.5, Color32::from_rgb(65, 63, 58)), egui::StrokeKind::Outside);

        // Label
        painter.text(
            egui::pos2(outer_rect.center().x, outer_rect.min.y + 37.0),
            egui::Align2::CENTER_CENTER,
            self.label,
            egui::FontId::proportional(8.0),
            Color32::from_rgb(185, 183, 178),
        );

        response
    }
}
