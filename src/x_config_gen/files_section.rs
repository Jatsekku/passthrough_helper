use std::fmt::{self};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FilesSectionElement {
    Comment(String),        // <0;+inf)
    FontPath(String),       // <0;+inf) 
    ModulePath(String),     // <0;+inf)
    LogFile(String),        // {0;1}
    XkbDir(String),         // {0;1}
}

impl fmt::Display for FilesSectionElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FilesSectionElement::Comment(value) => write!(f, "# {}\n", value),
            FilesSectionElement::FontPath(value) => write!(f, "FontPath \"{}\"\n", value), 
            FilesSectionElement::ModulePath(value) => write!(f, "ModulePath \"{}\"\n", value), 
            FilesSectionElement::LogFile(value) => write!(f, "LogFile \"{}\"\n", value),
            FilesSectionElement::XkbDir(value) => write!(f, "XkbDir \"{}\"\n", value)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct FilesSection {
    content: Vec<FilesSectionElement>,
    has_logfile: bool,
    has_xkbdir: bool
}

#[allow(dead_code)]
impl FilesSection {
    fn new() -> Self {
        Default::default()
    }

    create_function!(add_comment, &str, FilesSectionElement::Comment);
    create_function!(add_fontpath, &str, FilesSectionElement::FontPath);
    create_function!(add_modulepath, &str, FilesSectionElement::ModulePath);
    create_function!(set_logfile, &str, FilesSectionElement::LogFile, has_logfile);
    create_function!(set_xkbdir, &str, FilesSectionElement::XkbDir, has_xkbdir);
}