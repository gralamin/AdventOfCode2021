# AdventOfCode2021
Advent of Code for 2021 - https://adventofcode.com/2021

## Creating a new date executable

Per day, remember to:
```
export day=day25
cargo new $day
cp day01/Makefile $day/
touch $day/README.md
touch $day/input
touch $day/src/lib.rs
git add $day
git commit -m "$day: Added template"
```

By convention for this repo, so I can ignore it, all programs will be called `<foldername>.day` eg `day01.day`.

To format code, call:

```
make format
```

## Dependencies

To make a new lib:

```
cargo new --lib foo
```

Then you can refer to that lib in the Cargo.toml:

```
[dependencies.my_lib]
path = "../my_lib"
```

And in the code use
```
extern crate my_lib;
```

*Note*: Libs use a slightly different Makefile (no copy)

## Lib list

* `filelib` - File input/output helper methods to do with basic types (eg, not types in other libraries)
* `submarinelib` - Provides structs to represent a Submarine, where it is, its movement, etc.
* `ivec3` - a vector `x,y,z` for math operations.
* `boardlib` - Handles a 2 dimensional board and coordinate system that can be used with various types.
