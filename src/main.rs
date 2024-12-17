use std::{hash::Hash, mem::size_of_val, ops::Deref, path::Display, slice::SplitInclusive, str::SplitWhitespace, u32};
use cobalt_xml_merge::xml_parsing::XmlToken;
use imara_diff::{intern::{InternedInput, TokenSource}, Sink};
use logos::Logos;

const ASSET_TABLE_PATH: &str = "test_files/AssetTable/!original.xml";

fn main() {
    let path_patches = [
        "test_files/AssetTable/Boamo.xml",
        "test_files/AssetTable/PlayableBoss.xml"
    ];
    let patches = path_patches.iter().map(|path| std::fs::read_to_string(path).unwrap()).collect::<Vec<_>>();
    let table = std::fs::read_to_string(ASSET_TABLE_PATH).unwrap();
    //let mut base_lex = XmlToken::lexer(&table);



    //let base_tokens = tokenize(&table, 100_000);
    // println!("{}", base_tokens.len());
    let instant = std::time::Instant::now();
    let base = TagSpan::extract_tags(&table, table.len() / 1_024); // base assettable is around 3920 tags
    //println!("{:?}", size_of_val(&*base));
    let patch = TagSpan::extract_tags(&patches[0], patches[0].len() / base.len() + 256); // most patches are less than 256
    {
    let old = TagSource(base.iter());
    let new = TagSource(patch.iter());
    let sink = RangeCollector::default();
    let input = InternedInput::new(old, new);
    let changes = imara_diff::diff(imara_diff::Algorithm::Myers, &input, sink);
    for change in changes {
        println!("{:?}", change);
        match change {
            DiffKind::Replace { before, after } => {
                // println!("==replacing==");
                // for i in before {
                //     base[i].tokenize().iter().for_each(|t| print!("{}", t));
                // }
                // println!("\n==with==");
                // for i in after {
                //     patch[i].tokenize().iter().for_each(|t| print!("{}", t));
                // }
            }
            DiffKind::Delete { before } => {
                // println!("==deleting==");
                // for i in before {
                //     base[i].tokenize().iter().for_each(|t| print!("{}", t));
                // }
            }
            DiffKind::Insert { after, .. } => {
                // println!("==inserting==");
                // for i in after {
                //     patch[i].tokenize().iter().for_each(|t| print!("{}", t));
                // }
            }
        }
    }
    // TODO: apply insert and delete changes, should be simple and not require tokenization
    {

    }
    // TODO: for replace chunks / inline merge
    // - tokenize lines and compare via imara_diff (minimal varient for less variation?).
    //   - tokenization is per line
    //   - while diffing the iterator will chian through all lines within the changed chunk
    // - remember line and token span of changes, which will later be writtin to the patch format
    // - the lines of the base should store the tokenized version of the tags to save time on
    //   future comparisons on it
    {

    }


    // let patch: Vec<_> = patches.iter()
    //     .map(|p| {
    //         tag_spans(&p, p.len() / 1_000);
    //         //tokenize(p, base_tokens.len() + 100_000)
    //     })
    //     .collect();
    }
    println!("\n{:?}", instant.elapsed());
}

#[derive(Debug, Eq, Clone)]
struct TagSpan<'xml> {
    slice: &'xml str,
}

impl std::fmt::Display for TagSpan<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.slice)
    }
}

impl PartialEq for TagSpan<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.get_trimmed() == other.get_trimmed()
    }
}

impl std::hash::Hash for TagSpan<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.get_trimmed().as_bytes());
    }
}

impl<'xml> TagSpan<'xml> {
    pub fn new(slice: &'xml str) -> Self {
        Self {
            slice,
        }
    } 

    /// Accounts for formatting inconsistencies (e.g. like Astra-XMLs).<br/>
    /// ...var="" /><br/>
    /// ...var=""/><br/>
    /// Whitespace-sensitivity between attributes remains, but people seem to be consistent with it.
    #[inline]
    pub fn get_trimmed(&self) -> &'xml str {
        self.slice.trim_end_matches("/>").trim()
    }

    /// Captures everything wrapped with `<>`. The pre-merge diff.<br/>
    /// skiops `<!` comments, `<?` xml headers, and `<>` empty tags.
    pub fn extract_tags(string: &'xml str, size_hint: usize) -> Vec<TagSpan<'xml>> {
        let mut v = Vec::with_capacity(size_hint); // hint size to avoid some reallocations
        for line in string.split_inclusive('>') {
            let line = line.trim_start_matches(|c| c != '<');
            if line.len() < 2 { continue };
            match line.chars().nth(1) {
                Some('!') => continue, // <! comments
                Some('?') => continue, // <? xml header
                Some('>') => continue, // <> empty tags
                _ => v.push(TagSpan::new(line))
            }
        }
        v
    }

    /// assettable can be 1 million tokens, so don't run it on the whole thing
    pub fn tokenize(&self) -> Vec<XmlToken<'xml>> {
        use cobalt_xml_merge::xml_parsing::next_token;

        let mut lex = XmlToken::lexer(&self.slice);
        let mut tokens = Vec::with_capacity(&self.slice.len() / 4);
        while let Some(token) = next_token(&mut lex) {
            match token {
                XmlToken::CommentStart => {
                    while let Some(token) = next_token(&mut lex) {
                        if token == XmlToken::TagEnd {
                            break;
                        }
                    }
                    continue;
                }
                _ => {tokens.push(token);}
            }
        }
        return tokens;
    }
}

use std::slice::Iter;
struct TagSource<'a, T: Sized>(Iter<'a, T>);
impl<'xml, T: Sized + Eq + Hash> TokenSource for TagSource<'xml, T> {
    type Token = &'xml T;

    type Tokenizer = std::slice::Iter<'xml, T>;

    fn tokenize(&self) -> Self::Tokenizer {
        Iter::clone(&self.0)
    }

    fn estimate_tokens(&self) -> u32 {
        self.0.len() as u32
    }
}

#[derive(Default)]
struct RangeCollector {
    changes: Vec<DiffKind>,
}

impl Sink for RangeCollector {
    type Out = Vec<DiffKind>;

    fn process_change(&mut self, before: std::ops::Range<u32>, after: std::ops::Range<u32>) {
        match (before.len(), after.len()) {
            (0, 0) => unreachable!(),
            (0, _) => self.changes.push(DiffKind::Insert {
                before: before.start as usize,
                after: after.start as usize..after.end as usize,
            }),
            (_, 0) => self.changes.push(DiffKind::Delete {
                before: before.start as usize..before.end as usize,
            }),
            _ => self.changes.push(DiffKind::Replace {
                before: before.start as usize..before.end as usize,
                after: after.start as usize..after.end as usize,
            }),
        };
    }

    fn finish(self) -> Self::Out {
        self.changes
    }
}

#[derive(Debug)]
enum DiffKind {
    Insert { before: usize, after: std::ops::Range<usize> },
    Delete { before: std::ops::Range<usize> },
    Replace { before: std::ops::Range<usize>, after: std::ops::Range<usize> },
}