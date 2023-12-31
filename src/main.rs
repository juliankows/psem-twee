use std::{
	collections::{BTreeMap, HashMap},
	fs::File,
	io::{BufRead, BufReader},
	path::PathBuf,
};

use anyhow::Result;
use itertools::Itertools;
use video_rs::{Decoder, Encoder, EncoderSettings, Locator, Options};

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
	process_blocks(blocks);
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
				BlockConfig {
					title: t,
					text_content: content.join("\n"),
					links,
				},
			)
		})
		.collect()
}
fn process_blocks(blocks: BTreeMap<String, BlockConfig>) -> Result<()> {
	let paths: Vec<PathBuf> = blocks
		.keys()
		.map(|x| PathBuf::from(format!("{x}.mp4")))
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

		let src: Locator = p.into();
		let mut dec = Decoder::new(&src).unwrap();

		let dest: Locator = PathBuf::from(format!("out/{}.mpd", basename)).into();
		let options = Options::new_from_hashmap(&HashMap::from([
			("seg_duration".to_string(), "1".to_string()),
			(format!("b"), format!("500k")),
		]));
		let (width, height) = dec.size_out();
		let settings = EncoderSettings::for_h264_custom(
			width as usize,
			height as usize,
			video_rs::ffmpeg::format::Pixel::YUV420P,
			Options::new_from_hashmap(&HashMap::from([
				("preset".to_string(), "medium".to_string()),
				("b".to_string(), format!("500k")),
			])),
		);
		let mut enc = Encoder::new_with_options(&dest, settings, &options).unwrap();

		for fr in dec.decode_raw_iter() {
			enc.encode_raw(fr?)?;
		}
		enc.finish()?;
	}
	Ok(())
}
