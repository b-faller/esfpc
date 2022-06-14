#[macro_use]
extern crate pest_derive;

mod ast;
mod config;
mod parser;

use std::{fmt::Display, path::PathBuf, vec};

#[cxx::bridge(namespace = "ffi")]
mod ffi {
    #[derive(Debug, Clone)]
    enum FlightRule {
        Vfr,
        Ifr,
        Yankee,
        Zulu,
    }

    #[derive(Debug, Clone)]
    struct FlightPlan {
        rule: FlightRule,
        rfl: i32,
        adep: String,
        adest: String,
        sid: String,
    }

    #[derive(Debug)]
    struct Action {
        typ: ActionType,
        msg: String,
    }

    #[derive(Debug)]
    enum ActionType {
        Error,
        Warning,
        Info,
        Success,
    }

    extern "Rust" {
        fn init_plugin(dll_path: &str) -> Result<()>;
        fn exit_plugin();
        fn check_flightplan(fp: FlightPlan) -> Result<Action>;
    }
}

impl Display for ffi::FlightRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Vfr => write!(f, "V"),
            Self::Ifr => write!(f, "I"),
            Self::Yankee => write!(f, "Y"),
            Self::Zulu => write!(f, "Z"),
            _ => unreachable!(),
        }
    }
}

impl From<config::Action> for ffi::Action {
    fn from(config_action: config::Action) -> Self {
        Self {
            typ: match config_action.typ {
                config::ActionType::Error => ffi::ActionType::Error,
                config::ActionType::Warning => ffi::ActionType::Warning,
                config::ActionType::Info => ffi::ActionType::Info,
                config::ActionType::Success => ffi::ActionType::Success,
            },
            msg: config_action.msg,
        }
    }
}

static mut PLUGIN: Option<Plugin> = None;

struct Plugin {
    configs: Vec<config::Config>,
}

impl Plugin {
    fn new(dll_path: PathBuf) -> Result<Self, std::io::Error> {
        let rules_dir = dll_path
            .parent()
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "DLL path has no parent directory",
                )
            })?
            .join("rules");

        let mut configs = vec![];
        for entry in rules_dir.read_dir()? {
            let path = entry?.path();
            let file = std::fs::File::open(path)?;
            let reader = std::io::BufReader::new(file);
            let config = serde_json::from_reader(reader)?;
            configs.push(config);
        }

        Ok(Self { configs })
    }
}

pub fn init_plugin(dll_path: &str) -> Result<(), std::io::Error> {
    let dll_path = PathBuf::from(dll_path);
    let plugin = Plugin::new(dll_path)?;
    unsafe { PLUGIN = Some(plugin) }
    Ok(())
}

pub fn exit_plugin() {}

pub fn check_flightplan(fp: ffi::FlightPlan) -> Result<ffi::Action, &'static str> {
    unsafe {
        match &PLUGIN {
            Some(plugin) => check_flightplan_impl(plugin, fp).map(ffi::Action::from),
            None => unreachable!(),
        }
    }
}

fn check_flightplan_impl(
    plugin: &Plugin,
    fp: ffi::FlightPlan,
) -> Result<config::Action, &'static str> {
    for config in &plugin.configs {
        for rule in &config.rules {
            return match ast::eval(&rule.condition, &fp) {
                Ok(ast::LitKind::Bool(true)) => Ok(rule.action.clone()),
                Ok(ast::LitKind::Bool(false)) => continue,
                Ok(_) => Err("expression does not evaluate to true or false"),
                Err(e) => Err(e),
            };
        }
    }
    Ok(config::Action {
        typ: config::ActionType::Warning,
        msg: "UNK".into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::ActionType::*;

    #[test]
    fn eddf_aneki() {
        let fp_valid = ffi::FlightPlan {
            rule: ffi::FlightRule::Ifr,
            rfl: 35000,
            adep: "EDDF".into(),
            adest: "EDDS".into(),
            sid: "ANEKI1L".into(),
        };
        let fp_invalid_rfl = ffi::FlightPlan {
            rfl: 34000,
            ..fp_valid.clone()
        };
        let fp_invalid_dst = ffi::FlightPlan {
            adest: "EDDM".into(),
            ..fp_valid.clone()
        };

        let plugin = Plugin::new("esfpc.dll".into()).unwrap();

        // Odd RFL
        assert_eq!(
            Ok(config::Action::new(Success, "OK".into())),
            check_flightplan_impl(&plugin, fp_valid)
        );

        // Even RFL
        assert_eq!(
            Ok(config::Action::new(Error, "RFL".into())),
            check_flightplan_impl(&plugin, fp_invalid_rfl)
        );

        // Wrong DST
        assert_eq!(
            Ok(config::Action::new(Error, "DST".into())),
            check_flightplan_impl(&plugin, fp_invalid_dst)
        );
    }
}
