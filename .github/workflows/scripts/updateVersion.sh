#!/bin/bash

# takes a string and increments the last number
# e.g. $(updateVersion 1.1.0) -> 1.1.1
# e.g. $(updateVersion 1.1.0-dev.1) -> 1.1.0-dev.2 

[[ ${1} =~ ^(.*[^0-9])?([0-9]+)$ ]]  && \
    [[ ${#BASH_REMATCH[1]} -gt 0 ]] && \
    printf "%s%0${#BASH_REMATCH[2]}d" "${BASH_REMATCH[1]}" "$((10#${BASH_REMATCH[2]} + 1 ))" || \
    printf "%0${#BASH_REMATCH[2]}d" "$((10#${BASH_REMATCH[2]} + 1))" || \
    printf "${1}"
