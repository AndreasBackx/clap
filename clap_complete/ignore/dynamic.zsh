#compdef dynamic

_something() {
    echo "group 1"
    echo "option 1"
    echo "display 1"

    echo ""

    echo "group 2"
    echo "option 2"
    echo "display 2"
    echo "option 3"
    echo "YAY"
}

_clap_complete_dynamic() {
    local items
    items=("${(@f)$(_something)}")


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

    # _arguments \
    #     '-a[Enable option A]' \
    #     '--bee[Enable option BEE]' \
    #     '--bea[Enable option BEA]' \
    #     '--baa[Enable option BAA]' \
    #     '-b[Enable option B]' \
    #     '-f[file]:File to process:_files'

    # local -a options
    # options=(
    #     'option1:Description for option 1'
    #     'o1:Description for option 1'
    #     'option2:Description for option 2'
    #     'option3:Description for option 3'
    # )
    # _describe 'option' options

    # descriptions=(A " B" C D)
    # compadd -ld descriptions -J groupnameA -o nosort -E 1 -- a " b" c d
    # compadd -ld descriptions -J groupnameA -X explanationA -o nosort -- "\033[0;31mRed\033[0m White" b c d
    # compadd -ld descriptions -J groupnameB -X explanationB -C -o nosort -- a b c d

    # compadd -E 1

    # descriptions=(X Y Z)
    # compadd -d descriptions -J groupnameZ -X explanationZ -o nosort -- x y z

    # _arguments -C \
    #     '(-)'{-h,--help}'[Show help]' \
    #     '(-u --update -p --platform)'{-u,--update}'[Update]'

    # local -a subcmds topics
    # subcmds=('c:description for c command' 'd:description for d command')
    # topics=('e:description for e help topic' 'f:description for f help topic')
    # _describe 'command' subcmds -- topics

    # IFS="\n"
    # for a in $(echo "a\nb c"); do
    #     echo $a
    # done
    # while IFS= read -r line; do
    #     echo "tester: $line"
    # done < "dynamic" complete zsh --index ${CURRENT} -- "$words"
    # compadd_args=($("dynamic" complete zsh --index ${CURRENT} -- "$words"))
}

compdef _clap_complete_dynamic dynamic
