/// Example inline with text $E = mc^2$.
/// Example in separate line:
///   $$m = \frac{m_0}{\sqrt{1-\frac{v^2}{c^2}}}$$
/// Not all valid $\LaTeX$ is allowed. See [Katex](https://katex.org).
fn main() {
    println!("Hello, Latex in docs!");
    println!(r#"Set environmental variable RUSTDOCFLAGS to "--html-in-header src/docs-header.html""#);
    println!("And execute");
    println!("cargo doc --no-deps --open.")
}
