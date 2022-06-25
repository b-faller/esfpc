mod config;
mod lang;

use std::fmt::Display;
use std::path::PathBuf;

use lang::ast;

#[cxx::bridge(namespace = "ffi")]
mod ffi {
    #[derive(Debug, Clone)]
    enum AircraftType {
        /// ? - unknown
        Unknown,
        /// L - landplane
        Landplane,
        /// S - seaplane
        Seaplane,
        /// A - amphibian
        Amphibian,
        /// H - helicopter
        Helicopter,
        /// G - gyrocopter
        Gyrocopter,
        /// T - tilt-wing AC
        TiltWing,
    }

    #[derive(Debug, Clone)]
    enum WakeTurbulenceCategory {
        ///  ? - unknown
        Unknown,
        ///  L - light
        Light,
        ///  M - medium
        Medium,
        ///  H - heavy
        Heavy,
        ///  J - super heavy
        Super,
    }

    #[derive(Debug, Clone)]
    enum FaaEquipmentCode {
        /// ? - unknown
        Unknown,
        /// /T no DME, Transponder without mode A+C
        T,
        /// /X no DME, No Transponder
        X,
        /// /U no DME, Transponder with mode A+C
        U,
        /// /D DME, No Transponder
        D,
        /// /B DME, Transponder without mode A+C
        B,
        /// /A DME, Transponder with mode A+C
        A,
        /// /M TACAN only, No Transponder
        M,
        /// /N TACAN only, Transponder without mode A+C
        N,
        /// /P TACAN only, Transponder with mode A+C
        P,
        /// /Y simple RNAV, No Transponder
        Y,
        /// /C simple RNAV, Transponder without mode A+C
        C,
        /// /I simple RNAV, Transponder with mode A+C
        I,
        /// /E advanced RNAV with Dual FMS
        E,
        /// /F advanced RNAV with Single FMS
        F,
        /// /G advanced RNAV with GPS or GNSS
        G,
        /// /R advanced RNAV with RNP capability
        R,
        /// /W advanced RNAV with RVSM capability
        W,
        /// /Q advanced RNAV with RNP and RVSM
        Q,
    }

    #[derive(Debug, Clone)]
    enum EngineType {
        /// ? - unknown
        Unknown,
        /// P - piston
        Piston,
        /// T - turboprop/turboshaft
        Turboprop,
        /// J - jet
        Jet,
        /// E - electric
        Electric,
    }

    #[derive(Debug, Clone)]
    struct Aircraft {
        typ: AircraftType,
        wtc: WakeTurbulenceCategory,
        faa_equip_code: FaaEquipmentCode,
        eng_typ: EngineType,
        eng_count: u8,
        is_rvsm_capable: bool,
    }

    #[derive(Debug, Clone)]
    enum FlightRule {
        Vfr,
        Ifr,
        Yankee,
        Zulu,
    }

    #[derive(Debug, Clone)]
    struct FlightPlan {
        ac: Aircraft,
        rule: FlightRule,
        cfl: u32,
        rfl: u32,
        dep: String,
        dep_rwy: String,
        arr: String,
        sid: String,
        route: String,
    }

    #[derive(Debug)]
    struct Action {
        typ: ActionType,
        msg: String,
    }

    #[derive(Debug)]
    enum ActionType {
        Success,
        Info,
        Warning,
        Error,
    }

    extern "Rust" {
        fn check_flightplan(fp: FlightPlan) -> Result<Action>;
        // fn on_function_call(plugin: Pin<&mut EsPlugin>);
        // fn on_get_tag_item(plugin: Pin<&mut EsPlugin>);
    }

    #[namespace = ""]
    unsafe extern "C++" {
        include!("esfpc/cxx/main.hpp");

        #[namespace = "EuroScopePlugIn"]
        type CPlugIn;

        type EsPlugin;

        fn create_plugin() -> UniquePtr<EsPlugin>;
        fn display_user_message(self: Pin<&mut EsPlugin>, message: &str);
        fn get_dll_path() -> Result<String>;
    }
}

impl Display for ffi::AircraftType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Unknown => write!(f, "?"),
            Self::Landplane => write!(f, "L"),
            Self::Seaplane => write!(f, "S"),
            Self::Amphibian => write!(f, "A"),
            Self::Helicopter => write!(f, "H"),
            Self::Gyrocopter => write!(f, "G"),
            Self::TiltWing => write!(f, "T"),
            _ => unreachable!(),
        }
    }
}

impl Display for ffi::WakeTurbulenceCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Unknown => write!(f, "?"),
            Self::Light => write!(f, "L"),
            Self::Medium => write!(f, "M"),
            Self::Heavy => write!(f, "H"),
            Self::Super => write!(f, "S"),
            _ => unreachable!(),
        }
    }
}

impl Display for ffi::FaaEquipmentCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Unknown => write!(f, "?"),
            Self::T => write!(f, "T"),
            Self::X => write!(f, "X"),
            Self::U => write!(f, "U"),
            Self::D => write!(f, "D"),
            Self::B => write!(f, "B"),
            Self::A => write!(f, "A"),
            Self::M => write!(f, "M"),
            Self::N => write!(f, "N"),
            Self::P => write!(f, "P"),
            Self::Y => write!(f, "Y"),
            Self::C => write!(f, "C"),
            Self::I => write!(f, "I"),
            Self::E => write!(f, "E"),
            Self::F => write!(f, "F"),
            Self::G => write!(f, "G"),
            Self::R => write!(f, "R"),
            Self::W => write!(f, "W"),
            Self::Q => write!(f, "Q"),
            _ => unreachable!(),
        }
    }
}

impl Display for ffi::EngineType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Unknown => write!(f, "?"),
            Self::Piston => write!(f, "P"),
            Self::Turboprop => write!(f, "T"),
            Self::Jet => write!(f, "J"),
            Self::Electric => write!(f, "E"),
            _ => unreachable!(),
        }
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
    /// Our plugin holds a unique pointer to automatically deallocate the C++ plugin when this Rust counterpart is dropped.
    ///
    /// ## Warning
    /// This is not really a unique pointer. EuroScope holds a reference to the C++ plugin as well!
    cpp_plugin: cxx::UniquePtr<ffi::EsPlugin>,
    configs: Vec<config::Config>,
}

impl Drop for Plugin {
    fn drop(&mut self) {
        self.cpp_plugin
            .pin_mut()
            .display_user_message("ESFPC unloaded.");
    }
}

impl Plugin {
    fn start(mut cpp_plugin: cxx::UniquePtr<ffi::EsPlugin>) -> Result<Self, std::io::Error> {
        let rules_dir = find_rules_dir()?;
        println!("Rules directory: {}", rules_dir.display());

        let mut configs = vec![];
        for entry in rules_dir.read_dir()? {
            let path = entry?.path();
            let file = std::fs::File::open(path)?;
            let reader = std::io::BufReader::new(file);
            let config = serde_json::from_reader(reader)?;
            configs.push(config);
        }

        cpp_plugin.pin_mut().display_user_message("ESFPC loaded.");

        Ok(Self {
            cpp_plugin,
            configs,
        })
    }
}

fn find_rules_dir() -> Result<PathBuf, std::io::Error> {
    let dir = std::env::current_dir()?.join("rules");
    if dir.exists() {
        return Ok(dir);
    }

    let dll_path =
        ffi::get_dll_path().map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    let dir = PathBuf::from(dll_path)
        .parent()
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "DLL path has no parent directory",
            )
        })?
        .join("rules");
    if dir.exists() {
        return Ok(dir);
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "No rules directory found",
    ))
}

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
            return match ast::eval_cond(&rule.condition, &fp) {
                Ok(true) => Ok(rule.action.clone()),
                Ok(false) => continue,
                Err(e) => Err(e),
            };
        }
    }
    Ok(config::Action {
        typ: config::ActionType::Warning,
        msg: "UNK".into(),
    })
}

#[export_name = "?EuroScopePlugInInit@@YAXPAPAVCPlugIn@EuroScopePlugIn@@@Z"]
fn plugin_init(plugin_instance: *mut *mut ffi::CPlugIn) {
    // Safeguard, that the EuroScope pointer can be dereferenced later.
    assert!(!plugin_instance.is_null());

    // Create a new C++ plugin instance on the heap as unique pointer.
    let cpp_plugin = ffi::create_plugin();

    // Turn the unique pointer into a raw pointer.
    // This is required, because we have to set the EuroScope plugin pointer to the same instance.
    // We are now responsible again to free the memory of cpp_plugin.
    let cpp_plugin_ptr = cpp_plugin.into_raw();

    // Recreate a unique pointer to handle the deallocation on drop.
    // When drop is called, EsPlugin is freed and we are no longer responsible.
    let cpp_plugin = unsafe { cxx::UniquePtr::from_raw(cpp_plugin_ptr) };

    match Plugin::start(cpp_plugin) {
        Ok(plugin) => unsafe {
            // Initialize static variable with out plugin.
            PLUGIN = Some(plugin);
            // Set the EuroScope plugin pointer.
            *plugin_instance = cpp_plugin_ptr as *mut ffi::CPlugIn;
        },
        Err(_) => unsafe {
            // Set the EuroScope plugin pointer to null, as the plugin creation failed.
            *plugin_instance = std::ptr::null_mut();
        },
    }
}

#[export_name = "?EuroScopePlugInExit@@YAXXZ"]
fn plugin_exit() {
    unsafe { PLUGIN = None };
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
            dep: "EDDF".into(),
            arr: "EDDS".into(),
            sid: "ANEKI1L".into(),
            ..Default::default()
        };
        let fp_invalid_rfl = ffi::FlightPlan {
            rfl: 34000,
            ..fp_valid.clone()
        };
        let fp_invalid_dst = ffi::FlightPlan {
            arr: "EDDM".into(),
            ..fp_valid.clone()
        };

        let plugin = Plugin::start(ffi::create_plugin()).unwrap();

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

    #[test]
    fn eddf_komib() {
        let fp_valid = ffi::FlightPlan {
            rule: ffi::FlightRule::Ifr,
            rfl: 35000,
            dep: "EDDF".into(),
            arr: "EDDN".into(),
            sid: "KOMIB3D".into(),
            ..Default::default()
        };
        let fp_invalid_rfl = ffi::FlightPlan {
            rfl: 34000,
            ..fp_valid.clone()
        };
        let fp_invalid_dst = ffi::FlightPlan {
            arr: "EDDM".into(),
            ..fp_valid.clone()
        };

        let plugin = Plugin::start(ffi::create_plugin()).unwrap();

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

    #[test]
    fn eddf_tobak() {
        let fp_valid = ffi::FlightPlan {
            rule: ffi::FlightRule::Ifr,
            rfl: 35000,
            dep: "EDDF".into(),
            arr: "EDDN".into(),
            sid: "TOBAK7M".into(),
            route: "TOBAK7M/25C TOBAK N858 NOSEX DCT KLF".into(),
            ..Default::default()
        };
        let fp_invalid_rfl = ffi::FlightPlan {
            rfl: 34000,
            ..fp_valid.clone()
        };
        let fp_invalid_route = ffi::FlightPlan {
            route: "TOBAK7M/25C TOBAK Z10 NOSEX DCT KLF".into(),
            ..fp_valid.clone()
        };

        let plugin = Plugin::start(ffi::create_plugin()).unwrap();

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

        // Wrong RTE
        assert_eq!(
            Ok(config::Action::new(Error, "RTE".into())),
            check_flightplan_impl(&plugin, fp_invalid_route)
        );
    }
}
