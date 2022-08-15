# fls-bench

Benchmarks for FLS and Yen algorithm

## Where the code come from?

The implementation of Yen's algorithm can be found [here](https://github.com/TiagoCavalcante/yen).

The implementation of the fixed length search can be found [here](https://github.com/TiagoCavalcante/fixed-length-search).

## How to build?

```sh
$ cargo build --release
```

## How to run?

```sh
$ ./target/build/fls-bench > times
```

## What is the difference?

As you can see FLS is waaay faster than Yen's algorithm for finding paths with a specif length in unweighted graphs:

![Comparsion of Yen's algorithm and FLS](https://user-images.githubusercontent.com/62714153/184565536-e835e754-f3be-4c63-960b-d5f52bcba6ab.svg)
