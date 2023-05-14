use once_cell::sync::Lazy;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub static LONG_VERSION: Lazy<String> = Lazy::new(|| {
    let version = if let Some(hash) = built_info::GIT_COMMIT_HASH {
        if let Some(true) = built_info::GIT_DIRTY {
            format!("{} ({}*)", built_info::PKG_VERSION, hash)
        } else {
            format!("{} ({})", built_info::PKG_VERSION, hash)
        }
    } else {
        built_info::PKG_VERSION.to_string()
    };
    format!(
        "{version}\nbuilt with {}\nbuild timestamp: {}",
        built_info::RUSTC_VERSION,
        built_info::BUILT_TIME_UTC
    )
});

pub fn install_utils(filters: &[&str], display_file_line: bool) -> eyre::Result<()> {
    let _ = dotenvy::dotenv(); //ignore error
    install_tracing(filters, display_file_line);
    install_eyre()?;
    Ok(())
}

fn install_eyre() -> eyre::Result<()> {
    let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default().into_hooks();

    eyre_hook.install()?;

    std::panic::set_hook(Box::new(move |pi| {
        tracing::error!("{}", panic_hook.panic_report(pi));
    }));
    Ok(())
}

fn install_tracing(filters: &[&str], display_file_line: bool) {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer()
        .with_target(false)
        .with_line_number(display_file_line)
        .with_file(display_file_line);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .map(|f| {
            filters
                .iter()
                .fold(f, |f, filter| f.add_directive(filter.parse().unwrap()))
        })
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}
