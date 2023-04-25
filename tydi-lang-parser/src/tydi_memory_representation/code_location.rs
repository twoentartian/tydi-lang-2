use std::{sync::{Arc, RwLock}, usize};

use serde::{Serialize};

use crate::tydi_parser::*;

#[derive(Clone, Debug, Serialize)]
pub struct CodeLocation {
    begin: Option<usize>,
    end: Option<usize>,
}

impl CodeLocation {
    pub fn new_unknown() -> Self {
        return Self {
            begin: None,
            end: None,
        };
    }

    pub fn new(begin:usize, end:usize) -> Self {
        return Self {
            begin: Some(begin),
            end: Some(end),
        };
    }

    pub fn new_only_begin(begin:usize) -> Self {
        return Self {
            begin: Some(begin),
            end: None,
        };
    }

    pub fn new_from_pest_rule(src: &Pair<Rule>) -> Self {
        return Self {
            begin: Some(src.as_span().start_pos().pos()),
            end: Some(src.as_span().end_pos().pos()),
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

    pub fn show(&self, src: Option<Arc<RwLock<String>>>) -> String {
        if src.is_none() {
            return format!("token location: {}~{}", self.begin.unwrap(), self.end.unwrap());
        }
        let src = src.as_ref().unwrap().read().unwrap();
        if self.begin.is_some() && self.end.is_some() {
            let code_span = src[self.begin.unwrap()..self.end.unwrap()].to_string();
            let begin_line = CodeLocation::count_lines(&src[0..self.begin.unwrap()].to_string());
            let end_line = CodeLocation::count_lines(&src[0..self.end.unwrap()].to_string());
            return format!("line = {}~{}:\n{}", begin_line, end_line, code_span);
        }
        if self.begin.is_some() {
            let begin_line = CodeLocation::count_lines(&src[0..self.begin.unwrap()].to_string());
            let code_span = CodeLocation::get_line(&*src, begin_line);
            return format!("{}: {}", begin_line, code_span);
        }
        unreachable!()
    }
}

pub trait TraitCodeLocationAccess {
    fn set_code_location(& mut self, loc: CodeLocation);

    fn get_code_location(&self) -> CodeLocation;
}
