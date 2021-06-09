pub mod helpers;

use vst::plugin::PluginParameters;
use vst::util::ParameterTransfer;

pub enum Params {
    TH,
    RATIO,
    ATTACK,
    RELEASE,
    GAINDYNAMICS,
    UNKNOWN,
}

impl Params {
    pub fn from_i32(index: i32) -> Self {
        match index {
            0 => Self::TH,
            1 => Self::RATIO,
            2 => Self::ATTACK,
            3 => Self::RELEASE,
            4 => Self::GAINDYNAMICS,
            _ => Self::UNKNOWN,
        }
    }

    pub fn to_i32(param: Self) -> usize {
        match param {
            Self::TH => 0,
            Self::RATIO => 1,
            Self::ATTACK => 2,
            Self::RELEASE => 3,
            Self::GAINDYNAMICS => 4,
            Self::UNKNOWN => 999,
        }
    }
}

#[derive(Default)]
pub struct DymensionExpanderParameter {
    pub num_parameters: i32,
    pub transfer: ParameterTransfer,
}

impl DymensionExpanderParameter {
    pub fn new() -> Self {
        const NUM_PARAMS: usize = 5;
        let params = ParameterTransfer::new(NUM_PARAMS);
        params.set_parameter(Params::to_i32(Params::TH), 1.0);
        params.set_parameter(Params::to_i32(Params::RATIO), 0.0);
        params.set_parameter(
            Params::to_i32(Params::ATTACK),
            helpers::normalize(30.0, 1.0e3, 1.0),
        );
        params.set_parameter(
            Params::to_i32(Params::RELEASE),
            helpers::normalize(120.0, 3.0e3, 20.0),
        );
        params.set_parameter(
            Params::to_i32(Params::GAINDYNAMICS),
            helpers::normalize(1.0, 1.0, 0.0001),
        );
        Self {
            num_parameters: NUM_PARAMS as i32,
            transfer: params,
        }
    }

    pub fn get_denormalized_parameter(&self, index: i32) -> f32 {
        let param = Params::from_i32(index);
        let value = self.transfer.get_parameter(index as usize);
        match param {
            Params::TH => {
                let get_value = helpers::denormalize(value, 1.0, 0.0001);
                helpers::linear_to_dB(get_value)
            }
            Params::RATIO => helpers::denormalize(value, 30.0, 1.0),
            Params::ATTACK => helpers::denormalize(value, 1.0e3, 1.0),
            Params::RELEASE => helpers::denormalize(value, 3.0e3, 20.0),
            Params::GAINDYNAMICS => {
                let get_value = helpers::denormalize(value, 1.0, 0.0001);
                helpers::linear_to_dB(get_value)
            }
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
                let attack_value = helpers::normalize(value, 1.0e3, 1.0);
                self.transfer.set_parameter(index as usize, attack_value);
            }
            Params::RELEASE => {
                let release_value = helpers::normalize(value, 3.0e3, 20.0);
                self.transfer.set_parameter(index as usize, release_value);
            }
            Params::GAINDYNAMICS => {
                let lin_value = helpers::dB_to_linear(value);
                let set_value = helpers::normalize(lin_value, 1.0, 0.0001);
                self.transfer.set_parameter(index as usize, set_value);
            }
            _ => (),
        }
    }

    pub fn get_indicator(&self, index: i32) -> String {
        let param = Params::from_i32(index);
        match param {
            Params::GAINDYNAMICS => format!("{:.1}", self.get_denormalized_parameter(index)),
            _ => "".to_string(),
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
            Params::ATTACK => format!("{:.0}", self.get_denormalized_parameter(index)),
            Params::RELEASE => format!("{:.0}", self.get_denormalized_parameter(index)),
            _ => "".to_string(),
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        self.transfer.get_parameter(index as usize)
    }

    fn set_parameter(&self, index: i32, value: f32) {
        self.transfer.set_parameter(index as usize, value)
    }

    fn can_be_automated(&self, index: i32) -> bool {
        let param = Params::from_i32(index);
        match param {
            Params::GAINDYNAMICS => false,
            _ => true,
        }
    }
}
