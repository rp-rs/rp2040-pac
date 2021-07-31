#!/usr/bin/env bash
#set -x
set -e

# This enables easier doc consumption for humans. Sorts structs alphanumerically
# This script should only be used to alphabetize structs with #[doc] tags, for example
# struct Peripherals {
#    #[doc = "ADC"]
#    pub ADC: ADC,
#    #[doc = "BUSCTRL"]
#    pub BUSCTRL: BUSCTRL,
#    ...
# }
# Struct fields without these tags can be sorted much more trivially.

# TODO: Simple: Add target files as well if we need to sort within multiple files.

# This array should be populated with enough characters to locate a struct that needs to have its fields alphabetized.
alphaTargets=('struct Peripherals')

for ((i = 0; i < ${#alphaTargets[@]}; i++)); do

  # File line count
  maxLen=$(cat src/lib.rs | wc -l)

  # This will find the line number before the starting line of the block to replace
  blockStart=$(cat src/lib.rs | grep "${alphaTargets[$i]}" -m 1 -n | cut -d ":" -f1)

  # This will find the line number after the ending line of the block to replace
  blockEnd=$(cat src/lib.rs | grep "${alphaTargets[$i]}" -A $maxLen | grep -m 1 -n "}" | cut -d ":" -f1)
  blockEndLine=$(($blockEnd - 2)) # used for grep display count after match
  blockEnd=$((blockEndLine + blockStart)) # used for tail

  # Calculate the tail number needed to crop to blockEnd
  blockTail=$((maxLen-blockEnd))

  # This will replace the parts that need to be sorted.
  toReplace=$(cat src/lib.rs | grep "${alphaTargets[$i]}" -A $blockEndLine | tail -n $blockEndLine)
  if [ "$(uname)" == "Darwin" ]; then
    alphabetized=$(echo "$toReplace" | sed '$!N;s/\n/ /' | sort | sed 's/     /\n    /')
  else
    alphabetized=$(echo "$toReplace" | sed '$!N;s/\n/ /' | sort | sed -E 's/\s{5}/\n    /')
  fi

  # Grab the parts that we aren't sorting
  libSrcHead=$(cat src/lib.rs | head -n $blockStart)
  libSrcTail=$(cat src/lib.rs | tail -n $blockTail)

  # Write out the sorted file
  echo "$libSrcHead" > src/lib.rs
  echo "$alphabetized" >> src/lib.rs
  echo "$libSrcTail" >> src/lib.rs

done
