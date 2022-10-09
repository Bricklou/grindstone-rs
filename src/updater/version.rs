#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VersionType {
    Vanilla,
    Forge(String),
    MCP,
}

pub struct MinecraftVersionBuilder {
    version_id: String,
    version_type: VersionType,
}

#[derive(Debug, Clone)]
pub struct MinecraftVersion {
    pub id: String,
    pub version_type: VersionType,
}

impl Into<String> for MinecraftVersion {
    fn into(self) -> String {
        self.id
    }
}

impl Default for MinecraftVersionBuilder {
    fn default() -> Self {
        MinecraftVersionBuilder {
            version_id: "latest".to_string(),
            version_type: VersionType::Vanilla,
        }
    }
}

impl MinecraftVersionBuilder {
    pub fn version_id<S: Into<String>>(mut self, id: S) -> Self {
        self.version_id = id.into();
        self
    }

    pub fn version_type(mut self, v_type: VersionType) -> Self {
        self.version_type = v_type;
        self
    }

    pub fn build(self) -> MinecraftVersion {
        MinecraftVersion {
            id: self.version_id,
            version_type: self.version_type,
        }
    }
}
