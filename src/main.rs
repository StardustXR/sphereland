use color_eyre::eyre::{bail, Result};
use manifest_dir_macros::directory_relative_path;
use sphereland::Sphereland;
use stardust_xr_fusion::{client::Client, items::ItemUI};
use tracing_subscriber::EnvFilter;

pub mod sphereland;
pub mod surface;
pub mod toplevel;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
	tracing_subscriber::fmt()
		.compact()
		.with_env_filter(EnvFilter::from_env("LOG_LEVEL"))
		.init();
	let (client, event_loop) = Client::connect_with_async_loop().await?;
	client.set_base_prefixes(&[directory_relative_path!("res")]);

	let sphereland = ItemUI::register(&client)?.wrap(Sphereland::new())?;
	client.wrap_root_raw(sphereland.wrapped())?;

	tokio::select! {
		_ = tokio::signal::ctrl_c() => Ok(()),
		_ = event_loop => bail!("Server crashed"),
	}
}
