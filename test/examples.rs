use std::fs;
use std::process::Command;

#[test]
fn run_all_examples() {
    let example_dir = "examples";

    let entries = fs::read_dir(example_dir).expect("Could not read examples directory");

    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            let file_stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .expect("Could not get file name");

            println!("Running example: {}", file_stem);

            let status = Command::new("cargo")
                .args(&["run", "--example", file_stem])
                .status()
                .expect("Failed to execute cargo");

            assert!(
                status.success(),
                "Example `{}` failed with non-zero exit",
                file_stem
            );
        }
    }
}
