use std::{
	env, fs,
	path::Path,
	process::{Command, ExitStatus, Output},
};

#[cfg(not(target_os = "windows"))]
use std::os::unix::process::ExitStatusExt;

#[cfg(target_os = "windows")]
use std::os::windows::process::ExitStatusExt;

fn main() {
	println!("cargo:rerun-if-changed=src/");
	println!("cargo:rerun-if-env-changed=REDLIB_GIT_COMMIT");
	emit_git_rerun_triggers();
	let git_hash = env::var("REDLIB_GIT_COMMIT").ok().and_then(normalize_git_hash).unwrap_or_else(git_head_hash);
	println!("cargo:rustc-env=GIT_HASH={git_hash}");
}

fn git_head_hash() -> String {
	let output = String::from_utf8(
		Command::new("git")
			.args(["rev-parse", "HEAD"])
			.output()
			.unwrap_or(Output {
				stdout: vec![],
				stderr: vec![],
				status: ExitStatus::from_raw(0),
			})
			.stdout,
	)
	.unwrap_or_default();
	normalize_git_hash(output).unwrap_or_else(|| "dev".to_string())
}

fn normalize_git_hash(hash: impl AsRef<str>) -> Option<String> {
	let hash = hash.as_ref().trim();
	if hash.is_empty() || hash == "dev" {
		None
	} else {
		Some(hash.to_string())
	}
}

fn emit_git_rerun_triggers() {
	let head_path = Path::new(".git/HEAD");
	if !head_path.exists() {
		return;
	}

	println!("cargo:rerun-if-changed={}", head_path.display());

	let Ok(head) = fs::read_to_string(head_path) else {
		return;
	};
	let Some(reference) = head.strip_prefix("ref: ").map(str::trim) else {
		return;
	};

	let ref_path = Path::new(".git").join(reference);
	if ref_path.exists() {
		println!("cargo:rerun-if-changed={}", ref_path.display());
		return;
	}

	let packed_refs_path = Path::new(".git/packed-refs");
	if packed_refs_path.exists() {
		println!("cargo:rerun-if-changed={}", packed_refs_path.display());
	}
}
