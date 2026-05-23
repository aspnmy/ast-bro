// Proxy binary shipped with the final ast-outline 2.1.1 release so users can
// start invoking the short `sb` alias immediately. Execs into the sibling
// `ast-outline` (or falls back to PATH).
fn main() {
    use std::env::args;
    use std::io::ErrorKind;
    use std::process::{Command, Stdio};

    // Try same directory first (e.g. ./target/release/ast-outline), then fall back to PATH.
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join(format!("ast-outline{}", std::env::consts::EXE_SUFFIX))));
    let program = exe_dir
        .filter(|p| p.exists())
        .unwrap_or_else(|| "ast-outline".into());

    let res = Command::new(&program)
        .args(args().skip(1))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();

    match res {
        Ok(mut child) => match child.wait() {
            Ok(status) => std::process::exit(status.code().unwrap_or(1)),
            Err(e) => {
                eprintln!("error: failed to wait on 'ast-outline': {}", e);
                std::process::exit(1);
            }
        },
        Err(e) if e.kind() == ErrorKind::NotFound => {
            eprintln!("error: 'ast-outline' binary not found.");
            eprintln!("");
            eprintln!("This 'sb' is a preview shim shipped with the final ast-outline 2.1.1 release.");
            eprintln!("For the real ast-bro toolkit (renamed from ast-outline), install:");
            eprintln!("  - cargo install ast-bro");
            eprintln!("  - npm install -g @ast-bro/cli");
            eprintln!("  - pip install ast-bro");
            eprintln!("  - brew install aeroxy/tap/ast-bro");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("error: failed to execute 'ast-outline': {}", e);
            std::process::exit(1);
        }
    }
}
