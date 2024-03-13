use crate::bindings::exports::example::service::main_interface::Guest;

mod bindings;

struct Component;

impl Guest for Component {
    fn main() {
        let rec = bindings::example::service::my_resource::Rec::new();
        bindings::example::service::a_interface::run_a(&rec);
        bindings::example::service::b_interface::run_b(&rec);
        // bindings::example::service::b_interface::run_b3();
    }
}

bindings::export!(Component with_types_in bindings);