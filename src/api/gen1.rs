use crate::Version;

//! The generating executable should emit [GenerateConfig] as json on stdout.
//! Then Apivolve CLI will send [GenerateChangesInput] in desired format on its stdin.

#[derive(Debug, Serialize, Deserialize)]
pub enum GenerateInputFormat {
    Json,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateConfig {
    apivolve_version: Version,
    format: GenerateInputFormat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateChangesInput {

}
