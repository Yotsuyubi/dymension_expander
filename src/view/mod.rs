use crate::parameters;
use crate::vst::host::Host;
use std::sync::Arc;
use vst::plugin::HostCallback;
use vst::plugin::PluginParameters;

use parameters::DymensionExpanderParameter;

mod loader;

#[derive(Default)]
pub struct DymensionExpanderGUI {
    params: Arc<DymensionExpanderParameter>,
    #[allow(dead_code)]
    host: HostCallback,
}

impl DymensionExpanderGUI {
    pub fn new(params: Arc<DymensionExpanderParameter>, host: HostCallback) -> Self {
        Self {
            params: params,
            host: host,
        }
    }

    pub fn javascript_callback(&self) -> vst_gui::JavascriptCallback {
        let params = Arc::clone(&self.params);
        let host = self.host;
        Box::new(move |message: String| {
            let mut tokens = message.split_whitespace();

            let command = tokens.next().unwrap_or("");
            let argument = tokens.next().unwrap_or("").parse::<f32>();

            match command {
                "getGain" => {
                    return params.get_parameter_text(0);
                }
                "setGain" => {
                    params.set_parameter(0, argument.unwrap());
                    host.automate(0, params.get_parameter(0));
                }
                "mouseOverGain" => {
                    host.begin_edit(0);
                }
                "releaseGain" => {
                    host.end_edit(0);
                }
                _ => {}
            }

            String::new()
        })
    }

    pub fn get_html() -> String {
        loader::get_html()
    }
}
