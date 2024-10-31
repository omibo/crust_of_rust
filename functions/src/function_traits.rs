/// Function pointers and function items do not capture any data from the environment.
/// They have no associated data or lifetimes.
/// Function pointers can implement all of the `Fn`, `FnMut`, and `FnOnce` traits
/// because they can be called multiple times without capturing or modifying any state.
/// Function items (like `bar`) can be coerced to function pointers.
///
/// Regarding the `Fn` traits hierarchy:
/// - If a type implements `Fn`, it also implements `FnMut` and `FnOnce`.
/// - If a type implements `FnMut`, it also implements `FnOnce`.
/// - If a type implements only `FnOnce`, it does not necessarily implement `Fn` or `FnMut`.
///
/// Therefore:
/// - If we require `FnOnce`, we can accept types that implement `Fn`, `FnMut`, or `FnOnce`.
/// - If we require `FnMut`, we can accept types that implement `Fn` or `FnMut`.
/// - If we require `Fn`, we can only accept types that implement `Fn`.

fn main() {
    // `bar::<i32>` is a monomorphized version of the generic function `bar<T>`.
    // It can be passed to `quox` because it implements the `Fn()` trait.
    quox(bar::<i32>);
}

fn bar<T>() {}

fn baz(f: fn()) {
    // `baz` accepts a function pointer `fn()`.
    // Function pointers can be obtained by coercing function items.
}

fn quox<F>(f: F)
where
    // `Fn` is a trait representing callable entities that can be called with `&self`.
    // `fn` is a function pointer type.
    // Function pointers implement all three `Fn` traits (`Fn`, `FnMut`, `FnOnce`).
    F: Fn(),
{
    // `quox` accepts any type `F` that implements the `Fn()` trait.
}

/// Closures can demonstrate the differences between `Fn`, `FnMut`, and `FnOnce`.
fn main_closure() {
    // This is a non-capturing closure. It does not capture any variables from its environment.
    // Non-capturing closures can be coerced to function pointers.
    // They implement `Fn`, `FnMut`, and `FnOnce` because they can be called multiple times without side effects.
    let f = |x: i32, y: i32| x + y;
    // Since `f` can be coerced to a function pointer, we can pass it to `baz`.
    baz(f);

    let mut z = String::new();

    // This closure captures `z` by immutable reference.
    // It can be called multiple times without mutating `z`.
    // Therefore, it implements `Fn`, `FnMut`, and `FnOnce`.
    let f = || {
        println!("Hello {}", z);
    };

    // This closure captures `z` by mutable reference.
    // It modifies `z`, so it requires `&mut self` to be called.
    // Therefore, it implements `FnMut` and `FnOnce`, but not `Fn`.
    let f = || {
        z.clear();
    };

    // This closure captures `z` by value (moves `z` into the closure).
    // Since `z` is moved, the closure can only be called once.
    // Therefore, it implements `FnOnce` but not `FnMut` or `Fn`.
    let f = || {
        drop(z);
    };
    // Note: After `z` is moved into the closure, it cannot be used again.
}
