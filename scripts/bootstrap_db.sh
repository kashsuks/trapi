#!/usr/bin/env bash

set -euo pipefail

usage() {
  cat <<'EOF'
Usage:
  scripts/bootstrap_db.sh <database_url>

Example:
  scripts/bootstrap_db.sh postgres://user:pass@host:5432/dbname

Notes:
  - Requires `psql` to be installed and available on PATH.
  - Applies every `.sql` file in `migrations/` in filename order.
EOF
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

if [[ $# -ne 1 ]]; then
  usage
  exit 1
fi

if ! command -v psql >/dev/null 2>&1; then
  echo "error: psql is required but was not found on PATH" >&2
  exit 1
fi

database_url="$1"
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/.." && pwd)"
migrations_dir="${repo_root}/migrations"

if [[ ! -d "${migrations_dir}" ]]; then
  echo "error: migrations directory not found at ${migrations_dir}" >&2
  exit 1
fi

shopt -s nullglob
migration_files=("${migrations_dir}"/*.sql)
shopt -u nullglob

if [[ ${#migration_files[@]} -eq 0 ]]; then
  echo "error: no .sql migration files found in ${migrations_dir}" >&2
  exit 1
fi

echo "Applying ${#migration_files[@]} migration(s) to target database..."

for migration_file in "${migration_files[@]}"; do
  echo "-> $(basename "${migration_file}")"
  psql "${database_url}" -v ON_ERROR_STOP=1 -f "${migration_file}"
done

echo "Schema bootstrap complete."
