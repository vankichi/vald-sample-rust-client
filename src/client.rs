// The following is an example.
// use payload::v1::Insert;
// use vald::v1::insert_client::InsertClient;

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
}

fn main() {
    todo!("implement client using vald.v1")
}
