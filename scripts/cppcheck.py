import subprocess
import sys

CPPCHECK_FLAGS = ["--enable=all", "--inconclusive", "--platform=win32W", "--std=c++20",
                  "--inline-suppr", "--suppress=missingIncludeSystem", "--error-exitcode=1"]


def cppcheck():
    flags = str.join(" ", CPPCHECK_FLAGS)
    cmd = \
        f"cppcheck {flags} " \
        f"-I ../ --suppress=*:*/EuroScopePlugIn.hpp " \
        f"-I target/i686-pc-windows-msvc/cxxbridge/ --suppress=*:target/* " \
        f"cxx/*"
    print(cmd, file=sys.stderr)
    subprocess.run(cmd, shell=True, check=True)


if __name__ == "__main__":
    cppcheck()
