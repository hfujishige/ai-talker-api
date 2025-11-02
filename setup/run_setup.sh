#!/usr/bin/env zsh

psql_user="postgres"
psql_db="asterisk"
psql_host="127.0.0.1"
psql_port="5432"

#　run create database users sql 
script_dir="$(cd "$(dirname "$0")" >/dev/null 2>&1 && pwd)"
sql_file="$script_dir/20250410150000_create_database_users.sql"

if [ ! -f "$sql_file" ]; then
    echo "SQL file not found: $sql_file" >&2
    exit 1
fi

psql -U "$psql_user" -h "$psql_host" -p "$psql_port" -d "$psql_db" -f "$sql_file"

# check created users and permissions
psql -U $psql_user -d $psql_db -h $psql_host -p $psql_port <<EOSQL

-- check created users are created.
\du

-- check table permissions
\dp

EOSQL

echo "Database users created and permissions checked."
