fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = &[
        "../protos/verification.proto",
    ];

    let includes = &[
        "../protos/",
    ];

    for proto in protos {
        println!("cargo:rerun-if-changed={}", proto);
    }

    tonic_build::configure().compile(protos, includes)?;

    Ok(())
}