#pragma once

#pragma comment(lib, "ws2_32")
#pragma comment(lib, "advapi32")
#pragma comment(lib, "userenv")
#pragma comment(lib, "bcrypt")

#include "EuroScopePlugIn.hpp"

#define PLUGIN_NAME "esfpc"
#define PLUGIN_VERSION "0.1.0"
#define PLUGIN_DEVELOPER "Benjamin Faller"
#define PLUGIN_COPYRIGHT "MIT"
#define PLUGIN_VIEW_AVISO "EuroScope rule-based flightplan checker"
#define PLUGIN_WELCOME_MESSAGE "You read this?"

using namespace EuroScopePlugIn;

class EsPlugin : public EuroScopePlugIn::CPlugIn {
public:
  EsPlugin();
  virtual ~EsPlugin();
};

EsPlugin *es_plugin = NULL;

void __declspec(dllexport) EuroScopePlugInInit(EuroScopePlugIn::CPlugIn **);
void __declspec(dllexport) EuroScopePlugInExit(void);
