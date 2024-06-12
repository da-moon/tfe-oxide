# Release Checklist

This is a list of steps to complete when making a new release.

# Before the release

- [ ] 1. Create a new issue in the repo with the name `x.x.x` and copy-paste this checklist into it (add blockers and additional tasks, if they exist).
- [ ] 2. Update all official examples.
- [ ] 3. Review the commit and PR history since the last release. Ensure that all relevant changes are included in `CHANGELOG.md`, and that breaking changes are annotated.
- [ ] 4. Ensure the `README.md` reflects API changes.
- [ ] 5. Update the `CHANGELOG.md` with the new release version.
- [ ] 6. Ensure the version listed in `Cargo.toml` is updated.
- [ ] 7. Update Rust tools: `rustup update`.
- [ ] 8. Run `cargo make verify` to ensure tests pass, and `clippy` / `fmt` are run.
- [ ] 9. Commit and push the repo.
- [ ] 10. Check that CI pipeline passed.
- [ ] 11. Run `cargo package`.
- [ ] 12. Run `cargo publish`.
- [ ] 13. Add a release on [Github](https://github.com/da-moon/tfe-oxide/releases), following the format of previous releases.
