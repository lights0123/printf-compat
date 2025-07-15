# Release process

1. `git checkout master && git pull`
2. `git checkout -b <some-branch-name>`
3. Update the `version` field in `Cargo.toml`.
5. Update `CHANGELOG.md`.
6. Commit `Cargo.toml` and `CHANGELOG.md`. The commit message must start
   with `release:`.
7. Push the branch and create a PR.
8. Merge the PR in "Rebase and merge" mode (to preserve the commit subject).

After merging, the new release will automatically be created on
<https://crates.io>. A git tag will also be created automatically.

See <https://crates.io/crates/auto-release> for more details of how the
release process is implemented.
