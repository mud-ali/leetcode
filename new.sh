#!/bin/zsh

read "dir_name?Enter problem name: "
sanitized_name=$(echo "$dir_name" | tr '[:upper:]' '[:lower:]' | tr ' ' '-' | tr -d '.' )
mkdir "$sanitized_name"

read "lang?Choose a language (js, ts, py, cpp, java, c): "
cd "$sanitized_name"
touch "main.$lang"
# cp ../templates/build.$lang.sh ./build.sh
# chmod +x build.sh
echo "# $dir_name \n" > README.md