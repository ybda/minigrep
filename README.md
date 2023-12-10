# Usage
```
Contents of file example.txt:
Rust:
safe, fast, productive.
Pick three.
Duct tape.
Trust me.

$ ./minigrep example.txt rust
Lines containing query `rust` in file `example.txt` (case sensetive):
- Trust me.

$ ./minigrep example.txt rust i
Lines containing query `rust` in file `example.txt` (case insensetive):
- Rust:
- Trust me.
```