use std::{
    env, fs,
    path::Path,
    process::{Command, Stdio},
    thread,
};

use anyhow::{Context, Result, bail, ensure};

fn main() -> Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let target = root.join("target");

    let status = Command::new("cargo")
        .current_dir(root)
        .args([
            "build",
            "--locked",
            "--release",
            "--bin",
            "limo",
            "--target-dir",
        ])
        .arg(&target)
        .status()?;
    ensure!(status.success(), "failed to build limo ({status})");
    let limo = target
        .join("release")
        .join(format!("limo{}", env::consts::EXE_SUFFIX));

    let mut files = Vec::new();
    for entry in fs::read_dir(root.join("tests"))? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_file() {
            files.push(path);
        }
    }

    let (mut passed, mut failed) = (0, 0);
    thread::scope(|scope| {
        let handles: Vec<_> = files
            .iter()
            .map(|path| {
                let limo = &limo;
                (path, scope.spawn(move || run_file(root, limo, path)))
            })
            .collect();
        for (path, handle) in handles {
            let name = path.strip_prefix(root).unwrap_or(path).display();
            match handle.join().expect("test worker panicked") {
                Ok(0) => {}
                Ok(runs) => {
                    passed += 1;
                    println!("PASS {name} ({runs} runs)");
                }
                Err(error) => {
                    failed += 1;
                    eprintln!("FAIL {name}\n{error:#}");
                }
            }
        }
    });

    println!("{passed} files passed; {failed} failed");
    ensure!(passed + failed > 0, "no RUN directives found in tests/");
    ensure!(failed == 0, "{failed} test files failed");
    Ok(())
}

fn run_file(root: &Path, limo: &Path, path: &Path) -> Result<usize> {
    let source = fs::read_to_string(path)?;
    let file = path.to_str().context("test path is not valid UTF-8")?;
    let mut lines = source.lines().enumerate().peekable();
    let mut runs = 0;

    while let Some((line, text)) = lines.next() {
        let text = text.trim_start();
        let Some(command) = text.strip_prefix("// RUN:") else {
            ensure!(
                !text.starts_with("// STDOUT:") && !text.starts_with("// STDERR:"),
                "line {}: STDOUT and STDERR must immediately follow RUN, STDOUT or STDERR",
                line + 1
            );
            continue;
        };

        let args = shlex::split(command)
            .with_context(|| format!("line {}: invalid RUN quoting", line + 1))?;
        ensure!(!args.is_empty(), "line {}: RUN command is empty", line + 1);
        let args: Vec<_> = args.iter().map(|arg| arg.replace("%s", file)).collect();

        let mut expected_stdout = None;
        let mut expected_stderr = None;
        while let Some((_, text)) = lines.peek() {
            let text = text.trim_start();
            let (expected, output) = if let Some(output) = text.strip_prefix("// STDOUT:") {
                (&mut expected_stdout, output)
            } else if let Some(output) = text.strip_prefix("// STDERR:") {
                (&mut expected_stderr, output)
            } else {
                break;
            };
            let expected = expected.get_or_insert_with(String::new);
            expected.push_str(output.strip_prefix(' ').unwrap_or(output));
            expected.push('\n');
            lines.next();
        }

        let program = if args[0] == "limo" {
            limo
        } else {
            Path::new(&args[0])
        };

        let output = Command::new(program)
            .args(&args[1..])
            .current_dir(root)
            .stdin(Stdio::null())
            .output()
            .with_context(|| format!("line {}: failed to execute {command}", line + 1))?;

        // Only the declared output streams are asserted; exit status is ignored.
        for (stream, expected, actual) in [
            ("stdout", expected_stdout, output.stdout.as_slice()),
            ("stderr", expected_stderr, output.stderr.as_slice()),
        ] {
            if let Some(expected) = expected {
                let actual = std::str::from_utf8(actual)?.replace("\r\n", "\n");
                if actual != expected {
                    bail!(
                        "line {}: {command}\n{stream} mismatch\nexpected: {expected:?}\n  actual: {actual:?}\nstderr:\n{}",
                        line + 1,
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
        }
        runs += 1;
    }

    Ok(runs)
}
