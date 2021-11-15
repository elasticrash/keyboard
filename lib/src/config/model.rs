extern crate serde;

use serde::Deserialize;

#[serde(default)]
#[derive(Deserialize, Clone, Debug)]
pub struct Layout {
    pub layout: Vec<Vec<Key>>,
    pub options: ConfigurableOptions,
    pub metadata: Metadata,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Key {
    pub size: f32,
    #[serde(alias = "char")]
    pub display: String,
    pub k_type: i8,
}

impl Default for Layout {
    fn default() -> Layout {
        Layout {
            layout: vec![],
            options: ConfigurableOptions::default(),
            metadata: Metadata::default()
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Options {
    pub layout: Vec<Vec<Key>>,
}

#[serde(default)]
#[derive(Deserialize, Clone, Debug)]
pub struct ConfigurableOptions {
    pub plate_height: i32,
    pub screw_holes: bool,
    pub row: Vec<DirectionOptions>,
    pub column: Vec<DirectionOptions>
}

impl Default for ConfigurableOptions {
    fn default() -> ConfigurableOptions {
        ConfigurableOptions {
            plate_height: 20,
            screw_holes: false,
            row: vec![],
            column: vec![],
        }
    }
}

#[serde(default)]
#[derive(Deserialize, Clone, Debug)]
pub struct Metadata {
    pub spacing:f32
}

impl Default for Metadata {
    fn default() -> Metadata {
        Metadata {
            spacing: 42.,
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct DirectionOptions {
    pub index: i32,
    pub offset: f64,
}