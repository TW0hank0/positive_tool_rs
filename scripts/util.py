

import subprocess


def run_cmd(command: list[str], cwd: str | None = None):
    print(f"===> Running command: {' '.join(command)}")
    process = subprocess.run(
        command,
        timeout=60 * 40,
        stderr=subprocess.PIPE,
        stdout=subprocess.PIPE,
        text=True,
        cwd=cwd
    )
    if process.returncode == 0:
        print("=====> Process fnished sucessful.")
    else:
        print(f"=====> Process exited with non-zero code ({process.returncode})!")
        print("--- process stdout ---")
        print(process.stdout)
        print("--- process stderr ---")
        print(process.stderr)
        print("--- end-of output ---")
