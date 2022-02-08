#!/usr/bin/env bash

if [[ $(git diff HEAD^ HEAD -- $1) != '' ]]; then
    # modified
    echo 'true'
else
    # unmodified
    echo 'false'
fi