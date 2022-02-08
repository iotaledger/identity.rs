#!/usr/bin/env bash

#https://git-scm.com/docs/git-diff#Documentation/git-diff.txt-emgitdiffemltoptionsgt--ltpathgt82308203
if [[ $(git diff HEAD^ HEAD -- $1) != '' ]]; then
    # modified
    echo 'true'
else
    # unmodified
    echo 'false'
fi