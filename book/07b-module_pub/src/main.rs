extern crate module_pub;

fn main() {
    module_pub::client::connect();
    module_pub::network::server::ping();
    try_mod();
}

// if item is public it can be accessed through any of its parent modules.
// if item is private it can only be accessed by it immediate parent module and any of it's
// parent's child module.
mod outermost {
    mod inside {
        pub fn inner_fn() {
            println!("inner_fn");
        }

        fn inner_secret_fn() {
            println!("inner_secret");
        }
    }

    pub fn middle_fn() {
        println!("middle_fn");
        middle_secret_fn(); // in current scope
        inside::inner_fn(); // in current scope
        // inside::inner_secret_fn(); // not in current scope
    }

    fn middle_secret_fn() {
        println!("middle_secret_fn");
    }
}

fn try_mod() {
    outermost::middle_fn();
    // outermost::middle_secret_fn();
    // outermost::inside::inner_fn();
    // outermost::inside::inner_secret_fn();
}
