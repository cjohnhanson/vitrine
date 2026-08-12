//! `vitrine serve`: serve the static files of the repo root, and run
//! the response inbox. Any static server does the other artifact work.
//! This command adds the round-trip only.
//!
//! A POST to /respond/<slug> with a JSON body goes to
//! `.vitrine/<slug>/responses/<stamp>.json` and to `latest.json`. The
//! server also writes one line for each response to stdout
//! (`response saved: <slug>`). A watcher reads that line, then notifies
//! an agent.

use std::io::Read as _;
use std::path::{Component, Path, PathBuf};

use percent_encoding::percent_decode_str;
use tiny_http::{Header, Method, Response, Server};

const MAX_BODY: usize = 1 << 20; // the maximum response body size, 1 MiB

pub fn serve(repo: &Path, port: u16) -> std::io::Result<()> {
    let server =
        Server::http(("127.0.0.1", port)).map_err(|e| std::io::Error::other(e.to_string()))?;
    println!(
        "vitrine: serving {} on http://127.0.0.1:{port}/",
        repo.display()
    );

    for mut request in server.incoming_requests() {
        let url = request.url().to_string();
        let outcome = match (request.method(), url.as_str()) {
            (Method::Post, u) if u.starts_with("/respond/") => {
                let slug = u.trim_start_matches("/respond/").to_string();
                handle_respond(repo, &slug, &mut request)
            }
            (Method::Get | Method::Head, u) => handle_get(repo, u),
            _ => Outcome::Error(405, "method not allowed"),
        };
        let _ = match outcome {
            Outcome::File(bytes, mime) => request.respond(
                Response::from_data(bytes)
                    .with_header(Header::from_bytes("Content-Type", mime).expect("static header")),
            ),
            Outcome::Saved(slug) => {
                println!("response saved: {slug}");
                request.respond(Response::from_string(r#"{"ok":true}"#).with_header(
                    Header::from_bytes("Content-Type", "application/json").expect("static header"),
                ))
            }
            Outcome::Error(code, msg) => {
                request.respond(Response::from_string(msg).with_status_code(code))
            }
        };
    }
    Ok(())
}

enum Outcome {
    File(Vec<u8>, &'static str),
    Saved(String),
    Error(u16, &'static str),
}

/// Map a request path to a repo file. Refuse a path that goes above the
/// repo root. This function is public, so a test calls it directly.
#[must_use]
pub fn sanitize(repo: &Path, url_path: &str) -> Option<PathBuf> {
    let path = url_path.split('?').next().unwrap_or("");
    let decoded = percent_decode_str(path).decode_utf8().ok()?;
    let rel = decoded.trim_start_matches('/');
    let joined = repo.join(rel);
    let mut depth = 0i32;
    for c in Path::new(rel).components() {
        match c {
            Component::ParentDir => {
                depth -= 1;
                if depth < 0 {
                    return None;
                }
            }
            Component::Normal(_) => depth += 1,
            Component::CurDir => {}
            Component::RootDir | Component::Prefix(_) => return None,
        }
    }
    Some(if joined.is_dir() {
        joined.join("index.html")
    } else {
        joined
    })
}

fn handle_get(repo: &Path, url: &str) -> Outcome {
    let Some(path) = sanitize(repo, url) else {
        return Outcome::Error(400, "bad path");
    };
    std::fs::read(&path).map_or(Outcome::Error(404, "not found"), |bytes| {
        Outcome::File(bytes, mime_for(&path))
    })
}

fn handle_respond(repo: &Path, slug: &str, request: &mut tiny_http::Request) -> Outcome {
    if slug.is_empty() || !slug.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-') {
        return Outcome::Error(400, "bad slug");
    }
    let artifact = repo.join(".vitrine").join(slug);
    if !artifact.is_dir() {
        return Outcome::Error(404, "no artifact with that slug");
    }
    let mut body = String::new();
    if request
        .as_reader()
        .take(MAX_BODY as u64 + 1)
        .read_to_string(&mut body)
        .is_err()
        || body.len() > MAX_BODY
    {
        return Outcome::Error(400, "the body is unreadable or too large");
    }
    if serde_json::from_str::<serde_json::Value>(&body).is_err() {
        return Outcome::Error(400, "the body must be JSON");
    }

    let responses = artifact.join("responses");
    if std::fs::create_dir_all(&responses).is_err() {
        return Outcome::Error(500, "cannot write the response");
    }
    let stamp = std::env::var("VITRINE_STAMP").unwrap_or_else(|_| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or_else(|_| "0".to_string(), |d| d.as_millis().to_string())
    });
    if std::fs::write(responses.join(format!("{stamp}.json")), &body).is_err()
        || std::fs::write(artifact.join("latest.json"), &body).is_err()
    {
        return Outcome::Error(500, "cannot write the response");
    }
    Outcome::Saved(slug.to_string())
}

fn mime_for(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("js") => "text/javascript",
        Some("css") => "text/css",
        Some("json") => "application/json",
        Some("md") => "text/markdown; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("gif") => "image/gif",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_refuses_traversal_and_absolute() {
        let repo = Path::new("/repo");
        assert!(sanitize(repo, "/../etc/passwd").is_none());
        assert!(sanitize(repo, "/a/../../etc/passwd").is_none());
        assert!(
            sanitize(repo, "/%2e%2e/etc/passwd").is_none(),
            "the dots are percent-encoded"
        );
        assert_eq!(
            sanitize(repo, "/a/b.js?v=1"),
            Some(PathBuf::from("/repo/a/b.js"))
        );
        assert_eq!(
            sanitize(repo, "/.vitrine/x/../y/index.html"),
            Some(PathBuf::from("/repo/.vitrine/x/../y/index.html")),
            "an interior .. that stays under the root is allowed"
        );
    }
}
