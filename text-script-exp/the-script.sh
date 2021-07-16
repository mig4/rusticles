#!/bin/bash

set -euo pipefail

awk -f the-script.awk -- tests/resources/prometheus.resource-capacity.util.txt \
  | grep -v -E '^null\b' \
  | column -t \
  > tests/resources/prometheus.resource-capacity.old-new-comparison.txt
