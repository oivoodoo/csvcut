#!/bin/bash

if [ "$#" -lt 2 ]; then
    echo "Usage: $0 <input_csv> <columns_to_remove>"
    exit 1
fi

INPUT_FILE="$1"
COLUMNS_TO_REMOVE="$2"

if [ ! -f "$INPUT_FILE" ]; then
    echo "Error: Input file '$INPUT_FILE' does not exist."
    exit 1
fi

docker run --rm -v "$(pwd):/data" csvcut "/data/$(basename "$INPUT_FILE")" "$COLUMNS_TO_REMOVE" > "$(dirname "$INPUT_FILE")/output_$(basename "$INPUT_FILE")"

echo "Output saved to: $(dirname "$INPUT_FILE")/output_$(basename "$INPUT_FILE")"