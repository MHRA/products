#!/usr/bin/env bash

set -e

istioctl verify-install
istioctl manifest apply -f ../../../../infrastructure/environments/development/control-plane.yml
