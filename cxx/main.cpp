#include "esfpc/cxx/main.hpp"
#include "esfpc/cxx/util.hpp"
#include "esfpc/include/EuroScopePlugIn.hpp"
#include "esfpc/src/lib.rs.h"
#include "rust/cxx.h"
#include <format>
#include <memory>
#include <stdexcept>
#include <stdint.h>
#include <string>

#define ITEM_STRING_SIZE 16

EsPlugin *es_plugin = NULL;

EsPlugin::EsPlugin(void) noexcept
    : CPlugIn(EuroScopePlugIn::COMPATIBILITY_CODE, PLUGIN_NAME, PLUGIN_VERSION,
              PLUGIN_DEVELOPER, PLUGIN_COPYRIGHT) {
  // Create a new entry which can be shown in a list such as departure list.
  this->RegisterTagItemType("VFPC", TAG_ITEM_FPCHECK);
  // Create a function which can be called when clicking on a tag in a list.
  this->RegisterTagItemFunction("Check FP", TAG_FUNC_FPCHECK);

  this->DisplayUserMessage(PLUGIN_NAME, nullptr, "C++ plugin created", true,
                           true, false, false, false);
}

EsPlugin::~EsPlugin() noexcept {
  this->DisplayUserMessage(PLUGIN_NAME, nullptr, "C++ plugin deleted", true,
                           true, false, false, false);
}

void EsPlugin::OnFunctionCall(int function_id, const char *, POINT, RECT) {
  switch (function_id) {
  case TAG_FUNC_FPCHECK:
    handleTagClick();
    break;
  default:
    break;
  }
}

void EsPlugin::OnGetTagItem(EuroScopePlugIn::CFlightPlan flight_plan,
                            EuroScopePlugIn::CRadarTarget, int item_code, int,
                            char item_string[16], int *color_code, COLORREF *,
                            double *) {
  switch (item_code) {
  case TAG_ITEM_FPCHECK:
    updateTag(flight_plan, item_string, color_code);
    break;
  default:
    break;
  }
}

void EsPlugin::handleTagClick() {
  EuroScopePlugIn::CFlightPlan flight_plan = FlightPlanSelectASEL();
  if (!flight_plan.IsValid()) {
    this->DisplayUserMessage(PLUGIN_NAME, nullptr, "Flight plan is invalid",
                             true, true, false, false, false);
    return;
  }

  try {
    ffi::Action action = checkFlightPlan(flight_plan);
    this->DisplayUserMessage(PLUGIN_NAME, nullptr, "Check succeded", true, true,
                             false, false, false);
  } catch (rust::Error &e) {
    std::string msg = std::format("Check failed: {}", e.what());
    this->DisplayUserMessage(PLUGIN_NAME, nullptr, msg.c_str(), true, true,
                             false, false, false);
  }
}

void EsPlugin::display_user_message(rust::Str message) {
  std::string msg = std::string(message);
  this->DisplayUserMessage(PLUGIN_NAME, nullptr, msg.c_str(), true, true, false,
                           false, false);
}

void updateTag(EuroScopePlugIn::CFlightPlan flight_plan, char item_string[16],
               int *color_code) {
  if (!flight_plan.IsValid()) {
    return;
  }

  ffi::Action action;
  try {
    action = checkFlightPlan(flight_plan);
  } catch (rust::Error &) {
    // Flight plan check ran into error.
    strncpy_s(item_string, ITEM_STRING_SIZE, "ERR", _TRUNCATE);
    *color_code = EuroScopePlugIn::TAG_COLOR_EMERGENCY;
    return;
  }

  switch (action.typ) {
  case ffi::ActionType::Error:
    *color_code = EuroScopePlugIn::TAG_COLOR_EMERGENCY;
    break;
  case ffi::ActionType::Warning:
    *color_code = EuroScopePlugIn::TAG_COLOR_ASSUMED;
    break;
  case ffi::ActionType::Info:
    *color_code = EuroScopePlugIn::TAG_COLOR_NOTIFIED;
    break;
  case ffi::ActionType::Success:
    *color_code = EuroScopePlugIn::TAG_COLOR_DEFAULT;
    break;
  }

  strncpy_s(item_string, ITEM_STRING_SIZE, action.msg.c_str(), _TRUNCATE);
}

ffi::Action checkFlightPlan(EuroScopePlugIn::CFlightPlan flight_plan) {
  ffi::FlightPlan fp = getFlightPlan(flight_plan);
  return ffi::check_flightplan(fp);
}

// cppcheck-suppress unusedFunction
rust::String get_dll_path() {
  char path[MAX_PATH];
  HMODULE hm = NULL;

  if (GetModuleHandleExW(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS |
                             GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
                         (LPCWSTR)&EuroScopePlugInInit, &hm) == 0) {
    throw std::runtime_error(
        std::format("Could not get DLL module handle: {}", GetLastError()));
  }

  if (GetModuleFileName(hm, path, sizeof(path)) == 0) {
    throw std::runtime_error(
        std::format("Could not get DLL file path: {}", GetLastError()));
  }

  return rust::String(path);
}

// cppcheck-suppress unusedFunction
std::unique_ptr<EsPlugin> create_plugin() {
  return std::make_unique<EsPlugin>();
}
