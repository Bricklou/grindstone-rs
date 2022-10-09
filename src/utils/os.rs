use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Platform {
    #[serde(alias = "linux")]
    Linux,
    #[serde(alias = "osx")]
    MacOs,
    #[serde(alias = "windows")]
    Windows,
    Other,
}

impl Platform {
    pub fn current() -> Self {
        match std::env::consts::OS {
            "linux" => Self::Linux,
            "macos" => Self::MacOs,
            "windows" => Self::Windows,
            _ => Self::Other,
        }
    }

    pub fn matches_current(&self) -> bool {
        self == &Self::current()
    }

    pub fn classpath_seperator(&self) -> char {
        match self {
            Platform::Windows => ';',
            _ => ':',
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Architecture {
    #[serde(alias = "x86")]
    I386,
    #[serde(alias = "x86_64")]
    AMD64,
}

impl Architecture {
    pub fn current() -> Self {
        if cfg!(target_pointer_width = "32") {
            Self::I386
        } else {
            Self::AMD64
        }
    }

    pub fn get_bits(&self) -> u8 {
        match self {
            Architecture::I386 => 32,
            Architecture::AMD64 => 64,
        }
    }
}
