use std::{fs, path::Path};

fn main() {
    let src_dir = Path::new("src");
    let mut warnings = Vec::new();

    visit_files(src_dir, &mut |path| {
        if let Ok(content) = fs::read_to_string(path) {
            let mut line_number = 1;
            for line in content.lines() {
                // Look for add_system with schedule=Update but no run_if in the same attribute
                if line.contains("#[add_system(")
                    && line.contains("schedule = Update")
                    && !line.contains("run_if")
                {
                    warnings.push(format!(
                        "{}:{}: Warning: Missing run_if on add_system with schedule=Update",
                        path.display(),
                        line_number
                    ));
                }
                line_number += 1;
            }
        }
    });

    if !warnings.is_empty() {
        println!("Warnings: add_systems scheduled on Update missing run_if conditions:");
        for warning in &warnings {
            println!("  - {}", warning);
        }
    }
}

// Recursive directory walker on src/
fn visit_files(dir: &Path, cb: &mut dyn FnMut(&Path)) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                visit_files(&path, cb);
            } else if path.extension().map(|ext| ext == "rs").unwrap_or(false) {
                cb(&path);
            }
        }
    }
}
