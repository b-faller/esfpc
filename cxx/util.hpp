#pragma once

#include "esfpc/include/EuroScopePlugIn.hpp"
#include <cstdint>

namespace ffi {
enum class AircraftType : uint8_t;
enum class WakeTurbulenceCategory : uint8_t;
enum class FaaEquipmentCode : uint8_t;
enum class EngineType : uint8_t;
enum class FlightRule : uint8_t;
struct FlightPlan;
} // namespace ffi

/// Convert char to FFI enum.
ffi::AircraftType getAircraftType(char);

/// Convert char to FFI enum.
ffi::WakeTurbulenceCategory getWakeTurbulenceCategory(char);

/// Convert char to FFI enum.
ffi::FaaEquipmentCode getFaaEquipmentCode(char);

/// Convert char to FFI enum.
ffi::EngineType getEngineType(char);

/// Convert C string to FFI enum.
ffi::FlightRule getFlightRule(const char *flight_rule);

/// Build flight plan struct from EuroScope flight plan.
ffi::FlightPlan getFlightPlan(EuroScopePlugIn::CFlightPlan flight_plan);