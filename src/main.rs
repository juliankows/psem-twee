use std::{
	collections::BTreeMap,
	env,
	fs::{self, File},
	io::{BufRead, BufReader},
	path::PathBuf,
	process::{self, Command},
};

use anyhow::Result;
use itertools::Itertools;
use serde::Serialize;

mod embed;

type BlockId = String;

#[derive(Serialize, Debug)]
struct Block {
	title: String,
	text_content: String,
	links: Vec<BlockId>,
}

fn main() -> Result<()> {
	let args: Vec<String> = env::args().collect();
	let twee_file: PathBuf = match args.get(1) {
		Some(t) => t,
		None => {
			println!("!! no .twee file as argument");
			println!("usage: {} <twee file>", args[0]);
			process::exit(1);
		}
	}
	.into();
	if !twee_file.exists() {
		println!("file {} does not exist!", twee_file.display());
		process::exit(2);
	}
	let f = File::open(twee_file)?;
	let rdr = BufReader::new(f);
	let blocks = read_twee(rdr.lines().filter_map(|x| x.ok()));

	for l in blocks.values() {
		println!("{}", str::repeat("-", 50));
		println!("--- {} ---", l.title);
		println!("{}", l.text_content);
		println!("linked to {:?}", l.links);
	}
	fs::create_dir_all("out")?;
	{
		let blocks = serde_json::to_string(&blocks)?;
		fs::write("out/config.json", blocks)?;
	}
	embed::write_assets()?;
	process_blocks(blocks)?;
	Ok(())
}
fn read_twee(rdr: impl Iterator<Item = String>) -> BTreeMap<String, Block> {
	rdr.map(|x| x.trim().to_string())
		.group_by(|x| x.starts_with("::"))
		.into_iter()
		.map(|x| -> Vec<String> { x.1.collect() })
		.tuples()
		.filter(|(t, _)| {
			!(t[0].contains(&"StoryData".to_string()) || t[0].contains(&"StoryTitle".to_string()))
		})
		.map(|(t, c)| {
			let t = t[0]
				.trim_start_matches(":: ")
				.split_once('{')
				.unwrap_or_else(|| (&t[0], ""))
				.0
				.trim()
				.to_string();
			let (links, content): (Vec<String>, Vec<String>) =
				c.into_iter().partition(|x| x.starts_with("[["));
			let links: Vec<String> = links
				.into_iter()
				.map(|x| {
					x.trim_start_matches("[[")
						.trim_end_matches("]]")
						.to_string()
				})
				.collect();
			(
				t.clone(),
				Block {
					title: t,
					text_content: content.join("\n"),
					links,
				},
			)
		})
		.collect()
}
fn process_blocks(blocks: BTreeMap<String, Block>) -> Result<()> {
	let paths: Vec<PathBuf> = blocks
		.keys()
		.map(|x| PathBuf::from(format!("in/{x}.mp4")))
		.collect();
	//if paths.iter().any(|x| !x.exists()) {
	//	paths
	//		.iter()
	//		.filter(|x| !x.exists())
	//		.for_each(|x| println!("{} existiert nicht", x.display()));
	//}
	for p in paths {
		if !p.exists() {
			continue;
		}
		println!("transcoding {}", p.display());
		let p2 = p.clone();
		let basename = p2.file_stem().unwrap().to_str().unwrap();
		fs::create_dir_all(format!("out/files/{basename}"))?;
		#[cfg(target_os = "linux")]
		let ffmpeg = "ffmpeg";
		#[cfg(target_os = "windows")]
		let ffmpeg = ".\\ffmpeg.exe";
		Command::new(ffmpeg)
			.arg("-i")
			.arg(format!("{}", p.display()))
			.arg("-f")
			.arg("dash")
			.arg("-window_size")
			.arg("0")
			.arg(format!("out/files/{basename}/manifest.mpd"))
			.spawn()?
			.wait()?;
	}
	Ok(())
}
