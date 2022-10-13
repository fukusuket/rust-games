# Reversi
Reversi games for personal Rust study.

## How to use

```
./reversi
```

## Input

```
  1 2 3 4 5 6 7 8
1 - - - - - - - -
2 - - - - - - - -
3 - - - - - - - -
4 - - - 0 X - - -
5 - - - X 0 - - -
6 - - - - - - - -
7 - - - - - - - -
8 - - - - - - - -

Xのポイント:2 0のポイント:2
Xのじゅんばんです

こまをおく、たてのかずをおしえてください
3
こまをおく、よこのかずをおしえてください
4
```

## Output

```
  1 2 3 4 5 6 7 8
1 - - - - - - - -
2 - - - - - - - -
3 - - - X - - - -
4 - - - X X - - -
5 - - - X 0 - - -
6 - - - - - - - -
7 - - - - - - - -
8 - - - - - - - -

Xのポイント:4 0のポイント:1
0のじゅんばんです
```


## Acknowledgements
This Game was created by customizing the following repository.
- [rust-reversi](https://github.com/KOBA789/rust-reversi) repository by @KOBA789
