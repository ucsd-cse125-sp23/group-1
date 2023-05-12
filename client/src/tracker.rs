use crate::sprite_renderer::Sprite;
use cgmath::{vec2, Matrix4, Vector2};

pub struct Tracker {
    line_width: f32,
    top_rect1: Sprite,
    top_rect2: Sprite,
    bottom_rect1: Sprite,
    bottom_rect2: Sprite,
    left_rect1: Sprite,
    left_rect2: Sprite,
    right_rect1: Sprite,
    right_rect2: Sprite,
}

impl Tracker {
    pub unsafe fn new(projection: Matrix4<f32>, shader_id: u32) -> Tracker {
        let t_rect1 = Sprite::new(projection, shader_id);
        let b_rect1 = Sprite::new(projection, shader_id);
        let l_rect1 = Sprite::new(projection, shader_id);
        let r_rect1 = Sprite::new(projection, shader_id);
        let t_rect2 = Sprite::new(projection, shader_id);
        let b_rect2 = Sprite::new(projection, shader_id);
        let l_rect2 = Sprite::new(projection, shader_id);
        let r_rect2 = Sprite::new(projection, shader_id);

        let tracker = Tracker {
            line_width: 10.0,
            top_rect1: t_rect1,
            top_rect2: t_rect2,
            bottom_rect1: b_rect1,
            bottom_rect2: b_rect2,
            left_rect1: l_rect1,
            left_rect2: l_rect2,
            right_rect1: r_rect1,
            right_rect2: r_rect2,
        };

        tracker
    }

    pub fn set_line_width(&mut self, width: f32) {
        self.line_width = width;
    }

    pub unsafe fn draw_from_vectors(&self, v1: Vector2<f32>, v2: Vector2<f32>) {
        // flip y since 0,0 is at the top left corner for screen space
        let v1 = vec2(v1.x, -v1.y);
        let v2 = vec2(v2.x, -v2.y);

        // draw top edge line
        let ratio1 = -300.0 / v1.y;
        let ratio2 = -300.0 / v2.y;
        let mut intersection1 = vec2(400.0, 300.0) + v1 * ratio1;
        let mut intersection2 = vec2(400.0, 300.0) + v2 * ratio2;
        if ratio1 < 0.0 {
            intersection1.x = 0.0;
        }
        if ratio2 < 0.0 {
            intersection2.x = 800.0;
        }
        println!("v1: {:?} v2: {:?}", intersection1, intersection2);
        if !(ratio1 < 0.0 && ratio2 < 0.0) {
            if intersection1.x < 800.0 {
                self.top_rect1.draw_from_corners(
                    intersection1,
                    vec2(intersection2.x.min(800.0), self.line_width),
                );
            }
            if intersection2.x > 0.0 {
                self.top_rect2.draw_from_corners(
                    vec2(intersection1.x.max(0.0), self.line_width),
                    intersection2,
                );
            }
        }

        // draw bottom edge line
        let ratio1 = 300.0 / v1.y;
        let ratio2 = 300.0 / v2.y;
        let mut intersection1 = vec2(400.0, 300.0) + v1 * ratio1;
        let mut intersection2 = vec2(400.0, 300.0) + v2 * ratio2;
        if ratio1 < 0.0 {
            intersection1.x = 800.0;
        }
        if ratio2 < 0.0 {
            intersection2.x = 0.0;
        }
        if !(ratio1 < 0.0 && ratio2 < 0.0) {
            if intersection1.x > 0.0 {
                self.top_rect1.draw_from_corners(
                    vec2(intersection2.x.max(0.0), 600.0 - self.line_width),
                    intersection1,
                );
            }
            if intersection2.x < 800.0 {
                self.top_rect2.draw_from_corners(
                    intersection2,
                    vec2(intersection1.x.min(800.0), 600.0 - self.line_width),
                );
            }
        }

        // draw left edge line
        let ratio1 = -400.0 / v1.x;
        let ratio2 = -400.0 / v2.x;
        let mut intersection1 = vec2(400.0, 300.0) + v1 * ratio1;
        let mut intersection2 = vec2(400.0, 300.0) + v2 * ratio2;
        if ratio1 < 0.0 {
            intersection1.y = 600.0;
        }
        if ratio2 < 0.0 {
            intersection2.y = 0.0;
        }
        if !(ratio1 < 0.0 && ratio2 < 0.0) {
            if intersection1.y > 0.0 {
                self.top_rect1.draw_from_corners(
                    intersection2,
                    vec2(self.line_width, intersection1.y.max(0.0)),
                );
            }
            if intersection2.y < 600.0 {
                self.top_rect2.draw_from_corners(
                    intersection2,
                    vec2(intersection1.x.max(0.0), self.line_width),
                );
            }
        }
    }
}
