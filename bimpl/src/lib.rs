use crate::bindings::exports::example::service::b_interface;
use crate::bindings::exports::example::service::b_interface::{Rec, RecBorrow};
use crate::bindings::exports::example::service::my_resource::GuestRec;

mod bindings;

struct Component;

impl b_interface::Guest for Component {
    fn run_b(rec: RecBorrow) {
        todo!()
    }
}

struct MyStruct {

}

impl GuestRec for MyStruct {
    fn new() -> Self {
        return MyStruct {};
    }
}

impl crate::bindings::exports::example::service::my_resource::Guest for Component {
    type Rec = MyStruct;
}

bindings::export!(Component with_types_in bindings);