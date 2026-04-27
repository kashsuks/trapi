# trapi
Track your fitness progress through and API

## Bootstrap a new database

To apply this repo's schema to a fresh Postgres database:

```bash
chmod +x scripts/bootstrap_db.sh
scripts/bootstrap_db.sh "postgres://user:pass@host:5432/dbname"
```

The script applies every SQL file in [`migrations/`](/Users/ksukshavasi/trapi/migrations) in filename order using `psql`.

## Deploy to Vercel

This repo is configured to run on Vercel as a Rust Function entrypoint at [`api/index.rs`](/Users/ksukshavasi/trapi/api/index.rs:1).

The main site root serves interactive API docs, and the raw OpenAPI document is available at `/openapi.json`.

Required environment variables:

- `DATABASE_URL`
- `JWT_SECRET`
- `RUST_LOG` optional

Recommended deployment flow:

```bash
vercel
```

Before the first production deploy, bootstrap the target Postgres database with:

```bash
scripts/bootstrap_db.sh "$DATABASE_URL"
```

## AI Usage

GPT Codex was used in this project to deploy to vercel and debug rust issues with the creation of the API
