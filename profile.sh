#!/bin/bash

OUTPUT_PATH="profile"
TEST=0
TOOL="grcov"
FORMAT="html"
OUTPUT_FILE=$FORMAT
RUN_OPTIONS=""
CLEAN=0

while [[ $# -gt 0 ]]; do
    key="$1"
    case $key in
        -o|--output)
        OUTPUT_PATH="$2"
        shift # past argument
        shift # past value
        ;;
        -t|--tool)
        TOOL="$2"
        shift # past argument
        shift # past value
        ;;
        -f|--format)
        FORMAT="$2"
        shift # past argument
        shift # past value
        ;;
        -c|--clean)
        CLEAN=1;
        shift
        ;;
        -e|--test)
        TEST=1;
        shift
        ;;
        --)    # unknown option
        shift # past argument
        while [[ $# -gt 0 ]]; do
            RUN_OPTIONS+="$1 " # save it in an array for later
            shift # past value
        done
        ;;
        *)
        echo "Invalid option: $1"
        exit 1
        ;;
    esac
done

PROFRAW_FILE=$OUTPUT_PATH/cargo-test-%p-%m.profraw
PROFDATA_FILE=$OUTPUT_PATH/cargo-test-%p-%m.profdata

export LLVM_PROFILE_FILE="$PROFRAW_FILE"
export RUSTFLAGS="-C instrument-coverage"
export CARGO_INCREMENTAL=0

rm -rf $OUTPUT_PATH

if [ $CLEAN -eq 1 ]; then
    cargo clean
fi

if [[ "$TOOL" == "grcov" ]]; then
    if [ $TEST -eq 1 ]; then
        cargo test
    else
        cargo run -- $RUN_OPTIONS
    fi

    if [[ "$FORMAT" == "lcov" ]]; then
        OUTPUT_FILE=lcov.info
    elif [[ "$FORMAT" == "cobertura" ]]; then
        OUTPUT_FILE=cobertura.info
    elif [[ "$FORMAT" == "html" ]]; then
        OUTPUT_FILE=html
    else
        echo "Invalid output format: $FORMAT"
        exit 1
    fi

    grcov $OUTPUT_PATH --binary-path target/debug/deps/ -s . -t $FORMAT --branch --ignore-not-existing --ignore '../*' --ignore "/*" --ignore "target/*" -o $OUTPUT_PATH/$OUTPUT_FILE
else
    echo "Invalid type: $TOOL"
    exit 1
fi