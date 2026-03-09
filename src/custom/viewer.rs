pub struct ViewerConfig {
    /// type name
    pub field_name: Option<String>,
    /// name for 'hasPreviousPage' field
    pub object_name: Option<String>,
}

impl Default for ViewerConfig {
    fn default() -> Self {
        ViewerConfig {
            field_name: None,
            object_name: None,
        }
    }
}
