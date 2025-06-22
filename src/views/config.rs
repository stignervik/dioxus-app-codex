use dioxus::prelude::*;

const CONFIG_CSS: Asset = asset!("/assets/styling/config.css");

/// The Config page component rendered for the `/` route
#[component]
pub fn Config() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CONFIG_CSS }
        div { id: "config",
            h2 { "Configured Units" }
            table { id: "config-table",
                tbody {
                    tr {
                        td { input { r#type: "number", min: "0", max: "1000", value: "0" } }
                        td { "Manual Call Points" }
                    }
                    tr {
                        td { input { r#type: "number", min: "0", max: "1000", value: "0" } }
                        td { "Detectors" }
                    }
                    tr {
                        td { input { r#type: "number", min: "0", max: "1000", value: "0" } }
                        td { "Detection Zones" }
                    }
                    tr {
                        td { input { r#type: "number", min: "0", max: "1000", value: "0" } }
                        td { "Alarm Zones" }
                    }
                    tr {
                        td { input { r#type: "number", min: "0", max: "1000", value: "0" } }
                        td { "Fire Alarm Devices" }
                    }
                    tr {
                        td { input { r#type: "number", min: "0", max: "1000", value: "0" } }
                        td { "Outputs" }
                    }
                }
            }
            div { id: "config-buttons",
                button { id: "configure-btn", "Configure" }
                button { id: "clear-btn", "Clear" }
            }
        }
    }
}
