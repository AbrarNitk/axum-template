#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub service: ServiceSettings,
}

#[derive(serde::Deserialize, Debug)]
pub struct ServiceSettings {
    pub bind: String,
    pub port: u16,
    pub profile: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct TelemetrySettings {
    #[serde(rename = "otel-exporter-otlp-protocol")]
    pub otel_exporter_otlp_protocol: String,
    #[serde(rename = "otel-exporter-otlp-endpoint")]
    pub otel_exporter_otlp_endpoint: String,
    #[serde(rename = "otel-service-name")]
    pub otel_service_name: String,
    #[serde(rename = "log-level")]
    pub log_level: String,
}

impl Settings {
    pub fn with_file(profile_name: &str) -> Result<Self, config::ConfigError> {
        let settings_file_path = format!("settings/{}.toml", profile_name);
        let settings = config::Config::builder()
            .add_source(config::File::with_name("settings/default.toml"))
            .add_source(config::File::with_name(&settings_file_path))
            .build()?;
        settings.try_deserialize()
    }
}
