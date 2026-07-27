

import subprocess
import sys


def main():
    ignore_dir = [
        "**/.git/**",
        "**/.venv/**",
        "dist/**",
        "pkg/**",
        "target/**",
        "build/**",
        "**/__pycache__/**",
        "**/*.lock",
        "/.python-version",
        "**/*.png",
        "**/*.kra",
        "**/*.ttf",
        "**/*.otf",
        "assets/**",
        "**/*.json",
        # auto generated license info
        "**/ThirdPartyLicense-Rust.*",
        "**/ThirdPartyLicense-Python.*",
        "auto_generated/**",
        "**/*.icon",
        "**/*.ico",
        "docs/book/**",
        # for ci
        "**/rust-init.sh",
        # website
        "website/docs/**",
        "website_build/**",
        # `old_pmj_client` 包含第三方程式碼
        # [android-iced-example](https://github.com/ibaryshnikov/android-iced-example)
        "crates/old_pmj_client/src/android/**",
        # Author: [iced team](https://github.com/iced-rs/)
        # This file is from project [iced](https://github.com/iced-rs/iced/).
        "crates/pmj_client_desktop/src/easing.rs",
        # Author: [iced team](https://github.com/iced-rs/)
        # This file is from project [iced](https://github.com/iced-rs/iced/).
        "crates/pmj_client_desktop/src/circular.rs",
    ]
    ignored = []
    for dir in ignore_dir:
        ignored.extend(["-ignore", dir])
    command = [
        "addlicense",
        "-check",
        "-f",
        "templates/addlicense.template",
    ]
    command.extend(ignored.copy())
    command.append(".")
    print("Run Command:", " ".join(command))
    print("-" * 10)
    process = subprocess.run(
        command,
        # check=True,
        stdout=sys.stdout,
        stdin=sys.stdin,
        stderr=sys.stderr,
        timeout=180,
    )
    print("-" * 10)
    if process.returncode != 0:
        print("Something Wrong!")
        fix_command = [
            "addlicense",
            "-f",
            "templates/addlicense.template",
        ]
        fix_command.extend(ignored.copy())
        fix_command.append(".")
        print("Fix Command:", " ".join(fix_command))
        sys.exit(1)
    else:
        print("Check Finish.")


if __name__ == "__main__":
    main()
