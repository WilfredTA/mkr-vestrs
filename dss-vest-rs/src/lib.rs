mod bindings {
    include!(concat!(env!("OUT_DIR"), "/dss-vest.rs"));
}

pub use bindings::*;