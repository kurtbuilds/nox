
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TsConfig {
    pub exclude: Option<Vec<String>>,
    pub extends: Option<String>,
    pub files: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
    pub references: Option<References>,
    pub type_acquisition: Option<TypeAcquisition>,
    pub compiler_options: Option<CompilerOptions>,
}