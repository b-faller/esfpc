#pragma once

#include "esfpc/include/EuroScopePlugIn.hpp"
#include "rust/cxx.h"
#include <memory>
#include <string>

#define PLUGIN_NAME "esfpc"
#define PLUGIN_VERSION "0.1.0"
#define PLUGIN_DEVELOPER "Benjamin Faller"
#define PLUGIN_COPYRIGHT "GPL v3"
#define PLUGIN_VIEW_AVISO "EuroScope rule-based flightplan checker"

// ID to match when OnGetTagItem is called
#define TAG_ITEM_FPCHECK 0x0001DEAD
// Function ID to match when OnFunctionCall is called
#define TAG_FUNC_FPCHECK 0x1001DEAD

namespace ffi {
struct Action;
}

class EsPlugin : public EuroScopePlugIn::CPlugIn {
public:
  EsPlugin() noexcept;
  ~EsPlugin() noexcept;

  void OnFunctionCall(int function_id, const char *item_string, POINT point,
                      RECT area) final override;

  void OnGetTagItem(EuroScopePlugIn::CFlightPlan flight_plan,
                    EuroScopePlugIn::CRadarTarget radar_target, int item_code,
                    int tag_data, char item_string[16], int *color_code,
                    COLORREF *rgb, double *font_size) final override;

  /// Called if the "Check FP" function is triggered.
  void handleTagClick();

  void display_user_message(rust::Str message);
};

/// Checks the flightplan and updates the tag with the check result.
void updateTag(EuroScopePlugIn::CFlightPlan flight_plan, char item_string[16],
               int *color_code);

/// Check a flightplan
///
/// Throws rust::Error if the flight plan could not be checked.
ffi::Action checkFlightPlan(EuroScopePlugIn::CFlightPlan flight_plan);

/// Get the absolute path to the DLL during runtime.
rust::String get_dll_path();

/// Create a C++ plugin instance.
std::unique_ptr<EsPlugin> create_plugin();
