use crate::toplevel::Toplevel;
use rustc_hash::FxHashMap;
use stardust_xr_fusion::{
	client::FrameInfo,
	items::{
		panel::{PanelItem, PanelItemInitData},
		ItemAcceptorHandler, ItemUIHandler,
	},
	node::NodeType,
	HandlerWrapper,
};

pub struct Sphereland {
	panel_items: FxHashMap<String, HandlerWrapper<PanelItem, Toplevel>>,
}
impl Sphereland {
	pub fn new() -> Self {
		Sphereland {
			panel_items: FxHashMap::default(),
		}
	}

	pub fn frame(&mut self, info: FrameInfo) {
		for item in self.panel_items.values() {
			item.lock_wrapped().update(&info);
		}
	}

	fn add_item(&mut self, uid: &str, item: PanelItem, init_data: PanelItemInitData) {
		let Ok(toplevel) = Toplevel::create(item.alias(), init_data) else {return};
		let handler = item.wrap(toplevel).unwrap();
		// handler.lock_wrapped().mouse.lock_wrapped().panel_item_ui =
		// 	Arc::downgrade(handler.wrapped());
		self.panel_items.insert(uid.to_string(), handler);
	}
	fn remove_item(&mut self, uid: &str) {
		self.panel_items.remove(uid);
	}
}
impl ItemUIHandler<PanelItem> for Sphereland {
	fn item_created(&mut self, uid: &str, item: PanelItem, init_data: PanelItemInitData) {
		self.add_item(uid, item, init_data);
	}
	fn item_destroyed(&mut self, uid: &str) {
		self.remove_item(uid);
	}

	fn item_captured(&mut self, uid: &str, _acceptor_uid: &str, item: PanelItem) {
		let _ = item.reset_touches();
		let Some(toplevel) = self.panel_items.get(uid) else {return};
		toplevel.lock_wrapped().set_enabled(false);
	}
	fn item_released(&mut self, uid: &str, _acceptor_uid: &str, item: PanelItem) {
		let _ = item.reset_touches();
		let Some(toplevel) = self.panel_items.get(uid) else {return};
		toplevel.lock_wrapped().set_enabled(true);
	}
}
impl ItemAcceptorHandler<PanelItem> for Sphereland {
	fn captured(&mut self, uid: &str, item: PanelItem, init_data: PanelItemInitData) {
		self.add_item(uid, item, init_data);
	}
	fn released(&mut self, uid: &str) {
		self.remove_item(uid);
	}
}
