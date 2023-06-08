use cgmath::{Vector3, Matrix4, Point3, SquareMatrix, EuclideanSpace, InnerSpace, perspective, Deg, Vector2, vec3, Quaternion, Transform};
use shared::shared_components::PositionComponent;
use std::time::Instant;
use std::ffi::CStr;
use crate::model::Model;
use crate::shader::Shader;
use crate::camera::Camera;

const FADE_RATE: f32 = 4.0;
const HALFHEIGHT: f32 = 0.5;

pub struct Tracer {
    model_ind: usize,
    p1: Vector3<f32>,
    p2: Vector3<f32>,
    alpha: f32,
}

impl Tracer {
    // return false when ready to be removed
    pub unsafe fn draw(&mut self, shader: &Shader, drawmodels: &Vec<Model>, camera: &Camera, delta: f32) -> bool {
        let mut rot_mat = Matrix4::look_at_dir(Point3::from_vec(self.p1), self.p2-self.p1, self.p2 - (camera.Position + (camera.Up * HALFHEIGHT)).to_vec());
        rot_mat = rot_mat.invert().expect("Tracer rotation matrix not invertible");
        let sca_mat: Matrix4<f32> = Matrix4::from_nonuniform_scale(1.0, 1.0, (self.p2 - self.p1).magnitude());
        let model = rot_mat * sca_mat;
        shader.set_mat4(c_str!("model"), &model);

        self.alpha = (self.alpha - (FADE_RATE * delta)).max(0.0);
        shader.set_float(c_str!("alpha"), self.alpha);

        let drawmodel = &drawmodels[self.model_ind % drawmodels.len()];
        drawmodel.draw(shader);

        self.alpha > 0.0
    }
}

pub struct TracerManager {
    shader: Shader,
    screen_size: Vector2<f32>,
    models: Vec<Model>,
    tracers: Vec<Tracer>,
    prev: Instant,
}

impl TracerManager {
    pub fn new(models: Vec<Model>, screen_size: Vector2<f32>) -> Self {
        TracerManager {
            shader: Shader::new("shaders/forcefield.vs", "shaders/forcefield.fs"),
            screen_size,
            models,
            tracers: vec![],
            prev: Instant::now()
        }
    }

    pub fn add_tracer(&mut self, player_id: usize, position: &PositionComponent, hit_point: Vector3<f32>, is_player: bool) {
        // setup position matrix
        let model_pos = vec3(position.x, position.y, position.z);
        let pos_mat = Matrix4::from_translation(model_pos);

        // setup rotation matrix
        let rot_mat = Matrix4::from(Quaternion::new(
            position.qw, position.qx, position.qy, position.qz,
        ));

        // setup scale matrix
        let scale_mat = Matrix4::from_scale(1.0);

        let model = pos_mat * scale_mat * rot_mat;

        let origin = if is_player {
            Point3::new(0.5, 0.0, 0.0)
        } else {
            Point3::new(0.5, -0.4, 0.0)
        };
        let p1 = model.transform_point(origin).to_vec();

        self.tracers.push(Tracer {
            model_ind: player_id, 
            p1, 
            p2: hit_point, 
            alpha: 1.0 
        })
    }

    pub unsafe fn draw_tracers(&mut self, camera: &Camera) {
        let now = Instant::now();
        let delta = now.duration_since(self.prev).as_secs_f32();
        self.prev = now;

        self.shader.use_program();

        let view = camera.GetViewMatrix();
        self.shader.set_mat4(c_str!("view"), &view);
        let projection: Matrix4<f32> = perspective(
            Deg(camera.Zoom),
            self.screen_size.x / self.screen_size.y,
            0.1,
            10000.0,
        );
        self.shader.set_mat4(c_str!("projection"), &projection);

        self.tracers.retain_mut(|tracer| {
            tracer.draw(&self.shader, &self.models, &camera, delta)
        })
    }
}