use eyre::WrapErr;
use glob::glob;

fn main() -> eyre::Result<()> {
    let protos = glob("protos/**/*.proto")
        .wrap_err("Failed to read glob pattern")?
        .into_iter()
        .map(|file| file.unwrap())
        .collect::<Vec<_>>();

    // rerun the build if any of the protos files change
    for file in &protos {
        println!("cargo:rerun-if-changed={:?}", file.display());
    }

    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile(protos.as_slice(), &["protos"])?;

    Ok(())
}
