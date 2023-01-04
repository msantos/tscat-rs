# SYNOPSIS

tscat *option* [*label*]

# DESCRIPTION

tscat: timestamp stdin to stdout/stderr

tscat timestamps standard input and writes the output to standard output,
standard error or both.

# EXAMPLES

    $ echo test | tscat
    2022/12/26 10:54:23 test

    $ echo test | tscat foo
    2022/12/26 10:54:37 foo test

    # duplicate output to stdout/stderr
    $ echo test | tscat --output=3 foo
    2022/12/26 10:55:06 foo test
    2022/12/26 10:55:06 foo test


    $ echo test | tscat --output=3 foo > /dev/null
    2022/12/26 10:55:06 foo test

    $ echo test | tscat --output=3 foo 2> /dev/null
    2022/12/26 10:55:06 foo test

# Build

    cargo build

# OPTIONS

--output *1|2|3*
: stdout=1, stderr=2, both=3 [default: 1]

--format *string*
: timestamp format (see strftime(3)) [default: %F%T%z]

-h, --help
: usage summary

# ALTERNATIVES

* [tscat](https://github.com/msantos/tscat)
