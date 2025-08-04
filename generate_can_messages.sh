#!/bin/bash

DBC="{$1}"
MESSAGE="./src/gui/"

if [[ -n "$1" ]]; then
    DBC="$1"
fi

dbc-codegen $DBC $MESSAGE
