
pub mod variables;
mod flow_control;
mod method;

mod basic_types {
    pub mod array;
    pub mod numbers;
    pub mod functions;
    pub mod statements_expressions;
    pub mod char_bool;

}
mod compound_types {
    pub mod slice;
    pub mod string;
    pub mod tuple;
    pub mod struct1;
    pub mod enum1;


}

mod ownership_borrowing {
    pub mod ownership;
    pub mod reference_borrowing;
}
mod pattern_match{
    pub mod match1;
    pub mod patterns1;
}

mod generics_traits{
   pub mod generics;

}
fn main() {}