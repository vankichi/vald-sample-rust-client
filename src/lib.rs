pub mod vald {
    pub mod v1 {
        include!("./proto/vald.v1.rs");
    }
}

pub mod payload {
    pub mod v1 {
        include!("./proto/payload.v1.rs");
    }
}

pub mod google {
    pub mod rpc {
        include!("./proto/google.rpc.rs");
    }
    pub mod api {
        include!("./proto/google.api.rs");
    }
    pub mod protobuf {
        include!("./proto/google.protobuf.rs");
    }
}
