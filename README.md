# jsw

`jsw` deletes Jules sessions in bulk.

The current implementation re-lists the first page after each delete batch and keeps going until no sessions remain. This avoids leaving gaps while the session list is shrinking.

## Local run

Set `JULES_API_KEY` in `.env`, then run:

```bash
cargo run
```

`src/main.rs` controls the delete mode:

- `DELETE_LIMIT = Some(10)` deletes only 10 sessions for a dry run.
- `DELETE_LIMIT = None` keeps deleting until no sessions remain.

## GitHub Actions

The repository includes a manual workflow at `.github/workflows/run.yml`.

Required repository secret:

- `JULES_API_KEY`

Run it from the Actions tab with `workflow_dispatch`.
