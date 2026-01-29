# Ownership
Ownership is Rust’s core model for memory safety without a GC, based on simple rules the compiler checks at compile time.

## High-level idea

- **Ownership** is how Rust manages heap memory automatically without garbage collection.
- Compiler enforces rules at compile time; no runtime overhead from ownership checks.
- Once you “get” ownership, many Rust features (borrowing, lifetimes, etc.) become intuitive.

## Stack vs heap

- **Stack**: LIFO, fixed-size data only, fast push/pop, contiguous and cache-friendly.
- **Heap**: For data with unknown/changing size; allocator finds space and returns a pointer stored on the stack.
- Accessing heap data is slower (pointer indirection, less locality) and allocation is more expensive than stack operations.

## Ownership rules

- **Each** value has exactly one owner at a time.
- **At most** one owner; when owner goes out of scope, value is dropped and its resources freed.
- Scope = region where a binding is valid; value is valid from declaration until end of scope.

## String vs string literal

- String literal **`let s = "hello";`** is immutable, fixed, baked into the binary (fast, no heap).
- **`String`** is heap-allocated, growable, created via **`String::from("hello")`**, can be mutable with **`let mut`** and methods like **`push_str`**.
- **`String`** needs runtime allocation and deterministic cleanup when it goes out of scope.

## Memory and drop

- **`String::from`** requests heap memory; pointer + length + capacity live on stack, bytes on heap.
- When a **`String`** goes out of scope, Rust automatically calls **`drop`**, which frees its heap memory (RAII-style).
- Reassigning a variable with a new **`String`** causes the old value to be dropped immediately (original heap allocation freed).

## Move semantics

- Simple types (e.g., **`i32`**): **`let x = 5; let y = x;`** copies the value; both valid because it’s cheap stack-only data.
- For **`String`**: **`let s1 = String::from("hello"); let s2 = s1;`** moves ownership; **`s1`** becomes invalid, only **`s2`** can be used.
- Move = shallow copy of stack fields (pointer, len, capacity) + invalidation of source to avoid double free.

## Clone and Copy

- **`clone()`** performs an explicit deep copy of heap data (e.g., **`let s2 = s1.clone();`** keeps both valid).
- **`Copy`** trait: types that are small, stack-only, and trivially copyable (no custom **`Drop`**) are implicitly copied, not moved.
- Examples of **`Copy`**: all integer types, **`bool`**, all floats, **`char`**, and tuples of only **`Copy`** types.

## Ownership and functions

- Passing by value follows the same rules as assignment:
    - For **`String`**, passing into a function moves ownership; caller can’t use it afterward unless it’s returned.
    - For **`Copy`** types (e.g., **`i32`**), the value is copied; caller can still use it after the call.
- Returning values transfers ownership to the caller; combining parameters and return values can “pass ownership through” functions.

## Patterns with return values

- Function can **give** ownership (**`fn gives_ownership() -> String`**) and caller becomes the owner of the return value.
- Function can **take and give back** ownership (parameter by value, then return it), but this is verbose if done just to temporarily use a value.
- Tuple returns can bundle “value + computed data” (e.g., return **`(String, usize)`** for **`(s, s.len())`**).

## Mental model cheat lines

- **Exactly one** owner for any heap-backed value at a time.
- Assignment and function calls **move** non-**`Copy`** values by default.
- **`Copy`** = cheap stack copy; **`clone()`** = explicit, possibly expensive deep copy.
- When a value goes out of scope and still owns resources, **`drop`** runs and frees them.



# References and borrowing

References in Rust let you access data without taking ownership, enabling functions to use values without needing to return them back.

## Basic references (borrowing)

- A reference **`&T`** is like a pointer: it stores an address of data owned by someone else, and is guaranteed to be valid for its lifetime.
- Passing **`&String`** to a function lets the function read the string without taking ownership, so the caller can still use the **`String`** after the call.
- When a function parameter is a reference, nothing needs to be returned to “give back” ownership, because ownership was never taken.

## Immutable vs mutable references

- References are immutable by default: with **`&T`**, the referenced data cannot be modified through that reference.
- Attempting to call a mutating method (like **`push_str`**) on **`&String`** fails, because immutable references do not allow mutation of the underlying data.
- To mutate borrowed data, use mutable references: make the value **`mut`**, pass **`&mut value`**, and accept **`&mut T`** in the function parameter.

## Borrowing rules for mutability

- If you have one mutable reference **`&mut T`** to a value, you cannot have any other references (mutable or immutable) to that same value at the same time.
- Having two simultaneous mutable references to the same data (e.g. **`r1 = &mut s; r2 = &mut s;`**) causes a compile error (**`E0499`**).
- This restriction prevents data races at compile time:
    - multiple pointers to same data,
    - at least one writing,
    - no synchronization mechanism.

## Mixing mutable and immutable references

- You cannot have a mutable reference **`&mut T`** while there is an active immutable reference **`&T`** to the same value.
- Multiple immutable references are allowed because they only read data and cannot interfere with each other.
- A reference’s **scope** is from where it’s created to its last use; once an immutable reference is no longer used, the compiler may allow a later mutable reference since the scopes no longer effectively overlap.

## Scoping tricks to satisfy the borrow checker

- You can create a new inner scope with **`{ ... }`** so that a mutable reference goes out of scope before creating another reference.
- Example pattern:
    - create **`r1 = &mut s`** inside a block,
    - block ends, **`r1`** is dropped,
    - then create a new **`&mut s`** safely afterward.

## Dangling references

- A dangling reference is a pointer to memory that has already been freed; Rust prevents these at compile time.
- Returning **`&String`** that refers to a local **`String`** inside a function is illegal, because the local is dropped when the function ends, leaving the reference pointing to invalid memory.
- The compiler reports that the return type “contains a borrowed value, but there is no value for it to be borrowed from,” and also that you “cannot return reference to local variable.”
- Correct approach: return the owned **`String`** (just **`String`**) instead of a reference, so ownership moves out and the data remains valid.

## Summary rules cheat sheet

- At any given time: **either** one mutable reference **`&mut T`** **or** any number of immutable references **`&T`**, but not both.
- References must always be valid: they cannot outlive the data they point to, so Rust forbids returning references to local data that will be dropped.