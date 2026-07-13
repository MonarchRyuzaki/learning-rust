# Rust Ownership, Borrowing, and References

This document outlines the core memory management model in Rust, based on the concepts of Ownership, Borrowing, and References.

## 1. The Ownership Model
- **Heap vs Stack:** For complex types like `String`, the stack holds the metadata (pointer to heap, length, and capacity), while the actual data lives on the heap.
- **Move Semantics:** When you assign a heap-allocated variable to another (e.g., `s2 = s1`) or pass it to a function `fn1(s1)`, ownership is transferred (moved). The original variable `s1` becomes invalid and cannot be used again.
- **Single Owner:** There can only be one owner of data at a time.
- **Drop (Cleanup):** When the owner goes out of scope, Rust automatically drops it and clears the associated heap memory.

## 2. Borrowing & References
Instead of moving ownership, you can **borrow** data using references. 

**The Golden Rule of Borrowing:**
At any given time, for a specific piece of data, you can have EITHER:
- Exactly **one** mutable reference (`&mut T`)
- **OR** any number of immutable references (`&T`)

You can NEVER have a mutable reference and an immutable reference active at the exact same time.

## 3. Edge Cases & The "Locking" Mechanism

The "owner" acts as a manager. When data is borrowed, the owner's own permissions are temporarily restricted depending on the type of borrow.

### Situation 1: Mutable Owner, Immutable Borrow
If you have a mutable owner and create an immutable reference, the owner is **frozen**.
```rust
let mut s1 = String::from("hello");
let r1 = &s1; // Immutable borrow

// s1.push_str(" world"); // ❌ ERROR: Cannot mutate `s1` while `r1` is borrowing it.

println!("{}", r1); // Last use of r1
s1.push_str(" world"); // ✅ OK: r1 is no longer in use, s1 is unfrozen.
```

### Situation 2: Mutable Owner, Mutable Borrow
If you have a mutable owner and create a mutable reference, the owner is **completely locked**. You cannot read or write using the owner until the mutable reference is done.
```rust
let mut s1 = String::from("hello");
let r1 = &mut s1; // Mutable borrow

r1.push_str(" world"); // ✅ r1 can mutate the data

// println!("{}", s1);     // ❌ ERROR: Cannot read s1 while mutably borrowed
// s1.push_str(" again");  // ❌ ERROR: Cannot write to s1 while mutably borrowed

println!("{}", r1); // Last use of r1
println!("{}", s1); // ✅ OK: s1 regains its permissions
```

### Situation 3: Immutable Owner, Mutable Borrow
An immutable owner cannot grant mutable references.
```rust
let s1 = String::from("hello");

// let r1 = &mut s1; // ❌ ERROR: Cannot borrow an immutable variable as mutable.
```
