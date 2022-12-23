#!/usr/bin/env sh

# abort on errors
set -e

# clean-up any previous builds
rm -fr target/dist

# build static files
pushd sage-yew
trunk build --dist ../target/dist --release
popd

# navigate into the build ouptput dir
pushd target/dist
git init
git add -A
git commit -m "gh-pages"

# force push to the "publishing source" of GitHub pages
# in this case, the gh-pages branch
# git push -f git@github.com:ttdonovan/sage-theory-crafting.git main:gh-pages

# back to previous directory
popd