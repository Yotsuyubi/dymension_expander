#[macro_use]
extern crate vst;
extern crate time;
extern crate vst_gui;

mod logics;
mod parameters;
mod view;

use logics::delay::delay;
use logics::gain_dynamics::GainDynamics;
use logics::peak_level_detector::PeakLevelDetector;
use parameters::helpers::{dB_to_linear, linear_to_dB};
use parameters::DymensionExpanderParameter;
use view::DymensionExpanderGUI;

use vst::buffer::AudioBuffer;
use vst::editor::Editor;
use vst::plugin::HostCallback;
use vst::plugin::{Category, Info, Plugin, PluginParameters};

use std::collections::VecDeque;
use std::f64::consts::PI;
use std::sync::Arc;

type SamplePair = (f32, f32);

struct DymensionExpander {
    params: Arc<DymensionExpanderParameter>,
    gui: DymensionExpanderGUI,
    sample_rate: f32,
    buffers: Vec<VecDeque<SamplePair>>,
    size: f32,
    old_size: f32,
    gain_dynamics: GainDynamics,
    peak_level_detector: PeakLevelDetector,
}

impl DymensionExpander {
    /// Update the delay buffers with a new size value.
    fn resize(&mut self, n: f32) {
        let old_size = self.old_size;

        for (i, buffer) in self.buffers.iter_mut().enumerate() {
            // Calculate the size difference between delays.
            let old_delay = delay(i, old_size);
            let new_delay = delay(i, n);

            let diff = new_delay - old_delay;

            // Add empty samples if the delay was increased, remove if decreased.
            if diff > 0 {
                for _ in 0..diff {
                    buffer.push_back((0.0, 0.0));
                }
            } else if diff < 0 {
                for _ in 0..-diff {
                    let _ = buffer.pop_front();
                }
            }
        }

        self.old_size = n;
    }
}

impl Plugin for DymensionExpander {
    fn new(host: HostCallback) -> Self {
        let params = Arc::new(parameters::parameters_factory());
        let gui = DymensionExpanderGUI::new(Arc::clone(&params), host);
        let rample_rate = 44100.0;

        const NUM_DELAYS: usize = 4;
        let mut buffers = Vec::new();

        // Generate delay buffers.
        for i in 0..NUM_DELAYS {
            let samples = delay(i, 0.12);
            let mut buffer = VecDeque::with_capacity(samples as usize);

            // Fill in the delay buffers with empty samples.
            for _ in 0..samples {
                buffer.push_back((0.0, 0.0));
            }

            buffers.push(buffer);
        }
        Self {
            params: Arc::clone(&params),
            gui: gui,
            sample_rate: rample_rate,
            buffers: buffers,
            size: 0.12,
            old_size: 0.12,
            gain_dynamics: GainDynamics::new(
                44100.0,
                Arc::clone(&params).get_parameter(2),
                Arc::clone(&params).get_parameter(3),
            ),
            peak_level_detector: PeakLevelDetector::new(rample_rate),
        }
    }

    fn get_info(&self) -> Info {
        Info {
            name: "Dymension Expander".to_string(),
            vendor: "Psykhedelic Mandala".to_string(),
            unique_id: 1338,
            category: Category::Effect,
            inputs: 2,
            outputs: 2,
            parameters: self.params.num_parameters,
            ..Info::default()
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn PluginParameters>
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let threshold = self.params.get_denormalized_parameter(0);
        let ratio = self.params.get_denormalized_parameter(1);
        let attack = self.params.get_denormalized_parameter(2);
        let release = self.params.get_denormalized_parameter(3);
        let size = self.size;

        self.gain_dynamics.set_attack(attack);
        self.gain_dynamics.set_release(release);

        let (inputs, outputs) = buffer.split();

        // Assume 2 channels
        if inputs.len() < 2 || outputs.len() < 2 {
            return;
        }

        // Resize if size changed
        if size != self.old_size {
            self.resize(size);
        }

        // Iterate over inputs as (&f32, &f32)
        let (l, r) = inputs.split_at(1);
        let stereo_in = l[0].iter().zip(r[0].iter());

        // Iterate over outputs as (&mut f32, &mut f32)
        let (mut l, mut r) = outputs.split_at_mut(1);
        let stereo_out = l[0].iter_mut().zip(r[0].iter_mut());

        for ((left_in, right_in), (left_out, right_out)) in stereo_in.zip(stereo_out) {
            let input_mean = (*left_in + *right_in) / 2.;
            #[allow(non_snake_case)]
            let peak_dB = linear_to_dB(self.peak_level_detector.tick(input_mean));

            #[allow(non_snake_case)]
            let gain_dB: f32 = if peak_dB < threshold {
                0.
            } else {
                -(peak_dB - threshold) * (1. - 1. / ratio)
            };

            let gain_dynamics = dB_to_linear(self.gain_dynamics.tick(gain_dB));

            // Push the new samples into the delay buffers.
            for buffer in &mut self.buffers {
                buffer.push_back((*left_in, *right_in));
            }

            let mut left_processed = 0.0;
            let mut right_processed = 0.0;

            // Recalculate time per sample
            let time_s = time::precise_time_ns() as f64 / 1_000_000_000.0;

            // Use buffer index to offset volume LFO
            for (n, buffer) in self.buffers.iter_mut().enumerate() {
                if let Some((left_old, right_old)) = buffer.pop_front() {
                    const LFO_FREQ: f64 = 0.5;
                    const WET_MULT: f32 = 0.66;

                    let offset = 0.25 * (n % 4) as f64;

                    // Sine wave volume LFO
                    let lfo = ((time_s * LFO_FREQ + offset) * PI * 2.0).sin() as f32;

                    let wet = (1. - gain_dynamics) * WET_MULT;
                    let mono = (left_old + right_old) / 2.0;

                    // Flip right channel and keep left mono so that the result is
                    // entirely stereo
                    left_processed += mono * wet * lfo;
                    right_processed += -mono * wet * lfo;
                }
            }

            // By only adding to the input, the output value always remains the same in mono
            *left_out = *left_in + left_processed;
            *right_out = *right_in + right_processed;
        }
    }

    // fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
    //     let gui = vst_gui::new_plugin_gui(
    //         String::from(self.gui.get_html()),
    //         self.gui.javascript_callback(),
    //         Some((500, 500)),
    //     );
    //     Some(Box::new(gui))
    // }
}

plugin_main!(DymensionExpander);
