use cgmath::{EuclideanSpace, InnerSpace, Matrix4, Point3, SquareMatrix, Vector3};
use crate::model::Model;
use crate::shader::Shader;
use std::ffi::{CStr};

pub struct Lasso {
    model: Model,
    debug_model: Model
}

impl Lasso {
    pub fn new() -> Lasso {
        let model = Model::new("resources/lasso/lasso.obj");
        let debug_model = Model::new("resources/debug_axis/debug_axis.obj");
        let lasso = Lasso {
            model,
            debug_model
        };
        lasso
    }

    pub unsafe fn draw_btw_points(&self, p1: Vector3<f32>, p2: Vector3<f32>, shader: &Shader) {
        let mut rot_mat = Matrix4::look_at_dir(Point3::from_vec(p1), p2-p1, Vector3::unit_y());
        rot_mat = rot_mat.invert().expect("Lasso rotation matrix not invertible");
        let sca_mat: Matrix4<f32> = Matrix4::from_nonuniform_scale(1.0, 1.0, (p2 - p1).magnitude());
        let model = rot_mat * sca_mat;
        shader.set_mat4(c_str!("model"), &model);

        self.model.draw(shader);

        // temp: draw debug axis
        let pos_mat = Matrix4::from_translation(p2);
        let model = pos_mat;
        shader.set_mat4(c_str!("model"), &model);
        self.debug_model.draw(shader);
    }
}