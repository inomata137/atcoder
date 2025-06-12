#!/bin/zsh

# takes exactly one argument
if [ "$#" -ne 1 ]; then
  echo "Usage: $0 <project_name>"
  exit 1
fi

PROJECT_NAME="$1"

# generate a project
cargo generate --path template --name "$PROJECT_NAME"

# add to cargo workspace members
if sed --version >/dev/null 2>&1; then
  # GNU sed
  sed -i "/# MEMBER_LAST_MARKER/i\    \"$PROJECT_NAME\"," Cargo.toml
else
  # BSD sed
  sed -i '' "/# MEMBER_LAST_MARKER/i\\
    \"$PROJECT_NAME\",
    # MEMBER_LAST_MARKER/" Cargo.toml
fi
