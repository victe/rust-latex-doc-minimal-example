# Minimal Rust project example for getting Latex in docs

This approach use a `HTML` header loading a script in the final `HTML` doc pages. You need to set environmental variable `RUSTDOCFLAGS` with paramenter `html-in-header` to a fragment of `HTML` file with the script that load and configure `Katex`.

The `html` file is something like this

```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.10.0/dist/katex.min.css" integrity="sha384-9eLZqc9ds8eNjO3TmqPeYcDj8n+Qfa4nuSiGYa6DjLNcv9BtN69ZIulL9+8CqC9Y" crossorigin="anonymous">
<script src="https://cdn.jsdelivr.net/npm/katex@0.10.0/dist/katex.min.js"                  integrity="sha384-K3vbOmF2BtaVai+Qk37uypf7VrgBubhQreNQe9aGsz9lB63dIFiQVlJbr92dw2Lx" crossorigin="anonymous"></script>
<script src="https://cdn.jsdelivr.net/npm/katex@0.10.0/dist/contrib/auto-render.min.js"    integrity="sha384-kmZOZB5ObwgQnS/DuDg6TScgOiWWBiVt0plIRkZCmE6rDZGrEOQeHM5PcHi+nyqe" crossorigin="anonymous"></script>
<script>
    document.addEventListener("DOMContentLoaded", function() {
        renderMathInElement(document.body, {
            delimiters: [
                {left: "$$", right: "$$", display: true},
                {left: "\\(", right: "\\)", display: false},
                {left: "$", right: "$", display: false},
                {left: "\\[", right: "\\]", display: true}
            ]
        });
    });
</script>
```

In this minimal example repo is in `src/docs-header.html`.

And you put `Latex` in your docs like

```rust
/// $$E = mc^2 $$
/// $$m = \frac{m_0}{\sqrt{1-\frac{v^2}{c^2}}}$$
fn energy(v: Velocity) -> Energy {...}

```

## Compiling the docs manually (without `.cargo/config`):

### NIX kind

```sh
RUSTDOCFLAGS="--html-in-header src/docs-header.html" cargo doc --no-deps --open
```

### Windows

#### cmd

```
set RUSTDOCFLAGS=--html-in-header src\docs-header.html
cargo doc --no-deps --open
```

#### PowerShell

```
$env:RUSTDOCFLAGS="--html-in-header .\src\docs-header.html"
cargo doc --no-deps --open
```

## Compile and copy to docs

Because the solution could not work in `docs.rs`, you can publish the docs on your own host. 
This minimal example is hosted with [github pages](https://pages.github.com/). You can see the docs generated, with `LaTeX` at
[minimal example docs with LaTex](https://victe.github.io/rust-latex-doc-minimal-example/rust_latex_doc_minimal_example/)

```sh
cargo doc -- && cp -r target/doc/ docs/
```
