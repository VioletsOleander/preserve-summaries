# Preserve Summaries

A very simple utility to help generate `CHANGELOG.md` together with `git-cliff`.

Notice that this program needs to read existing `CHANGELOG.md`, therefore `git-cliff | preserve-summaries | save -f
CHANGELOG.md` will not work because shell pipeline is designed for streaming operation and does not guarantee sequential
execution. Therefore such a command will encounter race condition, and very likely `preserve-summaries` will not read
what it want to read. （The actual  reason might be that Nushell save command will firstly clear the file then start waiting for stdin.）

Utilize a temporary intermediate file to avoid race condition.
