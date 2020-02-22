OGC Api - Features implementation for inclusion into [t-rex](https://github.com/t-rex-tileserver/t-rex)

Create .env file:
```
SERVER_ADDR=127.0.0.1:8080
PG.USER=t_rex
PG.PASSWORD=t_rex
PG.HOST=127.0.0.1
PG.PORT=5439
PG.DBNAME=t_rex_tests
PG.POOL.MAX_SIZE=16
```

Start test DB:

    docker run -p 127.0.0.1:5439:5432 -d --name trextestdb --rm sourcepole/trextestdb
