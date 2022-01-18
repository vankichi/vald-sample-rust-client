// build proto files
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(
            &[
                "proto/vald/insert.proto",
                "proto/vald/update.proto",
                "proto/vald/filter.proto",
                "proto/vald/object.proto",
                "proto/vald/remove.proto",
                "proto/vald/upsert.proto",
            ],
            &["proto"],
        )
        .unwrap_or_else(|e| panic!("protobuf compile error {}", e));
    Ok(())
}
