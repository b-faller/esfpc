#pragma once

#pragma comment(lib, "ws2_32")
#pragma comment(lib, "advapi32")
#pragma comment(lib, "userenv")
#pragma comment(lib, "bcrypt")

#include "EuroScopePlugIn.hpp"

#define PLUGIN_NAME "esfpc"
#define PLUGIN_VERSION "0.1.0"
#define PLUGIN_DEVELOPER "Benjamin Faller"
#define PLUGIN_COPYRIGHT "GPL v3"
#define PLUGIN_VIEW_AVISO "EuroScope rule-based flightplan checker"
#define PLUGIN_WELCOME_MESSAGE "You read this?"

// ID to match when OnGetTagItem is called
#define TAG_ITEM_FPCHECK 1
// Function ID to match when OnFunctionCall is called
#define TAG_FUNC_FPCHECK 100

using namespace EuroScopePlugIn;

class EsPlugin : public EuroScopePlugIn::CPlugIn {
public:
  EsPlugin();
  ~EsPlugin();

  virtual void OnFunctionCall(int FunctionId, const char *ItemString, POINT Pt,
                              RECT Area);

  void handle_checkfp();
};

EsPlugin *es_plugin = NULL;

void __declspec(dllexport) EuroScopePlugInInit(EuroScopePlugIn::CPlugIn **);
void __declspec(dllexport) EuroScopePlugInExit(void);
