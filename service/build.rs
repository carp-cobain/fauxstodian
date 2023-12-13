fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .compile(&["../proto/fauxstodian/v1/fauxstodian.proto"], &["../proto"])?;
    Ok(())
}
