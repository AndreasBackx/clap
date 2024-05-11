complete -x -c {bin} -a "("'COMPLETER'" complete fish -- (commandline --current-process --tokenize --cut-at-cursor) (commandline --current-token))"
