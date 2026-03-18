use std::fs;
use std::path::Path;

fn collect_rs_files(dir: &Path, files: &mut Vec<std::path::PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_rs_files(&path, files);
            } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                files.push(path);
            }
        }
    }
}

#[test]
fn all_pub_async_fns_in_services_have_tracing_instrument() {
    let services_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/services");
    let mut rs_files = Vec::new();
    collect_rs_files(&services_dir, &mut rs_files);

    let mut violations = Vec::new();

    for file_path in &rs_files {
        let relative = file_path
            .strip_prefix(env!("CARGO_MANIFEST_DIR"))
            .unwrap_or(file_path);

        if relative.ends_with("services/mod.rs") {
            continue;
        }

        let contents = match fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let lines: Vec<&str> = contents.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if !trimmed.contains("pub async fn") {
                continue;
            }

            let mut found_instrument = false;
            let mut j = i;
            while j > 0 {
                j -= 1;
                let prev = lines[j].trim();
                if prev.is_empty() {
                    continue;
                }
                if prev.contains("tracing::instrument") {
                    found_instrument = true;
                }
                break;
            }

            if !found_instrument {
                violations.push(format!(
                    "{}:{} - `{}` missing #[tracing::instrument]",
                    relative.display(),
                    i + 1,
                    trimmed,
                ));
            }
        }
    }

    if !violations.is_empty() {
        panic!(
            "Found pub async fn(s) in src/services/ without #[tracing::instrument]:\n{}",
            violations.join("\n")
        );
    }
}
