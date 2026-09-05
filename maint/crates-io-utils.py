#!/usr/bin/env python3
"""
Utilities for querying crates.io
"""

import requests
import sys
import time

CRATES_IO_URL_BASE = "https://crates.io/api"


def check_version(crate_name: str, version: str) -> tuple[bool, int]:
    """
    Queries
        https://crates.io/api/v1/crates/{crate_name}/{version}
    Expects to receive either
        HTTP 200 and a json document containing a `.version` key
        HTTP 404 and a json document containing a `.errors` key
    The HTTP code is returned as the second element of the tuple.
    """
    url = f"{CRATES_IO_URL_BASE}/v1/crates/{crate_name}/{version}"

    time.sleep(1)

    try:
        resp = requests.get(
            url, headers={"User-Agent": "maint/ scripts for Rust project (Python)"}
        )
        http_code = resp.status_code

        if http_code == 200:
            data = resp.json()
            if "version" in data:
                return True, http_code
            else:
                return False, http_code
        elif http_code == 404:
            return False, http_code
        else:
            print(
                f"unexpected HTTP response status code {http_code} from {url}",
                file=sys.stderr,
            )
            print(resp.text, file=sys.stderr)
            return False, http_code

    except requests.exceptions.RequestException as e:
        print(f"request failed: {e}", file=sys.stderr)
        return False, 0


def main():
    if len(sys.argv) != 3:
        print("Usage: python crates_io_api.py <crate_name> <version>")
        sys.exit(1)

    crate_name = sys.argv[1]
    version = sys.argv[2]

    exists, http_code = check_version(crate_name, version)

    if exists:
        print(f"{crate_name} {version} already published.")
        sys.exit(0)
    else:
        print(f"{crate_name} {version} needs publishing.")
        sys.exit(1)


if __name__ == "__main__":
    main()
