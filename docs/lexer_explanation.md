### Analysis of the Lexer Pattern in `meta_extractor.rs`

The code in `meta_extractor.rs` implements a **lexer** (or tokenizer) designed to process Markdown content and extract specific metadata blocks while preserving the rest of the content. The lexer operates line-by-line, using a **state machine** to determine the current context (e.g., inside or outside a code block) and applies actions based on the context.

---

### How the Lexer Works

1. **State Management**:
   - The lexer uses a state machine with the following states:
     - `Action::Initial`: The starting state.
     - `Action::StartBlock`: Indicates the start of a code block.
     - `Action::CloseBlock`: Indicates the end of a code block.
     - `Action::CaptureInContent`: Captures lines that are part of the regular content.
     - `Action::CaptureInMetaBlock`: Captures lines that are part of a metadata block.

2. **Code Block Detection**:
   - The lexer identifies code blocks by looking for lines that start with triple backticks (` ``` `).
   - If a code block is detected, the lexer checks if it is a metadata block by verifying the language specifier (e.g., `toml`) and the presence of the `#!meta` marker.

3. **Metadata Extraction**:
   - Metadata blocks are identified by the combination of:
     - A code block with a language specifier (e.g., ` ```toml `).
     - The first line inside the block being `#!meta`.
   - The content of such blocks is captured and stored as `MdBlock` objects.

4. **Content Preservation**:
   - Lines that are not part of metadata blocks are preserved as regular content.
   - The lexer ensures that the original content (excluding metadata blocks) is reconstructed.

5. **Output**:
   - The lexer returns two outputs:
     - A list of `MdBlock` objects representing the extracted metadata blocks.
     - A string containing the original content with metadata blocks removed.

---

### What the Lexer is Designed to Extract

The lexer is designed to extract **metadata blocks** from Markdown content. These blocks are:
- Enclosed in triple backticks (` ``` `).
- Marked with a specific language (e.g., `toml`).
- Identified by the `#!meta` marker on the first line inside the block.

The rest of the content is preserved as-is, with the metadata blocks removed.

---

### Summary of the Lexer's Functionality

The lexer:
1. Processes Markdown content line-by-line.
2. Identifies and extracts metadata blocks marked with `#!meta` inside code blocks.
3. Preserves the rest of the content, excluding the metadata blocks.
4. Returns the extracted metadata blocks and the cleaned content.

This functionality is useful for applications that need to separate metadata (e.g., configuration or front matter) from the main content in Markdown files.