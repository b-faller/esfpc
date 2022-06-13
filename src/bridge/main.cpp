#include "main.hpp"
#include "EuroScopePlugIn.hpp"
#include "esfpc/src/lib.rs.h"

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
  case TAG_ITEM_FPCHECK: {
    if (!FlightPlan.IsValid()) {
      return;
    }
    int32_t rfl = FlightPlan.GetFinalAltitude();
    const char *typ = FlightPlan.GetFlightPlanData().GetPlanType();

    try {
      FpCheckResult fpc_res = check_flightplan(rfl);
      switch (fpc_res) {
      case FpCheckResult::Route:

      case FpCheckResult::Ok:
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

    break;
  }
  default:
    break;
  }
}

void EsPlugin::handle_checkfp() {
  this->DisplayUserMessage(PLUGIN_NAME, nullptr, "test", true, true, false,
                           false, false);
}

void __declspec(dllexport)
    EuroScopePlugInInit(EuroScopePlugIn::CPlugIn **ppPlugInInstance) {
  *ppPlugInInstance = es_plugin = new EsPlugin();
}

void __declspec(dllexport) EuroScopePlugInExit(void) { delete es_plugin; }
