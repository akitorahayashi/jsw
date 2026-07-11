# jsw

`jsw` deletes Jules sessions in bulk.

The current implementation re-lists the first page after each delete batch and keeps going until no sessions remain. This avoids leaving gaps while the session list is shrinking.

## Execution

This tool runs exclusively via GitHub Actions. Tagged releases publish the Linux executable to GitHub Releases, and the manual workflow at `.github/workflows/run.yml` downloads and executes that published binary.

Required repository secret:

- `JULES_API_KEY`

Run it from the Actions tab with `workflow_dispatch`.

Workflow inputs:

- `release_tag`: optional release tag to execute. Leave it empty to download the latest published release.
- `delete_limit`: optional non-negative integer. Leave it empty to keep deleting until no sessions remain.

Environment inputs control the delete mode:

- `DELETE_LIMIT=<n>` deletes only `n` sessions for a dry run.
- Empty or unset `DELETE_LIMIT` keeps deleting until no sessions remain.

Tagged pushes matching `v*` publish the release binary consumed by the manual workflow.
