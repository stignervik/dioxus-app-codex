use dioxus::prelude::*;

const CONFIG_CSS: Asset = asset!("/assets/styling/config.css");

/// The Config page component rendered for the `/` route
#[component]
pub fn Config() -> Element {
    // Signals for each input so we can update them programmatically
    let mut manual_call_points = use_signal(|| "0".to_string());
    let mut detectors = use_signal(|| "0".to_string());
    let mut detection_zones = use_signal(|| "0".to_string());
    let mut alarm_zones = use_signal(|| "0".to_string());
    let mut fire_alarm_devices = use_signal(|| "0".to_string());
    let mut outputs = use_signal(|| "0".to_string());

    let clear_all = move |_| {
        manual_call_points.set("0".to_string());
        detectors.set("0".to_string());
        detection_zones.set("0".to_string());
        alarm_zones.set("0".to_string());
        fire_alarm_devices.set("0".to_string());
        outputs.set("0".to_string());
    };

    rsx! {
        document::Link { rel: "stylesheet", href: CONFIG_CSS }
        div { id: "config",
            h2 { "Configured Units" }
            table { id: "config-table",
                tbody {
                    tr {
                        td { input {
                            r#type: "number",
                            min: "0",
                            max: "1000",
                            value: "{manual_call_points}",
                            oninput: move |evt| manual_call_points.set(evt.value().clone()),
                        } }
                        td { "Manual Call Points" }
                    }
                    tr {
                        td { input {
                            r#type: "number",
                            min: "0",
                            max: "1000",
                            value: "{detectors}",
                            oninput: move |evt| detectors.set(evt.value().clone()),
                        } }
                        td { "Detectors" }
                    }
                    tr {
                        td { input {
                            r#type: "number",
                            min: "0",
                            max: "1000",
                            value: "{detection_zones}",
                            oninput: move |evt| detection_zones.set(evt.value().clone()),
                        } }
                        td { "Detection Zones" }
                    }
                    tr {
                        td { input {
                            r#type: "number",
                            min: "0",
                            max: "1000",
                            value: "{alarm_zones}",
                            oninput: move |evt| alarm_zones.set(evt.value().clone()),
                        } }
                        td { "Alarm Zones" }
                    }
                    tr {
                        td { input {
                            r#type: "number",
                            min: "0",
                            max: "1000",
                            value: "{fire_alarm_devices}",
                            oninput: move |evt| fire_alarm_devices.set(evt.value().clone()),
                        } }
                        td { "Fire Alarm Devices" }
                    }
                    tr {
                        td { input {
                            r#type: "number",
                            min: "0",
                            max: "1000",
                            value: "{outputs}",
                            oninput: move |evt| outputs.set(evt.value().clone()),
                        } }
                        td { "Outputs" }
                    }
                }
            }
            div { id: "config-buttons",
                button { id: "configure-btn", "Configure" }
                button { id: "clear-btn", onclick: clear_all, "Clear" }
            }
        }
    }
}
