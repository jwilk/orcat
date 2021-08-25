#!/usr/bin/env bash

# Copyright © 2021 Jakub Wilk <jwilk@jwilk.net>
# SPDX-License-Identifier: MIT

set -e -u
echo 1..2
base="${0%/*}/.."
prog="${ORCAT_TEST_TARGET:-"$base/orcat"}"
echo "# test target = $prog"

print()
{
    printf '%s' "$@"
}

declare -i n=0
check_diff()
{
    n+=1
    diff=$(diff -u <(print "$xout") <(print "$out")) || true
    if [ -z "$diff" ]
    then
        echo "ok $n $check"
    else
        sed -e 's/^/# /' <<< "$diff"
        echo "not ok $n $check"
    fi
    unset check out xout
}

check='basics'
out=$("$prog" <(print FOO) <(printf '\0\0bar'))
xout='FOoar'
check_diff

check='resilence to short reads'
out=$("$prog" <(print foo; sleep 1; print bar))
xout='foobar'
check_diff

# vim:ts=4 sts=4 sw=4 et ft=sh
