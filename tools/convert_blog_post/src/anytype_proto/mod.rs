// pub mod common {
//     include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));
// }

// pub mod anytype_model {
//     include!(concat!(env!("OUT_DIR"), "/anytype.model.rs"));
// }

// pub mod anytype {
//     include!(concat!(env!("OUT_DIR"), "/anytype.rs"));
// }
extern crate enum_repr;
pub mod anytype;

pub mod anytype_model;

pub mod trait_impl;
