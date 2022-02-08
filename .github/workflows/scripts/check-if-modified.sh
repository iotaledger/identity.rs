if [[ $(git diff HEAD^ HEAD -- $1) != '' ]]; then
    # modified
    echo 0
else
    # unmodified
    echo 1
fi