use crate::md::MdBlock;
use crate::Result;
use serde_json::Value;

#[derive(Debug)]
enum Action {
	Initial,
	StartBlock,
	CloseBlock,
	CaptureInContent,
	CaptureInMetaBlock,
}

/// Returns the merge root value (if at least one), and the content, without the `#!meta` code blocks.
pub fn meta_extract(content: &str) -> Result<(Vec<MdBlock>, String)> {
	let lines = content.lines();

	let mut content: Vec<&str> = Vec::new();
	let mut md_blocks: Vec<MdBlock> = Vec::new();

	type MetaBlock<'a> = (Option<String>, Vec<&'a str>);
	let mut current_meta_block: Option<MetaBlock> = Default::default();
	let mut in_block = false;
	let mut in_candidate_meta_block = false;
	let mut first_block_line = false;
	let mut action = Action::Initial;
	let mut previous_line: Option<&str> = None;

	for line in lines {
		// -- Determine Action
		// Getting in or out of a code block
		if line.starts_with("```") {
			first_block_line = false;
			if in_block {
				in_block = false;
				in_candidate_meta_block = false;
				action = Action::CloseBlock;
			} else {
				in_block = true;
				first_block_line = true;
				let is_meta_lang = line.strip_prefix("```").map(|v| v.trim() == "toml").unwrap_or_default();
				in_candidate_meta_block = is_meta_lang;
				action = Action::StartBlock;
			}
		}
		// Lines that are not ```
		else {
			// -- If in block
			if in_block {
				if in_candidate_meta_block {
					if first_block_line {
						if line.trim() == "#!meta" {
							first_block_line = false;
							action = Action::CaptureInMetaBlock
						} else {
							action = Action::CaptureInContent;
						}
					}
					//
					else {
						// Same action
					}
				} else {
					action = Action::CaptureInContent;
				}
			}
			// -- Not in block
			else {
				action = Action::CaptureInContent;
			}
		}

		// -- Process the Action
		println!("->> {action:?} - {line}");
		match action {
			Action::Initial => {
				println!("->> INITIAL {action:?}");
			}
			Action::StartBlock => {
				// We do not know yet, needs to wait for next action.
			}
			Action::CloseBlock => {
				//
				match current_meta_block {
					Some(meta_block) => {
						let md_block = MdBlock {
							lang: meta_block.0,
							content: meta_block.1.join("\n"),
						};
						md_blocks.push(md_block);
						current_meta_block = None
					}
					None => content.push(line),
				}
			}
			Action::CaptureInContent => {
				if first_block_line {
					if let Some(prev_line) = previous_line {
						content.push(prev_line);
						// TODO: Should not change state, e.g., implent a new Action::CaptureInContentAndPrevLine
						first_block_line = false;
					}
				}
				// FIXME: need to capture previous_line when in block that is not meta
				content.push(line)
			}
			Action::CaptureInMetaBlock => {
				//
				match current_meta_block.as_mut() {
					Some(mut meta_block) => meta_block.1.push(line),
					None => {
						//
						let lang = line.strip_prefix("```").map(|v| v.trim().to_string());
						// type MetaBlock<'a> = (Option<String>, Vec<&'a str>);
						current_meta_block = Some((lang, Vec::new()))
					}
				}
			}
		}

		previous_line = Some(line);
	}

	let content = content.join("\n");

	Ok((md_blocks, content))
}
