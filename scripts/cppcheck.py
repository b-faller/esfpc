import subprocess
import sys

CPPCHECK_FLAGS = ["--enable=all", "--inconclusive", "--platform=win32W", "--std=c++20",
                  "--inline-suppr", "--suppress=missingIncludeSystem", "--error-exitcode=1"]


def cppcheck():
    cmd = ["cppcheck", *CPPCHECK_FLAGS, "-I", "../", "--suppress=*:*/EuroScopePlugIn.hpp", "-I",
           "target/i686-pc-windows-msvc/cxxbridge/", "--suppress=*:target/*", "cxx/*"]
    print(str.join(" ", cmd), file=sys.stderr)
    subprocess.run(cmd, check=True)


if __name__ == "__main__":
    cppcheck()
