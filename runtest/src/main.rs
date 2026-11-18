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
        if entry.file_type()?.is_file()
            && matches!(
                path.extension().and_then(|ext| ext.to_str()),
                Some("js" | "mjs")
            )
        {
            files.push(path);
        }
    }
    ensure!(!files.is_empty(), "no .js or .mjs files found in tests/");

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
    ensure!(failed == 0, "{failed} test files failed");
    Ok(())
}

#[derive(Default)]
struct Expectations {
    exit_ok: Option<bool>,
    stdout: Option<String>,
    stderr: Option<String>,
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
                !["// EXIT:", "// STDOUT:", "// STDERR:"]
                    .iter()
                    .any(|prefix| text.starts_with(prefix)),
                "line {}: expectation must immediately follow RUN or another expectation",
                line + 1
            );
            continue;
        };

        let mut expected = Expectations::default();
        while let Some(&(index, text)) = lines.peek() {
            let text = text.trim_start();
            if let Some(value) = text.strip_prefix("// EXIT:") {
                ensure!(
                    expected.exit_ok.is_none(),
                    "line {}: duplicate EXIT",
                    index + 1
                );
                expected.exit_ok = Some(match value.trim() {
                    "ok" => true,
                    "fail" => false,
                    value => bail!(
                        "line {}: expected EXIT ok or fail, got {value:?}",
                        index + 1
                    ),
                });
            } else {
                let (stream, output) = if let Some(output) = text.strip_prefix("// STDOUT:") {
                    (&mut expected.stdout, output)
                } else if let Some(output) = text.strip_prefix("// STDERR:") {
                    (&mut expected.stderr, output)
                } else {
                    break;
                };
                let stream = stream.get_or_insert_with(String::new);
                stream.push_str(output.strip_prefix(' ').unwrap_or(output));
                stream.push('\n');
            }
            lines.next();
        }

        run_command(root, limo, file, command, expected)
            .with_context(|| format!("line {}: {command}", line + 1))?;
        runs += 1;
    }

    if runs == 0 {
        let command = "limo --timeout-ms 5000 %s";
        run_command(root, limo, file, command, Expectations::default()).context(command)?;
        runs = 1;
    }
    Ok(runs)
}

fn run_command(
    root: &Path,
    limo: &Path,
    file: &str,
    command: &str,
    expected: Expectations,
) -> Result<()> {
    let args = shlex::split(command).context("invalid RUN quoting")?;
    let args: Vec<_> = args.iter().map(|arg| arg.replace("%s", file)).collect();
    let (program, args) = args.split_first().context("RUN command is empty")?;
    let program = if program == "limo" {
        limo
    } else {
        Path::new(program)
    };
    let output = Command::new(program)
        .args(args)
        .current_dir(root)
        .stdin(Stdio::null())
        .output()
        .context("failed to execute command")?;

    let exit_ok = expected.exit_ok.unwrap_or(true);
    if output.status.code().is_none() || output.status.success() != exit_ok {
        bail!(
            "expected exit {}, got {}\nstdout:\n{}\nstderr:\n{}",
            if exit_ok { "ok" } else { "fail" },
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
    }
    for (stream, expected, actual) in [
        ("stdout", expected.stdout, output.stdout.as_slice()),
        ("stderr", expected.stderr, output.stderr.as_slice()),
    ] {
        if let Some(expected) = expected {
            let actual = std::str::from_utf8(actual)?.replace("\r\n", "\n");
            if actual != expected {
                bail!(
                    "{stream} mismatch\nexpected: {expected:?}\n  actual: {actual:?}\nstderr:\n{}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
    }
    Ok(())
}
