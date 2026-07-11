# jsw

`jsw` deletes Jules sessions in bulk.

The current implementation re-lists the first page after each delete batch and keeps going until no sessions remain. This avoids leaving gaps while the session list is shrinking.

## Execution

This tool runs exclusively via GitHub Actions. Tagged releases publish the Linux executable to GitHub Releases, and the manual workflow at `.github/workflows/run.yml` downloads and executes that published binary.

Required repository secret:

- `JULES_API_KEY`

Run it from the Actions tab with `workflow_dispatch`.

The manual workflow always downloads the latest published release and keeps deleting sessions until none remain.

Tagged pushes matching `v*` publish the release binary consumed by the manual workflow.
