#!/bin/zsh

if [ $# -lt 1 ]; then
    echo "Usage: $0 '<new_member_name>'"
    exit 1
fi

MEMBER_NAME=$1
CONFIG_FILE='Cargo.toml'

cargo generate --path template --name $MEMBER_NAME

sed -i.bak "/^[[:space:]]*# MEMBER_LAST_MARKER/i\\
    \"$MEMBER_NAME\",
" $CONFIG_FILE && rm -f "$CONFIG_FILE.bak"
