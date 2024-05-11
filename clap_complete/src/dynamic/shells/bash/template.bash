_clap_complete_NAME() {
    export IFS=$'\013'
    local SUPPRESS_SPACE=0
    if compopt +o nospace 2>/dev/null; then
        SUPPRESS_SPACE=1
    fi
    if [[ ${SUPPRESS_SPACE} == 1 ]]; then
        SPACE_ARG="--no-space"
    else
        SPACE_ARG="--space"
    fi

    mapfile -t COMPREPLY < <(
        "COMPLETER" complete bash --index "${COMP_CWORD}" --type "${COMP_TYPE}" ${SPACE_ARG} -- "${COMP_WORDS[@]}"
    )
    if [[ $? != 0 ]]; then
        unset COMPREPLY
    elif [[ $SUPPRESS_SPACE == 1 ]] && [[ "${COMPREPLY-}" =~ [=/:]$ ]]; then
        compopt -o nospace
    fi
}
complete OPTIONS -F _clap_complete_NAME NAME
