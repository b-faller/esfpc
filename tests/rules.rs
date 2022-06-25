use esfpc::check_flightplan_impl;
use esfpc::config::Action;
use esfpc::config::ActionType::*;
use esfpc::ffi::*;
use esfpc::Plugin;

fn default_ac() -> Aircraft {
    Aircraft {
        typ: AircraftType::Landplane,
        wtc: WakeTurbulenceCategory::Medium,
        faa_equip_code: FaaEquipmentCode::Q,
        eng_typ: EngineType::Jet,
        eng_count: 2,
        is_rvsm_capable: true,
    }
}

fn default_fp() -> FlightPlan {
    FlightPlan {
        ac: default_ac(),
        rule: FlightRule::Ifr,
        cfl: 4000,
        rfl: 35000,
        dep: "EDDF".to_string(),
        dep_rwy: "18".to_string(),
        arr: "EDDM".to_string(),
        sid: "CINDY4S".to_string(),
        route: "CINDY Z74 HAREM T104 ROKIL".to_string(),
    }
}

#[test]
fn eddf_aneki() {
    let fp_valid = FlightPlan {
        rule: FlightRule::Ifr,
        rfl: 35000,
        dep: "EDDF".into(),
        arr: "EDDS".into(),
        sid: "ANEKI1L".into(),
        ..default_fp()
    };
    let fp_invalid_rfl = FlightPlan {
        rfl: 34000,
        ..fp_valid.clone()
    };
    let fp_invalid_dst = FlightPlan {
        arr: "EDDM".into(),
        ..fp_valid.clone()
    };

    let plugin = Plugin::start(create_plugin()).unwrap();

    // Odd RFL
    assert_eq!(
        Ok(Action::new(Success, "OK".into())),
        check_flightplan_impl(&plugin, fp_valid)
    );

    // Even RFL
    assert_eq!(
        Ok(Action::new(Error, "RFL".into())),
        check_flightplan_impl(&plugin, fp_invalid_rfl)
    );

    // Wrong DST
    assert_eq!(
        Ok(Action::new(Error, "DST".into())),
        check_flightplan_impl(&plugin, fp_invalid_dst)
    );
}

#[test]
fn eddf_komib() {
    let fp_valid = FlightPlan {
        rule: FlightRule::Ifr,
        rfl: 35000,
        dep: "EDDF".into(),
        arr: "EDDN".into(),
        sid: "KOMIB3D".into(),
        ..default_fp()
    };
    let fp_invalid_rfl = FlightPlan {
        rfl: 34000,
        ..fp_valid.clone()
    };
    let fp_invalid_dst = FlightPlan {
        arr: "EDDM".into(),
        ..fp_valid.clone()
    };

    let plugin = Plugin::start(create_plugin()).unwrap();

    // Odd RFL
    assert_eq!(
        Ok(Action::new(Success, "OK".into())),
        check_flightplan_impl(&plugin, fp_valid)
    );

    // Even RFL
    assert_eq!(
        Ok(Action::new(Error, "RFL".into())),
        check_flightplan_impl(&plugin, fp_invalid_rfl)
    );

    // Wrong DST
    assert_eq!(
        Ok(Action::new(Error, "DST".into())),
        check_flightplan_impl(&plugin, fp_invalid_dst)
    );
}

#[test]
fn eddf_tobak() {
    let fp_valid = FlightPlan {
        rule: FlightRule::Ifr,
        rfl: 35000,
        dep: "EDDF".into(),
        arr: "EDDN".into(),
        sid: "TOBAK7M".into(),
        route: "TOBAK7M/25C TOBAK N858 NOSEX DCT KLF".into(),
        ..default_fp()
    };
    let fp_invalid_rfl = FlightPlan {
        rfl: 34000,
        ..fp_valid.clone()
    };
    let fp_invalid_route = FlightPlan {
        route: "TOBAK7M/25C TOBAK Z10 NOSEX DCT KLF".into(),
        ..fp_valid.clone()
    };

    let plugin = Plugin::start(create_plugin()).unwrap();

    // Odd RFL
    assert_eq!(
        Ok(Action::new(Success, "OK".into())),
        check_flightplan_impl(&plugin, fp_valid)
    );

    // Even RFL
    assert_eq!(
        Ok(Action::new(Error, "RFL".into())),
        check_flightplan_impl(&plugin, fp_invalid_rfl)
    );

    // Wrong RTE
    assert_eq!(
        Ok(Action::new(Error, "RTE".into())),
        check_flightplan_impl(&plugin, fp_invalid_route)
    );
}

#[test]
fn eddf_non_rnav_sids() {
    for sid in ["MTR5C", "RID8C", "RID3Q", "TAU2Q"] {
        let fp_valid = FlightPlan {
            rule: FlightRule::Ifr,
            rfl: 9000,
            sid: sid.into(),
            ac: Aircraft {
                faa_equip_code: FaaEquipmentCode::A,
                ..default_ac()
            },
            ..default_fp()
        };
        let fp_invalid_rfl = FlightPlan {
            rfl: 10000,
            ..fp_valid.clone()
        };
        let fp_invalid_equip = FlightPlan {
            ac: Aircraft {
                faa_equip_code: FaaEquipmentCode::G,
                ..default_ac()
            },
            ..fp_valid.clone()
        };

        let plugin = Plugin::start(create_plugin()).unwrap();

        assert_eq!(
            Ok(Action::new(Success, "OK".into())),
            check_flightplan_impl(&plugin, fp_valid)
        );

        // Too high FL
        assert_eq!(
            Ok(Action::new(Error, "RFL".into())),
            check_flightplan_impl(&plugin, fp_invalid_rfl)
        );

        // RNAV capable
        assert_eq!(
            Ok(Action::new(Error, "RNV".into())),
            check_flightplan_impl(&plugin, fp_invalid_equip)
        );
    }
}
