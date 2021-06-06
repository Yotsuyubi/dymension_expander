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
                "get-threshold" => {
                    return params.get_parameter_text(0);
                }
                "set-threshold" => {
                    if argument.is_ok() {
                        params.set_parameter_with_normalize(0, argument.unwrap());
                        host.automate(0, params.get_parameter(0));
                    }
                }
                "edit-begin-threshold" => {
                    host.begin_edit(0);
                }
                "edit-end-threshold" => {
                    host.end_edit(0);
                }

                "get-ratio" => {
                    return params.get_parameter_text(1);
                }
                "set-ratio" => {
                    if argument.is_ok() {
                        params.set_parameter_with_normalize(1, argument.unwrap());
                        host.automate(1, params.get_parameter(1));
                    }
                }
                "edit-begin-ratio" => {
                    host.begin_edit(1);
                }
                "edit-end-ratio" => {
                    host.end_edit(1);
                }

                "get-attack" => {
                    return params.get_parameter_text(2);
                }
                "set-attack" => {
                    if argument.is_ok() {
                        params.set_parameter_with_normalize(2, argument.unwrap());
                        host.automate(2, params.get_parameter(2));
                    }
                }
                "edit-begin-attack" => {
                    host.begin_edit(2);
                }
                "edit-end-attack" => {
                    host.end_edit(2);
                }

                "get-release" => {
                    return params.get_parameter_text(3);
                }
                "set-release" => {
                    if argument.is_ok() {
                        params.set_parameter_with_normalize(3, argument.unwrap());
                        host.automate(3, params.get_parameter(3));
                    }
                }
                "edit-begin-release" => {
                    host.begin_edit(3);
                }
                "edit-end-release" => {
                    host.end_edit(3);
                }

                _ => {}
            }

            String::new()
        })
    }

    pub fn get_html(&self) -> String {
        loader::get_html()
    }
}
