use crate::md::MdBlock;
use crate::Result;
use serde_json::Value;

///
/// For example, will be bound to Lua as
/// ```lua
/// local value_obj, content = utils.text.meta_extract(content)
/// ```
pub fn meta_extract(content: &str) -> Result<(Value, String)> {
	todo!()
}

// region:    --- Support

#[derive(Debug)]
enum Action {
	Initial,
	StartBlock,
	CloseBlock,
	CaptureInContent,
	CaptureInMetaBlock,
}

/// Returns the merge root value (if at least one), and the content, without the `#!meta` code blocks.
fn meta_extract_md_blocks_and_content(content: &str) -> Result<(Vec<MdBlock>, String)> {
	let lines = content.lines();

	let mut content: Vec<&str> = Vec::new();
	let mut md_blocks: Vec<MdBlock> = Vec::new();

	// (lang, block_content_lines)
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
		match action {
			Action::Initial => {
				// Should never be here per logic
				// println!("INITIAL {action:?}");
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
						// TODO: Should assess if we need to change state here, or implement a new Action::CaptureInContentAndPrevLine
						first_block_line = false;
					}
				}
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

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

	use super::*;

	#[test]
	fn test_meta_extract_md_blocks_and_content_simple() -> Result<()> {
		// -- Setup & Fixtures
		// region:    --- fx_md
		let fx_md = r#"
Hey some content over here. 

```toml
#!meta
model = "deepseek-chat"
files = ["src/**/*rs"]
```

This is is pretty cool

```toml
#!meta
temperature = 0.0
```

And some more2

```toml
some = "stuff"
```

```python
#!meta
def some() 
 return 123
```


		"#;
		// endregion: --- fx_md

		// -- Exec
		let (md_blocks, content) = meta_extract_md_blocks_and_content(fx_md)?;

		// -- Debug
		// println!("\n====\n");

		// println!("->> meta md_blocks {}", md_blocks.len());
		// println!("->> content:\n{content}");

		// -- Check
		// assert meta blocks
		assert_eq!(md_blocks.len(), 2);
		let meta_block = md_blocks.first().ok_or("Should have at least one meta block")?;
		assert!(
			meta_block.content.contains(r#"files = ["src/**/*rs"]"#),
			"should have files"
		);
		let meta_block = md_blocks.get(1).ok_or("Should have at least thow meta block")?;
		assert!(
			meta_block.content.contains(r#"temperature = 0.0"#),
			"should have temperature"
		);
		// assert content
		assert!(
			content.contains("Hey some content over here."),
			"Hey some content over here."
		);
		assert!(content.contains(r#"```toml"#), "```toml");
		assert!(content.contains(r#"some = "stuff""#), "some = stuff");
		assert!(content.contains(r#"```python"#), "```python");
		assert!(content.contains(r#"def some()"#), "def some()");
		assert!(content.contains("And some more2"), "And some more2");

		Ok(())
	}
}

// endregion: --- Tests
