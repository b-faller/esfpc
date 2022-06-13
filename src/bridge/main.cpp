#include "main.hpp"
#include "EuroScopePlugIn.hpp"
#include "esfpc/src/lib.rs.h"
#include <string.h>

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

void EsPlugin::OnFunctionCall(int FunctionId, const char *ItemString, POINT Pt,
                              RECT Area) {
  switch (FunctionId) {
  case TAG_FUNC_FPCHECK:
    handle_checkfp();
    break;
  default:
    break;
  }
}

void EsPlugin::OnGetTagItem(EuroScopePlugIn::CFlightPlan FlightPlan,
                            EuroScopePlugIn::CRadarTarget RadarTarget,
                            int ItemCode, int TagData, char sItemString[16],
                            int *pColorCode, COLORREF *pRGB,
                            double *pFontSize) {
  switch (ItemCode) {
  case TAG_ITEM_FPCHECK:
    this->handle_fpcheck(FlightPlan, sItemString, pColorCode);
    break;
  default:
    break;
  }
}

void EsPlugin::handle_fpcheck(EuroScopePlugIn::CFlightPlan FlightPlan,
                              char sItemString[16], int *pColorCode) {
  if (!FlightPlan.IsValid()) {
  }
  ffi::FlightRule rule =
      get_flight_rule(FlightPlan.GetFlightPlanData().GetPlanType());
  int32_t rfl = FlightPlan.GetFinalAltitude();
  ffi::FlightPlan fp = {rule, rfl};

  try {
    ffi::FpCheckResult fpc_res = ffi::check_flightplan(fp);
    switch (fpc_res) {
    case ffi::FpCheckResult::Ok:
    default:
      strncpy_s(sItemString, ITEM_STRING_SIZE, "OK", _TRUNCATE);
      *pColorCode = EuroScopePlugIn::TAG_COLOR_DEFAULT;
      break;
    }
  } catch (rust::Error e) {
    strncpy_s(sItemString, ITEM_STRING_SIZE, "ERR", _TRUNCATE);
    *pColorCode = EuroScopePlugIn::TAG_COLOR_EMERGENCY;
    return;
  }
}

void EsPlugin::handle_checkfp() {
  this->DisplayUserMessage(PLUGIN_NAME, nullptr, "test", true, true, false,
                           false, false);
}

ffi::FlightRule get_flight_rule(const char *c_rule) {
  std::string rule = std::string(c_rule);
  if (rule == "V") {
    return ffi::FlightRule::Vfr;
  } else if (rule == "I") {
    return ffi::FlightRule::Ifr;
  } else if (rule == "Y") {
    return ffi::FlightRule::Yankee;
  } else if (rule == "Z") {
    return ffi::FlightRule::Zulu;
  }
}

void __declspec(dllexport)
    EuroScopePlugInInit(EuroScopePlugIn::CPlugIn **ppPlugInInstance) {
  *ppPlugInInstance = es_plugin = new EsPlugin();
}

void __declspec(dllexport) EuroScopePlugInExit(void) { delete es_plugin; }
