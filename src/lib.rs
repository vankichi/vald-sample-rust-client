pub mod vald {
    pub mod v1 {
        tonic::include_proto!("vald.v1");
    }
}

pub mod payload {
    pub mod v1 {
        tonic::include_proto!("payload.v1");
    }
}

pub mod google {
    pub mod rpc {
        tonic::include_proto!("google.rpc");
    }
    pub mod api {
        tonic::include_proto!("google.api");
    }
    pub mod protobuf {
        tonic::include_proto!("google.protobuf");
    }
}
