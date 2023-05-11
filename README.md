# School Event Safety Simulation

This simulation guides a student through creating an event for school, and
follow through the safety implications of it.

## Decision Tree

1. The player decides to run a "fun" event.
2. The player chooses a feature for the event.
3. There is a target fun level, and each event contributes a different amount
   to it.
4. Each event feature has its own risks, and counter measures can be added to
   the event to counter act them.
5. The player repeats adding events until the target fun level is reached.

## Building

### Dependencies

1. Install rust: https://www.rust-lang.org/tools/install/
2. Install trunk: https://trunkrs.dev/
3. Download Tailwindcss executable: https://github.com/tailwindlabs/tailwindcss/releases/

### Build

Rename the tailwind executable to tailwindcss:

```
mv tailwindcss-os-architecture tailwindcss
```

Build and serve with Trunk:

```
trunk serve
```

## License

MIT License, Copyright (c) 2023 Lachlan Wilger.
