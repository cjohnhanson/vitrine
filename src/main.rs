//! The vitrine CLI. The subcommands are new, sync, serve, resolve,
//! extract, render, and docs. Exit code 0 means success. Exit code 1
//! means failure.

use std::path::PathBuf;
use std::process::ExitCode;

use vitrine::{bake, extract, refs, render, scaffold, serve};

const DOCS_GETTING_STARTED: &str = include_str!("../docs/getting-started.md");
const DOCS_AUTHORING: &str = include_str!("../docs/authoring.md");

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("new") => run_new(&args[1..]),
        Some("sync") => run_sync(&args[1..]),
        Some("serve") => run_serve(&args[1..]),
        Some("resolve") => run_resolve(&args[1..]),
        Some("extract") => run_extract(&args[1..]),
        Some("render") => run_render(&args[1..]),
        Some("docs") => run_docs(&args[1..]),
        Some(other) => fail(&format!(
            "unknown command `{other}` (available: new, sync, serve, resolve, extract, render, docs)"
        )),
        None => fail("usage: vitrine <new|sync|serve|resolve|extract|render|docs>"),
    }
}

fn fail(msg: &str) -> ExitCode {
    eprintln!("vitrine: {msg}");
    ExitCode::FAILURE
}

fn cwd() -> Option<PathBuf> {
    std::env::current_dir().ok()
}

fn run_new(args: &[String]) -> ExitCode {
    let Some(slug) = args.first() else {
        return fail("usage: vitrine new <slug>");
    };
    let Some(repo) = cwd() else {
        return fail("cannot get the working directory");
    };
    match scaffold::new(&repo, slug) {
        Ok(dir) => {
            println!("created {}", dir.display());
            ExitCode::SUCCESS
        }
        Err(e) => fail(&e.to_string()),
    }
}

fn run_sync(args: &[String]) -> ExitCode {
    let Some(repo) = cwd() else {
        return fail("cannot get the working directory");
    };
    let root = repo.join(".vitrine");
    let slugs: Vec<String> = if let Some(slug) = args.first() {
        vec![slug.clone()]
    } else {
        let Ok(entries) = std::fs::read_dir(&root) else {
            return fail("no .vitrine/ directory (run `vitrine new <slug>` first)");
        };
        let mut all: Vec<String> = entries
            .filter_map(Result::ok)
            .filter(|e| e.path().join("index.html").is_file())
            .filter_map(|e| e.file_name().into_string().ok())
            .collect();
        all.sort();
        all
    };

    for slug in &slugs {
        let dir = root.join(slug);
        let page = dir.join("index.html");
        let Ok(html) = std::fs::read_to_string(&page) else {
            return fail(&format!("no artifact at {}", page.display()));
        };
        match bake::bake(&html, &dir) {
            Ok(baked) => {
                if baked == html {
                    println!("unchanged {slug}");
                } else if std::fs::write(&page, &baked).is_ok() {
                    println!("synced {slug}");
                } else {
                    return fail(&format!("cannot write {}", page.display()));
                }
            }
            Err(e) => return fail(&format!("{slug}: {e}")),
        }
    }
    ExitCode::SUCCESS
}

fn run_serve(args: &[String]) -> ExitCode {
    let mut port: u16 = 4114;
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--port" => match it.next().map(|v| v.parse()) {
                Some(Ok(p)) => port = p,
                _ => return fail("--port requires a number"),
            },
            other => return fail(&format!("unexpected argument `{other}`")),
        }
    }
    let Some(repo) = cwd() else {
        return fail("cannot get the working directory");
    };
    match serve::serve(&repo, port) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => fail(&e.to_string()),
    }
}

fn run_resolve(args: &[String]) -> ExitCode {
    let Some(reference) = args.first() else {
        return fail("usage: vitrine resolve <tisket:id|zettel:id|file:path>[#anchor]");
    };
    let Some(repo) = cwd() else {
        return fail("cannot get the working directory");
    };
    match refs::resolve(&repo, reference) {
        Ok(resolved) => {
            match resolved.anchor {
                Some(a) => println!("{}#{a}", resolved.relative),
                None => println!("{}", resolved.relative),
            }
            ExitCode::SUCCESS
        }
        Err(e) => fail(&e.to_string()),
    }
}

fn run_extract(args: &[String]) -> ExitCode {
    match args {
        [file] => {
            let Ok(md) = std::fs::read_to_string(file) else {
                return fail(&format!("cannot read {file}"));
            };
            for anchor in extract::anchors(&md) {
                println!("{anchor}");
            }
            ExitCode::SUCCESS
        }
        [file, anchor] => {
            let Ok(md) = std::fs::read_to_string(file) else {
                return fail(&format!("cannot read {file}"));
            };
            extract::extract(&md, anchor).map_or_else(
                || fail(&format!("anchor `{anchor}` not found in {file}")),
                |section| {
                    print!("{section}");
                    ExitCode::SUCCESS
                },
            )
        }
        _ => fail("usage: vitrine extract <file.md> [anchor]"),
    }
}

fn run_render(args: &[String]) -> ExitCode {
    let Some(file) = args.first() else {
        return fail("usage: vitrine render <file.md>");
    };
    let Ok(md) = std::fs::read_to_string(file) else {
        return fail(&format!("cannot read {file}"));
    };
    print!("{}", render::render(render::strip_front_matter(&md)));
    ExitCode::SUCCESS
}

fn run_docs(args: &[String]) -> ExitCode {
    match args.first().map(String::as_str) {
        None => {
            println!("Topics (vitrine docs <topic>):\n  getting-started\n  authoring");
            ExitCode::SUCCESS
        }
        Some("getting-started") => {
            print!("{DOCS_GETTING_STARTED}");
            ExitCode::SUCCESS
        }
        Some("authoring") => {
            print!("{DOCS_AUTHORING}");
            ExitCode::SUCCESS
        }
        Some(other) => fail(&format!(
            "unknown topic `{other}` (getting-started, authoring)"
        )),
    }
}
