fn function_type() {
    // Assigning `bar` to `_x` gives `_x` the type of `bar`, which is a function item.
    // A function item is a unique type representing a specific function at compile time.
    // It is not a function pointer and has zero size at runtime.
    // It references the specific function `bar`.
    // If `bar` were a generic function `bar<T>`, we couldn't use `bar` directly here
    // because it wouldn't represent a concrete function.
    // We would need to specify a concrete type, like `bar::<i32>`, to create a function item.
    // let _x = bar;
}

fn bar() {}

// Explain function pointers.
fn baz(f: fn(i32) -> i32) {
    // Here, `f` is a function pointer, which has the size of a pointer.
    // This is because `f` is of type `fn(i32) -> i32`, a function pointer type.
    // If `bar` were a generic function `bar<T>`, we could pass a specific instance like `bar::<u32>` to `baz`.
}