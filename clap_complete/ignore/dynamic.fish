function get_dynamic_completions
    echo --bar\t'FOO BAR'
    echo --baz\t'BAZ INFO'
    echo --biz\t'Very long text that will not fit on a single line and if it does then that is a big problem but it will not so do not worry about it. Very long text that will not fit on a single line and if it does then that is a big problem but it will not so do not worry about it. Very long text that will not fit on a single line and if it does then that is a big problem but it will not so do not worry about it. Very long text that will not fit on a single line and if it does then that is a big problem but it will not so do not worry about it. Very long text that will not fit on a single line and if it does then that is a big problem but it will not so do not worry about it. '
end

complete -c dynamic -a '(get_dynamic_completions)' -f
