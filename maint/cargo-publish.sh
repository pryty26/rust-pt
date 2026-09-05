#!/usr/bin/env bash
#
# usage:
#   maint/cargo-publish [<OPTIONS>] VERSION
#
# options:
#   --dry-run
#   --force                go ahead even if problems detected
#   --branch BRANCH        expect to be releasing origin/BRANCH, not HEAD
#   --origin ORIGIN        replace repo URL to look for BRANCH on
#
# Preconditions:
#   See doc/Release.md
#
# Running a different version to the one in the arti.git being published:
# You can invoke this as ../PATH/TO/maint/cargo-publish.
# That maint/ directory must contain compatible versions of the support
# utilities, but it need not match the one in the CWD.
# The version that is published is the one from the CWD.

set -e

origin='https://gitlab.torproject.org/pryty26/rust-pt'
branch='main'
maint="$(dirname "$0")"
: "${CARGO:=cargo}"

source "$maint/crates-io-utils.sh"

# Note for users of nailing-cargo: use this setting:
#   CARGO='nailing-cargo --preclean=full --no-nail --git -o'

# No longer need to source crates-io-utils.py - it's called directly as a Python script

#===== utilities =====

all_ok=true

problem () {
    echo >&2 "problem: $*"
    all_ok=false
}

check_equal () {
    local a="$1"
    local b="$2"
    local a_what="$3"
    local b_what="$4"
    if [ "$a" != "$b" ]; then
	problem "mismatch: $a_what ($a) != $b_what ($b)"
    fi
}

#===== temporary files =====

tmp_trap_exit_setup

#===== argument parsing =====

dry_run=false
force=false

while [ $# != 0 ]; do
    case "$1" in
	--) break;;
	-*) ;;
	*) break;;
    esac
    arg="$1"; shift
    case "$arg" in
	--dry-run) dry_run=true ;;
	--force) force=true ;;
	--branch|--origin)
	    eval "${arg#--}=\$1"
	    shift || fail "$arg takes an argument"
	;;
	*) fail "unknown option: $1";;
    esac
done

case $# in
    1) version=$1 ;;
    *) fail "bad usage, needs version"
esac

#----- check that we are identical to our main repo's `main`

git fetch --quiet "$origin" "$branch"

origin_head=$(git rev-parse FETCH_HEAD~0)
our_head=$(git rev-parse HEAD~0)

check_equal "$our_head" "$origin_head" \
	    "our HEAD commit" "origin commit $origin"

# TODO somehow check that CI failed there!

#----- check workspace dependencies versions are consistent

echo "Checking workspace dependency versions..."

if ! python3 "$maint/check-version.py"; then
    fail "Dependency version check failed"
fi

#----- check that pt_core version matches

pt_core=$("$maint"/list-crates.py -p pt_core --version)
pt_core=${pt_core##* }

check_equal "$pt_core" "$version" \
	    "version of the main crate, in-tree" \
	    "specified version to release"

#----- compare already-published versions -----

to_publish=""

"$maint"/list-crates.py --version >"$tmp/all-crates"

exec 3<"$tmp/all-crates"

# shellcheck disable=SC2162 # we don't need -r, it has no backslashes
while <&3 read p v; do
    printf "checking status of %-30s %10s ..." "$p" "$v"

    # Use Python script to check if version is already published
    if python3 "$maint/crates_io_api.py" "$p" "$v" >/dev/null 2>&1; then
        echo ' already published.'
    else
        echo ' needs publishing.'
        to_publish+=" $p"
    fi
done

#===== commitment point =====

prefix=xxxx

running () {
    echo "    $*"
    "$@"
}

if $dry_run; then
    if $all_ok; then
	echo 'all seems OK, would run the following commands:'
	prefix='echo'
    elif $force; then
	echo 'PROBLEMS, but --force passed, would run:'
	prefix='echo'
    else
	echo 'PROBLEMS - would not run!'
	prefix='echo false'
    fi
else
    if $all_ok; then
	echo 'all OK, running publication!'
	prefix='running'
    elif $force; then
	echo 'PROBLEMS, but --force passed, going ahead!'
	prefix='running'
    else
	fail 'problems (see above), stopping'
    fi
fi

for p in $to_publish; do
    # shellcheck disable=SC2086 # we want to split on spaces in $prefix and $CARGO
    $prefix $CARGO publish -p "$p"
done

echo 'all published.'

tmp_trap_exit_finish_ok