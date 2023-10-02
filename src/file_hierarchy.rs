mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `hierarchy::function()`");
}

fn private_function() {
    println!("called `hierarchy::private_function()`");
}

pub fn indirect_access() {
    print!("called `hierarchy::indirect_access()`, that\n> ");

    private_function();
}

pub fn call_inaccessible_mod_fn() {
    inaccessible::public_function();
}
