# tested on windows

import sys
import subprocess
import time


def build_debug():
    time_start = time.time()
    exit_code = subprocess.call(["cmake", "--build", "."])
    print(f"\nBuild finished in {time.time() - time_start} milliseconds")
    print("-----------------------------------------------")
    return exit_code


# debug
def run():
    exit_code = subprocess.call("./Debug/giz", cwd="./Debug")
    return exit_code


# argv[0] is the file name
# BUILD and run
if sys.argv[1] == "run":
    build_exit_code = build_debug()
    if build_exit_code == 1:
        exit(1)

    giz_exit_code = run()

    print("-----------------------------------------------")

    if giz_exit_code == 1:
        print("Exited abruptly")
    elif giz_exit_code == 3221225477:
        print("Access violation")
    else:
        # unhandled exit code
        print(f"Exit code: {hex(giz_exit_code)} ({giz_exit_code})")
