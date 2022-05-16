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
- `.github` - Used only if "yes" is selected when asked whether to use github actions, adds a "Rust Stable" workflow which will check, test, and lint the project, a "Rust Nightly" workflow which will do the same but on the nightly toolchain, and a "Coverage" workflow which will collect coverage information about the project and upload to both coveralls.io and codecov.io (either or both can be adjusted by deleting the relevant section from `.github/workflows/coverage.yml`)
- `cargo-generate.toml` - template-only file that will *not* be transferred to the newly created project
- `Cargo.toml` - obvious, but it will be filled in with as much information as possible, as well as outlining several cool techniques
- `LICENSE-*` - Only the selected license file will be kept and renamed accordingly
- `post-setup.rhai` - A few post-generate commands to correctly layout the new project
- `README.md` - The readme for the **template**, the document you are currently reading
- `README.PROJECT.md` - A skeleton readme for the newly generated project
- `wf/` - Holds the workflows so that they are not run when the template is updated (there's probably a better way to do this but oh well)
- `.vscode/` - Creates a `code-workspace` file to use with VSCode. Can be deleted if you plan on using a different editor/IDE

## Todo-List
- [ ] Add support for repo hosts other than [Github](https://www.github.com)
    - This will be a bit complicated as many of the README badges are set up through github
- [x] Add support for more licenses
    - [ ] Are there any others that should be added?
- [x] Add option for adding tests (integration tests)
- [x] Add option for adding benches (with criterion probably?)
- [x] Add flamegraph generation example
- [ ] Add option for adding examples folder?
- [ ] Find more cool badges to pad that README file :blush: