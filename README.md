# TypeScript Union Benchmarking

Generates humongous TypeScript union for testing

Find out more at <insert article here>!

## Usage

### Creating the TS file

Change `POWER` in `main.rs` to the desired value, then run the binary to create a TypeScript to be typechecked

```bash
cargo run
```

### TypeChecking

`npm run typecheck` and `npm run typecheck:go` will typecheck with `tsc` and `tsgo` respectively. You may need to increase the allotted RAM for `tsc` if you run out

### timing

Use `/usr/bin/time -v <command>` for any commands you'd like to benchmark

For example, this benchmarks only typechecking

```bash
cargo run && (cd ts-playground && /usr/bin/time -v npm run typecheck)
```
