# Publication checklist

The local repository foundation does not publish a remote repository or a release.
Complete this checklist when the maintainer chooses to publish.

- [x] Choose the project name: **Eros**.
- [ ] Confirm the public repository owner and repository slug.
- [ ] Review license and ownership notices for the actual contributed code/assets.
- [ ] Check the Git index and history for legacy material, runtime data and credentials.
- [ ] Confirm a clean checkout works without `legado/` or private machine paths.
- [ ] Set and document a private security and conduct-reporting contact.
- [ ] Enable private vulnerability reporting, if supported by the chosen host.
- [ ] Configure protected branches and required checks to match active contributors.
- [ ] Add real owner handles to CODEOWNERS only after maintainers are identified.
- [ ] Verify the workflow on the remote host; local execution is not a hosted CI run.
- [ ] Keep README feature claims aligned with implemented behavior.

A release additionally needs reproducible builds, verified Ubuntu installation,
documented runtime prerequisites and meaningful acceptance checks. There is no
release artifact at the repository-foundation stage.
