## Work item

- ID:
- Arch / extension:
- Depends on:
- Blocks:

## Scope

- [ ] Implemented or designed:
- [ ] Intentionally excluded:
- [ ] Deferred:

## Implementation

- C files:
- Rust modules:
- Value types:
- Feature attributes:
- Immediates or masks:

## Tests

- Host:
- QEMU:
- wasmtime:
- Compile-only:
- Symbol check:
- `make fmt-check`:
- `make test`:
- `make coverage`:

## Zero-cost

- Confirmed instructions:
- `call` absent:
- No store/load between instructions:
- Not confirmed:

## Docs

- [ ] Arch README table updated
- [ ] rustdoc updated
- [ ] `docs/zero-cost.md` updated or not needed
- [ ] `docs/<arch>/feasibility.md` updated or not needed
- [ ] Scalable/stateful ISA-to-API mapping updated or not needed

## Checklist

- [ ] No `core::arch`
- [ ] C features are a subset of Rust features
- [ ] Endianness and NaN behavior documented
- [ ] Panics documented
- [ ] `Depends on:` merged
