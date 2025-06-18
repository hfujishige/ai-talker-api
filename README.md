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
`cargo llvm-cov --workspace --html`  
ran unit test is output to `target-cov/html` directory.'

