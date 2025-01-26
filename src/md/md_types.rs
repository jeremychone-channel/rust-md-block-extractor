pub struct MdBlock {
	pub lang: Option<String>,
	pub content: String,
}

// NOTE: In the future, use the MetaBlock rather than the generic MdBlock when extract_blocks
pub enum MetaLang {
	Toml,
	// Yaml
}

pub struct MetaBlock {
	pub lang: MetaLang,
	pub content: String,
}
