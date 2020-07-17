# passgen

```bash
$ passgen help
passgen 1.0.0
Password genrator (random vs diceware)

USAGE:
    passgen [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -d, --digit      Using digits in passwords
    -h, --help       Prints help information
    -s, --symbols    Using symbols in passwords
    -V, --version    Prints version information

OPTIONS:
    -l, --length <LENGTH>    Passwords length [default: 24]
    -n, --number <NUMBER>    Number of passwords [default: 7]

SUBCOMMANDS:
    diceware    Using diceware for passwords
    help        Prints this message or the help of the given subcommand(s)
```

Examples:

```bash
$ passgen
GvYTkZhGZwfdpiJKtesbAqvA
nIXrsMkmFovNVmwlXGdQoBys
bstqadOpUIZypggvXCEAcBYs
LvrCxmkzoqVehuvVgLlNdBvc
AIjxGnTMYChZKdbjdJKLDzEg
zbvipDjWycpefdFYkEPvRwQv
IJLWSlzEfBDhrnuCOfpfsiAa
```

```bash
$ passgen diceware -e
unwieldy-occupier-doormat-disabled-track-maverick-saturday
hundredth-pantry-greedy-footing-mutable-commerce-uneven
contend-april-immersion-defog-hardener-compactly-thermal
tricolor-surprise-subarctic-astride-hush-cuddle-disband
repossess-lily-skintight-detached-unwieldy-coveted-quill
chump-turmoil-repent-dance-frostbite-baffling-serve
closure-camping-frenzied-daydream-reflex-chute-demeaning
```