#!/usr/bin/env bash

# Exit on error.
set -e

# Find last tag before current commit, or the root commit if there isn't any other tag.
PREVIOUS_TAG=$(git describe --tags --match "v*" --abbrev=0 @^ 2>/dev/null || git rev-list --max-parents=0 HEAD)

# List log subjects of commits between last tag and current commit.
LOG=$(git log --reverse --pretty=format:'%s' "${PREVIOUS_TAG}"..@)

# Find all new features and fixes according to conventional commits.
FEATS=$(echo "${LOG}" | grep -e "^ *feat:" | sed 's/[^:]*: *//')
FIXES=$(echo "${LOG}" | grep -e "^ *fix:" | sed 's/[^:]*: *//')

# Print the release notes.
if [ -n "${FEATS}" ]; then
  echo "## :sparkles: New Features"
  echo
  echo "${FEATS}" | while read -r line; do
    echo "- ${line}"
  done
fi
if [ -n "${FIXES}" ]; then
  if [ -n "${FEATS}" ]; then
    echo
  fi
  echo "## :bug: Bug Fixes"
  echo
  echo "${FIXES}" | while read -r line; do
    echo "- ${line}"
  done
fi
