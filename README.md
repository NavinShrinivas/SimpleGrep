# SimpleGrep 


## About prpject 

This project aims to be grep, simply written in rust. It's main aim is to be as fast as or faster than Grep.

## Building project 

You have to have the cargo toolchain preisntalled for building this project.
```bash
$ cargo build
$ time ./target/debug/simple_grep Gutenberg ./large_text.txt
```
> Note : without the release build, speeds are not great, hence this is they way to test your optimisations L 
```bash
$ cargo build --release
$ time ./target/release/simple_grep Gutenberg ./large_text.txt
```
## Maintainer 
[Navin Shrinivas](https://github.com/NavinShrinivas)


## About speed

> WOOOOOOOOOOT!! After building a release build with compiler optimisations SimpleGrep was just ~0.02 seconds slower.

A very simple and modified implementation of GREP in rust. Sadly SimpleGrep is not as fast as grep, why so? 
- GNU grep has the ability to skip kernel buffers to read text files directly, this alone take 0.3 second of 0.6 (in below screen shots).
- It implements a superior boyer-moores algorithm 
- It was written by far far experienced developers
- lastly, SimpleGrep is still under developments with main focus of higher speeds.

That given, SimpleGrep as of now borrows tricks from grep for speed like : 
- Using one large buffer to store the input file
  - If the program were to be memory efficient, we would have to split it into lines and searching for newlines takes time.

More avenues of speed up : 
- reduce number of instruction for each unmatch
- refactor code for reused values inorder to not calculat them everytime.

## Screenshots 
![image](./1.png)
![image](./2.png)
![image](./3.png)
![image](./4.png)

## Other WIP (apart from speed that is) 
- Implementing strict search 
- Implementing recurse search
