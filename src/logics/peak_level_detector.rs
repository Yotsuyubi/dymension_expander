// from Digital Dynamic Range Compressor Design A Tutorial and Analysis
// https://www.eecs.qmul.ac.uk/~josh/documents/2012/GiannoulisMassbergReiss-dynamicrangecompression-JAES2012.pdf

pub struct PeakLevelDetector {
    fs: f32,
    peak_output: f32,
    input_abs: f32,
    attack_time: f32,
    release_time: f32,
}

impl PeakLevelDetector {
    pub fn new(sample_rate: f32, new_attack_time: f32, new_release_time: f32) -> Self {
        PeakLevelDetector {
            fs: sample_rate,
            peak_output: 0.0,
            input_abs: 0.0,
            attack_time: new_attack_time,
            release_time: new_release_time,
        }
    }

    pub fn tick(&mut self, input_sample: f32) -> f32 {
        self.input_abs = input_sample.abs();
        let alpha = if self.input_abs > self.peak_output {
            self.calc_alpha(self.attack_time)
        } else {
            self.calc_alpha(self.release_time)
        };
        self.peak_output = self.peak_output * alpha + (1.0 - alpha) * self.input_abs;
        return self.peak_output;
    }

    pub fn set_attack(&mut self, new_attack_time: f32) {
        self.attack_time = new_attack_time;
    }

    pub fn set_release(&mut self, new_release_time: f32) {
        self.release_time = new_release_time;
    }

    fn calc_alpha(&mut self, time_constant: f32) -> f32 {
        (-1.0 / (self.fs * time_constant)).exp()
    }
}
