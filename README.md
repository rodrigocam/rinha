# rinha

Tree-Walking Interpreter implemented in Rust.

## Como executar

First build the container:

```sh
cd rinha
docker build . -t="rinha"
```

Finally execute:
```sh
docker run -it rinha examples/fib.json
```

## Language Features

- [x] Call
- [x] Function
- [x] Let
- [x] Var
- [x] Int
- [x] Str
- [x] Binary
- [x] If
- [x] Print
- [x] First
- [x] Second
- [x] Bool
- [x] Tuple
