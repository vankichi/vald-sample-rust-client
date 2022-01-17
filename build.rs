// build proto files
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_proto = "./proto";
    let insert_proto = "./proto/vald/insert.proto";

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(&[insert_proto], &[root_proto])
        .unwrap_or_else(|e| panic!("protobuf compile error {}", e));
    Ok(())
}
