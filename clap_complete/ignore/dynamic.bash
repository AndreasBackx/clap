_clap_complete_dynamic() {
    # COMPREPLY=(A "B (Description)" C D)
    COMPREPLY=($COMP_CWORD $COMP_TYPE "\"$COMP_WORDS\"")
}
complete -F _clap_complete_dynamic dynamic

# _clap_complete_dynamic() {
#     local IFS=$'\013'
#     local SUPPRESS_SPACE=0
#     if compopt +o nospace 2>/dev/null; then
#         SUPPRESS_SPACE=1
#     fi
#     if [[ ${SUPPRESS_SPACE} == 1 ]]; then
#         SPACE_ARG="--no-space"
#     else
#         SPACE_ARG="--space"
#     fi
#     COMPREPLY=($COMP_CWORD $COMP_TYPE "\"$COMP_WORDS\"")
#     # COMPREPLY=($("dynamic" complete bash --index ${COMP_CWORD} --type ${COMP_TYPE} ${SPACE_ARG} --ifs="$IFS" -- "${COMP_WORDS[@]}"))
#     if [[ $? != 0 ]]; then
#         unset COMPREPLY
#     elif [[ $SUPPRESS_SPACE == 1 ]] && [[ "${COMPREPLY-}" =~ [=/:]$ ]]; then
#         compopt -o nospace
#     fi
# }
# complete -F _clap_complete_dynamic dynamic
