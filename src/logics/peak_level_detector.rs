pub struct PeakLevelDetector {
    b0_attack: f32,
    b0_release: f32,
    b0: f32,
    a1: f32,
    fs: f32,
    peak_output: f32,
    input_abs: f32,
    release_time: f32,
}

impl PeakLevelDetector {
    pub fn new(sample_rate: f32) -> Self {
        let release_time = 0.1;
        let a1 = (-1. / (release_time * sample_rate)).exp();
        let b0_release = 1. - a1;
        PeakLevelDetector {
            b0_attack: 1.,
            b0_release: b0_release,
            b0: 0.,
            a1: a1,
            fs: sample_rate,
            peak_output: 0.,
            input_abs: 0.,
            release_time: release_time,
        }
    }

    pub fn tick(&mut self, input_sample: f32) -> f32 {
        self.input_abs = input_sample.abs();
        if self.input_abs > self.peak_output {
            self.b0 = self.b0_attack;
        } else {
            self.b0 = self.b0_release;
        }
        self.peak_output += self.b0 * (self.input_abs - self.peak_output);
        return self.peak_output;
    }

    pub fn set_detector(&mut self, sample_rate: f32) {
        self.fs = sample_rate;
        self.peak_output = 0.;
        self.b0_attack = 1.;
        self.a1 = (-1. / (self.release_time * self.fs)).exp();
        self.b0_release = 1. - self.a1;
    }
}
