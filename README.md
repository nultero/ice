# ice

I often dump something like `echo 'some thing/task/idea' | lolcat` at the end of my shell rc files,
it works surprisingly well as a bulletin board and it's not hard to edit quickly.

But `lolcat`'s being written in Ruby means it's very slow to individually start and
parse several lines or files if I've `$()`'d some things. This clogs up my terminal start / refresh.

So this is `ice`, which does something similar but is limited to blue colors because I like blue. 
There's virtually no startup time with this ~~Golang~~ Rust binary compared to a Ruby script.
