# Minimal Rust project example for using Latex in docs

Documenting code in `Rust` is straightforward and appealing, but you can not write mathematical expressions.

The solution is to include in the final HTML doc page a script to parse the mathematical expressions when rendering the page.

We use Latex for the mathematical syntax and Katex for parsing the expressions in the final HTML file. The [parser script](./src/docs-header.html) get `Katex` from CDN and configure escape characters to use in documentation code.

Now you can do this:

```rust
/// $$E = mc^2 $$
/// $$m = \frac{m_0}{\sqrt{1-\frac{v^2}{c^2}}}$$
fn energy(v: Velocity) -> Energy {...}
```

As a result, see the documentation page of this repo example at [docs.rs](https://docs.rs/rust-latex-doc-minimal-example/).

## Including the parser script in the final doc

`rustdoc` has [command-line arguments](https://doc.rust-lang.org/rustdoc/command-line-arguments.html#--html-in-header-include-more-html-in-head) for including HTML in the header, before the content and after the content. We use `--html-in-header` because we need to link to the CDN for getting `Katex`.

For passing `--html-in-header` to `cargo doc` command there are two options:

1) Using the environmental variable RUSTDOCFLAGS ([see cargo rustdoc](https://doc.rust-lang.org/cargo/commands/cargo-rustdoc.html))

Linux

```sh
RUSTDOCFLAGS="--html-in-header src/docs-header.html" cargo doc
```

Windows cmd

```bat
set RUSTDOCFLAGS=--html-in-header src\docs-header.html
cargo doc
```

Windows PowerShell

```bat
$env:RUSTDOCFLAGS="--html-in-header .\src\docs-header.html"
cargo doc
```

2) Setting the `build.rustdocflags` config value in `.cargo\config.toml` (in this Repo is configured)

```toml
[build]
rustdocflags = [ "--html-in-header", "./src/docs-header.html" ]
```

## Publish the crate and showing documentation in [docs.rs](https://docs.rs)

The crate must be a `lib`. If it is only a `bin`, [docs.rs](https://docs.rs) do not build the documentation.

We need to add the header file ubication in the cargo manifest:
 
```toml
[package.metadata.docs.rs]
rustdoc-args = [ "--html-in-header", "./src/docs-header.html" ]
```

## Publish the doc in your hosting

If your crate is a `bin` or you don't want to publish your crate at [crates.io](https://crates.io), you can always publish the `target/doc` directory at your convenience.

In this repo, we use [github pages](https://pages.github.com/) in the `docs` directory. In this case, just use:

```sh
rm -rf docs/ && cargo clean --doc && cargo doc && cp -r target/doc/ docs/
```

If you publish your crate to [crates.io](https://crates.io) set the `documentation` entry in the cargo manifest to your final URL destination. In this crate, this entry should look like this:

```toml
# Cargo.toml
[package]
...
documentation = "https://victe.github.io/rust-latex-doc-minimal-example/rust_latex_doc_minimal_example/"
...
```

