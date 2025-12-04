#!/bin/zsh

read "dir_name?Enter problem name: "
sanitized_name=$(echo "$dir_name" | tr '[:upper:]' '[:lower:]' | tr ' ' '-' | tr -d '.' )
mkdir "$sanitized_name"

read "lang?Choose a language by entering the appropriate file extension: "
cd "$sanitized_name"
touch "main.$lang"

autoload -Uz zmv
zmv '([0-9]#)-(*)' '${(l:4::0:)1}-${2}'