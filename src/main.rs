fn main() {
    println!("Hello, Latex in docs!");
    println!(
        r#"Set environmental variable RUSTDOCFLAGS to "--html-in-header src/docs-header.html" ."#
    );
    println!("if you cloned the repo, then it is not necessary because you already have this configured in .cargo/config .");
    println!("And execute:");
    println!("cargo doc --open.")
}
