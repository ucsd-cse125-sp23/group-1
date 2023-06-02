use std::time::Instant;
use cgmath::{Deg, Euler};
use noise::{NoiseFn, OpenSimplex};

const TRAUMA_REDUCTION_RATE: f32 = 1.0;
const NOISE_SEED: f64 = 50.0;
const MAX_PITCH: f32 = 10.0;
const MAX_YAW: f32 = 10.0;
const MAX_ROLL: f32 = 5.0;

pub struct ScreenShake {
    trauma: f32,
    start: Instant,
    prev: Instant,
    pub euler: Euler<Deg<f32>>,
}

impl Default for ScreenShake {
    fn default() -> Self {
        ScreenShake {
            trauma: 0.0, 
            start: Instant::now(), 
            prev: Instant::now(),
            euler: Euler::new(Deg(0.0),Deg(0.0),Deg(0.0)),
        }
    }
}

impl ScreenShake {
    pub fn add_trauma(&mut self, amount: f32) {
        self.trauma += amount;
        if self.trauma > 1.0 {
            self.trauma = 1.0;
        } else if self.trauma < 0.0 {
            self.trauma = 0.0;
        }
    }

    pub fn shake_camera(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.prev).as_secs_f32();
        self.prev = now;
        self.trauma = (self.trauma - (TRAUMA_REDUCTION_RATE * delta)).max(0.0);
        let intensity = self.trauma.powf(2.0);
        self.euler.x = Deg(MAX_PITCH * intensity * self.get_noise(0, now));
        self.euler.y = Deg(MAX_YAW * intensity * self.get_noise(1, now));
        self.euler.z = Deg(MAX_ROLL * intensity * self.get_noise(2, now));
    }

    fn get_noise(&mut self, seed: u32, now: Instant) -> f32 {
        let noise = OpenSimplex::new(seed);
        let time = now.duration_since(self.start).as_secs_f64();
        noise.get([time * NOISE_SEED, 0.0]) as f32
    }
}