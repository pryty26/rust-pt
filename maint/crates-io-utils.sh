# Utilities for querying crates.io
# shellcheck shell=bash

CRATES_IO_URL_BASE=https://crates.io/api

fail () {
    echo >&2 "$0: error: $*"
    exit 12
}

tmp_trap_exit_setup () {
    if [ "$MAINT_DDLETE_CREATE_TMP" != "" ]; then
	rm -rf -- "$MAINT_DDLETE_CREATE_TMP"
	mkdir -- "$MAINT_DDLETE_CREATE_TMP"
	tmp="$MAINT_DDLETE_CREATE_TMP"
    else
	tmp=$(mktemp -d)
	trap 'set +e; rm -rf "$tmp"; exit $exit_rc' 0
    fi
    exit_rc=8
}

tmp_trap_exit_finish_status () {
    exit_rc=$1
}

tmp_trap_exit_finish_ok () {
    tmp_trap_exit_finish_status 0
}