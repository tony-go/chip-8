# CHIP-8

Emulator of CHIP-8 CPU.

_Written in the context of "Rust In Action" book_.

## Run

```text
$ cargo run
```

Output should be:

```text
Creating CPU...
position_in_memory: 0
stack_pointer 0
OP CALL
position_in_memory: 100
stack_pointer 1
OP ADD
position_in_memory: 102
stack_pointer 1
OP ADD
position_in_memory: 104
stack_pointer 1
OP RET
position_in_memory: 2
stack_pointer 0
OP CALL
position_in_memory: 100
stack_pointer 1
OP ADD
position_in_memory: 102
stack_pointer 1
OP ADD
position_in_memory: 104
stack_pointer 1
OP RET
position_in_memory: 4
stack_pointer 0
OP END
Succeed!
```
