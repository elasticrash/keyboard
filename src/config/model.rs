use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Layout {
    pub layout: Vec<Vec<Key>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Key {
    pub size: f32,
    #[serde(alias = "char")]
    pub display: String,
    pub k_type: i8,
}
