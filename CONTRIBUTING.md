# Contributing

This doc is the whole workflow. If you're ever unsure what to do, come back here.

## The loop

1. **Pick an issue.** Go to Issues, filter by the current milestone, lowest number first.
2. **Assign yourself** so nobody duplicates work.
3. **Branch** off of `main` (see [branching](#branching)).
4. **Do the work.** The issue tells you exactly what to build, what must hold true, and how to test it. If the issue is unclear, comment on it – that's a spec bug, not your fault.
5. **Make a PR** (see [pull requests](#pull-requests)).
6. **Address review, squash-merge, delete the branch.** Done. Go to step 1.

## Branching

Branch from `main`, named:

```
<type>/<issue-number>-<short-description>
```

Types: `feat` `fix` `docs` `test` `chore` `refactor`

Example: `feat/12-resource-enum`, `fix/31-robber-stays-put`, `docs/7-readme-arch`

```bash
git checkout main && git pull
git checkout -b feat/12-resource-enum
```

## Commits

Short, imperative, present tense: `add Resource enum`, `fix desert producing wool`.  
Don't stress about commit hygiene on your branch – we squash-merge, so only the PR title survives into `main`'s history. 

## Pull requests

- Title: roughly the issue title, e.g. `feat: Resource enum (#12)`
- Body: the template auto-fills. Make sure `Closes #N` points at your issue this auto-closes it on merge.
- Keep it small. If your PR is growing past the issue's scope, stop and make a new issue for the next thing.
- One approving review, then **squash and merge**, then delete the branch.

## Definition of done

A Pull Request is merged when:

- [ ] `cargo fmt --all` produces no changes
- [ ] `cargo clippy --workspace --all-targets` produces no warnings
- [ ] `cargo test --workspace` passes, including the new tests you added
- [ ] Every **Done when** item on the issue is satisfied

CI runs the first three automatically on every PR.

## Local commands

```bash
cargo test -p catan-core        # test just the engine crate (fast)
cargo test --workspace          # test everything
cargo fmt --all                 # auto-format
cargo clippy --workspace --all-targets   # lint
```
