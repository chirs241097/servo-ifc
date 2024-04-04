#!/bin/bash                                                                                  
set -euo pipefail

SERVO_DIR="servo.nosync"
readonly SERVO_DIR

SERVO_BIN="servo.nosync/target/release/servo"
readonly SERVO_BIN

SERVO_IFC_DIR="servo-ifc.nosync"
readonly SERVO_IFC_DIR

SERVO_IFC_BIN="servo-ifc.nosync/target/release/servo"
readonly SERVO_IFC_BIN

RESULT_DIR="dom_results"
readonly RESULT_DIR

#Lines of code for lines change                                                              
#   - Make temp directory, move unmodified version into it                                   
#   - replace unmodified version with modified version                                       
#   - Use git diff to report changes                                                         
#   - Restore current implementation from temp directory                                     
#                                                                                            
# Args:                                                                                      
#  $1 - unmodified directory                                                                 
#  $2 - modified directory                                                                   
function loc() {
    local unmodified_source_directory
    unmodified_source_directory="$1"

    local modified_source_directory
    modified_source_directory="$2"

    mkdir temp
    mv "${unmodified_source_directory}/"* temp
    cp -r "${modified_source_directory}/"* "$unmodified_source_directory"
    git diff --stat -w
    rm -r "$unmodified_source_directory/"*
    mv temp/* "$unmodified_source_directory"
    rm -r temp
}

# Evaluate run times                                                                         
function eval_runtime() {
    rm -f "${RESULT_DIR}/servo-no-ifc.txt" "${RESULT_DIR}/servo-ifc.txt"

    echo "Compiling unmodified servo"
    pushd "servo.nosync" >/dev/null 2>/dev/null
    echo "Shifted stack"
    #./mach build --release
    echo "Compiled unmodified servo"
    popd >/dev/null 2>/dev/null

    echo "Compiling modified servo"
    pushd "servo-ifc.nosync" >/dev/null 2>/dev/null
    #./mach build --release
    popd >/dev/null 2>/dev/null

    echo "run time" > "${RESULT_DIR}/servo-no-ifc.txt"
    echo "run time" > "${RESULT_DIR}/servo-ifc.txt"


    echo "running tests"
    file="servo-ifc.nosync/dom_list.txt"
    echo $file
    #domore=false
    #firstfile="tests/wpt/web-platform-tests/dom/nodes/remove-from-shadow-host-and-adopt-into-iframe-ref.html"
    while read -r line; do
	#if [[ "$line" == "$firstfile" ]]; then
	    domore=true
	#fi
	#if [ $domore == true ]; then
	    echo "$domore"
            echo "non-ifc $line"
            pushd "servo.nosync" >/dev/null 2>/dev/null
            ./target/release/servo -o junk $line
            popd >/dev/null 2>/dev/null
            echo "ifc $line"
            pushd "servo-ifc.nosync" >/dev/null 2>/dev/null
            ./target/release/servo -o junk $line
	    popd >/dev/null 2>/dev/null
	    echo ""
        #fi
    done <$file
}


eval_runtime