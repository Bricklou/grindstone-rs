use crate::utils::os::{Architecture, Platform};
use serde::{Deserialize, Serialize};

/// A rule that can enable/disable functionality on a specific platform/architecture.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Rule {
    /// If the rule allows or disallows functionality.
    pub action: Action,
    /// The OS configuration the rule applies on.
    pub os: Option<Os>,
    /// Features that need to be enabled for this rule to apply.
    pub features: Option<Features>,
}

impl Rule {
    /// Checks wether the rule allows functionality or not.
    pub fn allows(&self) -> bool {
        if let Some(os) = &self.os {
            if let Some(platform) = &os.platform {
                if platform != &Platform::current() {
                    return !self.action.to_bool();
                }
            }
        }

        if let Some(features) = &self.features {
            if features.is_demo_user.is_some() || features.has_custom_resolution.is_some() {
                return false;
            }
        }

        self.action.to_bool()
    }
}

/// Action of a rule.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Action {
    /// Allow functionality
    #[serde(alias = "allow")]
    Allow,
    /// Disallow functionality
    #[serde(alias = "disallow")]
    Disallow,
}

impl Action {
    /// Convert the action to a `bool`.
    pub fn to_bool(&self) -> bool {
        match self {
            Action::Allow => true,
            Action::Disallow => false,
        }
    }
}

/// OS configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Os {
    /// The platform.
    #[serde(alias = "name")]
    pub platform: Option<Platform>,
    /// Version of the platform.
    pub version: Option<String>,
    /// Architecture of the platform.
    pub arch: Option<Architecture>,
}

/// Special features.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Features {
    /// Demo User.
    pub is_demo_user: Option<bool>,
    /// Custom resolution feature.
    pub has_custom_resolution: Option<bool>,
}
