use eframe::egui;

pub struct Animator {
    value: f32,
    target: f32,
    speed: f32,
}

impl Animator {
    pub fn new(initial: f32, duration_secs: f32) -> Self {
        Self {
            value: initial,
            target: initial,
            speed: 1.0 / duration_secs.max(0.01),
        }
    }

    #[allow(dead_code)]
    pub fn target(&mut self, target: f32) {
        self.target = target;
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        let dt = ctx.input(|i| i.unstable_dt).min(0.05);
        let diff = self.target - self.value;
        if diff.abs() > 0.001 {
            self.value += diff.signum() * (dt * self.speed * 4.0).min(diff.abs());
        } else {
            self.value = self.target;
        }
    }

    #[allow(dead_code)]
    pub fn value(&self) -> f32 {
        self.value
    }

    #[allow(dead_code)]
    pub fn is_done(&self) -> bool {
        (self.value - self.target).abs() < 0.001
    }
}
