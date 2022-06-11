import subprocess
from pathlib import Path

VC_STARTUP_SCRIPT = Path(
    "C:/Program Files (x86)/Microsoft Visual Studio/2022/BuildTools/VC/Auxiliary/Build/vcvarsall.bat")

BASE_PATH = Path().cwd()

INCLUDE_PATH = BASE_PATH / "include/"
CXX_HEADER = INCLUDE_PATH / "cxx.hpp"
CXX_SOURCE = INCLUDE_PATH / "cxx.cpp"

RS_SOURCE = BASE_PATH / "src/lib.rs"
CPP_SOURCE = BASE_PATH / "src/bridge/main.cpp"

EUROSCOPE_LIB = BASE_PATH / "lib/EuroScopePlugInDll.lib"
RS_DLL = BASE_PATH / "target/i686-pc-windows-msvc/release/esfpc.lib"

OUTPUT_PATH = BASE_PATH / "build"
OUTPUT_DLL = OUTPUT_PATH / "esfpc.dll"


def generate_cxx_sources():
    subprocess.run(["cxxbridge", RS_SOURCE,
                   "--header", "--output", CXX_HEADER])
    subprocess.run(["cxxbridge", RS_SOURCE, "--output", CXX_SOURCE])


def compile_rust():
    subprocess.run(["cargo", "build", "--release"])


def compile_cpp():
    if not OUTPUT_PATH.exists():
        OUTPUT_PATH.mkdir()
    status = subprocess.run(
        f"\"{VC_STARTUP_SCRIPT}\" x86 && cl \"{CPP_SOURCE}\" \"{CXX_SOURCE}\" /arch:IA32 /LD -I \"{INCLUDE_PATH}\" /link /OUT:\"{OUTPUT_DLL}\" \"{EUROSCOPE_LIB}\" \"{RS_DLL}\"", shell=True, capture_output=True, cwd=OUTPUT_PATH)

    print(status.stdout.decode("utf8"))
    print(status.stderr.decode("utf8"))

    status.check_returncode()


def build():
    generate_cxx_sources()
    compile_rust()
    compile_cpp()


def main():
    build()


if __name__ == "__main__":
    main()
