## news

I just want to read my tech news on the command line.

Also a playground for async http client.

Perhaps split out by subcommand in the future.

- [x] hn
- [x] reddit /r/rust
- [ ] users.rust-lang.org
- [ ] internals.rust-lang.org
- [ ] lobste.rs

this lets me scroll, with colors:
```
$ news | less -r
```

## deps
Removing reqwest for ureq.

Dep count with reqwest (tokio and tracing): 190
Dep count with ureq and env-logger: 105
