use tracing::subscriber::SetGlobalDefaultError;

pub fn init() -> Result<(), SetGlobalDefaultError> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    tracing::info!("Starting {} v{}", crate_name(), crate_version());

    Ok(())
}

#[inline]
fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

#[inline]
fn crate_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
