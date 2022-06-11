#include "main.hpp"
#include "cxx.hpp"
#include <string>

EsPlugin::EsPlugin(void)
    : CPlugIn(EuroScopePlugIn::COMPATIBILITY_CODE, PLUGIN_NAME,
              PLUGIN_VERSION, PLUGIN_DEVELOPER, PLUGIN_COPYRIGHT) {}

EsPlugin::~EsPlugin() {}

// extern "C" int test_func();

void __declspec(dllexport)
    EuroScopePlugInInit(EuroScopePlugIn::CPlugIn **ppPlugInInstance) {
  *ppPlugInInstance = es_plugin = new EsPlugin();

  int c = add(2, 4);
  std::string test = std::to_string(c);

  es_plugin->DisplayUserMessage(PLUGIN_NAME, nullptr, test.c_str(), true, true, false, false, false);
}

void __declspec(dllexport) EuroScopePlugInExit(void) { delete es_plugin; }
