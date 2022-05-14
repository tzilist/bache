use crate::config::Args;

pub async fn start(args: Args) -> eyre::Result<()> {
    dbg!(args);

    Ok(())
}
