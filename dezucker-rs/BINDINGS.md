# dezucker C Bindings

### Prerequisites
- Rust toolchain (cargo, rustc)

### Generation
Execute the generation script from the crate root:
```bash
./scripts/generate-bindings.sh
```
This produces `dezucker.h` and library artifacts in `target/release/`.

### Linking
Link against the generated library using GCC:
```bash
gcc main.c -o main -L./target/release -ldezucker
```
Ensure `dezucker.h` is in the include path.
