#!/bin/bash

TARGET_BRANCH=main
DRAFT=
RELEASE=0
ARTIFACTS=

while [[ $# -gt 0 ]]; do
    key="$1"
    case $key in
        -r|--release)
        RELEASE=1
        shift # past argument
        ;;
        -a|--artifacts)
        ARTIFACTS=$2
        shift # past argument
        shift # past argument
        ;;
        -b|--branch)
        TARGET_BRANCH=$2
        shift # past value
        shift # past argument
        ;;
        -d|--draft)
        DRAFT="--draft"
        shift # past argument
        ;;
        *)
        echo "Invalid argument: $1"
        exit 1
        ;;
    esac
done

# Get bin name from Cargo.toml
APP=$(cat Cargo.toml | grep name | tail -n 1 | sed 's/^[^"]*"\([^"]*\)".*/\1/')

# Get version name from Cargo.toml
VERSION=$(cat Cargo.toml | grep -m 1 version | grep -Eo '[[:digit:]]+\.[[:digit:]]+\.[[:digit:]]+')

# Get current branch name
BRANCH=$(git branch --show-current)
if [ "$BRANCH" != "$TARGET_BRANCH" ]; then
    git checkout $TARGET_BRANCH
    BRANCH=$TARGET_BRANCH
fi

# Get tag from latest commit, if any
TAG=$(git describe --tags --exact-match $COMMIT 2> /dev/null | grep -Eo '[[:digit:]]+\.[[:digit:]]+\.[[:digit:]]+')
if [ -z $TAG ]; then
    echo "Latest commit does not contain a tag!"
    exit 1
else
    if [ "$TAG" != "$VERSION" ]; then
        echo "Version does not match commit tag! $VERSION vs $TAG"
        exit 1
    fi
fi

echo "App: $APP"
echo "Branch: $BRANCH"
echo "Tag/Version: $TAG"

if [ $RELEASE -eq 1 ]; then
    gh release view $TAG > /dev/null 2>&1
    if [ $? -eq 0 ]; then
        echo "Release already created on tag: $TAG!"
        exit 1
    fi

    echo "Crating release..."
    gh release create $TAG $DRAFT --target $BRANCH --notes-file CHANGELOG.md --title "yamp $VERSION"
fi

if [ ! -z $ARTIFACTS ]; then
    gh release view $TAG > /dev/null 2>&1
    if [ $? -ne 0 ]; then
        echo "Release not created on tag: $TAG!"
        exit 1
    fi

    for file in $ARTIFACTS/*; do
        echo "Uploading files: $file"
         gh release upload $TAG $file
    done
fi

gh release view $TAG
