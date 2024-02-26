use serde::Deserialize;
#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    #[serde(alias = "action")]
    pub action: Option<String>,
    #[serde(alias = "language")]
    pub language: Option<String>,
    #[serde(alias = "score_threshold")]
    pub score_threshold: Option<f64>,
    #[serde(alias = "server")]
    pub server: Option<String>,
    #[serde(alias = "server_url")]
    pub server_url: String,
}
