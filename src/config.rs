/// Configuration for TXT record serialization
#[derive(Debug, Clone)]
pub struct TxtRecordConfig {
    /// Separator for array indices (default: "_")
    pub array_separator: String,
    /// Separator for object fields (default: ".")
    pub object_separator: String,
    /// Maximum length for each record in format "key=value" (default: 255)
    pub record_len: usize,
    /// Suffix for array length metadata keys (default: "_len")
    pub array_len_suffix: String,
}

impl Default for TxtRecordConfig {
    fn default() -> Self {
        Self {
            array_separator: "_".to_string(),
            object_separator: ".".to_string(),
            record_len: 255,
            array_len_suffix: "_len".to_string(),
        }
    }
}
