#[cfg(test)]
mod test {
    use crate::{tydi_parser::*};

    fn try_parse(code: String, target: Rule) -> Result<(), String> {
        let parse_result = TydiLangSrc::parse(target,&code);
        if parse_result.is_err() {
            return Err(String::from("fail to parse the code"));
        }
        let parse_result = parse_result.ok().unwrap().next().unwrap();
        let mut pass : bool = false;

        match parse_result.as_rule() {
            Rule::ID => {
                let value: &str = parse_result.as_str();
                if value == code {
                    pass = true;
                }
            }
            _ => {},
        }

        if pass {
            return Ok(());
        }
        else {
            return Err(String::from("unknown error"));
        }
    }

    #[test]
    fn parse_id() {
        //check valid
        try_parse(String::from("abcdef"), Rule::ID).ok().unwrap();
        try_parse(String::from("instance0"), Rule::ID).ok().unwrap();
        try_parse(String::from("_abcdef"), Rule::ID).ok().unwrap();
        try_parse(String::from("__abcdef__"), Rule::ID).ok().unwrap();
        try_parse(String::from("_0"), Rule::ID).ok().unwrap();

        //check invalid
        try_parse(String::from("0x00"), Rule::ID).err().unwrap();
        try_parse(String::from("01234"), Rule::ID).err().unwrap();
        try_parse(String::from("0abcd"), Rule::ID).err().unwrap();
        try_parse(String::from("a+b"), Rule::ID).err().unwrap();
        
        //check ID_BLOCK_LIST
        try_parse(String::from("impl"), Rule::ID).err().unwrap();
        try_parse(String::from("streamlet"), Rule::ID).err().unwrap();
        try_parse(String::from("const"), Rule::ID).err().unwrap();
        try_parse(String::from("int"), Rule::ID).err().unwrap();
        try_parse(String::from("str"), Rule::ID).err().unwrap();
        try_parse(String::from("bool"), Rule::ID).err().unwrap();
        try_parse(String::from("float"), Rule::ID).err().unwrap();
        try_parse(String::from("type"), Rule::ID).err().unwrap();
        try_parse(String::from("instance"), Rule::ID).err().unwrap();

    }

}