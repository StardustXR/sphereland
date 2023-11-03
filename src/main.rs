use color_eyre::eyre::{bail, Result};
use manifest_dir_macros::directory_relative_path;
use sphereland::Sphereland;
use stardust_xr_fusion::{
	client::{Client, ClientState, FrameInfo, RootHandler},
	items::{panel::PanelItem, ItemUI},
	HandlerWrapper,
};
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

pub mod sphereland;
pub mod surface;
pub mod toplevel;

struct Root {
	sphereland: HandlerWrapper<ItemUI<PanelItem>, Sphereland>,
}
impl Root {
	fn new(client: Arc<Client>) -> Result<Self> {
		let sphereland = ItemUI::register(&client)?.wrap(Sphereland::new())?;
		Ok(Root { sphereland })
	}
}
impl RootHandler for Root {
	fn frame(&mut self, info: FrameInfo) {
		self.sphereland.lock_wrapped().frame(info);
	}
	fn save_state(&mut self) -> ClientState {
		ClientState::default()
	}
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
	tracing_subscriber::fmt()
		.compact()
		.with_env_filter(EnvFilter::from_env("LOG_LEVEL"))
		.init();
	let (client, event_loop) = Client::connect_with_async_loop().await?;
	client.set_base_prefixes(&[directory_relative_path!("res")]);

	let _wrapped_root = client.wrap_root(Root::new(client.clone())?);

	tokio::select! {
		_ = tokio::signal::ctrl_c() => Ok(()),
		_ = event_loop => bail!("Server crashed"),
	}
}
