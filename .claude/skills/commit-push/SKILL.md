---
name: commit-push
description: Stage changed files, create a commit, and push to origin. Usage: /commit-push <commit message>
disable-model-invocation: true
---

The user wants to commit and push their current changes. The commit message is: $ARGUMENTS

Steps:
1. Run `git status` to see what's changed.
2. Run `git add -A` to stage all changes (but warn if any `.env` or credential files are staged — ask the user before proceeding if so).
3. Run `cargo fmt --all` to ensure code is formatted before committing.
4. Commit with the provided message:
   ```
   git commit -m "$ARGUMENTS"
   ```
   If $ARGUMENTS is empty, ask the user for a commit message before proceeding.
5. Run `git push origin` to push to the remote (`git@github.com:gilang-as/helios.git`).
6. Report success with the commit hash.

If the repo has no remote set up yet, run:
```
git remote add origin git@github.com:gilang-as/helios.git
git push -u origin main
```
