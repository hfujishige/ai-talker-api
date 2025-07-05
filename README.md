# Development
## Softwares

This software need rust.

# Unit Test
## cargo libraries for unit test
- create data volume directory(one time only)
`mkdir -p container_volumes/postgres_data`

- run rdbms
start pgsql container.
```shell
./pgsql_container.sh start
```

- install cargo-llvm-cov(one time only)  
`cargo install cargo-llvm-cov`

- run unit test  
<run>  
```sh
cargo llvm-cov --workspace --html
```
or
```sh
RUST_BACKTRACE=1 cargo llvm-cov --workspace --html
```
ran unit test is output to `target-cov/html` directory.'

- run part of unit test
```sh
cargo test -- --list
cargo llvm-cov -- --test-threads=1 --include-ignored --exact \ 
      tests::restapi::api::v1::pjsip_realtime::create_account::test_create_pjsip_realtime_account
cargo llvm-cov -- --test-threads=1 --include-ignored --exact \ 
      tests::restapi::api::v1::pjsip_realtime::create_account_with_external_id::tests::test_create_pjsip_account_with_external_id_success


```

```

# DATABASE Connection String

define connection string at .env files (.env, .env.test, ...)
```sh
cargo sqlx prepare
```

or
```sh
export DATABASE_URL=postgres://username:password@localhost/dbname
cargo sqlx prepare
```