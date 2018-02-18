# Taquin

A command line utility to solve fifteen puzzle instances.

The implementation uses a slow BFS.
The main aim of the project was to try error handling in rust.

## Installation

Install rust and clone the repo, then you will be able to install it with cargo.

```
curl https://sh.rustup.rs -sSf | sh
git clone https://github.com/PBertinJohannet/taquin
cargo install
```


## Usage

You must pass the grid to the program.
The lines are separated by colons ":" and the cells by dots  

To solve the following grid 

| 1 | 2 | 3 |

| 4 | 5 | 6 |

| 7 | . | 8 | 

```
$ taquin 1.2.3:4.5.6:7.0.8

Found solution in 17 us
Solution in 1 moves
1 : RIGHT
```

By default, the solved grids are 3x3 grid.
You can specify the size of the grid with the size argument.

```
taquin --size 2 1.2:0.3
```

or

```
$ taquin -s2 1.2:0.3

Found solution in 9 us
Solution in 1 moves
1 : RIGHT
```

To see help : 

```
taquin -h
```


or 
```
taquin --help
```

