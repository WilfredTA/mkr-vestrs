mod bindings {
    include!(concat!(env!("OUT_DIR"), "/dss-vest.rs"));
    //include!(concat!(env!("OUT_DIR"), "/ds-token.rs"));
}

pub use bindings::*;