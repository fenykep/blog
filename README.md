# supersimple static site generator

Basically just a convenience wrapper for [pulldown_cmark](https://pulldown-cmark.github.io/pulldown-cmark/)

First build with
```
cargo build --release
```
Then use the binary at /target/release/ with at least a markdown file as argument.

For some reason on Linux you have to run the binary as root, till I find out the reason you have to `sudo ./blog_back markdownFileToRender.md`, then `chmod $username` the created html file.
