# Brainfuck Interpreter
A simple brainfuck interpreter written in Rust. Also it can transpile to C, but it doesn't check for errors.
## Build
```
    cargo build --release
```
## Usage
To run the interpreter:
```
    ./bf <sourceFile>
```
To transpile to c:
```
    ./bf --transpile test.bf
```
