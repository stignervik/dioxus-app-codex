use crate::model::UnitStore;
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
    let mut unit_store = use_signal(UnitStore::new);
    let mut selected_unit_index = use_signal(|| None::<usize>);

    let clear_all = move |_| {
        manual_call_points.set("0".to_string());
        detectors.set("0".to_string());
        detection_zones.set("0".to_string());
        alarm_zones.set("0".to_string());
        fire_alarm_devices.set("0".to_string());
        outputs.set("0".to_string());
        unit_store.write().clear();
        selected_unit_index.set(None);
    };

    let configure_units = move |_| {
        let mcp: usize = manual_call_points().parse().unwrap_or(0);
        let det: usize = detectors().parse().unwrap_or(0);
        let dz: usize = detection_zones().parse().unwrap_or(0);
        let az: usize = alarm_zones().parse().unwrap_or(0);
        let fad: usize = fire_alarm_devices().parse().unwrap_or(0);
        let out: usize = outputs().parse().unwrap_or(0);

        let mut store = UnitStore::new();

        for i in 0..mcp {
            store.add_unit(&format!("Manual Call Point {}", i + 1));
        }
        for i in 0..det {
            store.add_unit(&format!("Detector {}", i + 1));
        }
        for i in 0..dz {
            store.add_unit(&format!("Detection Zone {}", i + 1));
        }
        for i in 0..az {
            store.add_unit(&format!("Alarm Zone {}", i + 1));
        }
        for i in 0..fad {
            store.add_unit(&format!("Fire Alarm Device {}", i + 1));
        }
        for i in 0..out {
            store.add_unit(&format!("Output {}", i + 1));
        }

        unit_store.set(store);
        selected_unit_index.set(None);
    };

    rsx! {
        document::Link { rel: "stylesheet", href: CONFIG_CSS }
        div { id: "config",
            h2 { "Configured Units" }
            table { id: "config-table",
                tbody {
                    tr {
                        td {
                            input {
                                r#type: "number",
                                min: "0",
                                max: "1000",
                                value: "{manual_call_points}",
                                oninput: move |evt| manual_call_points.set(evt.value().clone())
                            }
                        }
                        td { "Manual Call Points" }
                    }
                    tr {
                        td {
                            input {
                                r#type: "number",
                                min: "0",
                                max: "1000",
                                value: "{detectors}",
                                oninput: move |evt| detectors.set(evt.value().clone())
                            }
                        }
                        td { "Detectors" }
                    }
                    tr {
                        td {
                            input {
                                r#type: "number",
                                min: "0",
                                max: "1000",
                                value: "{detection_zones}",
                                oninput: move |evt| detection_zones.set(evt.value().clone())
                            }
                        }
                        td { "Detection Zones" }
                    }
                    tr {
                        td {
                            input {
                                r#type: "number",
                                min: "0",
                                max: "1000",
                                value: "{alarm_zones}",
                                oninput: move |evt| alarm_zones.set(evt.value().clone())
                            }
                        }
                        td { "Alarm Zones" }
                    }
                    tr {
                        td {
                            input {
                                r#type: "number",
                                min: "0",
                                max: "1000",
                                value: "{fire_alarm_devices}",
                                oninput: move |evt| fire_alarm_devices.set(evt.value().clone())
                            }
                        }
                        td { "Fire Alarm Devices" }
                    }
                    tr {
                        td {
                            input {
                                r#type: "number",
                                min: "0",
                                max: "1000",
                                value: "{outputs}",
                                oninput: move |evt| outputs.set(evt.value().clone())
                            }
                        }
                        td { "Outputs" }
                    }
                }
            }
            div { id: "config-buttons",
                button { id: "configure-btn", onclick: configure_units, "Configure" }
                button { id: "clear-btn", onclick: clear_all, "Clear" }
            }
            div {
                style: "display: flex; justify-content: center; gap: 20px; margin-top: 20px;",
                // Left side - Unit List
                div {
                    style: "flex: 1; max-width: 600px;",
                    h3 { style: "text-align: center;", "Units ({unit_store().count()})" }
                    ul {
                        style: "max-height: 300px; overflow-y: auto; border: 1px solid white; padding: 15px; list-style: none; margin: 0;",
                        for (index, unit) in unit_store().get_all_units().iter().skip(1).enumerate() {
                            li {
                                style: if selected_unit_index() == Some(index) {
                                    "background-color: rgba(255, 255, 255, 0.2); padding: 8px; cursor: pointer; margin: 3px 0;"
                                } else {
                                    "padding: 8px; cursor: pointer; margin: 3px 0;"
                                },
                                onclick: move |_| selected_unit_index.set(Some(index)),
                                "{unit.name()}"
                            }
                        }
                    }
                }
                // Right side - Selected Unit
                div {
                    style: "flex: 1; max-width: 600px;",
                    h3 { style: "text-align: center;", "Selected Unit" }
                    div {
                        style: "min-height: 300px; border: 1px solid white; padding: 15px; background-color: rgba(255, 255, 255, 0.05);",
                        if let Some(selected_idx) = selected_unit_index() {
                            if let Some(selected_unit) = unit_store().get_all_units().iter().skip(1).nth(selected_idx) {
                                div {
                                    h4 { "Unit Details:" }
                                    p { "Name: {selected_unit.name()}" }
                                    // Add more unit details here as needed
                                }
                            }
                        } else {
                            div {
                                style: "color: rgba(255, 255, 255, 0.5); text-align: center; padding-top: 50px;",
                                "No unit selected"
                            }
                        }
                    }
                }
            }
        }
    }
}
