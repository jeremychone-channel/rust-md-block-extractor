// region:    --- Modules

mod md;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

// endregion: --- Modules

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

	use super::*;

	#[test]
	fn test_quick_test() -> Result<()> {
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
		let (md_blocks, content) = md::meta_extract(fx_md)?;

		println!("\n====\n");

		println!("->> meta md_blocks {}", md_blocks.len());
		println!("->> content:\n{content}");

		// -- Check

		Ok(())
	}
}

// endregion: --- Tests
