# example_actix_web

```shell
git clone https://github.com/erlangparasu/example_actix_web.git
```

```shell
cd example_actix_web
```

## How To Run

```shell
cargo run --release
```

<br>

> Open new terminal session and execute:

```shell
curl -X GET http://127.0.0.1:8181/\?name=World
```


## Run via Docker

```shell
docker compose build
```

```shell
docker compose up -d
```

<br>

> Open new terminal session and execute:

```shell
curl -X GET http://127.0.0.1:8181/\?name=World
```
