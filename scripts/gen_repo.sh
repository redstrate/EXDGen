#!/bin/bash

# This script generates the Physis sheets repository.
# Given a game version, it will:
# 1. Download the relevant EXDSchema
# 2. Switch to a (new if needed) branch of Physis Sheets
# 3. Regenerate the sheets and commit it

game_version="$1"
sheets_path="$2"

# setup temp dir
mkdir temp
pushd temp

# clone the schema we requested
git clone -b "ver/$game_version" https://github.com/xivdev/EXDSchema/
popd

# prepare sheets repo
pushd $sheets_path
git switch -c "ver/$game_version"
popd

# call the tool
cargo run -- "temp/EXDSchema" "$sheets_path" "$game_version"

# commit the changes
pushd $sheets_path
git add .
git commit --author="Excel Generator <bot@ryne.moe>" -m "Regenerate sheets"
popd

# cleanup temp dir
rm -rf temp/
