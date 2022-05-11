// we can see existing variables.
// note that template and Rhai variables are separate!
let is_library = variable::get("is_library");
debug(`is_library: ${is_library}`);

let use_nightly = variable::get("use_nightly");
debug(`use_nightly: ${use_nightly}`);

let gh_username = variable::get("gh_username");
debug(`gh_username: ${gh_username}`);

let gh_workflows = variable::get("gh_workflows");
debug(`gh_workflows: ${gh_workflows}`);

let project_name = variable::get("project-name");
debug(`project_name: ${project_name}`);

let license = variable::get("license");
debug(`license: ${license}`);

switch license.to_upper() {
  "APACHE" => {
    file::rename("LICENSE-APACHE", "LICENSE");
  }
  "MIT" => {
    file::rename("LICENSE-MIT", "LICENSE");
  }
  "MPL-2.0" => {
    file::rename("LICENSE-MPL", "LICENSE");
  }
  "MIT + APACHE" => {}
  "UNRESTRICTED" => {}
  _ => {
    debug(`unknown license type: ${license}`);
  }
}

file::rename("README.PROJECT.md", "README.md");
file::rename(".vscode/project.code-workspace", `.vscode/${project_name}.code-workspace`);
file::rename("gitignore", ".gitignore");

if gh_workflows {
  file::rename("wf/rust.yml", ".github/workflows/rust.yml");
  file::rename("wf/coverage.yml", ".github/workflows/coverage.yml");
  if use_nightly {
    file::rename("wf/rust_nightly.yml", ".github/workflows/rust_nightly.yml");
  }
  file::delete("wf");
  file::delete(".github/workflows/.gitkeep");
}
