#!/bin/bash

DBC="{$1}"
MESSAGE="./src"

if [[ -n "$1" ]]; then
    DBC="$1"
fi

dbc-codegen $DBC $MESSAGE
