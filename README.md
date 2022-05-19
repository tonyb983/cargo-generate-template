# Rust Project Template

Asks a few deep soul-searching questions and then creates a new rust project accordingly. Don't worry, it's mostly harmless.

## Template Usage 

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate tonyb983/cargo-generate-template
```

## Files / Folders
- `.gitignore` - this is for the TEMPLATE, not the project created from it, it will be deleted, and `gitignore` will be renamed to `.gitignore`
- `.github` - Empty folder serving as the endpoint for GH actions should the user opt-in to them
- `cargo-generate.toml` - template-only file that will *not* be transferred to the newly created project
- `Cargo.toml` - obvious, but it will be filled in with as much information as possible, as well as outlining several cool techniques
- `gitignore` - the `gitignore` file for the newly created project, will be renamed to `.gitignore`
- `licenses` - holds all of the license files, the selected license will be moved to the root folder and the rest will be deleted
- `optional` - folder containing all optional features that a project can elect to use
    - `benches/` - contains `criterion` benchmarking example
    - `flames/` - empty folder that serves as the output directory for generated flamegraph dumps
    - `tests/` - integration test examples
    - `vscode/` - `PROJECT.code-workspace` file in case the user wants to use VSCode
    - `wf/` - holds the github workflows that will be moved to `.github/workflows/` if the user elects to use them
    - `flame_example.rs` - simple example(s) showing how to measure and dump flamegraph data
    - `puffin_example.rs` - simple example showing how to use the `puffin` crate to do profiling
- `post-setup.rhai` - A few post-generate commands to correctly layout the new project
- `pre-setup.rhai` - This script moves the files needed for any optional features that are chosen
- `README.md` - The readme for the **template**, the document you are currently reading
- `README.PROJECT.md` - A skeleton readme for the newly generated project
- `.vscode/` - Creates a `code-workspace` file to use with VSCode. Can be deleted if you plan on using a different editor/IDE

## Todo-List
- [ ] Add support for repo hosts other than [Github](https://www.github.com)
    - This will be a bit complicated as many of the README badges are set up through github
- [x] Add support for more licenses
    - [ ] Are there any others that should be added?
- [x] Add option for adding tests (integration tests)
- [x] Add option for adding benches (with criterion probably?)
- [x] Add flamegraph generation example
- [x] Add `puffin` profiling example? (similar to the flamegraph example)
- [x] Add option for adding examples folder?
- [ ] Add support for workspaces? (might be somewhat complex, maybe it should be a separate template?)
- [ ] Find more cool badges to pad that README file :blush: