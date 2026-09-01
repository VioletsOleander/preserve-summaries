# Preserve Summaries

A very simple utility to help generate `CHANGELOG.md` together with `git-cliff`.

> Notice that this program needs to read existing `CHANGELOG.md`. An observed situation is that
> `git-cliff | preserve-summaries | save -f CHANGELOG.md` will not work in nushell. The reason is
> supposed to be that the `save` command will firstly clear the file before starting waiting for
> stdin. Therefore a suspected sequential execution does not actually happen, and
> `preserve-summaries` will read an empty file instead of the desired `CHANGELOG.md`. In this
> situation, a temporary intermediate file should be used.
