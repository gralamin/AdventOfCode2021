# Decompiled input

We need to try and understand our input here to get a speed increase

Each block starts with:

```
inp w
mul x 0
add x z
mod x 26
```

This means we get a number in, and immediately set x to z, but a digit within 0-25, eg:

```
w = input()
x = z % 26
```

We then see either:

```
div z 1
or
div z 26
```

Lets ignore this for now, because we don't use z for a bit, and it might become more clear if we re-arrange the code.

The next steps are:

```
add x {c}
eql x w
eql x 0
```

Where {c} is a constant, that I'm not sure what its for, but this basically means:

```
x = if (x + c) != w {1} else {0}
```

We then get into y doing things:

```
mul y 0
add y 25
mul y x
add y 1
```

Pretty simple:

```
y = 25 * x + 1
```

The rest of the block is:

```
mul z y
mul y 0
add y w
add y {d}
mul y x
add z y
```

Where {d} is a new constant. This Translates to:

```
z = z * y
y = (w + d) * x;
z = z + y;
```

All together the code per block is (with some rearrangement):

```
w = input()
x = z % 26
x = if (x + c) != w {1} else {0}
y = 25 * x + 1
z = z / 1 or z / 26
z = z * y
y = (w + d) * x;
z = z + y;
```

We can probably simplify this a bit:

```
w = input()
x = int((z % 26) + c != w)  // converts bool to int
z = z / a
z = 25zx + z + xw + xd
```

Lets think through this:
- We get an input number
- We set x to be in range 0-26, apply a magic number, and check if it matches w. If it doesn't, its a 1, if it does, its a 0.
- We then divide z by either 1 (no op) or 26
- z is then set depending highly on x
   * if x is a 1, its multiplied to 25 times whatever value it has, 1 is added, and then w and and some constant are added
   * if x is a 0, its set to 1.

What if we keep track of what z is each iteration, in a trial run, compared to the number put in?
If we can somehow treat the number we put in as a variable instead