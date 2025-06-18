Here's a fresh `README.md` tailored to your exercise and experience, with clear steps and credit to the **Btrust Builders** program:

---

````markdown
# 🦀 Rust Week 1 Exercise — Rust for Bitcoiners

This repository contains Week 1 of the **Rust for Bitcoiners Developer Pathway** by **Btrust Builders**. It provides a hands-on introduction to Rust programming with a Bitcoin context — focusing on parsing and understanding Bitcoin transactions at the bytecode level.

---

## 🧑‍💻 Author

**Ogbu Emmanuel Otsima**  
Biomedical Engineer | Blockchain Developer | Rust Learner  
GitHub: [@otsimaofficial](https://github.com/otsimaofficial)

---

## 📚 Exercise Overview

The goal of this exercise was to:
- Clone a template project with unit tests
- Understand and implement Rust functions step-by-step
- Learn how to handle byte-level data (e.g., Bitcoin transaction hex)
- Pass all test cases and push changes to a remote fork with GitHub Actions

---

## ✅ Completed Task: `extract_tx_version`

### Function:
```rust
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String>
````

### Objective:

Extract the version number (first 4 bytes) from a raw Bitcoin transaction hex string. Handle errors like:

* Hex decode failure
* Short input strings

### My Implementation:

```rust
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    if raw_tx_hex.len() < 8 {
        return Err("Transaction data too short".to_string());
    }

    let version_hex = &raw_tx_hex[0..8];
    let bytes = hex::decode(version_hex).map_err(|_| "Hex decode error".to_string())?;
    let version = u32::from_le_bytes(bytes.try_into().map_err(|_| "Failed to convert to u32")?);
    Ok(version)
}
```

---

## 🧪 Test Output

```bash
cargo test --test unit_tests
```

✅ All 4 tests passed:

* test\_valid\_tx\_version
* test\_invalid\_hex
* test\_short\_input
* test\_version\_2

---

## ⚙️ Project Setup & Process

### 1. Fork the repo

Forked from: [btrust-builders/rust-week-1-exercises](https://github.com/btrust-builders/rust-week-1-exercises)

### 2. Clone with SSH

```bash
git clone git@github.com:otsimaofficial/rust-week-1-exercises.git
cd rust-week-1-exercises
```

### 3. Build and test locally

```bash
cargo build
cargo test -- --nocapture
```

### 4. Format and lint code

```bash
cargo fmt
cargo clippy
```

### 5. Push to remote fork

```bash
git add .
git commit -m "Implement extract_tx_version with full test coverage"
git push origin main
```

### 6. Confirm CI Success ✅

GitHub Actions runs tests automatically. A green check ✅ indicates all tests passed.

---

## 🚀 Next Steps

Continue with Week 2 of the Rust for Bitcoiners program, solving the next function with more byte manipulation and learning advanced Rust concepts.

---

## 🙏 Acknowledgment

This exercise is part of the **Rust for Bitcoiners Developer Pathway** by [**Btrust Builders**](https://github.com/btrust-builders), an initiative aimed at nurturing African open-source Bitcoin developers.

---

```

---
