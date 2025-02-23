//! # Module System Examples
//! 
//! This module demonstrates Rust's module system concepts:
//! - Private modules and functions by default
//! - Child module access to parent module functions
//! - Parent module restrictions on accessing child module items

/// This module shows how parent and child modules interact
/// when all items are private by default.
mod parent_mod {
    /// A private greeting function in the parent module.
    /// This function is accessible by the child module through
    /// the `super` keyword, even though it's private.
    fn greeting() {
        println!("Hello from parent module");
    }

    /// Demonstrates attempted access to child module's function.
    /// Note: This will fail to compile since child_mod's functions are private.
    fn use_child() {
        println!("I need a function from child");
        /// Parent cannot use child module's functions, unless they are defined public.
        // child_mod::intermed_function();
    }

    mod child_mod {
        /// A private greeting function in the child module.
        /// Shows how child modules can access parent module items using `super`.
        fn greeting() {
            println!("Hello from child module");
            /// Child module can use parent's functions.
            /// If you uncomment the following, it works fine, although the parent is not public.
            // super::greeting();
        }

        /// A private intermediate function that the parent module attempts to access.
        /// This demonstrates that private functions in child modules are not
        /// accessible to parent modules.
        fn intermed_function() {
            println!("I am the intermediate function in child");
        }
    }
}

mod reexport;

/// Using re-export concept by pub use, I can use the following code.
use reexport::re_child_mod;