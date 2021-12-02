# AdventOfCode2021
Advent of Code for 2021

Per day, remember to:
```
export day=day02
cargo new $day
cp helloworld/Makefile $day/
touch $day/README.md
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