use std::collections::BTreeSet;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use ix_trace_rs::trace;

struct ScratchWorktree {
    root: PathBuf,
}

impl ScratchWorktree {
    fn new(repository: &Path) -> Self {
        let root = (0..100)
            .map(|nonce| {
                std::env::temp_dir().join(format!(
                    "quire-rt-one-copy-{}-{nonce}",
                    std::process::id()
                ))
            })
            .find(|path| fs::create_dir(path).is_ok())
            .expect("a unique temporary directory is available");
        let worktree = Self { root };
        let output = Command::new("git")
            .current_dir(repository)
            .args(["worktree", "add", "--detach"])
            .arg(&worktree.root)
            .arg("HEAD")
            .output()
            .expect("git can create a scratch worktree");
        assert!(
            output.status.success(),
            "git worktree add failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        worktree
    }

    fn make_deny(&self) -> Output {
        // Isolate the one-copy rule: cargo-deny may reject a synthetic lock
        // entry before Make reaches its own lockfile check.
        Command::new("make")
            .current_dir(&self.root)
            .args(["deny", "CARGO=true"])
            .output()
            .expect("make deny can run in the scratch worktree")
    }
}

impl Drop for ScratchWorktree {
    fn drop(&mut self) {
        let _ = Command::new("git")
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .args(["worktree", "remove", "--force"])
            .arg(&self.root)
            .status();
        let _ = fs::remove_dir(&self.root);
    }
}

#[trace("TC-202", "FR-275-AC-6")]
#[test]
fn deny_rejects_a_second_first_party_git_lock_entry() {
    let worktree = ScratchWorktree::new(Path::new(env!("CARGO_MANIFEST_DIR")));
    let baseline = worktree.make_deny();
    assert!(
        baseline.status.success(),
        "unmodified lock failed: {}",
        String::from_utf8_lossy(&baseline.stderr)
    );

    let original = fs::read_to_string(worktree.root.join("Cargo.lock")).unwrap();
    let first_party: BTreeSet<&str> = original
        .split("[[package]]")
        .filter_map(|package| {
            let name = package
                .lines()
                .find_map(|line| line.strip_prefix("name = \"")?.strip_suffix('"'))?;
            package
                .lines()
                .any(|line| line.starts_with("source = \"git+https://github.com/agent-ix/"))
                .then_some(name)
        })
        .collect();
    assert!(!first_party.is_empty(), "no first-party Git crates found");

    for name in first_party.into_iter().chain(["synthetic-first-party"]) {
        let lock_path = worktree.root.join("Cargo.lock");
        fs::write(&lock_path, &original).unwrap();
        let mut lock = OpenOptions::new().append(true).open(&lock_path).unwrap();
        // A new first-party name needs two entries; a resolved name needs one extra.
        let copies = if name == "synthetic-first-party" { 2 } else { 1 };
        for _ in 0..copies {
            writeln!(
                lock,
                "\n[[package]]\nname = \"{name}\"\nversion = \"0.0.0\"\nsource = \"git+https://github.com/agent-ix/{name}?branch=main\""
            )
            .unwrap();
        }
        drop(lock);

        let duplicate = worktree.make_deny();
        assert!(!duplicate.status.success(), "duplicate {name} was accepted");
        assert!(
            String::from_utf8_lossy(&duplicate.stderr)
                .contains(&format!("one-copy: {name} has 2 entries in Cargo.lock")),
            "wrong failure for {name}: {}",
            String::from_utf8_lossy(&duplicate.stderr)
        );
    }
}
