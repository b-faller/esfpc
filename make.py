import subprocess
from pathlib import Path

VC_STARTUP_SCRIPT = Path(
    "C:/Program Files (x86)/Microsoft Visual Studio/"
    "2022/BuildTools/VC/Auxiliary/Build/vcvarsall.bat")

BASE_PATH = Path().cwd()

INCLUDE_PATH = BASE_PATH / "include"
TARGET_ARCH = BASE_PATH / "target/i686-pc-windows-msvc"
INCLUDE_CXX_PATH = TARGET_ARCH / "cxxbridge"

RS_SOURCE = BASE_PATH / "src/lib.rs"
CPP_SOURCE = BASE_PATH / "src/bridge/main.cpp"
CXX_SOURCE = TARGET_ARCH / "cxxbridge/esfpc/src/lib.rs.cc"

EUROSCOPE_LIB = BASE_PATH / "lib/EuroScopePlugInDll.lib"
RS_DLL = TARGET_ARCH / "release/esfpc.lib"

OUTPUT_PATH = BASE_PATH / "build"
OUTPUT_DLL = OUTPUT_PATH / "esfpc.dll"


def compile_rust():
    subprocess.run(
        ["cargo", "build", "--target=i686-pc-windows-msvc", "--release"], check=True)


def compile_cpp():
    if not OUTPUT_PATH.exists():
        OUTPUT_PATH.mkdir()
    status = subprocess.run(
        f"\"{VC_STARTUP_SCRIPT}\" x86 && "
        f"cl \"{CPP_SOURCE}\" \"{CXX_SOURCE}\" "
        f"/std:c++20 /arch:IA32 /EHsc /LD /MD -I \"{INCLUDE_PATH}\" -I \"{INCLUDE_CXX_PATH}\" "
        f"/link /OUT:\"{OUTPUT_DLL}\" \"{EUROSCOPE_LIB}\" \"{RS_DLL}\"",
        shell=True, capture_output=True, cwd=OUTPUT_PATH)

    print(status.stdout.decode("utf8"))
    print(status.stderr.decode("utf8"))

    status.check_returncode()


def build():
    compile_rust()
    compile_cpp()


def main():
    build()


if __name__ == "__main__":
    main()
