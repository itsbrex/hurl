#!/bin/bash
set -Eeuo pipefail

hurl --parallel tests_ok/stdout/stdout.hurl
