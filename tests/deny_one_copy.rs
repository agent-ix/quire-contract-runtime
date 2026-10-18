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

#[trace("FR-275-AC-6")]
#[test]
fn deny_rejects_a_second_first_party_git_lock_entry() {
    let worktree = ScratchWorktree::new(Path::new(env!("CARGO_MANIFEST_DIR")));
    let baseline = worktree.make_deny();
    assert!(
        baseline.status.success(),
        "unmodified lock failed: {}",
        String::from_utf8_lossy(&baseline.stderr)
    );

    let mut lock = OpenOptions::new()
        .append(true)
        .open(worktree.root.join("Cargo.lock"))
        .unwrap();
    writeln!(
        lock,
        "\n[[package]]\nname = \"quire-exact\"\nversion = \"0.1.0\"\nsource = \"git+https://github.com/agent-ix/quire-exact?branch=main\""
    )
    .unwrap();
    drop(lock);

    let duplicate = worktree.make_deny();
    assert!(!duplicate.status.success(), "duplicate lock was accepted");
    assert!(
        String::from_utf8_lossy(&duplicate.stderr)
            .contains("one-copy: quire-exact has 2 entries in Cargo.lock"),
        "wrong failure: {}",
        String::from_utf8_lossy(&duplicate.stderr)
    );
}
