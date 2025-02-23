pub use re_parent_mod::re_child_mod;

mod re_parent_mod {
    fn hello() {
        println!("Hello from parent");
    }

    fn goodbye() {
        println!("Bye Bye from parent")
    }

    pub mod re_child_mod {
        fn hello() {
            println!("Hello from child");
        }
    
        fn goodbye() {
            println!("Bye Bye from child")
        }
    }
}