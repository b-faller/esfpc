#pragma once

#include "EuroScopePlugIn.hpp"
#include "esfpc/src/lib.rs.h"
#include "main.hpp"
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

  virtual void OnFunctionCall(int FunctionId, const char *ItemString, POINT Pt,
                              RECT Area);

  virtual void OnGetTagItem(EuroScopePlugIn::CFlightPlan FlightPlan,
                            EuroScopePlugIn::CRadarTarget RadarTarget,
                            int ItemCode, int TagData, char sItemString[16],
                            int *pColorCode, COLORREF *pRGB, double *pFontSize);

  void handle_fpcheck(EuroScopePlugIn::CFlightPlan FlightPlan,
                      char sItemString[16], int *pColorCode);
  void handle_checkfp();
};

ffi::FlightRule get_flight_rule(const char *c_rule);

void __declspec(dllexport) EuroScopePlugInInit(EuroScopePlugIn::CPlugIn **);
void __declspec(dllexport) EuroScopePlugInExit(void);
