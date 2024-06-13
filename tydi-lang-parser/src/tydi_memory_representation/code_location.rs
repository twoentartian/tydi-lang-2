use std::{sync::Arc, usize};

use serde::Serialize;

use crate::{tydi_parser::*, generate_name::generate_init_value, deep_clone::DeepClone};

#[derive(Clone, Debug)]
pub struct SrcInfo {
    pub file_name: String,
    pub file_content: String,
}

impl SrcInfo {
    pub fn new(file_name: String, file_content: String) -> Arc<Self> {
        return Arc::new(Self{
            file_name: file_name,
            file_content: file_content,
        });
    }

    pub fn new_init() -> Arc<Self> {
        return Arc::new(Self{
            file_name: generate_init_value(),
            file_content: generate_init_value(),
        });
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct CodeLocation {
    pub begin: Option<usize>,
    pub end: Option<usize>,
    #[serde(skip)]
    pub src_file: Arc<SrcInfo>,
}

impl DeepClone for CodeLocation {
    fn deep_clone(&self) -> Self {
        return self.clone();
    }
}

impl CodeLocation {
    pub fn new_unknown() -> Self {
        return Self {
            begin: None,
            end: None,
            src_file: SrcInfo::new_init(),
        };
    }

    pub fn new(begin:usize, end:usize, src_file: Arc<SrcInfo>) -> Self {
        return Self {
            begin: Some(begin),
            end: Some(end),
            src_file: src_file,
        };
    }

    pub fn new_only_begin(begin:usize, src_file: Arc<SrcInfo>) -> Self {
        return Self {
            begin: Some(begin),
            end: None,
            src_file: src_file,
        };
    }

    pub fn new_from_pest_rule(src: &Pair<Rule>, src_file: Arc<SrcInfo>) -> Self {
        return Self {
            begin: Some(src.as_span().start_pos().pos()),
            end: Some(src.as_span().end_pos().pos()),
            src_file: src_file,
        };
    }

    fn count_lines(text: &str) -> usize {
        let mut count = 0;
        for c in text.chars() {
            if c == '\n' {
                count += 1;
            }
        }
        return count+1;
    }

    fn get_line(text: &str, target_line: usize) -> String {
        let mut begin_loc = 0;
        let mut end_loc = 0;
        let mut current_line = 1;
        let mut current_line_begin_index = 0;
        let mut token_index = 0;
        let target_line = target_line - 1;
        let mut is_new_line = true;
        for c in text.chars() {
            if is_new_line {
                is_new_line = false;
                current_line_begin_index = token_index;
            }

            if target_line == current_line {
                begin_loc = current_line_begin_index;
            }
            if target_line+1 == current_line {
                end_loc = token_index;
                return text[begin_loc..end_loc].to_string();
            }
            if c == '\n' {
                current_line = current_line + 1;
                is_new_line = true;
            }
            token_index += 1;
        }
        if begin_loc == 0 && end_loc == 0 {
            return String::new();
        }
        if begin_loc != 0 && end_loc == 0 {
            return text[begin_loc..].to_string();
        }
        unreachable!()
    }

    pub fn show(&self, src_info: Option<Arc<SrcInfo>>) -> String {
        let src = match src_info {
            Some(src_info) => {
                Some(src_info.file_content.clone())
            },
            None => None,
        };

        // is the src empty?
        if src.is_none() {
            return format!("token location: {}~{}", self.begin.unwrap(), self.end.unwrap());
        }
        if **(src.as_ref().unwrap()) == generate_init_value() {
            return format!("location not available");
        }

        // not empty
        let src = src.as_ref().unwrap();
        if self.begin.is_some() && self.end.is_some() {
            let begin_line = CodeLocation::count_lines(&src[0..self.begin.unwrap()].to_string());
            let end_line = CodeLocation::count_lines(&src[0..self.end.unwrap()].to_string());
            let line_digit = end_line.to_string().len();

            let mut output = String::new();
            for current_line in begin_line .. end_line+1 {
                let line_str = current_line.to_string();
                let space_padding = line_digit - line_str.len();
                for _ in 0..space_padding {
                    output.push_str(" ");
                }
                output.push_str(&current_line.to_string());
                output.push_str(" | ");
                let code_of_target_line = CodeLocation::get_line(&*src, current_line+1);
                output.push_str(&format!("{}", code_of_target_line));
            }

            return output;
        }
        if self.begin.is_some() {
            let begin_line = CodeLocation::count_lines(&src[0..self.begin.unwrap()].to_string());
            let code_span = CodeLocation::get_line(&*src, begin_line+1);
            return format!("{}: {}", begin_line, code_span);
        }
        unreachable!()
    }
}

pub trait TraitCodeLocationAccess {
    fn set_code_location(& mut self, loc: CodeLocation);

    fn get_code_location(&self) -> CodeLocation;
}
