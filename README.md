# ANGMAR (beta)
Angmar is a tool for bruteforcing, path discovering. The tool is based on tools like `Gobuster`, `dirb`.

The new thing about the tool is it is writteon `Rust` programming language which provides speed.

Currently tool is only sending `GET` request for fuzzing.

More will come, tune in

Happy Hacking!

## How to use
`angmar --help` will print help menu
```
Usage:
  ./angmar [OPTIONS]

Angmar 0.1 (https://github.com/naiame-neltheri/angmar)

Optional arguments:
  -h,--help             Show this help message and exit
  -v,--verbose          Verbose output
  -u,--url URL          URL to scrape
  -w,--wordlist WORDLIST
                        Wordlist to use
  -t,--thread THREAD    Number of threads, default 0
```
