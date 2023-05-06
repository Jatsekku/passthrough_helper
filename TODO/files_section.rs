// // ok
// pub struct FilesSection {
//     comment: Option<Vec<String>>,
//     font_path: Option<Vec<String>>,
//     module_path: Option<Vec<String>>,
//     logfile_path: Option<String>,
//     xkb_dir: Option<String>,
// }

// pub struct ModuleSection {
//     comment: Option<Vec<String>>,
//     load: Option<String>,
//     disable: Option<String>,
//     load_driver:
// }

// pub struct ServerFlagsSection {
//     //custom setter?
//     options: Option<Vec<(String, String)>>,
// }

// pub struct XConfExtensions;

// pub struct XConfInputDevice;

// pub struct XConfInputClass;

// #[derive(Builder, Debug)]
// pub struct DeviceSection {
//     identifier: String,
//     driver: String,
//     screen: Option<u32>,
// }

// pub struct MonitorSection {
//     identifier: String,
//     vender_name: Option<String>,
//     model_name: Option<String>,
//     use_modes:
//     mode_line:
//     display_size: (u32, u32)
//     horiz_sync:
//     vert_refresh:

// }

// pub struct XConfModes {

// }

// pub struct XConfScreen {

// }

// pub struct XConfDisplay {

// }

// pub struct  XConfServerLayout {

// }

// pub struct XConfDRI {

// }

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FilesSectionElement {
    Comment(String),
    FontPath(String),
    ModulePath(String),
    LogFile(String),
    XkbDir(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FilesSection {
    content: Vec<FilesSectionElement>,
    has_log_file: bool,
    has_xkb_dir: bool,
}

impl FilesSection {
    fn new() -> Self {
        FilesSection {
            content: Vec::new(),
            has_log_file: false,
            has_xkb_dir: false,
        }
    }

    fn add_comment(mut self, comment: &str) -> Self {
        let text = format!("# {}", comment);
        self.content.push(FilesSectionElement::Comment(text));
        self
    }

    fn add_font_path(mut self, path: &str) -> Self {
        let mut path = format!("{},", path);
        self.content.push(FilesSectionElement::FontPath(path));
        self
    }

    fn add_module_path(mut self, path: &str) -> Self {
        let mut path = format!("{},", path);
        self.content.push(FilesSectionElement::ModulePath(path));
        self
    }

    fn add_logfile_path(mut self, path: &str) -> Result<Self, String> {
        if self.has_log_file {
            return Result::Err("LogFile already set".to_string());
        }

        self.content.push(FilesSectionElement::LogFile((path.to_string())));
        self.has_log_file = true;
        Result::Ok(self)
    }

    fn add_xkb_dir(mut self, dir: &str) -> Result<Self, String> {
        if self.has_log_file {
            return Result::Err("XkbDir already set".to_string());
        }

        self.content.push(FilesSectionElement::XkbDir((dir.to_string())));
        self.has_xkb_dir = true;
        Result::Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::files_section::{self, FilesSection, FilesSectionElement};

    #[test]
    fn new() {
        let files_section = FilesSection::new();

        assert_eq!(
            files_section,
            FilesSection {
                content: Vec::new(),
                has_log_file: false,
                has_xkb_dir: false,
            }
        );
    }

    #[test]
    fn add_single_comment() {
        let files_section = FilesSection::new().add_comment("Test comment");

        assert_eq!(
            files_section,
            FilesSection {
                content: vec![FilesSectionElement::Comment(String::from("# Test comment"))],
                has_log_file: false,
                has_xkb_dir: false,
            }
        )
    }

    #[test]
    fn add_multiple_comments() {
        let mut files_section = FilesSection::new()
            .add_comment("Test comment1")
            .add_comment("Comment2");

        assert_eq!(
            files_section,
            FilesSection {
                content: vec![
                    FilesSectionElement::Comment(String::from("# Test comment1")),
                    FilesSectionElement::Comment(String::from("# Comment2"))
                ],
                has_log_file: false,
                has_xkb_dir: false,
            }
        )
    }
}
