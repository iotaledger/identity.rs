# joins an array with a delimiter
joinBy() {
    local IFS="$1"; shift; echo "$*";
}

"$@"