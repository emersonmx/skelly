#!/bin/bash

set -euo pipefail

script_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
git_hooks_path="$(git rev-parse --show-toplevel)/.git/hooks"


find "$script_dir" -type f -executable -not -path "*/setup.sh" \
    -exec cp -vf "{}" "$git_hooks_path/" \;
