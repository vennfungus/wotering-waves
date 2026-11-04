use bevy::prelude::Vec2;

const STONE_START_HEIGHT: f32 = 150.0;
const STONE_FALL_SPEED: f32 = 320.0;
const WAVE_SPEED: f32 = 110.0;
const WAVELENGTH: f32 = 72.0;

pub struct Stone {
    target: Vec2,
    height: f32,
}

impl Stone {
    pub fn new(target: Vec2) -> Self {
        Self {
            target,
            height: STONE_START_HEIGHT,
        }
    }

    pub fn update(&mut self, delta: f32) -> bool {
        self.height -= STONE_FALL_SPEED * delta;
        self.height <= 0.0
    }

    pub fn position(&self) -> Vec2 {
        self.target + Vec2::Y * self.height.max(0.0)
    }

    pub fn target(&self) -> Vec2 {
        self.target
    }
}

pub struct Ripple {
    center: Vec2,
    age: f32,
}

impl Ripple {
    pub fn new(center: Vec2) -> Self {
        Self { center, age: 0.0 }
    }

    pub fn update(&mut self, delta: f32) {
        self.age += delta;
    }

    pub fn radius(&self) -> f32 {
        WAVE_SPEED * self.age
    }

    pub fn center(&self) -> Vec2 {
        self.center
    }

    pub fn amplitude_at(&self, position: Vec2) -> f32 {
        let distance = position.distance(self.center);
        let wave_number = std::f32::consts::TAU / WAVELENGTH;
        let phase = wave_number * (distance - WAVE_SPEED * self.age);
        phase.sin() * (-distance / 380.0).exp() * (-self.age / 8.0).exp()
    }
}
