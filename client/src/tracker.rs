use crate::sprite_renderer::Sprite;
use crate::util::vec2_angle;
use cgmath::{vec2, InnerSpace, Matrix4, Vector2};
use std::f32::consts::PI;

pub struct Tracker {
    line_width: f32,
    rect: Sprite,
}

impl Tracker {
    pub unsafe fn new(projection: Matrix4<f32>, shader_id: u32) -> Tracker {
        let rect = Sprite::new(projection, shader_id);

        let tracker = Tracker {
            line_width: 10.0,
            rect,
        };

        tracker
    }

    pub fn set_line_width(&mut self, width: f32) {
        self.line_width = width;
    }

    pub unsafe fn draw_from_vectors(&self, v1: Vector2<f32>, v2: Vector2<f32>) {
        let screen_size = vec2(800.0, 600.0);
        let top_right_angle = theta(vec2(-screen_size.x / 2.0, screen_size.y / 2.0));
        let top_left_angle = PI - top_right_angle;
        let bot_left_angle = PI + top_right_angle;
        let bot_right_angle = 2.0 * PI - top_right_angle;
        let angle1 = theta(v1);
        let angle2 = theta(v2);
        if angle1 > top_right_angle {
            let ratio1 = 300.0 / v1.y;
            let ratio2 = 300.0 / v2.y;
            let intersection1 = screen_size / 2.0 + v1 * ratio1;
            let intersection2 = screen_size / 2.0 + v1 * ratio1;
            self.rect.draw_from_corners(
                vec2(
                    intersection1.x,
                    screen_size.y,
                ),
                vec2(
                    intersection2.x,
                    screen_size.y - self.line_width,
                ),
            );
        }
    }
}

fn theta(vector: Vector2<f32>) -> f32 {
    let mut angle = vec2_angle(vec2(1.0, 0.0), vector);
    if angle < 0.0 {
        angle += 2.0 * PI;
    }
    return angle;
}

// fn intersection_x(x: f32, vec: Vector2<f32>) -> Vector2<f32>{
//     let ratio = x /
// }