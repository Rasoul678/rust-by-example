#![allow(dead_code)]
#![allow(unused_variables)]

mod file_hierarchy;
mod my_struct;
mod super_self;
mod use_declaration;
mod visibility;

use my_struct::my_struct as pub_struct;
use use_declaration::use_declaration::nested::function as other_function;
use visibility::my_mod;

use super_self::my_super_self::indirect_call;

fn main() {
    // Modules
    function();
    my_mod::function();

    // Public items, including those inside nested modules, can be
    // accessed from outside the parent module.
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) items can be called from anywhere in the same crate
    my_mod::public_function_in_crate();

    // pub(in path) items can only be called from within the module specified
    // Error! function `public_function_in_my_mod` is private
    // my_mod::nested::public_function_in_my_mod();
    // TODO ^ Try uncommenting this line

    // Private items of a module cannot be directly accessed, even if
    // nested in a public module:

    // Error! `private_function` is private
    //my_mod::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_function` is private
    //my_mod::nested::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    //my_mod::private_nested::function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    //my_mod::private_nested::restricted_function();
    // TODO ^ Try uncommenting this line

    // struct
    // Public structs with public fields can be constructed as usual
    let open_box = pub_struct::OpenBox {
        contents: "public information",
    };

    // and their fields can be normally accessed.
    println!("The open box contains: {}", open_box.contents);

    // Public structs with private fields cannot be constructed using field names.
    // Error! `ClosedBox` has private fields
    // let closed_box = pub_struct::ClosedBox { contents: "classified information" };
    // TODO ^ Try uncommenting this line

    // However, structs with private fields can be created using
    // public constructors
    let closed_box = pub_struct::ClosedBox::new("classified information");

    // and the private fields of a public struct cannot be accessed.
    // Error! The `contents` field is private
    // println!("The closed box contains: {}", closed_box.contents);
    // TODO ^ Try uncommenting this line

    // Easier access to `deeply::nested::function`
    other_function();

    println!("Entering block");
    {
        // This is equivalent to `use deeply::nested::function as function`.
        // This `function()` will shadow the outer one.
        use crate::use_declaration::use_declaration::nested::function;

        // `use` bindings have a local scope. In this case, the
        // shadowing of `function()` is only in this block.
        function();

        println!("Leaving block");
    }

    function();

    indirect_call();

    file_hierarchy::function();
    file_hierarchy::indirect_access();
    file_hierarchy::nested::function();
    file_hierarchy::call_inaccessible_mod_fn();
}

fn function() {
    println!("called `function()`");
}
