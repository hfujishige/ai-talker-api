# how to run

1. start pgsql container when it is not running:

```bash
./pgsql_container.sh start
```

2. Export the `DATABASE_URL` environment variable when it is not set:

```bash
DATABASE_USER=api_user_rw
DATABASE_PASSWD=HE4ycm8uCER3
DATABASE_CATALOG=asterisk
DATABASE_HOST=127.0.0.1
DATABASE_PORT=25432
export DATABASE_URL="postgres://${DATABASE_USER}:${DATABASE_PASSWD}@${DATABASE_HOST}:${DATABASE_PORT}/${DATABASE_CATALOG}"
```

3. Run all tests with coverage:

```bash
cargo llvm-cov
```
