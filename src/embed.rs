use std::{fs, path::PathBuf};

use anyhow::Result;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "web/dist/"]
struct Assets;

pub fn write_assets() -> Result<()> {
	for f in Assets::iter() {
		let cont: &[u8] = &Assets::get(&f).unwrap().data;
		let f: PathBuf = format!("out/{f}").into();
		match f.parent() {
			Some(p) => fs::create_dir_all(p)?,
			None => {}
		};
		fs::write(f, cont)?;
	}
	Ok(())
}
