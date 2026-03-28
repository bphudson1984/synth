use nih_plug::prelude::*;
use nih_plug_egui::egui::{self, Color32, Response, Sense, Stroke, Ui, Vec2, Widget};

const LED_ON: Color32 = Color32::from_rgb(255, 40, 30);
const LED_OFF: Color32 = Color32::from_rgb(60, 20, 18);
const SWITCH_BODY: Color32 = Color32::from_rgb(50, 50, 48);
const SWITCH_BORDER: Color32 = Color32::from_rgb(75, 75, 70);

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
        let total_height = 42.0;

        let (outer_rect, _) = ui.allocate_exact_size(
            Vec2::new(col_width, total_height),
            Sense::hover(),
        );

        let is_on = self.param.modulated_normalized_value() > 0.5;
        let painter = ui.painter();

        // LED at top
        let led_center = egui::pos2(outer_rect.center().x, outer_rect.min.y + 5.0);
        let led_r = 3.0;
        if is_on {
            painter.circle_filled(led_center, led_r * 2.0, Color32::from_rgba_premultiplied(255, 40, 30, 25));
            painter.circle_filled(led_center, led_r * 1.3, Color32::from_rgba_premultiplied(255, 40, 30, 50));
            painter.circle_filled(led_center, led_r, LED_ON);
        } else {
            painter.circle_filled(led_center, led_r, LED_OFF);
        }
        painter.circle_stroke(led_center, led_r, Stroke::new(0.5, Color32::from_gray(40)));

        // Switch button
        let sw_w = 24.0;
        let sw_h = 14.0;
        let switch_rect = egui::Rect::from_center_size(
            egui::pos2(outer_rect.center().x, outer_rect.min.y + 20.0),
            Vec2::new(sw_w, sw_h),
        );

        let response = ui.interact(switch_rect, ui.id().with(self.label), Sense::click());

        if response.clicked() {
            self.setter.begin_set_parameter(self.param);
            self.setter.set_parameter_normalized(self.param, if is_on { 0.0 } else { 1.0 });
            self.setter.end_set_parameter(self.param);
        }

        let rounding = egui::CornerRadius::same(2);
        painter.rect_filled(switch_rect, rounding, SWITCH_BODY);
        painter.rect_stroke(switch_rect, rounding, Stroke::new(0.5, SWITCH_BORDER), egui::StrokeKind::Outside);
        if is_on {
            painter.rect_filled(switch_rect.shrink(1.5), rounding, Color32::from_rgb(42, 42, 40));
        }

        // Label below
        painter.text(
            egui::pos2(outer_rect.center().x, outer_rect.min.y + 36.0),
            egui::Align2::CENTER_CENTER,
            self.label,
            egui::FontId::proportional(8.0),
            Color32::from_rgb(195, 195, 190),
        );

        response
    }
}
