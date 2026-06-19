use std::{
	fs,
	path::Path,
	process::{Command, ExitStatus, Output},
};

#[cfg(not(target_os = "windows"))]
use std::os::unix::process::ExitStatusExt;

#[cfg(target_os = "windows")]
use std::os::windows::process::ExitStatusExt;

fn main() {
	println!("cargo:rerun-if-changed=src/");
	emit_git_rerun_triggers();
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
	let output = output.trim();
	let git_hash = if output.is_empty() { "dev".to_string() } else { output.to_string() };
	println!("cargo:rustc-env=GIT_HASH={git_hash}");
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
