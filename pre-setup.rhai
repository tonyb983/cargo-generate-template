//! Pre-Setup Script
//!
//! I've chosen to use this to prompt the user for input rather than relying on `cargo-generate.toml`, it
//! bugged me a little bit that on each run, the order of prompts seemed to be randomized, this way they will
//! always follow the same order so as to be more consistent.

print("Running project pre-setup script...");
print("🌟 Welcome to the only Rust project template you'll ever need. 🌟");

let LICENSES = #{
  "Apache-2.0": "LICENSE-APACHE",
  "Apache-2.0 OR MIT": "",
  "BSL1": "LICENSE-BSL1",
  "BSD2": "LICENSE-BSD2",
  "BSD3": "LICENSE-BSD3",
  "GPLv2": "LICENSE-GPL2",
  "GPLv3": "LICENSE-GPL3",
  "MIT": "LICENSE-MIT",
  "MPLv2": "LICENSE-MPLv2",
  "Unlicense": "LICENSE-UNLICENSE",
  "None": "",
};

/// Prompt for project type, this is done by cargo generate but I live having control so whatev. Default = 'library'
let project_kind = variable::prompt("Is this project a library or binary?", "library", ["library", "binary"]);
variable::set("is_library", project_kind == "library");

/// Prompt for GH Username, used up repository badges and the GH Actions. Default = 'tonyb983' because i'm self-centered
/// TODO: Should / can this be made optional? Also, should we maybe prompt whether the user wants to keep all of the README badges?
let gh_username = variable::prompt("Please enter your github username (used for setting up gh actions and readme badges):", "tonyb983");
variable::set("gh_username", gh_username);

/// Ask whether user would like to set up github actions for check + test + link & code coverage. Default = 'true'
/// TODO: Should this be changed to default 'false'?
let gh_workflows = variable::prompt("Would you like to enable Github Actions for linting, testing, and code coverage?", true);
variable::set("gh_workflows", gh_workflows);

/// Ask whether the user would like to use the nightly toolchain instead of the stable toolchain. Default = 'true'
/// TODO: Should this be changed to default 'false'?
let use_nightly = variable::prompt("Would you like to use the nightly toolchain?", true);
variable::set("use_nightly", use_nightly);

/// Ask whether the user is going to use VSCode, if so we keep the .vscode folder. Default = 'true'
let use_vscode = variable::prompt("Are you planning on using VSCode as your editor for this project?", true);
variable::set("use_vscode", use_vscode);

/// Prompt for choice of license, this is used to keep the correct license file and delete the unnecessary ones. Default = 'None'
let license = variable::prompt("What license would you like to use for the project?", "None", LICENSES.keys());
variable::set("license", license);

/// Switch on license to rename and move the chosen license file, and delete the others
switch license {
  "None" => {
    file::delete("licenses");
  }
  "Apache-2.0 OR MIT" => {
    file::rename("licenses/LICENSE-APACHE", "LICENSE-APACHE");
    file::rename("licenses/LICENSE-MIT", "LICENSE-MIT");
    file::delete("licenses");
  }
  _ => {
    file::rename(`licenses/LICENSE-${license}`, "LICENSE");
    file::delete("licenses");
  }
  // Previously this switch held a full listing of each license, but since we are giving the user
  // a closed set of choices (the user must choose, they cannot type in a license) this should work
  // completely fine. There SHOULD be no possible way to get a license string other than what we expect.
}
