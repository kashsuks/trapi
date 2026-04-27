# trapi
Track your fitness progress through and API

## Bootstrap a new database

To apply this repo's schema to a fresh Postgres database:

```bash
chmod +x scripts/bootstrap_db.sh
scripts/bootstrap_db.sh "postgres://user:pass@host:5432/dbname"
```

The script applies every SQL file in [`migrations/`](/Users/ksukshavasi/trapi/migrations) in filename order using `psql`.
