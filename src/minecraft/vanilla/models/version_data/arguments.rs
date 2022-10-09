use serde::{Deserialize, Serialize};

use crate::utils::either::Either;

use super::rules::Rule;

/// Arguments that should be used when launching Minecraft
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Arguments {
    /// Game arguments
    pub game: Vec<Argument>,
    /// JVM arguments
    pub jvm: Vec<Argument>,
}

impl Arguments {
    /// Get all JVM arguments and also filters out some arguments that are not needed.
    pub fn jvm_arguments(&self) -> Vec<String> {
        Self::collect_args(&self.jvm)
    }

    /// Gets all game arguments and also filters out some arguments that are not needed.
    pub fn game_arguments(&self) -> Vec<String> {
        Self::collect_args(&self.game)
    }

    /// Collects all arguments and checks wether they are needed by checking defined rules.
    fn collect_args(args: &[Argument]) -> Vec<String> {
        let mut arguments = vec![];

        for argument in args {
            match argument {
                Argument::Simple(simple_argument) => {
                    if check_skip_argument(simple_argument) {
                        continue;
                    }

                    arguments.push(simple_argument.to_string());
                }
                Argument::Complex(complex_arg) => {
                    if !complex_arg.check_use() {
                        continue;
                    }

                    let values = complex_arg
                        .value()
                        .into_iter()
                        .filter(|arg| !check_skip_argument(arg));

                    arguments.extend(values);
                }
            }
        }

        arguments
    }
}

/// An argument that can be just a string, or a complex one that has rules defined.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Argument {
    /// Simple argument
    Simple(String),
    /// Complex argument with rules defined
    Complex(ComplexArgument),
}

/// Complex arguments that can define rules for usage.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ComplexArgument {
    /// The rules of this argument.
    #[serde(alias = "compatibilityRules")]
    pub rules: Vec<Rule>,
    /// Argument itself.
    /// Can be a single string or multiple ones.
    pub value: Either<String, Vec<String>>,
}

impl ComplexArgument {
    /// Checks if the arguments needs to be used on the current executing machine.
    pub fn check_use(&self) -> bool {
        for rule in &self.rules {
            if !rule.allows() {
                return false;
            }
        }

        true
    }

    /// Extracts the rules into a unified form.
    pub fn value(&self) -> Vec<String> {
        match &self.value {
            Either::Left(val) => vec![val.clone()],
            Either::Right(x) => x.clone(),
        }
    }
}

fn check_skip_argument(arg: &str) -> bool {
    matches!(
        arg,
        "--clientId" | "--xuid" | "${clientid}" | "${auth_xuid}"
    )
}
