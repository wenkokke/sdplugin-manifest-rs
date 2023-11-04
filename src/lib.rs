use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Manifest {
    pub actions: Vec<Action>,
    pub author: String,
    pub code_path: Option<String>,
    pub description: String,
    pub icon: String,
    pub name: String,
    pub version: String,
    #[serde(rename = "SDKVersion")]
    pub sdk_version: u32,
    #[serde(rename = "OS")]
    pub os: Vec<OS>,
    pub category: Option<String>,
    pub category_icon: Option<String>,
    pub code_path_mac: Option<String>,
    pub code_path_win: Option<String>,
    pub profiles: Option<Vec<Profile>>,
    pub property_inspector_path: Option<String>,
    pub default_window_size: Option<Vec<u32>>,
    #[serde(rename = "URL")]
    pub url: Option<String>,
    pub applications_to_monitor: Option<ApplicationsToMonitor>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Action {
    #[serde(rename = "UUID")]
    pub uuid: String,
    pub icon: Option<String>,
    pub name: String,
    pub states: Vec<State>,
    pub property_inspector_path: Option<String>,
    pub supported_in_multi_actions: Option<bool>,
    pub tooltip: Option<String>,
    pub disable_caching: Option<bool>,
    pub disable_automatic_states: Option<bool>,
    pub visible_in_actions_list: Option<bool>,
    pub user_title_enabled: Option<bool>,
    pub controllers: Option<Vec<Controller>>,
    pub encoder: Option<Encoder>,
}

pub type Controller = String;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct State {
    pub image: String,
    pub multi_action_image: Option<String>,
    pub name: Option<String>,
    pub title: Option<String>,
    pub show_title: Option<bool>,
    pub title_color: Option<String>,
    pub title_alignment: Option<String>,
    pub font_family: Option<String>,
    pub font_style: Option<String>,
    pub font_size: Option<String>,
    pub font_underline: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Encoder {
    #[serde(rename = "background")]
    pub background: Option<String>,
    pub icon: Option<String>,
    pub layout: Option<String>,
    pub stack_color: Option<String>,
    pub trigger_description: Option<TriggerDescription>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TriggerDescription {
    pub rotate: Option<String>,
    pub push: Option<String>,
    pub touch: Option<String>,
    pub long_touch: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OS {
    pub platform: String,
    pub minimum_version: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Profile {
    pub name: String,
    pub device_type: DeviceType,
    pub readonly: Option<bool>,
    pub dont_auto_switch_when_installed: Option<bool>,
}

pub type DeviceType = u32;

#[derive(Serialize, Deserialize)]
pub struct ApplicationsToMonitor {
    pub mac: Vec<BundleId>,
    pub windows: Vec<Exe>,
}

pub type BundleId = String;
pub type Exe = String;
