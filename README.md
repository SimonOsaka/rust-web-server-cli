## rust-web-server-cli
Generate rust web server code via mustache template [rust-web-server-mustache](https://github.com/SimonOsaka/rust-web-server-mustache).

[rust-web-server-cli](https://github.com/SimonOsaka/rust-web-server-cli) + [rust-web-server-mustache](https://github.com/SimonOsaka/rust-web-server-mustache) = [rust-web-server-example](https://github.com/SimonOsaka/rust-web-server-example)

### How to use

1. `git clone https://github.com/SimonOsaka/rust-web-server-cli.git`
2. `git clone https://github.com/SimonOsaka/rust-web-server-mustache.git`
3. `cd rust-web-server-cli`
4. `vi mustache.config.toml`
5. modify mustache path and example path.
```toml
mustache_path = "<input 'rust-web-server-mustache' path>"

example_path = "<input path to generate>"
```
6. Run command `cargo run`, code will be generate in example_path.