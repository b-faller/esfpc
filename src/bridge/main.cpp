#include "main.hpp"
#include "EuroScopePlugIn.hpp"
#include "esfpc/src/lib.rs.h"
#include "rust/cxx.h"
#include <format>
#include <stdexcept>
#include <string>

#define ITEM_STRING_SIZE 16

EsPlugin *es_plugin = NULL;

EsPlugin::EsPlugin(void)
    : CPlugIn(EuroScopePlugIn::COMPATIBILITY_CODE, PLUGIN_NAME, PLUGIN_VERSION,
              PLUGIN_DEVELOPER, PLUGIN_COPYRIGHT) {
  // Create a new entry which can be shown in a list such as departure list.
  this->RegisterTagItemType("VFPC", TAG_ITEM_FPCHECK);
  // Create a function which can be called when clicking on a tag in a list.
  this->RegisterTagItemFunction("Check FP", TAG_FUNC_FPCHECK);
}

EsPlugin::~EsPlugin() {}

// cppcheck-suppress unusedFunction
void EsPlugin::OnFunctionCall(int function_id, const char *, POINT, RECT) {
  switch (function_id) {
  case TAG_FUNC_FPCHECK:
    handleTagClick();
    break;
  default:
    break;
  }
}

// cppcheck-suppress unusedFunction
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
  // Get flight plan variables
  int32_t rfl = flight_plan.GetFinalAltitude();
  EuroScopePlugIn::CFlightPlanData fp_data = flight_plan.GetFlightPlanData();
  ffi::FlightRule rule = getFlightRule(fp_data.GetPlanType());
  rust::String adep = fp_data.GetOrigin();
  rust::String adest = fp_data.GetDestination();
  rust::String sid = fp_data.GetSidName();

  // Build flight plan struct
  ffi::FlightPlan fp = {rule, rfl, adep, adest, sid};
  // Check flight plan through Rust FFI.
  return ffi::check_flightplan(fp);
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

std::string getDllPath() {
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

  return std::string(path);
}

void __declspec(dllexport)
    EuroScopePlugInInit(EuroScopePlugIn::CPlugIn **ppPlugInInstance) {
  *ppPlugInInstance = es_plugin = new EsPlugin();

  std::string dll_path;
  try {
    dll_path = getDllPath();
  } catch (std::runtime_error &e) {
    es_plugin->DisplayUserMessage(PLUGIN_NAME, nullptr, e.what(), true, true,
                                  true, true, true);
    return;
  }

  try {
    ffi::init_plugin(rust::Str(dll_path));
  } catch (rust::Error &e) {
    es_plugin->DisplayUserMessage(PLUGIN_NAME, nullptr, e.what(), true, true,
                                  true, true, true);
    return;
  }
}

// cppcheck-suppress unusedFunction
void __declspec(dllexport) EuroScopePlugInExit() {
  ffi::exit_plugin();
  delete es_plugin;
}
