# Temperature Converter — Explained for a Python Developer

> **TL;DR** — A tiny Rust command-line tool that reads `100 C F` and prints
> `100 °C = 212 °F`. This doc explains every piece in Python terms, lists the fixes
> already made, and sketches optional "next level" upgrades. Run it with `cargo run`,
> test it with `cargo test`.

This document walks through the `temp_convert` program: what it does, every Rust
concept it uses (with Python comparisons), the improvements already made, and the
"next level" ideas worth knowing.

The program reads one line like `100 C F` from the keyboard and prints
`100 °C = 212 °F`.

---

## 0. Quick Glossary (Rust ↔ Python)

A cheat-sheet for the terms used below. Skim it now, refer back as needed.

| Rust term | Closest Python idea | One-line meaning |
|-----------|---------------------|------------------|
| `let x = 5` | `x = 5` | Bind a value to a name (immutable by default) |
| `let mut x` | `x = ...` (normal var) | A variable you're allowed to change |
| `fn` | `def` | Define a function |
| `f64` | `float` | A decimal number |
| `&str` / `String` | `str` | Borrowed text vs owned text |
| `enum` | `Enum` / fixed constants | A type with a fixed set of variants |
| `struct` | `class` (data only) | A bundle of named fields |
| `match` | `if/elif/else` | Branch on a value; must cover every case |
| `Result<T, E>` | return value *or* exception | Success (`Ok`) or failure (`Err`) |
| `Option<T>` | `T` or `None` | A value that might be absent |
| `?` | propagating an exception | "Unwrap, or return the error early" |
| `panic!` | uncaught exception | Crash with a message |
| `&x` | (implicit in Python) | Borrow a reference without taking ownership |
| `impl` | methods inside a `class` | Attach behavior to a type |
| `trait` | duck typing / `Protocol` / ABC | A shared set of methods a type can implement |
| `Display` | `__str__` | Human-facing text form |
| `Debug` | `__repr__` | Developer-facing text form |
| `println!` | `print(f"...")` | Print with `{}` placeholders |
| `macro!` (the `!`) | (no direct equal) | Code that expands at compile time |
| `const` | module-level constant | A value fixed at compile time |
| `cargo` | `pip` + `venv` + test runner | Build/run/test/dependency tool |
| `crate` | package / library | A unit of Rust code you can depend on |

---

## 1. The Big Picture

In Python you might write the whole thing as:

```python
def convert(temp, frm, to):
    ...
temp, frm, to = input().split()
print(convert(float(temp), frm, to))
```

Rust does the same job but forces you to be explicit about **types** and about
**what can go wrong**. That extra strictness is the whole point — the compiler
catches bugs before the program ever runs.

---

## 2. Concept-by-Concept (Rust vs Python)

### `enum` — a fixed set of choices

```rust
enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}
```

This says "a temperature unit can only ever be one of these three." In Python
you might use string constants `"C"`, `"F"`, `"K"` or an `Enum`. The Rust version
is safer: you literally cannot create a fourth, invalid unit. Typos become
compile errors instead of runtime surprises.

### `match` — like a supercharged `if/elif/else`

```rust
match from_unit {
    TemperatureUnit::Fahrenheit => (temp - 32.0) * 5.0 / 9.0,
    TemperatureUnit::Kelvin => temp - KELVIN_OFFSET,
    _ => temp,
}
```

Think of `match` as Python's `if/elif/else` chain, but the compiler **forces you
to handle every case**. The `_` is the catch-all, like a final `else`. If you
forget a case, the code won't compile. That's a feature — no silently-missed
branches.

### `fn` and types on every argument

```rust
fn convert_temp(temp: f64, from_unit: &TemperatureUnit, to_unit: &TemperatureUnit) -> f64 {
```

`f64` is a 64-bit floating-point number (Python's `float`). Unlike Python, you
must declare the type of every parameter and the return type (`-> f64`). The `&`
means "a borrowed reference" — you're lending the value, not giving it away
(more below).

### `const` — a named constant

```rust
const KELVIN_OFFSET: f64 = 273.15;
```

Instead of sprinkling the magic number `273.15` everywhere, we name it once.
Same idea as `KELVIN_OFFSET = 273.15` at the top of a Python module, but Rust
guarantees it can never be changed.

### The "pivot" design — avoid repeating yourself

There are 3 units, so there are 9 possible conversions. Writing all 9 formulas
would mean repeating the same math many times. Instead we use a trick:

```rust
fn convert_temp(temp, from_unit, to_unit) -> f64 {
    from_celsius(to_celsius(temp, from_unit), to_unit)
}
```

Every conversion goes **through Celsius**: first turn any unit into Celsius, then
turn Celsius into the target. Two small helper functions cover everything, and
each formula is written exactly once. Fewer copies = fewer places for bugs to hide.

Here's the flow. Instead of drawing arrows between every pair of units (9 arrows),
everything funnels through one middle point:

```
   WITHOUT the pivot (9 direct conversions):

      C ─────────► F          every pair needs its own
      C ─────────► K          formula = lots of repeated
      F ─────────► C          math = lots of places to
      F ─────────► K          introduce a bug
      K ─────────► C
      K ─────────► F
      (…and the 3 "same unit" cases)


   WITH the pivot (any unit → Celsius → any unit):

        any input                       any output
      ┌───────────┐                   ┌───────────┐
      │ °F  °K  °C│                    │°C  °F  °K │
      └─────┬─────┘                    └─────▲─────┘
            │                                │
            │  to_celsius(temp, from_unit)   │  from_celsius(temp, to_unit)
            │                                │
            └──────────►  °Celsius  ─────────┘
                         (common hub)

   convert_temp = from_celsius( to_celsius( value, from ), to )
```

Reading the bottom line: take the value, convert it **to** Celsius using its
source unit, then convert that Celsius value **from** Celsius into the target unit.
If the unit is already Celsius, the helper's catch-all (`_ => temp`) just passes the
number straight through unchanged. One hub, two short steps, every case covered.

### `Result` — values that might be an error

```rust
fn convert_to_unit(input_unit: char) -> Result<TemperatureUnit, String> {
    match input_unit {
        'C' => Ok(TemperatureUnit::Celsius),
        ...
        _ => Err("Wrong Temperature Unit Input".to_string()),
    }
}
```

This is the biggest mental shift from Python. In Python a function that can fail
usually **raises an exception**. In Rust, the function **returns** a `Result`:
either `Ok(value)` (success) or `Err(message)` (failure). The caller is forced to
deal with both. Nothing blows up unexpectedly — errors are part of the type.

### The `?` operator — "unwrap or bail out"

```rust
let from = convert_to_unit(from.parse::<char>()?)?;
```

The `?` is shorthand: "if this is `Ok`, give me the value inside; if it's `Err`,
stop this function immediately and return that error." It replaces a big
`match`/`try-except` block with a single character. The closest Python feeling is
letting an exception propagate up — but here it's explicit and visible.

For `?` to work, the function must itself return a `Result`. That's why `main`
is declared as:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
```

`Box<dyn std::error::Error>` just means "any kind of error" — a flexible catch-all
error type so different failures (bad number, bad unit) can all flow out the same way.

### `let ... else` — validate shape and bail early

```rust
let [temp, from, to] = parts.as_slice() else {
    return Err("Wrong number of inputs".into());
};
```

This says: "the input must be exactly three pieces — name them `temp`, `from`,
`to`. If it isn't three, run the `else` block." It's pattern-matching the *shape*
of the data. In Python you'd write `if len(parts) != 3: ...` followed by manual
indexing. The Rust version does the length check and the unpacking in one move,
and the `else` must exit (here, by returning an error).

### `&` and borrowing — Rust's memory model

`&from` means "lend a reference to `from`" instead of handing over ownership.
Python never makes you think about this because it manages memory with a garbage
collector. Rust has no garbage collector; instead it tracks who "owns" each value.
Borrowing with `&` lets a function read a value without taking it, so you can keep
using it afterward. That's why `convert_temp(..., &from, &to)` borrows the units —
we still want to print `from` and `to` on the next line.

### `Display` vs `Debug` — two ways to print

```rust
#[derive(Debug)]            // auto-generated developer-facing format
enum TemperatureUnit { ... }

impl std::fmt::Display for TemperatureUnit {   // human-facing format
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemperatureUnit::Celsius => write!(f, "°C"),
            ...
        }
    }
}
```

- `#[derive(Debug)]` auto-creates a "for programmers" printout, used with `{:?}`.
  Think of Python's `repr()`.
- `Display` is the "for humans" printout, used with `{}`. Think of Python's
  `__str__`. We wrote it by hand so the units show as `°C`, `°F`, `K`.

So `println!("{}", unit)` prints `°C`, while `{:?}` would print `Celsius`. Two
audiences, two formats.

### `println!` and the `!`

The `!` means it's a **macro**, not a regular function. `println!("{} = {}", a, b)`
fills each `{}` with the matching argument — very similar to Python's
`print(f"{a} = {b}")`. The format string with `{}` placeholders is required.

---

## 3. Tests — proving the code works

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_freezing_point_conversion() {
        assert!((convert_temp(0.0, &Celsius, &Fahrenheit) - 32.0).abs() < 1e-9);
    }
}
```

- `#[cfg(test)]` means "only compile this when running tests" — it's stripped out
  of the real program. Similar to a `tests/` folder in Python, but built into the
  language.
- `#[test]` marks a function as a test. Run them all with `cargo test`.
- `use super::*;` pulls in the functions from the file above so the tests can call
  them.
- `assert!(...)` is like Python's `assert`.

**One important gotcha — never compare floats with `==`.** Because of how
computers store decimals, `25.0` after a round-trip might come back as
`25.000000000004`. So instead of `assert_eq!(result, 25.0)`, we check the
difference is tiny: `(result - 25.0).abs() < 1e-9` ("within a hair's breadth").
Same advice applies in Python (`math.isclose`).

The five tests cover:
1. Freezing point: `0 °C → 32 °F`
2. Boiling point: `100 °C → 212 °F`
3. Absolute zero: `0 K → -273.15 °C`
4. Round-trip: `C → K → C` returns the original (catches sign-flip bugs)
5. Error path: an unknown unit like `'X'` correctly returns an error

---

## 4. Improvements Already Made

The program started broken and was rebuilt step by step. Here's the journey:

| Step | What changed | Why it matters |
|------|--------------|----------------|
| **Compile fixes** | Fixed a broken `println!`, undefined variable names, a misspelled enum (`Fahrenheint`) | The code couldn't even build before |
| **Correct parsing** | Each input piece (`parts[0]`, `[1]`, `[2]`) parsed to the right type | It was reusing the same index three times |
| **String → unit bridge** | Added `convert_to_unit` returning a `Result` | Turns user text into a real enum, safely |
| **Graceful errors** | Replaced crashes with `Result` handling and the `?` operator | Bad input no longer panics with an ugly stack trace |
| **Removed duplicate math** | Collapsed 9 conversion cases into the "pivot through Celsius" design | One formula per direction, written once |
| **Named constant** | Introduced `KELVIN_OFFSET` | No more magic `273.15` scattered around |
| **`main` returns `Result`** | Enabled the `?` operator everywhere | Errors flow out cleanly; no `.expect()` panics |
| **`let ... else` validation** | Checks the input is exactly 3 parts | Typing just `100` no longer crashes on a bad index |
| **`Display` formatting** | Units print as `°C` / `°F` / `K` for humans | Friendlier output than the developer `Debug` format |
| **Test suite** | Five tests including a round-trip and an error case | The logic is now *proven*, not just *hoped* |

---

## 5. "Next Level" Ideas (Not Yet Done)

These are bigger design ideas. They add power **and** complexity, so they're worth
doing only if the program grows beyond a small script. Listed roughly by how much
they teach versus how much they cost.

### A. Make illegal states impossible — a `Temperature` type

Right now a temperature is just a bare `f64`. Nothing stops you from passing a
Celsius number but labeling it Kelvin:

```rust
convert_temp(100.0, &Kelvin, &Celsius)  // is 100.0 really Kelvin? Nothing enforces it.
```

The idea: bundle the number **and** its unit together into one type (a `struct`
that holds both a value and a `TemperatureUnit`). Then conversion becomes a method
on that bundle, and the value can never be separated from its unit. The compiler
enforces the truth. This is the single most valuable concept here:
**"make illegal states unrepresentable."**

In Python terms, it's like turning a loose `(value, unit)` tuple into a proper
`class Temperature` that always knows its own unit.

### B. Reject physically impossible temperatures

Nothing can be colder than absolute zero (`0 K`, or `-273.15 °C`). A stricter
design would validate the value when it's created and return an error for, say,
`-300 °C`. Then impossible data simply never exists inside the program. This is
"defend the boundary, trust the inside."

### C. Real error types instead of `String`

Errors are currently plain text (`Result<_, String>`). Text errors can't be
inspected or handled programmatically — the caller can only print them. A better
design uses a dedicated error `enum` (e.g. `UnknownUnit(char)`,
`BelowAbsoluteZero`) so callers can `match` on exactly what went wrong. Comparable
to defining custom exception classes in Python instead of `raise Exception("...")`.

### D. Split into a library + a thin program

Right now `main.rs` mixes two jobs: the conversion math **and** reading/printing
to the terminal. A cleaner structure puts the math in a separate library file
(`lib.rs`) and keeps `main.rs` as a thin shell that only handles input/output.
Then the conversion logic becomes reusable by other programs and easier to test.
Same spirit as separating "business logic" from "the CLI script" in Python.

### E. Property-based testing

The five tests check specific examples. A more thorough approach
(using a crate like `proptest`) generates **thousands of random inputs** and
checks a rule holds for all of them — for example, "converting any value from A to
B and back always returns the original." It hunts for the one weird input that
breaks your code. Python's equivalent is the `hypothesis` library.

### F. The most senior skill — knowing when to stop

For a small learning script, most of section 5 is over-engineering. A key part of
good engineering judgment is recognizing when a program is "good enough for its
purpose" and not gold-plating it. This converter — correct, tested, with friendly
errors and output — is already complete for what it set out to do.

---

## 6. How to Run It

```bash
# Run the program (then type e.g. "100 C F" and press Enter)
cargo run

# Run the tests
cargo test
```

Input format: a number, the source unit, and the target unit, separated by spaces.
Units are the single letters `C`, `F`, or `K`. Example:

```
100 C F   ->   100 °C =  212 °F
```
