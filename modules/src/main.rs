mod parent_mod {
    fn greeting() {
        println!("Hello from parent module");
    }

    fn use_child() {
        println!("I need intermediate function from child");
        child_mod::intermed_function();
    }

    mod child_mod {
        fn greeting() {
            println!("Hello from child module");
            // super::greeting();
        }

        fn intermed_function() {
            println!("I am the intermediate function in child");
        }
    }
}