#!/bin/bash

RELEASE=0
DEBUG=1

APP=parser
PACKAGE_NAME="yamp"
TARGET=
BUILD_OPTIONS="--quiet"

while [[ $# -gt 0 ]]; do
    key="$1"
    case $key in
        -r|--release)
        RELEASE=1
        DEBUG=0
        shift # past argument
        ;;
        -t|--target)
        TARGET="$2"
        shift # past argument
        shift # past value
        ;;
        *)
        echo "Invalid argument: $1"
        exit 1
        ;;
    esac
done

if [ -z $TARGET ]; then
    echo "Please specify target!"
    exit 1
fi

TAG=$(git describe --tags --exact-match $COMMIT 2> /dev/null | grep -Eo '[[:digit:]]+\.[[:digit:]]+\.[[:digit:]]+')
if [ -z $TAG ]; then
    echo "Latest commit does not contain a tag!"
else
    echo "git tag: $TAG"
    if [ "$TAG" != "$VERSION" ]; then
        echo "Version does not match commit tag! $VERSION vs $TAG"
        exit 1
    fi
fi

if [ $RELEASE -eq 1 ]; then
    BUILD_OPTIONS="$BUILD_OPTIONS --release"
fi
BUILD_OPTIONS="$BUILD_OPTIONS --target=$TARGET"

echo "Target: $TARGET"

echo "Cleaning..."
# cargo clean --quiet

echo "Building... "
echo "Build options: $BUILD_OPTIONS"
cargo build $BUILD_OPTIONS
if [ $? -ne 0 ]; then
    exit 1
fi

TARGET_DIR=target/$TARGET
if [ $RELEASE -eq 1 ]; then
    BUILD_DIR=$TARGET_DIR/release
elif [ $DEBUG -eq 1 ]; then
    BUILD_DIR=$TARGET_DIR/debug
fi

VERSION=$($BUILD_DIR/$APP --version | grep -Eo '[[:digit:]]+\.[[:digit:]]+\.[[:digit:]]+')
echo "Application: $APP"
echo "Version: $VERSION"

COMMIT=$(git log -n 1 --pretty=format:"%H")
echo "Last commit: $COMMIT"

# Add version
PACKAGE_NAME="$PACKAGE_NAME-$VERSION"

# Add target
if [ $TARGET == "x86_64-unknown-linux-gnu" ]; then
    PACKAGE_NAME="$PACKAGE_NAME-x86_64-linux-gnu"
elif [ $TARGET == "x86_64-pc-windows-gnu" ]; then
    PACKAGE_NAME="$PACKAGE_NAME-x86_64-windows-gnu"
fi

# does it have git tag ?            no => +dev
# is it debug ?                    yes => +debug
# files changed and not commited ? yes => +dirty

# If build from debug, add -debug
if [ $DEBUG -eq 1 ]; then
    PACKAGE_NAME="$PACKAGE_NAME-debug"
fi

# If git tag is mssing, add -dev
if [ -z $TAG ]; then
    PACKAGE_NAME="$PACKAGE_NAME-dev"
fi

# Check if there are changed files (skip untracked), add -dirty
if [[ `git status --porcelain --untracked-files=no` ]]; then
    echo "There are files with changes not committed yet!"
    PACKAGE_NAME="$PACKAGE_NAME-dirty"
fi

rm -rf $TARGET_DIR/package artifacts
mkdir $TARGET_DIR/package artifacts
cp CHANGELOG.md $TARGET_DIR/package
cp $BUILD_DIR/$APP $TARGET_DIR/package

echo "Generating: $PACKAGE_NAME.tar.gz..."
tar -czf artifacts/$PACKAGE_NAME.tar.gz -C $TARGET_DIR/package CHANGELOG.md $APP

echo "Generating: $PACKAGE_NAME.zip..."
zip -rjq artifacts/$PACKAGE_NAME.zip $TARGET_DIR/package/CHANGELOG.md $TARGET_DIR/package/$APP