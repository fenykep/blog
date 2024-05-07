# supersimple static site generator

basically just a convenience wrapper for pulldown_cmark

first build with 
```
cargo build --release
```
then use the binary at /target/release/ with at least a markdown file as argument 

For some reason on Linux you have to run the binary as root, till I find out the reason you have to run blog_back with sudo, then chmod $username the created html file
