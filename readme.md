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

In this minimal example repo is in `src/pp.html`.

And you put `Latex` in your docs like

```rust
/// $$E = mc^2 $$
/// $$m = \frac{m_0}{\sqrt{1-\frac{v^2}{c^2}}}$$

```

For compiling the docs:

## NIX kind

```sh
RUSTDOCFLAGS="--html-in-header src/pp.html" cargo doc --no-deps --open
```

## Windows

### cmd

```
set RUSTDOCFLAGS=--html-in-header src\pp.html
cargo doc --no-deps --open
```

### PowerShell

```
$env:RUSTDOCFLAGS="--html-in-header .\src\pp.html"
cargo doc --no-deps --open
```