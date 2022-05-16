//! Post-Setup Script
//!
//! This cleans up any unnecessary files based on the prompts run during the pre-setup script.
//! I have to do this here since evidently `cargo-generate.toml` does not honor conditionals
//! that are set during the pre-setup script. I'm guess the order of execution goes: 1) cargo-generate.toml,
//! 2) pre-setup script, 3) project-wide text substitution and conditional segments, 4) post-setup script,
//! which is why the conditional segments of code throughout the project work okay with pre-setup prompt
//! variables, but the conditional segments inside cargo-generate.toml do NOT.

print("Running project post-setup script...");
print!("🚧 Cleaning up the last little odds and ends... 🚧");

let is_library = variable::get("is_library");
debug(`is_library: ${is_library}`);
let use_nightly = variable::get("use_nightly");
debug(`use_nightly: ${use_nightly}`);
let gh_workflows = variable::get("gh_workflows");
debug(`gh_workflows: ${gh_workflows}`);
let project_name = variable::get("project-name");
debug(`project_name: ${project_name}`);
let use_vscode = variable::get("use_vscode");
debug(`use_vscode: ${use_vscode}`);
let use_benches = variable::get("use_benches");
debug(`use_benches: ${use_benches}`);
let use_integration = variable::get("use_integration");
debug(`use_integration: ${use_integration}`);
let use_flamegraph = variable::get("use_flamegraph");
debug(`use_flamegraph: ${use_flamegraph}`);

// Set project `README` file
file::rename("README.PROJECT.md", "README.md");
// Set project `.gitignore` file
file::rename("gitignore", ".gitignore");

// Delete the appropriate "main" file based on whether this is a library or binary, and if binary delete the "default-runner"
if is_library {
    file::delete("src/main.rs");
} else {
    file::delete("src/lib.rs");
    file::delete("src/bin");
}

// If using VSCode, keep the .vscode directory and rename the code-workspace file, otherwise delete the directory
if use_vscode {
  file::rename("optional/vscode/project.code-workspace", `optional/vscode/${project_name}.code-workspace`);
  file::rename("optional/vscode", ".vscode");
}

// If using GH Actions, move workflow files from `wf` folder to `.github/workflows/`
if gh_workflows {
  file::rename("optional/wf/coverage.yml", ".github/workflows/coverage.yml");
  //  Delete `rust_nightly.yml` if not using the nightly toolchain, otherwise delete `rust.yml`
  if use_nightly {
    file::rename("optional/wf/rust_nightly.yml", ".github/workflows/rust_nightly.yml");
  } else {
    file::rename("optional/wf/rust.yml", ".github/workflows/rust.yml");
  }
  file::delete(".github/workflows/.gitkeep");
} else {
  file::delete(".github");
}

if use_benches {
  file::rename("optional/benches", "benches");
}

if use_integration {
  file::rename("optional/tests", "tests");
}

if use_flamegraph {
  file::rename("optional/flames", "flames");
  file::rename("optional/flame_example.rs", "src/flame_example.rs");
}

file::delete("optional");
