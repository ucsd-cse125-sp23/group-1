use std::time::Instant;

use cgmath::vec4;

use crate::sprite_renderer::Sprite;

pub struct Fadable {
    pub sprite: Sprite,
    fade_rate: f32,
    max_alpha: f32,
    pub alpha: f32,
    prev: Instant,
}

impl Fadable {
    pub fn new(sprite: Sprite, fade_rate: f32, max_alpha: f32) -> Self {
        Fadable {
            sprite,
            fade_rate,
            max_alpha,
            alpha: 0.0,
            prev: Instant::now()
        }
    }

    pub fn add_alpha(&mut self, amount: f32) {
        self.alpha += amount;
        if self.alpha > self.max_alpha {
            self.alpha = self.max_alpha;
        } else if self.alpha < 0.0 {
            self.alpha = 0.0;
        }
    }

    pub fn draw(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.prev).as_secs_f32();
        self.prev = now;
        self.alpha = (self.alpha - (self.fade_rate * delta)).max(0.0);
        let true_alpha = self.alpha.min(1.0).powf(2.0);
        let color = vec4(1.0, 1.0, 1.0, true_alpha);
        self.sprite.set_color(color);
        unsafe { self.sprite.draw() };
    }
}