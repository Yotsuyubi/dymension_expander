pub struct GainDynamics {
    attack_time: f32,
    release_time: f32,
    b0_attack: f32,
    b0_release: f32,
    b0: f32,
    fs: f32,
    output_gain: f32,
}

impl GainDynamics {
    pub fn new(sample_rate: f32, new_attack_time: f32, new_release_time: f32) -> Self {
        GainDynamics {
            attack_time: new_attack_time,
            release_time: new_release_time,
            b0_attack: new_attack_time,
            b0_release: new_release_time,
            b0: 0.,
            fs: sample_rate,
            output_gain: 1.,
        }
    }

    pub fn set_detector(&mut self, sample_rate: f32) {
        self.fs = sample_rate;
        self.output_gain = 0.;
        self.set_attack(self.attack_time);
        self.set_release(self.release_time);
    }

    pub fn tick(&mut self, input_gain: f32) -> f32 {
        if input_gain < self.output_gain {
            self.b0 = self.b0_attack;
        } else {
            self.b0 = self.b0_release;
        }

        self.output_gain += self.b0 * (input_gain - self.output_gain);

        return self.output_gain;
    }

    pub fn set_attack(&mut self, new_attack_time: f32) {
        self.attack_time = new_attack_time;
        self.b0_attack = 1. - (-1. / (self.attack_time * self.fs)).exp();
    }

    pub fn set_release(&mut self, new_release_time: f32) {
        self.release_time = new_release_time;
        self.b0_release = 1. - (-1. / (self.release_time * self.fs)).exp();
    }
}
