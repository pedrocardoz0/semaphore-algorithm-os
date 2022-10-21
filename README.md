# Algoritmo Semáforo

Este projeto tem como objetivo implementar o algoritimo semaforo binario proposto por Dijkstra em 1965.

## Objetivo

Impedir que threads paralelas sejam executadas ao mesmo tempo, ao utilizar o algoritmo o programa deve executar as threads sequencialmente.

## Setup

Para executar o arquivo binario basta executar:

```shell
$ ./target/debug/bank_thread
```

Caso queira instalar o rust rode: 
```shell
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Para rodar o projeto usando o rust:

```
$ cargo run
```

## Output

<img src="./public/thread_1.png"/>
<img src="./public/thread_2.png"/>