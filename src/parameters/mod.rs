pub mod helpers;

use vst::plugin::PluginParameters;
use vst::util::ParameterTransfer;

pub enum Params {
    TH,
    RATIO,
    ATTACK,
    RELEASE,
    UNKNOWN,
}

impl Params {
    fn from_i32(index: i32) -> Self {
        match index {
            0 => Self::TH,
            1 => Self::RATIO,
            2 => Self::ATTACK,
            3 => Self::RELEASE,
            _ => Self::UNKNOWN,
        }
    }
}

#[derive(Default)]
pub struct DymensionExpanderParameter {
    pub num_parameters: i32,
    pub transfer: ParameterTransfer,
}

impl DymensionExpanderParameter {
    pub fn get_denormalized_parameter(&self, index: i32) -> f32 {
        let param = Params::from_i32(index);
        let value = self.transfer.get_parameter(index as usize);
        match param {
            Params::TH => {
                let get_value = helpers::denormalize(value, 1.0, 0.0001);
                helpers::linear_to_dB(get_value)
            }
            Params::RATIO => helpers::denormalize(value, 30.0, 1.0),
            Params::ATTACK => helpers::denormalize(value, 1.0, 1e-3),
            Params::RELEASE => helpers::denormalize(value, 3.0, 20e-3),
            _ => (0.0),
        }
    }

    pub fn set_parameter_with_normalize(&self, index: i32, value: f32) {
        let param = Params::from_i32(index);
        match param {
            Params::TH => {
                let lin_value = helpers::dB_to_linear(value);
                let set_value = helpers::normalize(lin_value, 1.0, 0.0001);
                self.transfer.set_parameter(index as usize, set_value);
            }
            Params::RATIO => {
                let ratio_value = helpers::normalize(value, 30.0, 1.0);
                self.transfer.set_parameter(index as usize, ratio_value);
            }
            Params::ATTACK => {
                let attack_value = helpers::normalize(value, 1.0, 1e-3);
                self.transfer.set_parameter(index as usize, attack_value);
            }
            Params::RELEASE => {
                let release_value = helpers::normalize(value, 3.0, 20e-3);
                self.transfer.set_parameter(index as usize, release_value);
            }
            _ => (),
        }
    }
}

impl PluginParameters for DymensionExpanderParameter {
    fn get_parameter_label(&self, index: i32) -> String {
        let param = Params::from_i32(index);
        match param {
            Params::TH => "[dB]".to_string(),
            Params::RATIO => "[-]".to_string(),
            Params::ATTACK => "[ms]".to_string(),
            Params::RELEASE => "[ms]".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        let param = Params::from_i32(index);
        match param {
            Params::TH => "Threshold".to_string(),
            Params::RATIO => "Ratio".to_string(),
            Params::ATTACK => "Attack".to_string(),
            Params::RELEASE => "Release".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        let param = Params::from_i32(index);
        match param {
            Params::TH => format!("{:.1}", self.get_denormalized_parameter(index)),
            Params::RATIO => format!("{:.1}", self.get_denormalized_parameter(index)),
            Params::ATTACK => format!("{:.0}", self.get_denormalized_parameter(index) * 1000.0),
            Params::RELEASE => format!("{:.0}", self.get_denormalized_parameter(index) * 1000.0),
            _ => "".to_string(),
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        self.transfer.get_parameter(index as usize)
    }

    fn set_parameter(&self, index: i32, value: f32) {
        self.transfer.set_parameter(index as usize, value)
    }
}

pub fn parameters_factory() -> DymensionExpanderParameter {
    const NUM_PARAMS: usize = 4;
    let params = ParameterTransfer::new(NUM_PARAMS);
    params.set_parameter(0, 1.0);
    params.set_parameter(1, 0.0);
    params.set_parameter(2, helpers::normalize(30e-3, 1.0, 1e-3));
    params.set_parameter(3, helpers::normalize(120e-3, 3.0, 20e-3));
    DymensionExpanderParameter {
        num_parameters: NUM_PARAMS as i32,
        transfer: params,
    }
}
