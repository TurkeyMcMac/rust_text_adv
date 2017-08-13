# rust_text_adv
A simple text adventure parser

## Notation
`` @ `stage name (internal)` `Description.` ``

`` - `option name` `option destination (stage)` ``

Text that does not match one of these patterns is ignored.

#### Example
```
@ `BEGIN` `This begins an adventure. The name "BEGIN" is required.`
    - `foo` `#1`
    - `bar` `#2`
    - `baz` `#3`
@ `#1` `First.`
    - `Bar` `#2`
    - `Baz` `#3`
@ `#2` `Second.`
    - `FOO` `#1`
    - `BAZ` `#3`
@ `#3` `Third.`
    - `f00` `#1`
    - `b4r` `#2`
    - `end` `END` This is the end. The name "END" is not required.
    It can be any name which is not defined elsewhere.
```
