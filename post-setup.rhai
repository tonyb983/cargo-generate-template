//! Post-Setup Script
//!
//! This cleans up any unnecessary files based on the prompts run during the pre-setup script.
//! I have to do this here since evidently `cargo-generate.toml` does not honor conditionals
//! that are set during the pre-setup script. I'm guess the order of execution goes: 1) cargo-generate.toml,
//! 2) pre-setup script, 3) project-wide text substitution and conditional segments, 4) post-setup script,
//! which is why the conditional segments of code throughout the project work okay with pre-setup prompt
//! variables, but the conditional segments inside cargo-generate.toml do NOT.

fn log(x) {
    const DEBUG = false;
    if DEBUG {
        debug(x);
    }
}
log("Running project post-setup script...");
print!("🚧 Cleaning up the last little odds and ends... 🚧");

let is_library = variable::get("is_library");
log(`is_library: ${is_library}`);
let use_nightly = variable::get("use_nightly");
log(`use_nightly: ${use_nightly}`);
let gh_workflows = variable::get("gh_workflows");
log(`gh_workflows: ${gh_workflows}`);
let project_name = variable::get("project-name");
log(`project_name: ${project_name}`);
let use_vscode = variable::get("use_vscode");
log(`use_vscode: ${use_vscode}`);
let use_benches = variable::get("use_benches");
log(`use_benches: ${use_benches}`);
let use_integration = variable::get("use_integration");
log(`use_integration: ${use_integration}`);
let use_flamegraph = variable::get("use_flamegraph");
log(`use_flamegraph: ${use_flamegraph}`);

// Set project `README` file
log("moving README");
file::rename("README.PROJECT.md", "README.md");
// Set project `.gitignore` file
log("moving gitignore");
file::rename("gitignore", ".gitignore");

// Delete the appropriate "main" file based on whether this is a library or binary, and if binary delete the "default-runner"
if is_library {
    log("deleting main.rs");
    file::delete("src/main.rs");
} else {
    log("deleting lib.rs and src/bin");
    file::delete("src/lib.rs");
    file::delete("src/bin");
}

log("checking vscode");
// If using VSCode, keep the .vscode directory and rename the code-workspace file, otherwise delete the directory
if use_vscode {
  log("setting up vscode");
  file::rename("optional/vscode/project.code-workspace", `optional/vscode/${project_name}.code-workspace`);
  file::rename("optional/vscode", ".vscode");
}

log("checking gh_workflows");
// If using GH Actions, move workflow files from `wf` folder to `.github/workflows/`
if gh_workflows {
  log("setting up workflows");
  file::rename("optional/wf/coverage.yml", ".github/workflows/coverage.yml");
  //  Delete `rust_nightly.yml` if not using the nightly toolchain, otherwise delete `rust.yml`
  if use_nightly {
    log("setting up nightly workflow");
    file::rename("optional/wf/rust_nightly.yml", ".github/workflows/rust_nightly.yml");
  } else {
    log("setting up stable workflow");
    file::rename("optional/wf/rust.yml", ".github/workflows/rust.yml");
  }
  file::delete(".github/workflows/.gitkeep");
} else {
  log("deleting .github");
  file::delete(".github");
}

log("checking use_benches");
if use_benches {
  log("setting up benches");
  file::rename("optional/benches", "benches");
}

log("checking use_integration");
if use_integration {
  log("setting up tests");
  file::rename("optional/tests", "tests");
}

log("checking use_flamegraph");
if use_flamegraph {
  log("setting up flame stuff");
  file::rename("optional/flames", "flames");
  file::rename("optional/flame_example.rs", "src/flame_example.rs");
}

log("deleting optional");
file::delete("optional");
