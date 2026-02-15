# Cargo

```sh
cargo install sqlx-cli
```

# Database Migration

## Create Database. 
run these commands at project root directory.
```sh
db_scheme=postgres
db_host=127.0.0.1
db_port=25432
db_user=api_user_rw
db_pwd=q4p05yOt9V9g
db_catalog=asterisk

export DATABASE_URL=${db_scheme}://${db_user}:${db_pwd}@${db_host}:${db_port}/${db_catalog}
echo "DATABASE_URL=${DATABASE_URL}"
sqlx database create
```

## Generate Migration
### migration flow
1. generate migration file.
2. write the sql statement(s) in the migration file.
3. run the migration.
4. check the migration status.

1 . Create the migration file.  
run this command at project root directory.
```sh
sqlx migrate add create_users
Creating migrations/20250410152814_create_user_table.sql
```

2. write the migration file. create the users table statement.
```sql
DROP TABLE IF EXISTS users;
CREATE TABLE users (
   id SERIAL PRIMARY KEY,
   login_id VARCHAR(50) NOT NULL,
   name VARCHAR(100) NOT NULL,
   email VARCHAR(320) NOT NULL,  -- RFC 5321, 5322 max length
   password VARCHAR(255) NOT NULL,
   created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
   updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_username ON users(name);
CREATE INDEX idx_email ON users(email);
```

3. Run the migration.  
run this command at project root directory.
```sh
sqlx migrate run
```
