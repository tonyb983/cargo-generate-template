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

let project_kind = variable::prompt("Is this project a library or binary?", "library", ["library", "binary"]);
variable::set("is_library", project_kind == "library");

let gh_username = variable::prompt("Please enter your github username (used for setting up gh actions and readme badges):", "tonyb983");
variable::set("gh_username", gh_username);

let gh_workflows = variable::prompt("Would you like to enable Github Actions for linting, testing, and code coverage?", true);
variable::set("gh_workflows", gh_workflows);

let use_nightly = variable::prompt("Would you like to use the nightly toolchain?", true);
variable::set("use_nightly", use_nightly);

let use_vscode = variable::prompt("Are you planning on using VSCode as your editor for this project?", true);
variable::set("use_vscode", use_vscode);

let license = variable::prompt("What license would you like to use for the project?", "None", LICENSES.keys());
variable::set("license", license);

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
  // "BSL1" => {
  //   file::rename("licenses/LICENSE-BSL1", "LICENSE");
  //   file::delete("licenses");
  // }
  // "BSD2" => {
  //   file::rename("licenses/LICENSE-BSD2", "LICENSE");
  //   file::delete("licenses");
  // }
  // "BSD3" => {
  //   file::rename("licenses/LICENSE-BSD3", "LICENSE");
  //   file::delete("licenses");
  // }
  // "GPLv2" => {
  //   file::rename("licenses/LICENSE-GPLv2", "LICENSE");
  //   file::delete("licenses");
  // }
  // "GPLv3" => {
  //   file::rename("licenses/LICENSE-GPLv3", "LICENSE");
  //   file::delete("licenses");
  // }
  // "MIT" => {
  //   file::rename("licenses/LICENSE-MIT", "LICENSE");
  //   file::delete("licenses");
  // }
  // "MPLv2" => {
  //   file::rename("licenses/LICENSE-MPLv2", "LICENSE");
  //   file::delete("licenses");
  // }
  // "Unlicense" => {
  //   file::rename("licenses/LICENSE-UNLICENSE", "LICENSE");
  //   file::delete("licenses");
  // }
  // _ => {
  //   debug(`unknown license type: ${license}`);
  // }
}
