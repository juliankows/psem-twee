use std::{
	collections::{BTreeMap, HashMap},
	fs::File,
	io::{BufRead, BufReader},
	path::PathBuf,
};

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug)]
struct Block {
	title: String,
	content: String,
}

type BlockId = String;

struct BlockConfig {
	title: String,
	text_content: String,
	links: Vec<BlockId>,
}

fn main() -> Result<()> {
	let f = File::open("sample.twee")?;
	let rdr = BufReader::new(f);
	let blocks = read_twee(rdr.lines().filter_map(|x| x.ok()));

	for l in blocks.values() {
		println!("{}", str::repeat("-", 50));
		println!("--- {} ---", l.title);
		println!("{}", l.text_content);
		println!("linked to {:?}", l.links);
	}
	process_block(blocks);
	Ok(())
}
fn read_twee(rdr: impl Iterator<Item = String>) -> BTreeMap<String, BlockConfig> {
	rdr.map(|x| x.trim().to_string())
		.group_by(|x| x.starts_with("::"))
		.into_iter()
		.map(|x| -> Vec<String> { x.1.collect() })
		.tuples()
		.filter(|(t, _)| {
			!t.contains(&"StoryData".to_string()) && !t.contains(&"StoryTitle".to_string())
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
			(
				t.clone(),
				BlockConfig {
					title: t,
					text_content: content.join("\n"),
					links,
				},
			)
		})
		.collect()
}
fn process_block(blocks: BTreeMap<String, BlockConfig>) {
	let paths: Vec<PathBuf> = blocks
		.keys()
		.map(|x| PathBuf::from(format!("{x}.mp4")))
		.collect();
	if paths.iter().any(|x| !x.exists()) {
		paths
			.iter()
			.filter(|x| !x.exists())
			.for_each(|x| println!("{} existiert nicht", x.display()));
	}
}
