use crate::sprite_renderer::Sprite;
use crate::util::vec2_signed_angle;
use cgmath::{vec2, Matrix4, Vector2, Vector3, Point3, EuclideanSpace, Transform, Rad, InnerSpace, Angle, Basis2, Rotation2, Rotation, perspective, Deg, vec3, Vector4};
use std::f32::consts::PI;
use crate::camera::Camera;

pub struct Tracker {
    line_width: f32,
    line: Sprite,
    target_radius: f32,
    screen_size: Vector2<f32>,
}

impl Tracker {
    pub unsafe fn new(projection: Matrix4<f32>, shader_id: u32, target_radius: f32, screen_size: Vector2<f32>) -> Tracker {
        let rect = Sprite::new(projection, shader_id);

        let tracker = Tracker {
            line_width: 5.0,
            line: rect,
            target_radius,
            screen_size,
        };

        tracker
    }

    pub fn set_line_width(&mut self, width: f32) {
        self.line_width = width;
    }

    pub unsafe fn draw_tracker(&mut self, camera: &Camera, position: Vector3<f32>, color: Vector4<f32>, trackers: &mut Vec<(Rad<f32>, Vector2<f32>, Vector2<f32>, Vector4<f32>)>) {
        let point = Point3::from_vec(position);

        let projection = camera.GetViewMatrix().transform_point(point).to_vec();
        let angle: Rad<f32>;
        let v1;
        let v2;
        if vec2(projection.x, projection.y).magnitude() >= self.target_radius {
            angle = Angle::asin(self.target_radius / vec2(projection.x, projection.y).magnitude());
            v1 = Basis2::<f32>::from_angle(angle*2.0).rotate_vector(vec2(projection.x, projection.y));
            v2 = Basis2::<f32>::from_angle(angle*-2.0).rotate_vector(vec2(projection.x, projection.y));
        } else {
            angle = Rad::<f32>::turn_div_4();
            v1 = Basis2::<f32>::from_angle(angle*2.0).rotate_vector(vec2(projection.x, projection.y));
            v2 = v1;
        }

        let v1_proj = perspective(Deg(camera.Zoom), self.screen_size.x / self.screen_size.y, 0.1, 10000.0).transform_vector(vec3(v1.x,v1.y,projection.z));
        let v2_proj = perspective(Deg(camera.Zoom), self.screen_size.x / self.screen_size.y, 0.1, 10000.0).transform_vector(vec3(v2.x,v2.y,projection.z));

        let mut v1_norm = vec2(v1_proj.x, v1_proj.y).normalize();
        let mut v2_norm = vec2(v2_proj.x, v2_proj.y).normalize();

        if v1_norm.x.is_nan() || v1_norm.y.is_nan() || v2_norm.x.is_nan() || v2_norm.y.is_nan() {
            v1_norm = vec2(0.0,1.0);
            v2_norm = vec2(0.0,1.0);
        }

        let mut angle_proj = v2_norm.angle(v1_norm).normalize();
        if angle_proj == Rad(0.0) {
            angle_proj = Rad::full_turn();
        }

        // self.line.set_color(color);
        trackers.push((angle_proj, v1_norm, v2_norm, color));
        // self.draw_from_vectors(v1_norm, v2_norm);
    }

    pub unsafe fn draw_all_trackers(&mut self, mut trackers: Vec<(Rad<f32>, Vector2<f32>, Vector2<f32>, Vector4<f32>)>) {
        trackers.sort_by(|a, b| {
            match b.0.partial_cmp(&a.0) {
                Some(ord) => ord,
                None => panic!("\nERROR SORTING TRACKERS\nTRACKER 1: {:?}\nTRACKER 2: {:?}\n",*a,*b)
            }
        });
        for tracker in trackers {
            let mut color = tracker.3;
            // hardcoded alpha
            color.w = 0.5;
            self.line.set_color(color);
            self.draw_from_vectors(tracker.1, tracker.2);
        }
    }

    pub unsafe fn draw_from_vectors(&self, v1: Vector2<f32>, v2: Vector2<f32>) {
        let screen_size = self.screen_size;

        // corner positions
        let top_right_corner = screen_size / 2.0;
        // let top_left_corner = vec2(-screen_size.x / 2.0, screen_size.y / 2.0);
        // let bot_left_corner = vec2(-screen_size.x / 2.0, -screen_size.y / 2.0);
        // let bot_left_corner = vec2(screen_size.x / 2.0, -screen_size.y / 2.0);

        // corner angles
        let top_right_angle = theta(top_right_corner);
        let top_left_angle = PI - top_right_angle;
        let bot_left_angle = PI + top_right_angle;
        let bot_right_angle = 2.0 * PI - top_right_angle;

        // vector angles
        let angle1 = theta(v1);
        let angle2 = theta(v2);

        // draw top edge
        {
            let ratio1 = (screen_size.y / 2.0) / v1.y;
            let ratio2 = (screen_size.y / 2.0) / v2.y;
            let v1_intersection = screen_size / 2.0 + v1 * ratio1;
            let v2_intersection = screen_size / 2.0 + v2 * ratio2;

            let (v1_x1, v1_x2, v2_x1, v2_x2) = find_points(
                angle1,
                angle2,
                v1_intersection.x,
                v2_intersection.x,
                top_right_angle,
                top_left_angle,
                screen_size.x,
                0.0,
            );

            self.line.draw_from_corners(
                vec2(v1_x1, screen_size.y),
                vec2(v1_x2, screen_size.y - self.line_width),
            );
            self.line.draw_from_corners(
                vec2(v2_x2, screen_size.y),
                vec2(v2_x1, screen_size.y - self.line_width),
            );
        }

        // draw bottom edge
        {
            let ratio1 = -(screen_size.y / 2.0) / v1.y;
            let ratio2 = -(screen_size.y / 2.0) / v2.y;
            let v1_intersection = screen_size / 2.0 + v1 * ratio1;
            let v2_intersection = screen_size / 2.0 + v2 * ratio2;

            let (v1_x1, v1_x2, v2_x1, v2_x2) = find_points(
                angle1,
                angle2,
                v1_intersection.x,
                v2_intersection.x,
                bot_left_angle,
                bot_right_angle,
                0.0,
                screen_size.x,
            );

            self.line
                .draw_from_corners(vec2(v1_x2, self.line_width), vec2(v1_x1, 0.0));
            self.line
                .draw_from_corners(vec2(v2_x1, self.line_width), vec2(v2_x2, 0.0));
        }

        // draw left edge
        {
            let ratio1 = -(screen_size.x / 2.0) / v1.x;
            let ratio2 = -(screen_size.x / 2.0) / v2.x;
            let v1_intersection = screen_size / 2.0 + v1 * ratio1;
            let v2_intersection = screen_size / 2.0 + v2 * ratio2;

            let (v1_y1, v1_y2, v2_y1, v2_y2) = find_points(
                angle1,
                angle2,
                v1_intersection.y,
                v2_intersection.y,
                top_left_angle,
                bot_left_angle,
                screen_size.y,
                0.0,
            );

            self.line.draw_from_corners(
                vec2(0.0, v1_y2),
                vec2(self.line_width, v1_y1),
            );
            self.line.draw_from_corners(
                vec2(0.0, v2_y1),
                vec2(self.line_width, v2_y2),
            );
        }

        // draw right edge
        {
            let ratio1 = (screen_size.x / 2.0) / v1.x;
            let ratio2 = (screen_size.x / 2.0) / v2.x;
            let v1_intersection = screen_size / 2.0 + v1 * ratio1;
            let v2_intersection = screen_size / 2.0 + v2 * ratio2;

            let mut angle1 = angle1 + PI;
            if angle1 > 2.0 * PI {
                angle1 -= 2.0 * PI;
            }
            let mut angle2 = angle2 + PI;
            if angle2 > 2.0 * PI {
                angle2 -= 2.0 * PI;
            }
            let bot_right_angle = bot_right_angle - PI;
            let top_right_angle = top_right_angle + PI;

            let (v1_y1, v1_y2, v2_y1, v2_y2) = find_points(
                angle1,
                angle2,
                v1_intersection.y,
                v2_intersection.y,
                bot_right_angle,
                top_right_angle,
                0.0,
                screen_size.y,
            );

            self.line.draw_from_corners(
                vec2(screen_size.x - self.line_width, v1_y1),
                vec2(screen_size.x, v1_y2),
            );
            self.line.draw_from_corners(
                vec2(screen_size.x - self.line_width, v2_y2),
                vec2(screen_size.x, v2_y1),
            );
        }
    }
}

fn find_points(
    angle1: f32,
    angle2: f32,
    intersection1: f32,
    intersection2: f32,
    lower_angle: f32,
    higher_angle: f32,
    low_point: f32,
    high_point: f32,
) -> (f32, f32, f32, f32) {
    let v1_in_range = angle1 > lower_angle && angle1 < higher_angle;
    let v2_in_range = angle2 > lower_angle && angle2 < higher_angle;

    let mut v1_p2 = low_point;
    let mut v2_p2 = high_point;

    let v1_closest_angle = closest_angle_clockwise(angle1, lower_angle, higher_angle, angle2);
    let v2_closest_angle =
        closest_angle_counter_clockwise(angle2, lower_angle, higher_angle, angle1);

    let v1_p1;
    if v1_closest_angle == higher_angle {
        v1_p1 = high_point;
    } else if v1_in_range {
        v1_p1 = intersection1;
        if v2_closest_angle != higher_angle {
            v2_p2 = v1_p1;
        }
    } else {
        v1_p1 = 0.0;
        if v1_closest_angle == angle2 {
            v1_p2 = 0.0;
        }
    }

    let mut v2_p1;
    if v2_closest_angle == lower_angle {
        v2_p1 = low_point;
    } else if v2_in_range {
        v2_p1 = intersection2;
        if v1_closest_angle != lower_angle {
            v1_p2 = v2_p1;
        }
    } else {
        v2_p1 = 0.0;
        if v2_closest_angle == angle1 {
            v2_p2 = 0.0;
        }
    }

    if !(v1_closest_angle == lower_angle && v2_closest_angle == higher_angle) {
        v2_p1 = low_point;
        v2_p2 = low_point;
    }

    (v1_p1, v1_p2, v2_p1, v2_p2)
}

fn theta(vector: Vector2<f32>) -> f32 {
    return full_turn_angle(vec2(1.0, 0.0), vector);
}

fn full_turn_angle(from: Vector2<f32>, to: Vector2<f32>) -> f32 {
    let mut angle = vec2_signed_angle(from, to);
    if angle < 0.0 {
        angle += 2.0 * PI;
    }
    return angle;
}

fn closest_angle_clockwise(v1: f32, lower_range: f32, higher_range: f32, v2: f32) -> f32 {
    let mut num1 = v1 - lower_range;
    if num1 <= 0.0 {
        num1 += 2.0 * PI;
    }
    let mut num2 = v1 - higher_range;
    if num2 <= 0.0 {
        num2 += 2.0 * PI;
    }
    let mut num3 = v1 - v2;
    if num3 <= 0.0 {
        num3 += 2.0 * PI;
    }
    return if num1 < num2 && num1 < num3 {
        lower_range
    } else if num2 < num3 {
        higher_range
    } else {
        v2
    }
}

fn closest_angle_counter_clockwise(v1: f32, lower_range: f32, higher_range: f32, v2: f32) -> f32 {
    let mut num1 = v1 - lower_range;
    if num1 < 0.0 {
        num1 += 2.0 * PI;
    }
    let mut num2 = v1 - higher_range;
    if num2 < 0.0 {
        num2 += 2.0 * PI;
    }
    let mut num3 = v1 - v2;
    if num3 < 0.0 {
        num3 += 2.0 * PI;
    }
    return if num1 > num2 && num1 > num3 {
        lower_range
    } else if num2 > num3 {
        higher_range
    } else {
        v2
    }
}
