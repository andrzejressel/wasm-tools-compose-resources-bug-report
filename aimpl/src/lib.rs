use crate::bindings::example::service::b_interface::{run_b};
use crate::bindings::example::service::my_resource::Rec;
use crate::bindings::exports::example::service::a_interface::Guest;

mod bindings;

struct Component;

impl Guest for Component {
    fn run_a(rec: &Rec) {
        run_b(rec);
        // run_b2();
    }
}

bindings::export!(Component with_types_in bindings);