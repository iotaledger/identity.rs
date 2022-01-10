RELEASES_FOLDER=.github/releases/
FILES="$RELEASES_FOLDER*"
for file in $(eval echo $FILES)
do
  if [ -f "$file" ]
  then
    echo "Processing $file"
    filename=$(basename -- "$file")
    filename="${filename%.*}"
    filecontent=$(cat $file)
    replaced_content=$(echo $filecontent | sed '$!s/$/\\n/' | tr -d '\n')
    sed -i "\@\.\.\.$filename@a\ \n$replaced_content" CHANGELOG.md
  else
    echo "Warning: Could not find \"$file\""
  fi
done