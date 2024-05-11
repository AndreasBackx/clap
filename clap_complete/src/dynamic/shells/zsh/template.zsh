#compdef NAME

_clap_complete_NAME() {
    # echo "\"COMPLETER\" complete bash --index \"${COMP_CWORD}\" --type \"${COMP_TYPE}\" ${SPACE_ARG} -- \"${COMP_WORDS[@]}\""

    # mapfile -t items < <(
    #     "COMPLETER" complete zsh --index "$((CURRENT - 1))" -- "${words[*]}"
    # )

    local items
    items=("${(@f)$('COMPLETER' complete zsh --index $((CURRENT - 1)) -- ${words[*]} )}")


    for ((i = 1; i <= $#items; i++)); do
        local group="${items[i]}"

        typeset -a options=()
        typeset -a descriptions=()

        for ((j = i + 1; j <= $#items; j += 2)); do
            local option="${items[j]}"
            local description="${items[j + 1]}"

            if [[ -z "$option" ]]; then
                ((i+=1))
                break
            fi

            ((i+=2))

            options+=("$option")
            descriptions+=("$description")
        done

        compadd -d descriptions -J "$group" -X "$group ($i - $j)" -o nosort -- ${options[@]}
    done
}
complete OPTIONS -F _clap_complete_NAME NAME
