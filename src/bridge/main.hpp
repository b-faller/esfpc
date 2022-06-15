#pragma once

#include "EuroScopePlugIn.hpp"
#include "esfpc/src/lib.rs.h"
#include "rust/cxx.h"
#include <string>

#pragma comment(lib, "advapi32")
#pragma comment(lib, "bcrypt")
#pragma comment(lib, "msvcrt")
#pragma comment(lib, "userenv")
#pragma comment(lib, "ws2_32")

#define PLUGIN_NAME "esfpc"
#define PLUGIN_VERSION "0.1.0"
#define PLUGIN_DEVELOPER "Benjamin Faller"
#define PLUGIN_COPYRIGHT "GPL v3"
#define PLUGIN_VIEW_AVISO "EuroScope rule-based flightplan checker"
#define PLUGIN_WELCOME_MESSAGE "You read this?"

// ID to match when OnGetTagItem is called
#define TAG_ITEM_FPCHECK 0x0001DEAD
// Function ID to match when OnFunctionCall is called
#define TAG_FUNC_FPCHECK 0x1001DEAD

class EsPlugin : public EuroScopePlugIn::CPlugIn {
public:
  EsPlugin();
  ~EsPlugin();

  void OnFunctionCall(int function_id, const char *item_string, POINT point,
                      RECT area) final override;

  void OnGetTagItem(EuroScopePlugIn::CFlightPlan flight_plan,
                    EuroScopePlugIn::CRadarTarget radar_target, int item_code,
                    int tag_data, char item_string[16], int *color_code,
                    COLORREF *rgb, double *font_size) final override;

  /// Called if the "Check FP" function is triggered.
  void handleTagClick();
};

/// Checks the flightplan and updates the tag with the check result.
void updateTag(EuroScopePlugIn::CFlightPlan flight_plan, char item_string[16],
               int *color_code);

/// Check a flightplan
///
/// Throws rust::Error if the flight plan could not be checked.
ffi::Action checkFlightPlan(EuroScopePlugIn::CFlightPlan flight_plan);

/// Get the absolute path to the DLL during runtime.
std::string getDllPath();

/// Plugin entry point.
///
/// Called when the plugin is loaded.
void __declspec(dllexport) EuroScopePlugInInit(EuroScopePlugIn::CPlugIn **);

/// Plugin exit point.
///
/// Called when the plugin is unloaded.
void __declspec(dllexport) EuroScopePlugInExit();
