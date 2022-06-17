#include "esfpc/cxx/util.hpp"
#include "esfpc/include/EuroScopePlugIn.hpp"
#include "esfpc/src/lib.rs.h"
#include "rust/cxx.h"
#include <format>
#include <stdexcept>

ffi::AircraftType getAircraftType(char ac_typ) {
  switch (ac_typ) {
  case 'L':
    return ffi::AircraftType::Landplane;
  case 'S':
    return ffi::AircraftType::Seaplane;
  case 'A':
    return ffi::AircraftType::Amphibian;
  case 'H':
    return ffi::AircraftType::Helicopter;
  case 'G':
    return ffi::AircraftType::Gyrocopter;
  case 'T':
    return ffi::AircraftType::TiltWing;
  default:
    return ffi::AircraftType::Unknown;
  }
}

ffi::WakeTurbulenceCategory getWakeTurbulenceCategory(char eng_typ) {
  switch (eng_typ) {
  case 'L':
    return ffi::WakeTurbulenceCategory::Light;
  case 'M':
    return ffi::WakeTurbulenceCategory::Medium;
  case 'H':
    return ffi::WakeTurbulenceCategory::Heavy;
  case 'J':
    return ffi::WakeTurbulenceCategory::Super;
  default:
    return ffi::WakeTurbulenceCategory::Unknown;
  }
}

ffi::FaaEquipmentCode getFaaEquipmentCode(char wtc) {
  switch (wtc) {
  case 'T':
    return ffi::FaaEquipmentCode::T;
  case 'X':
    return ffi::FaaEquipmentCode::X;
  case 'U':
    return ffi::FaaEquipmentCode::U;
  case 'D':
    return ffi::FaaEquipmentCode::D;
  case 'B':
    return ffi::FaaEquipmentCode::B;
  case 'A':
    return ffi::FaaEquipmentCode::A;
  case 'M':
    return ffi::FaaEquipmentCode::M;
  case 'N':
    return ffi::FaaEquipmentCode::N;
  case 'P':
    return ffi::FaaEquipmentCode::P;
  case 'Y':
    return ffi::FaaEquipmentCode::Y;
  case 'C':
    return ffi::FaaEquipmentCode::C;
  case 'I':
    return ffi::FaaEquipmentCode::I;
  case 'E':
    return ffi::FaaEquipmentCode::E;
  case 'F':
    return ffi::FaaEquipmentCode::F;
  case 'G':
    return ffi::FaaEquipmentCode::G;
  case 'R':
    return ffi::FaaEquipmentCode::R;
  case 'W':
    return ffi::FaaEquipmentCode::W;
  case 'Q':
    return ffi::FaaEquipmentCode::Q;
  default:
    return ffi::FaaEquipmentCode::Unknown;
  }
}

ffi::EngineType getEngineType(char eng_typ) {
  switch (eng_typ) {
  case 'P':
    return ffi::EngineType::Piston;
  case 'T':
    return ffi::EngineType::Turboprop;
  case 'J':
    return ffi::EngineType::Jet;
  case 'E':
    return ffi::EngineType::Electric;
  default:
    return ffi::EngineType::Unknown;
  }
}

ffi::FlightRule getFlightRule(const char *flight_rule) {
  std::string rule = std::string(flight_rule);
  if (rule == "V") {
    return ffi::FlightRule::Vfr;
  } else if (rule == "I") {
    return ffi::FlightRule::Ifr;
  } else if (rule == "Y") {
    return ffi::FlightRule::Yankee;
  } else if (rule == "Z") {
    return ffi::FlightRule::Zulu;
  } else {
    throw std::invalid_argument(
        std::format("Invalid flight rule {}", flight_rule));
  }
}

ffi::FlightPlan getFlightPlan(EuroScopePlugIn::CFlightPlan flight_plan) {
  EuroScopePlugIn::CFlightPlanData fp_data = flight_plan.GetFlightPlanData();

  ffi::WakeTurbulenceCategory wtc =
      getWakeTurbulenceCategory(fp_data.GetAircraftWtc());
  ffi::AircraftType ac_typ = getAircraftType(fp_data.GetAircraftType());
  ffi::FaaEquipmentCode equip_code =
      getFaaEquipmentCode(fp_data.GetCapibilities());
  ffi::EngineType eng_typ = getEngineType(fp_data.GetEngineType());
  rust::u8 eng_count = static_cast<uint8_t>(fp_data.GetEngineNumber());
  bool is_rvsm_capable = fp_data.IsRvsm();
  ffi::Aircraft ac = {ac_typ,  wtc,       equip_code,
                      eng_typ, eng_count, is_rvsm_capable};

  ffi::FlightRule rule = getFlightRule(fp_data.GetPlanType());
  rust::u32 cfl = flight_plan.GetClearedAltitude();
  rust::u32 rfl = fp_data.GetFinalAltitude();
  rust::String dep = fp_data.GetOrigin();
  rust::String dep_rwy = fp_data.GetDepartureRwy();
  rust::String arr = fp_data.GetDestination();
  rust::String sid = fp_data.GetSidName();
  rust::String route = fp_data.GetRoute();

  ffi::FlightPlan fp = {ac, rule, cfl, rfl, dep, dep_rwy, arr, sid, route};
  return fp;
}