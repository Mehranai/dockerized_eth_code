# dockerized_eth_code
This is ( ETH MultiThread + ETH Node run (Reth) + Docker Reth + Docker ETH code )

## Create an Network for both Node + Clickhouse

```command
docker network create blockchain-net
```

## Run ETH Node

```command
cd node
docker compose up -d
docker logs -f reth-node
```
## Run ETH App + Clickhouse

```command
cd app
docker compose up -d --build
docker logs -f eth-app
```

## Test if Node and Clickhouse is running acurately

### RPC Ethereum
```command
curl http://localhost:8545
```

### ClickHouse
```command
curl http://localhost:8123
```





