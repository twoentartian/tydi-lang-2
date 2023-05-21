#[cfg(test)]
mod test {
    use crate::{tydi_parser::*};

    fn try_parse(code: String, rule_target: Rule, input_output_same: bool) -> Result<(), String> {
        let parse_result = TydiLangSrc::parse(rule_target,&code);
        if parse_result.is_err() {
            println!("{}", parse_result.err().unwrap().to_string());
            return Err(String::from("fail to parse the code"));
        }
        let parse_result = parse_result.ok().unwrap().next().unwrap();
        let mut pass : bool = false;

        if parse_result.as_rule() == rule_target {
            let value: &str = parse_result.as_str();
            if input_output_same {
                if value == code {
                    pass = true;
                }
            }
            else {
                pass = true;
            }
        }
        else {  //maybe the rule contains _{ .... }
            pass = true;
        }
        if pass {
            return Ok(());
        }
        else {
            return Err(String::from(parse_result.as_str()));
        }
    }

    #[test]
    fn parse_id() {
        //check valid
        try_parse(String::from("abcdef"), Rule::ID, true).ok().unwrap();
        try_parse(String::from("instance0"), Rule::ID, true).ok().unwrap();
        try_parse(String::from("_abcdef"), Rule::ID, true).ok().unwrap();
        try_parse(String::from("__abcdef__"), Rule::ID, true).ok().unwrap();
        try_parse(String::from("_0"), Rule::ID, true).ok().unwrap();

        //check invalid
        try_parse(String::from("0x00"), Rule::ID, true).err().unwrap();
        try_parse(String::from("01234"), Rule::ID, true).err().unwrap();
        try_parse(String::from("0abcd"), Rule::ID, true).err().unwrap();
        try_parse(String::from("a+b"), Rule::ID, true).err().unwrap();
        
        //check ID_BLOCK_LIST
        try_parse(String::from("impl"), Rule::ID, true).err().unwrap();
        try_parse(String::from("streamlet"), Rule::ID, true).err().unwrap();
        try_parse(String::from("const"), Rule::ID, true).err().unwrap();
        try_parse(String::from("int"), Rule::ID, true).err().unwrap();
        try_parse(String::from("string"), Rule::ID, true).err().unwrap();
        try_parse(String::from("bool"), Rule::ID, true).err().unwrap();
        try_parse(String::from("float"), Rule::ID, true).err().unwrap();
        try_parse(String::from("type"), Rule::ID, true).err().unwrap();
        try_parse(String::from("instance"), Rule::ID, true).err().unwrap();
    }

    #[test]
    fn parse_comment() {
        try_parse(String::from("abcdef// this is a line comment"), Rule::ID, false).ok().unwrap();
        try_parse(String::from("abcdef/* this is a block comment */"), Rule::ID, false).ok().unwrap();
        try_parse(String::from("abcdef/* this is a block \n comment */"), Rule::ID, false).ok().unwrap();
        try_parse(String::from("abcdef/* this is a block
         comment */"), Rule::ID, false).ok().unwrap();
    }

    #[test]
    fn parse_int() {
        try_parse(String::from("0"), Rule::INT, true).ok().unwrap();
        try_parse(String::from("0x08_ab_cd_ef"), Rule::INT, true).ok().unwrap();
        try_parse(String::from("0b0011_0011_0101"), Rule::INT, true).ok().unwrap();
        try_parse(String::from("0o5477_0123_4567"), Rule::INT, true).ok().unwrap();
    }

    #[test]
    fn parse_bool() {
        try_parse(String::from("true"), Rule::BOOL, true).ok().unwrap();
        try_parse(String::from("false"), Rule::BOOL, true).ok().unwrap();
    }

    #[test]
    fn parse_float() {
        try_parse(String::from("1.0"), Rule::FLOAT, true).ok().unwrap();
        try_parse(String::from("0.0"), Rule::FLOAT, true).ok().unwrap();
        try_parse(String::from("124781264781637.165456156"), Rule::FLOAT, true).ok().unwrap();
        try_parse(String::from("1000.00001"), Rule::FLOAT, true).ok().unwrap();
        try_parse(String::from("0"), Rule::FLOAT, true).err().unwrap();
        try_parse(String::from("1"), Rule::FLOAT, true).err().unwrap();
    }

    #[test]
    fn parse_string() {
        try_parse(String::from("\"this is a string\""), Rule::STRING, true).ok().unwrap();
        try_parse(String::from("\"this is \\t another \\n string\""), Rule::STRING, false).ok().unwrap();
    }

    #[test]
    fn parse_exp() {
        try_parse(String::from("\"this is a string\""), Rule::Exp, true).ok().unwrap();
        try_parse(String::from("123"), Rule::Exp, true).ok().unwrap();
        try_parse(String::from("false"), Rule::Exp, true).ok().unwrap();
        try_parse(String::from("{1,2,3,4,5,6,7}"), Rule::Exp, true).ok().unwrap();
        try_parse(String::from("{true, 1, 2.0, func()}"), Rule::Exp, true).ok().unwrap();
        try_parse(String::from("pow(2,8)"), Rule::Exp, true).ok().unwrap();
        try_parse(String::from("bitwise(1,10)"), Rule::Exp, true).ok().unwrap();
        try_parse(String::from("1>>8"), Rule::Exp, true).ok().unwrap();
        try_parse(String::from("1<<8"), Rule::Exp, true).ok().unwrap();
        try_parse(String::from("true || false"), Rule::Exp, true).ok().unwrap();
        try_parse(String::from("true && false"), Rule::Exp, true).ok().unwrap();
        try_parse(String::from("x.y"), Rule::Exp, true).ok().unwrap();
        try_parse(String::from("x->y"), Rule::Exp, true).ok().unwrap();
        try_parse(String::from("x[x].inner_data"), Rule::Exp, true).ok().unwrap();
        try_parse(String::from("x[0]->inner_data"), Rule::Exp, true).ok().unwrap();
        try_parse(String::from("1 >= 2"), Rule::Exp, true).ok().unwrap();
        try_parse(String::from("1 <= 2"), Rule::Exp, true).ok().unwrap();
        try_parse(String::from("-1 + 2"), Rule::Exp, true).ok().unwrap();
        try_parse(String::from("-1 + 2*x"), Rule::Exp, true).ok().unwrap();
        try_parse(String::from("x|y"), Rule::Exp, true).ok().unwrap();
        try_parse(String::from("x&y"), Rule::Exp, true).ok().unwrap();
        try_parse(String::from("10/5"), Rule::Exp, true).ok().unwrap();
        try_parse(String::from("(1+2)/3*x[4]/8"), Rule::Exp, true).ok().unwrap();
    }

    #[test]
    fn parse_logical_type() {
        try_parse(String::from("\
        Null
        "), Rule::LogicalType, false).ok().unwrap();
        try_parse(String::from("\
        Bit(8)
        "), Rule::LogicalType, false).ok().unwrap();
        try_parse(String::from("\
        Bit(x)
        "), Rule::LogicalType, false).ok().unwrap();
        try_parse(String::from("\
        Stream(Bit(1), d=x, u=x, t=x)
        "), Rule::LogicalType, false).ok().unwrap();
        try_parse(String::from("\
        Null
        "), Rule::LogicalType, false).ok().unwrap();
        try_parse(String::from("\
        Group x {
            //empty
        }
        "), Rule::LogicalType, false).ok().unwrap();
        try_parse(String::from("\
        Group x {
            value: int = 1;
            x : Bit(1);
            y : Stream(Bit(1));
        }
        "), Rule::LogicalType, false).ok().unwrap();
        try_parse(String::from("\
        Union x {
            value: int = 1;
            string0 = \"123\";
            x : Bit(1);
            y : Stream(Bit(1));
        }
        "), Rule::LogicalType, false).ok().unwrap();
        try_parse(String::from("\
        Group x <v0: int, v1: int> {
            value: int = 1;
            x : Bit(1);
            y : Stream(Bit(1));
        }
        "), Rule::LogicalType, false).ok().unwrap();
        try_parse(String::from("\
        Union x <v0: int, v1: int> {
            value: int = 1;
            string0 = \"123\";
            x : Bit(1);
            y : Stream(Bit(1));
        }
        "), Rule::LogicalType, false).ok().unwrap();
    }

    #[test]
    fn parse_tydi_streamlet() {
        try_parse(String::from("\
        package test;
        streamlet x {
            
        }
        "), Rule::TydiFile, false).ok().unwrap();
        try_parse(String::from("\
        package test;
        streamlet x<len: int, socket: impl of external_package.streamlet0<x,y>> {
            len = x;
            port_in : Stream(Bit(8)) in /clock_domain;
        }
        "), Rule::TydiFile, false).ok().unwrap();
        try_parse(String::from("\
        package test;
        streamlet x<len: int, socket: impl of external_package.streamlet0<x,y>> {
            len = x;
            port_in : Stream(bit_8) in /clock_domain @any_clockDomain @NoTypeCheck;
        }
        "), Rule::TydiFile, false).ok().unwrap();
        try_parse(String::from("\
        package test;
        streamlet x<len: int, socket: impl of external_package.streamlet0<x,y>> {
            len = x;
            port_in : Stream(Bit(8)) in [b] /clock_domain @any_clockDomain @NoTypeCheck;
            port_out : Stream(Bit(8)) out [b] /clock_domain @any_clockDomain @NoTypeCheck;
        }
        "), Rule::TydiFile, false).ok().unwrap();
        try_parse(String::from("\
        package test;
        streamlet x<len: int, socket: impl of external_package.streamlet0<x,y>> @attribute {
            len = x;
            port_in : Stream(Bit(8)) in [b] /clock_domain @any_clockDomain @NoTypeCheck;
            port_out : Stream(Bit(8)) out [b] /clock_domain @any_clockDomain @NoTypeCheck;
        }
        "), Rule::TydiFile, false).ok().unwrap();
    }

    #[test]
    fn parse_tydi_external_template() {
        try_parse(String::from("\
        package test;
        streamlet x <len:int, cat: impl of external_package.x<x, external_package.y<getGlobal(1)>>> {
            
        }
        "), Rule::TydiFile, false).ok().unwrap();
    }

    #[test]
    fn parse_tydi_implementation() {
        try_parse(String::from("\
        package test;
        impl x_impl of x <1,Bit(1)> {

        }
        "), Rule::TydiFile, false).ok().unwrap();
        try_parse(String::from("\
        package test;
        impl x_impl of external_package.x <1,Bit(1)> {
            
        }
        "), Rule::TydiFile, false).ok().unwrap();
        try_parse(String::from("\
        package test;
        impl x_impl<x:int, y:type> of external_package.x <1,Bit(1)> @attribute {
            
        }
        "), Rule::TydiFile, false).ok().unwrap();
        try_parse(String::from("\
        package test;
        impl x_impl of y {
            instance i0(impl0);
            instance i1(impl0<x,y,1>);
            instance i2(impl0<x,y,external_package.i>);
            instance i3(impl0<x,y,external_package.i>) [10];
        }
        "), Rule::TydiFile, false).ok().unwrap();
        try_parse(String::from("\
        package test;
        impl x_impl of y {
            #documentation#
            instance i0(impl0);
            instance i1(impl0<x,y,1>);
            instance i2(impl0<x,y,external_package.i>);
            i0.out => i1.in;
            i1.out => i2.in \"net_name\" @NoTypeCheck @SecondAttr;
        }
        "), Rule::TydiFile, false).ok().unwrap();
    }

    #[test]
    fn parse_tydi_array() {
        try_parse(String::from("\
        package test;
        logicalTypes = {Null, Bit(8)};
        access_external_value = external_package.data[0];
        streamlet x< a0:[int], a1: [type]> {
            
        }
        impl x_impl of y {
            #documentation#
            instance i0(impl0);
            instance i1(impl0<x,y,1>);
            instance i2(impl0<x,y,external_package.i>) [10];
            i0.out => i1.in;
            i1.out => i2.in \"net_name\" @NoTypeCheck @SecondAttr;
            i2[0].out => i2[1].in;
        }
        "), Rule::TydiFile, false).ok().unwrap();
    }

    #[test]
    fn parse_tydi_if_for() {
        try_parse(String::from("\
        package test;
        impl x_impl of y {
            #documentation#
            instance i2(impl0<x,y>) [10];
            for i in {0,1,2,3,4} {
                if (x) {
                    i2[i].out => i2[i].in;
                }
            }
        }
        "), Rule::TydiFile, false).ok().unwrap();
    }

    #[test]
    fn parse_tydi_miscellaneous() {
        //use external package
        try_parse(String::from("\
        package test;
        use external_package;
        "), Rule::TydiFile, false).ok().unwrap();
        //function
        try_parse(String::from("\
        package test;
        assert(x==y);
        "), Rule::TydiFile, false).ok().unwrap();
    }

}
