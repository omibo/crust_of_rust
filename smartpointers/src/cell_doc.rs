/*!
# Understanding `Cell<T>` in Rust for Interior Mutability

In Rust, variables are immutable by default, and the ownership model enforces strict rules to ensure safety and prevent data races. However, there are scenarios where you might need to mutate data even when you have only an immutable reference to it. Rust provides interior mutability patterns to handle such cases, and one of the tools for this purpose is the `Cell<T>` type from the `std::cell` module.

This document explores how `Cell<T>` works, its use cases, limitations, and provides examples to illustrate its behavior.

## What Is `Cell<T>`?

`Cell<T>` is a container that allows for interior mutability of types that implement the `Copy` trait. It enables you to get and set values inside the `Cell` even when you have only a shared (immutable) reference to it. This is particularly useful when you need to mutate data in a context where the Rust borrow checker would normally prevent it.

## Limitations and Safety Considerations

While `Cell<T>` provides flexibility, it's important to understand its limitations:

- **Not Thread-Safe**: `Cell<T>` is **not** safe to use across multiple threads. If two threads attempt to access and modify the same `Cell`, it can lead to race conditions because `Cell` does not enforce any synchronization mechanisms.

  > **Note**: If you need thread-safe interior mutability, consider using `Mutex<T>` or atomic types.

- **No Mutable References**: You cannot obtain a mutable reference (`&mut T`) to the value inside a `Cell`. This design prevents aliasing mutable references, which could violate Rust's safety guarantees.

## Using `Cell<T>` in Single-Threaded Contexts

Since you cannot get a mutable reference to the value inside a `Cell`, it is safe to use in single-threaded contexts. The inability to obtain `&mut T` ensures that mutations are controlled and that the usual aliasing rules are upheld.

*/

use std::cell::Cell;
use std::rc::Rc;

///
/// ### Example: Shared Configuration with `Cell<T>`
///
/// Consider a scenario where you have a shared configuration that multiple parts of your program need to access and modify. You can wrap your configuration struct in an `Rc` (Reference Counted pointer) for shared ownership and use `Cell<T>` for fields that require interior mutability.
///
struct Config {
    /// The `feature_enabled` field can be modified even when `Config` is wrapped in an `Rc`.
    feature_enabled: Cell<bool>,
}

fn main() {
    // Create a shared configuration wrapped in `Rc`.
    let config = Rc::new(Config {
        feature_enabled: Cell::new(false),
    });

    // Simulate different parts of the program holding the config.
    let component_a = Rc::clone(&config);
    let component_b = Rc::clone(&config);

    // Component A checks and updates the feature flag.
    if !component_a.feature_enabled.get() {
        // Cannot get `&mut`, but can update via `Cell`.
        component_a.feature_enabled.set(true);
        println!("Component A enabled the feature.");
    }

    // Component B reads the updated feature flag.
    if component_b.feature_enabled.get() {
        println!("Component B sees that the feature is enabled.");
    }

    // Demonstrates that in `Cell`, the `get` method returns a copy of the value inside,
    // not a reference. Therefore, if you have `let a = cell_var.get()`,
    // and later call `cell_var.set(i)`, the value of `a` does not change.

    // Initialize a `Cell` with the value `100`.
    let shared_data = Cell::new(100);

    // Retrieve a copy of the value inside the `Cell`.
    let first_read = shared_data.get(); // `first_read` is 100.

    // Modify the value inside the `Cell`.
    shared_data.set(200);

    // Retrieve another copy after modification.
    let second_read = shared_data.get(); // `second_read` is 200.

    // The `first_read` remains 100, demonstrating that it is unaffected by the change.
    println!("First read: {}", first_read);   // Outputs: First read: 100
    println!("Second read: {}", second_read); // Outputs: Second read: 200
}

