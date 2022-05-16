# {{project-name}}

<!-- BADGES -->
<!-- * License Badge -->{% if gh_workflows %}{% if use_nightly %}
<!-- * Workflow Status (for "Rust Nightly") -->{% else %}
<!-- * Workflow Status (for "Rust Stable") -->{% endif %}{% endif %}
<!-- * Rust Report Card - https://rust-reportcard.xuri.me -->
<!-- * Coverage Status - coveralls.io -->
<!-- * Coverage Status - codecov.io -->
<!-- * Lines of Code (via tokei) -->
<!-- * Github Last Commit (date) -->
![GitHub](https://img.shields.io/github/license/{{gh_username}}/{{project-name}}){% if gh_workflows %}{% if use_nightly %}
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/{{gh_username}}/{{project-name}}/Rust%20Nightly?label=Nightly){% else %}
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/{{gh_username}}/{{project-name}}/Rust%20Stable?label=Stable){% endif %}{% endif %}
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/{{gh_username}}/{{project-name}})](https://rust-reportcard.xuri.me/report/github.com/{{gh_username}}/{{project-name}})
[![Coverage Status](https://coveralls.io/repos/github/{{gh_username}}/{{project-name}}/badge.svg?branch=main)](https://coveralls.io/github/{{gh_username}}/{{project-name}}?branch=main)
[![codecov](https://codecov.io/gh/{{gh_username}}/{{project-name}}/branch/main/graph/badge.svg?token=0P01LCVPYC)](https://codecov.io/gh/{{gh_username}}/{{project-name}})
[![](https://tokei.rs/b1/github/tonyb983/REPOSITORY)](https://github.com/{{gh_username}}/{{project-name}})
![GitHub last commit](https://img.shields.io/github/last-commit/{{gh_username}}/{{project-name}})

## Project Description

## Examples
Basic Usage:
```rust
# use {{project-name}}::*;
# fn main() {
    todo!("Write some examples!");
# }
```

## Other Stuff
