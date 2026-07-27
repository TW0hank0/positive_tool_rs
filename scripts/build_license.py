

import json
import os
import shutil
import subprocess
import sys


def main():
    print("-" * 10, "cargo-about", "-" * 10)
    #
    all_commands = [
        [
            "cargo",
            "about",
            "generate",
            "--output-file",
            "auto_generated/ThirdPartyLicense-Rust.html",
            "--threshold",
            "1.0",
            "templates/about_html.hbs",
        ],
        [
            "cargo",
            "about",
            "generate",
            "--threshold",
            "1.0",
            "--output-file",
            "auto_generated/ThirdPartyLicense-Rust.json",
            "templates/about_json.hbs",
        ],
        [
            "cargo",
            "about",
            "generate",
            "--output-file",
            "auto_generated/ThirdPartyLicense-Rust.md",
            "--threshold",
            "1.0",
            "templates/about_markdown.hbs",
        ],
    ]
    #
    for command in all_commands:
        print(f"Run Command: {' '.join(command)}")
        subprocess.run(
            command,
            check=True,
            stdout=sys.stdout,
            stdin=sys.stdin,
            stderr=sys.stderr,
        )
    #
    print("Indenting json file...", end="")
    json_file_path = os.path.abspath(
        os.path.join(
            __file__,
            "..",
            "..",
            "auto_generated",
            "ThirdPartyLicense-Rust.json",
        )
    )
    with open(json_file_path, "r", encoding="utf-8") as f:
        json_data = json.load(f)
    with open(json_file_path, "w", encoding="utf-8") as f:
        json.dump(json_data, f, ensure_ascii=False, sort_keys=True, indent=4)
    print("Finish!")
    print("Copying ThirdPartyLicense.md ...", end="")
    shutil.copyfile(
        os.path.join(
            os.path.dirname(os.path.dirname(__file__)),
            "auto_generated",
            "ThirdPartyLicense-Rust.md",
        ),
        os.path.join(
            os.path.dirname(os.path.dirname(__file__)),
            "docs",
            "src",
            "license",
            "ThirdPartyLicense-Rust.md",
        ),
    )
    print("Finish!")


if __name__ == "__main__":
    try:
        main()
    finally:
        print()
