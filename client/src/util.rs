use cgmath::{InnerSpace, vec3, Vector2, Vector3};

pub fn project_on_plane(vector: Vector3<f32>, plane_normal: Vector3<f32>) -> Vector3<f32> {
    let num = plane_normal.dot(plane_normal);
    let num2 = vector.dot(plane_normal);
    return vec3(vector.x - plane_normal.x * num2 / num,
                vector.y - plane_normal.y * num2 / num,
                vector.z - plane_normal.z * num2 / num);
}

pub fn vec2_signed_angle(from: Vector2<f32>, to: Vector2<f32>) -> f32 {
    let dot = from.dot(to);
    let det = from.x * to.y - from.y * to.x;
    return det.atan2(dot);
}