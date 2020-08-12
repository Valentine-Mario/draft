# Draft

* Draft is a mini LLVM based compiler with an inbuilt REPL built with the Rust programming language

- With draft you can perfom simple maths operaqtions, define variable and conditionals. To enter the repl interactive mode, provide no argument. To run a draf file, run draft file.drf

- Some helpful commands in repl mode include: "load" used to load a draft script to memory, "history" to view your repl program history, "clear" to clear your repl program history, "quit" to exit the repl

#### Operations
Simple maths operations include:

```
2+7
>>> 9
2*3+(8)
>>>14

```

For numerical increment/decrement

```
a= 45
inc a

>>> 46

b=  10
dec b
>>> 9
```

To get the square of numbers

```
a=2
square 2
>>> 4

```

#### Conditionals

The if block exedcutes if the condition is equal zero

```
a=0
b=1
if a{
    b=10
}else{
    b=100
}
b
>>>10
```