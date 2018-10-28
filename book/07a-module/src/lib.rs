// a module can be extracted to a different file.
// all we need to do is declare the mod using `mod filname`
mod separated;
mod server;

// we can define a module in the lib.rs file
// the module name is network and we can access it's functions using the namespace syntax ::
// network::connect
mod network {
    fn connect() {}
}

// There can be multiple modules in a single file
// the two modules don't conflict with each other.
mod client {
    fn connect() {}
}

// we can also have modules nested inside modules where it makes sense to do so.
// even though the method name is the same for both the modules. It doesn't create any issues.
// we can call the funcy from mod a like a::funcy and from b like a::b::funcy
mod a {
    fn funcy() {}

    mod b {
        fn funcy() {}
    }
}
