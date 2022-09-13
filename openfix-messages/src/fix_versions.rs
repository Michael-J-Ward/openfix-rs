
//! Supported FIX versions configurable via feature flags

#[cfg(feature = "fixt11")]
pub mod fixt11 {
    pub mod fields {
        include!(concat!(env!("OUT_DIR"), "/FIXT11_fields.rs"));
    }
    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/FIXT11_messages.rs"));
    }
}

#[cfg(feature = "fix40")]
pub mod fix40 {
    pub mod fields {
        include!(concat!(env!("OUT_DIR"), "/FIX40_fields.rs"));
    }
    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/FIX40_messages.rs"));
    }
}

#[cfg(feature = "fix41")]
pub mod fix41 {
    pub mod fields {
        include!(concat!(env!("OUT_DIR"), "/FIX41_fields.rs"));
    }
    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/FIX41_messages.rs"));
    }
}

#[cfg(feature = "fix42")]
pub mod fix42 {
    pub mod fields {
        include!(concat!(env!("OUT_DIR"), "/FIX42_fields.rs"));
    }
    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/FIX42_messages.rs"));
    }
}

#[cfg(feature = "fix43")]
pub mod fix43 {
    pub mod fields {
        include!(concat!(env!("OUT_DIR"), "/FIX43_fields.rs"));
    }
    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/FIX43_messages.rs"));
    }
}

#[cfg(feature = "fix44")]
pub mod fix44 {
    pub mod fields {
        include!(concat!(env!("OUT_DIR"), "/FIX44_fields.rs"));
    }
    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/FIX44_messages.rs"));
    }
}

#[cfg(feature = "test_spec")]
pub mod test_spec {
    pub mod fields {
        include!(concat!(env!("OUT_DIR"), "/TEST_SPEC_fields.rs"));
    }
    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/TEST_SPEC_messages.rs"));
    }
}

#[cfg(feature = "test_spec")]
pub mod test_spec_sig {
    pub mod fields {
        include!(concat!(env!("OUT_DIR"), "/TEST_SPEC_SIG_fields.rs"));
    }
    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/TEST_SPEC_SIG_messages.rs"));
    }
}
