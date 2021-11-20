#!/usr/bin/env bash

function setup_git_merge_conflict() (
    set -euo pipefail
    IFS=$'\n\t'
    if [ $# != 0 ]; then
        printf 'no arguments expected\n' 1>&2
        return 1
    fi

    printf 'new git repo, creating v0.0.0\n'
    pth="$(mktemp --directory)"
    cd "$pth"
    git init 1>/dev/null
    main="$(git rev-parse --abbrev-ref HEAD)"
    mkdir -p apivolve
    cat > 'apivolve/v0.0.0.apiv' <<- EOM
apivolve v0.1

add object User {
    add id type Uint64
}
EOM
    git add :/ --all
    git commit -m ''

    printf 'done\n'
)

setup_git_merge_conflict
