fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(
        &[
            "../proto/entities.proto",
            "../proto/requests.proto",
            "../proto/events.proto",
            "../proto/service.proto",
        ],
        &["../proto"],
    )?;
    Ok(())
}
