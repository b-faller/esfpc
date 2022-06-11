#include "main.hpp"
#include "cxx.hpp"
#include <string>

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

void EsPlugin::handle_checkfp() {
  int c = add(2, 4);
  std::string test = std::to_string(c);

  this->DisplayUserMessage(PLUGIN_NAME, nullptr, test.c_str(), true, true,
                           false, false, false);
}

void __declspec(dllexport)
    EuroScopePlugInInit(EuroScopePlugIn::CPlugIn **ppPlugInInstance) {
  *ppPlugInInstance = es_plugin = new EsPlugin();
}

void __declspec(dllexport) EuroScopePlugInExit(void) { delete es_plugin; }
