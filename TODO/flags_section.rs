#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FlagsSectionElement {
    Comment(String),
    FontPath(String),
    ModulePath(String),
    LogFile(String),
    XkbDir(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FlagsSection {
    content: Vec<FlagsSectionElement>,

}

impl FlagsSection {
    fn add_comment(mut self, comment: &str) -> Self {
        let text = format!("# {}", comment);
        self.content.push(FlagsSectionElement::Comment(text));
        self
    }

}