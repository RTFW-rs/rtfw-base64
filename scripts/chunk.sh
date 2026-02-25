#!/usr/bin/env bash

baseA=$1
baseB=$2
if [[ -z "$baseA" || -z "$baseB" ]]; then
    echo "expects two arguments: baseA baseB"
    echo "example: ./chunk.sh 256 64"
    exit 1
fi

# Calculate the chunk size needed for base conversion
chunk_size=$(qalc --terse "lcm(log2($baseA),log2($baseB))")
echo "Chunk size (bits): $chunk_size"
echo "Chunk size (bytes): $((chunk_size / 8))"
