#[cfg(test)]
mod all_parse_test
{
    use std::{collections::HashMap, sync::{Arc, RwLock}};
    use crate::tydi_memory_representation::Project;

    #[test]
    fn sample_project_0() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            project_write.add_package(format!("./pack0.td"), format!("
            package pack0;
            use pack1;
            i = 10;
            i2 = pack1.i + 10;
            
            ")).expect("cannot add package");
            project_write.add_package(format!("./pack1.td"), format!("
            package pack1;
            use pack0;
    
            i = pack0.i + 10;
            
            ")).expect("cannot add package");
        }
        project.read().unwrap().evaluate_target(format!("i2"), format!("pack0")).expect("fail to evaluate");
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        println!("{json_output}");
        std::fs::write("./output.json", &json_output).unwrap();
    }
    
    #[test]
    fn sample_project_1() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            project_write.add_package(format!("./pack0.td"), format!("
            package pack0;
            use pack1;
            i = 10;
            i2 = i;
            
            ")).expect("cannot add package");
        }
        project.read().unwrap().evaluate_target(format!("i2"), format!("pack0")).expect("fail to evaluate");
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        println!("{json_output}");
        std::fs::write("./output.json", &json_output).unwrap();
    }
    
    #[test]
    fn sample_project_mutual_ref() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            project_write.add_package(format!("./pack0.td"), format!("
            package pack0;
            use pack1;
    
            i = pack1.i2 + 10;      //we should see deadlock if comment this line
            //i = pack1.i + 10;     //we should see deadlock if uncomment this line
    
            ")).expect("cannot add package");
            project_write.add_package(format!("./pack1.td"), format!("
            package pack1;
            use pack0;
            i2 = 90;
            i = pack0.i + 10;
            
            ")).expect("cannot add package");
        }
        project.read().unwrap().evaluate_target(format!("i"), format!("pack0")).expect("fail to evaluate");
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        println!("{json_output}");
        std::fs::write("./output.json", &json_output).unwrap();
    }
    
    #[test]
    fn sample_project_2() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            let src_pack0 = format!("
            package pack0;
    
            eight = 8;
            bit_8 = Bit(eight*10-1);
    
            ");
            let src_pack1 = format!("
            package pack1;
            use pack0;
    
            stream_bit_8_2 = Stream(pack0.bit_8);
            ");
    
    
    
            let status = project_write.add_package(format!("./pack0.td"), src_pack0);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
            let status = project_write.add_package(format!("./pack1.td"), src_pack1);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
        }
        project.read().unwrap().evaluate_target(format!("stream_bit_8_2"), format!("pack1")).expect("fail to evaluate");
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        println!("{json_output}");
        std::fs::write("./output.json", &json_output).unwrap();
    }
    
    #[test]
    fn sample_project_3() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            let src_pack0 = format!("
            package pack0;
    
            eight = 8;
    
            ");
            let src_pack1 = format!("
            package pack1;
            use pack0;
    
            bit_8 = Bit(pack0.eight);
            stream_bit_8 = Stream(bit_8);
            ");
    
    
    
            let status = project_write.add_package(format!("./pack0.td"), src_pack0);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
            let status = project_write.add_package(format!("./pack1.td"), src_pack1);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
        }
        project.read().unwrap().evaluate_target(format!("stream_bit_8"), format!("pack1")).expect("fail to evaluate");
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        println!("{json_output}");
        std::fs::write("./output.json", &json_output).unwrap();
    }
    
    #[test]
    fn sample_project_4() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            let src_pack0 = String::from(r#"
            package pack0;
    
            bit_8 = Bit(8);
    
            Group rgb {
                r : bit_8;
                g : bit_8;
                b : bit_8;
            }
    
            "#);
            let src_pack1 = format!("
            package pack1;
            use pack0;
    
            stream_rgb = Stream(pack0.rgb);
            ");
    
    
    
            let status = project_write.add_package(format!("./pack0.td"), src_pack0);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
            let status = project_write.add_package(format!("./pack1.td"), src_pack1);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
        }
        project.read().unwrap().evaluate_target(format!("stream_rgb"), format!("pack1")).expect("fail to evaluate");
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        println!("{json_output}");
        std::fs::write("./output.json", &json_output).unwrap();
    }
    
    #[test]
    fn sample_project_5() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            let src_pack0 = String::from(r#"
            package pack0;
            use pack1;
    
            Group rgb {
                r : pack1.bit_8;
                g : pack1.bit_8;
                b : pack1.bit_8;
            }
    
            "#);
            let src_pack1 = format!("
            package pack1;
            use pack0;
    
            bit_8 = Bit(8);
            stream_rgb = Stream(pack0.rgb);
            ");
    
    
    
            let status = project_write.add_package(format!("./pack0.td"), src_pack0);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
            let status = project_write.add_package(format!("./pack1.td"), src_pack1);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
        }
        project.read().unwrap().evaluate_target(format!("stream_rgb"), format!("pack1")).expect("fail to evaluate");
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        println!("{json_output}");
        std::fs::write("./output.json", &json_output).unwrap();
    }
    
    #[test]
    fn sample_project_6() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            let src_pack0 = String::from(r#"
            package pack0;
            use pack1;
    
            Group rgb {
                seven = 7;
                eight = 8;
                bit_8 = Bit(eight);
                r : bit_8;
                g : bit_8;
                b : bit_8;
            }
    
            "#);
            let src_pack1 = format!("
            package pack1;
            use pack0;
    
            stream_rgb = Stream(pack0.rgb);
            ");
    
    
    
            let status = project_write.add_package(format!("./pack0.td"), src_pack0);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
            let status = project_write.add_package(format!("./pack1.td"), src_pack1);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
        }
        project.read().unwrap().evaluate_target(format!("stream_rgb"), format!("pack1")).expect("fail to evaluate");
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        println!("{json_output}");
        std::fs::write("./output.json", &json_output).unwrap();
    }
    
    #[test]
    fn sample_project_7() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            let src_pack0 = String::from(r#"
            package pack0;
            use pack1;
    
            Union rgb {
                seven = 7;
                eight = 8;
                bit_8 = Bit(eight);
                r : bit_8;
                g : bit_8;
                b : bit_8;
            }
    
            "#);
            let src_pack1 = format!("
            package pack1;
            use pack0;
    
            stream_rgb = Stream(pack0.rgb);
            stream_rgb2 = Stream(pack0.rgb);
            ");
    
    
    
            let status = project_write.add_package(format!("./pack0.td"), src_pack0);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
            let status = project_write.add_package(format!("./pack1.td"), src_pack1);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
        }
        let evaluator = project.read().unwrap().evaluate_target(format!("stream_rgb"), format!("pack1")).expect("fail to evaluate");
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        println!("{json_output}");
        std::fs::write("./output.json", &json_output).unwrap();
    
        println!("{}", evaluator.read().unwrap().print_evaluation_record());
    }
    
    #[test]
    fn sample_project_8() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            let src_pack0 = String::from(r#"
            package pack0;
            
            bit8 = Bit(8);
            bit8_stream = Stream(bit8);
            "#);
            let src_pack1 = format!("
            package pack1;
            use pack0;
            zero = 0;
            eight = pack0.bit8.width;           //8
            complexity = pack0.bit8_stream.c;   //1
            throughtput = pack0.bit8_stream.t;  //1.0
            result = eight + complexity + throughtput;
            ");
    
    
    
            let status = project_write.add_package(format!("./pack0.td"), src_pack0);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
            let status = project_write.add_package(format!("./pack1.td"), src_pack1);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
        }
        let evaluator = project.read().unwrap().evaluate_target(format!("result"), format!("pack1")).expect("fail to evaluate");
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        println!("{json_output}");
        std::fs::write("./output.json", &json_output).unwrap();
    
        println!("{}", evaluator.read().unwrap().print_evaluation_record());
    }
    
    #[test]
    fn sample_project_access_value_in_group() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            let src_pack0 = String::from(r#"
            package pack0;
            use pack1;
    
            Group another_group {
                six = 6;
            }
    
            Group rgb {
                seven = 7;
                eight = 8;
                bit_8 = Bit(eight);
                r : bit_8;
                g : bit_8;
                b : bit_8;
                a : another_group;
            }
    
            "#);
            let src_pack1 = format!("
            package pack1;
            use pack0;
    
            seven = pack0.rgb.a.six;
            ");
    
    
    
            let status = project_write.add_package(format!("./pack0.td"), src_pack0);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
            let status = project_write.add_package(format!("./pack1.td"), src_pack1);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
        }
        let evaluator = project.read().unwrap().evaluate_target(format!("seven"), format!("pack1")).expect("fail to evaluate");
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        println!("{json_output}");
        std::fs::write("./output.json", &json_output).unwrap();
    
        println!("{}", evaluator.read().unwrap().print_evaluation_record());
    }
    
    #[test]
    fn sample_project_streamlet() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            let src_pack0 = String::from(r#"
            package pack0;
    
            bit_8 = Bit(8);
    
            bit_8_stream = Stream(bit_8);
    
            "#);
            let src_pack1 = String::from(r#"
            package pack1;
            use pack0;
    
            #this is just an exmple streamlet#
            streamlet bit_8_bypass {
                in_port : pack0.bit_8_stream in;
                out_port : pack0.bit_8_stream out;
            }
            "#);
    
    
    
            let status = project_write.add_package(format!("./pack0.td"), src_pack0);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
            let status = project_write.add_package(format!("./pack1.td"), src_pack1);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
        }
        let evaluator = project.read().unwrap().evaluate_target(format!("bit_8_bypass"), format!("pack1")).expect("fail to evaluate");
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        println!("{json_output}");
        std::fs::write("./output.json", &json_output).unwrap();
    
        println!("{}", evaluator.read().unwrap().print_evaluation_record());
    }
    
    #[test]
    fn sample_project_nested_stream() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            let src_pack0 = String::from(r#"
            package pack0;
    
            char_8 = Bit(8);
    
            char_string_stream = Stream(char_8, d=1);
    
            timestamp = Bit(60);
    
            Group char_string_timestamp {
                string0: char_string_stream;
                time_stamp: timestamp;
            }
    
            char_string_timestamp_stream = Stream(char_string_timestamp);
    
            "#);
            let src_pack1 = String::from(r#"
            package pack1;
            use pack0;
    
            #this is just an exmple streamlet#
            streamlet bypass {
                in_port : pack0.char_string_timestamp_stream in;
                out_port : pack0.char_string_timestamp_stream out;
            }
            "#);
    
    
    
            let status = project_write.add_package(format!("./pack0.td"), src_pack0);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
            let status = project_write.add_package(format!("./pack1.td"), src_pack1);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
        }
        let evaluator = project.read().unwrap().evaluate_target(format!("bypass"), format!("pack1")).expect("fail to evaluate");
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        println!("{json_output}");
        std::fs::write("./output.json", &json_output).unwrap();
    
        println!("{}", evaluator.read().unwrap().print_evaluation_record());
    }
    
    #[test]
    fn sample_project_simple_impl_1() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            let src_pack0 = String::from(r#"
            package pack0;
    
            char_8 = Bit(8);
    
            char_string_stream = Stream(char_8, d=1);
    
            timestamp = Bit(60);
    
            Group char_string_timestamp {
                string0: char_string_stream;
                time_stamp: timestamp;
            }
    
            char_string_timestamp_stream = Stream(char_string_timestamp);
    
            "#);
            let src_pack1 = String::from(r#"
            package pack1;
            use pack0;
    
            #this is just an exmple streamlet#
            streamlet bypass {
                in_port : pack0.char_string_timestamp_stream in;
                out_port : pack0.char_string_timestamp_stream out;
            }
    
            impl bypass_i of bypass {
                self.in_port => self.out_port;
            }
            "#);
    
    
    
            let status = project_write.add_package(format!("./pack0.td"), src_pack0);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
            let status = project_write.add_package(format!("./pack1.td"), src_pack1);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
        }
        let evaluator = project.read().unwrap().evaluate_target(format!("bypass_i"), format!("pack1")).expect("fail to evaluate");
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        println!("{json_output}");
        std::fs::write("./output.json", &json_output).unwrap();
    
        println!("{}", evaluator.read().unwrap().print_evaluation_record());
    }
    
    
    #[test]
    fn sample_project_simple_impl_2() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            let src_pack0 = String::from(r#"
            package pack0;
    
            char_8 = Bit(8);
    
            char_string_stream = Stream(char_8, d=1);
    
            timestamp = Bit(60);
    
            Group char_string_timestamp {
                string0: char_string_stream;
                time_stamp: timestamp;
            }
    
            char_string_timestamp_stream = Stream(char_string_timestamp);
    
            "#);
            let src_pack1 = String::from(r#"
            package pack1;
            use pack0;
    
            #this is just an example streamlet#
            streamlet bypass {
                in_port : pack0.char_string_timestamp_stream in;
                out_port : pack0.char_string_timestamp_stream out;
            }
    
            impl bypass_i of bypass {
                self.in_port => self.out_port;
            }
    
            #this is just an example streamlet#
            streamlet bypass2 {
                in_port2 : pack0.char_string_timestamp_stream in;
                out_port2 : pack0.char_string_timestamp_stream out;
            }
    
            impl bypass2_i of bypass2 {
                self.in_port2 => self.out_port2;
            }
    
            impl bypass_i2 of bypass {
                instance nested_self(bypass2_i);
                self.in_port => nested_self.in_port2;
                nested_self.out_port2 => self.out_port;
    
                i = 0;
            }
            "#);
    
    
    
            let status = project_write.add_package(format!("./pack0.td"), src_pack0);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
            let status = project_write.add_package(format!("./pack1.td"), src_pack1);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
        }
        let evaluator = project.read().unwrap().evaluate_target(format!("bypass_i2"), format!("pack1")).expect("fail to evaluate");
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        println!("{json_output}");
        std::fs::write("./output.json", &json_output).unwrap();
    
        println!("{}", evaluator.read().unwrap().print_evaluation_record());
    }
    
    #[test]
    fn sample_project_simple_array_0() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            let src_pack0 = String::from(r#"
            package pack0;
            three = 3;
            array_0 = [0, 1.0, "2", three, three+1, Bit(5)];
    
            "#);
            let src_pack1 = String::from(r#"
            package pack1;
            use pack0;
    
            value = pack0.array_0[3];
    
            "#);
    
    
    
            let status = project_write.add_package(format!("./pack0.td"), src_pack0);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
            let status = project_write.add_package(format!("./pack1.td"), src_pack1);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
        }
        let evaluator = project.read().unwrap().evaluate_target(format!("value"), format!("pack1")).expect("fail to evaluate");
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        println!("{json_output}");
        std::fs::write("./output.json", &json_output).unwrap();
    
        println!("{}", evaluator.read().unwrap().print_evaluation_record());
    }
    
    #[test]
    fn sample_project_simple_if_0() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            let src_pack0 = String::from(r#"
            package pack0;
    
            char_8 = Bit(8);
    
            char_string_stream = Stream(char_8, d=1);
    
            timestamp = Bit(60);
    
            flag = true && false;
    
            Group char_string_timestamp {
                string0: char_string_stream;
                time_stamp: timestamp;
            }
    
            char_string_timestamp_stream = Stream(char_string_timestamp);
    
            "#);
            let src_pack1 = String::from(r#"
            package pack1;
            use pack0;
    
            streamlet bypass {
                in_port : pack0.char_string_timestamp_stream in;
                out_port : pack0.char_string_timestamp_stream out;
            }
    
    
            streamlet bypass2 {
                in_port2 : pack0.char_string_timestamp_stream in;
                out_port2 : pack0.char_string_timestamp_stream out;
            }
    
            impl bypass2_i of bypass2 {
                self.in_port2 => self.out_port2;
            }
    
            impl bypass_i of bypass {
                if pack0.flag {
                    self.in_port => self.out_port;
                }
                else {
                    instance nested_self(bypass2_i);
                    self.in_port => nested_self.in_port2;
                    nested_self.out_port2 => self.out_port;
                }
            }
            "#);
    
            let status = project_write.add_package(format!("./pack0.td"), src_pack0);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
            let status = project_write.add_package(format!("./pack1.td"), src_pack1);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
        }
        let evaluator = project.read().unwrap().evaluate_target(format!("bypass_i"), format!("pack1"));
        let evaluator = match evaluator {
            Ok(e) => e,
            Err(e) => {
                println!("{}", e.print());
                return;
            },
        };
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        std::fs::write("./output.json", &json_output).unwrap();
    
        println!("{}", evaluator.read().unwrap().print_evaluation_record());
    }
    
    #[test]
    fn sample_project_simple_for_0() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            let src_pack0 = String::from(r#"
            package pack0;
    
            char_8 = Bit(8);
    
            char_string_stream = Stream(char_8, d=1);
    
            timestamp = Bit(60);
    
            flag = true && false;
    
            Group char_string_timestamp {
                string0: char_string_stream;
                time_stamp: timestamp;
            }
    
            char_string_timestamp_stream = Stream(char_string_timestamp);
    
            "#);
            let src_pack1 = String::from(r#"
            package pack1;
            use pack0;
    
            streamlet bypass {
                in_port : pack0.char_string_timestamp_stream in;
                out_port : pack0.char_string_timestamp_stream out;
            }
    
    
            streamlet bypass2 {
                in_port2 : pack0.char_string_timestamp_stream in;
                out_port2 : pack0.char_string_timestamp_stream out;
            }
    
            impl bypass2_i of bypass2 {
                self.in_port2 => self.out_port2;
            }
    
            impl bypass_i of bypass {
                for_array = [0,1,2];
                for_not_array = 0;
                for i in for_array {
                    if i > 1 {
                        data = i + 2;
                    }
                    instance nested_self(bypass2_i);
                    self.in_port => nested_self.in_port2;
                    nested_self.out_port2 => self.out_port;
                }
            }
            "#);
    
            let status = project_write.add_package(format!("./pack0.td"), src_pack0);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
            let status = project_write.add_package(format!("./pack1.td"), src_pack1);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
        }
        let evaluator = project.read().unwrap().evaluate_target(format!("bypass_i"), format!("pack1"));
        let evaluator = match evaluator {
            Ok(e) => e,
            Err(e) => {
                println!("{}", e.print());
                return;
            },
        };
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        std::fs::write("./output.json", &json_output).unwrap();
    
        println!("{}", evaluator.read().unwrap().print_evaluation_record());
    }
    
    
    #[test]
    fn sample_project_simple_template_0() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            let src_pack0 = String::from(r#"
            package pack0;
    
            Group any_bit<n: int> {
                data: Bit(n);
            }
    
            "#);
            let src_pack1 = String::from(r#"
            package pack1;
            use pack0;
    
            bit8 = pack0.any_bit<8>;
            "#);
    
            let status = project_write.add_package(format!("./pack0.td"), src_pack0);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
            let status = project_write.add_package(format!("./pack1.td"), src_pack1);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
        }
        let evaluator = project.read().unwrap().evaluate_target(format!("bit8"), format!("pack1"));
        let evaluator = match evaluator {
            Ok(e) => e,
            Err(e) => {
                println!("{}", e.print());
                return;
            },
        };
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        std::fs::write("./output.json", &json_output).unwrap();
    
        println!("{}", evaluator.read().unwrap().print_evaluation_record());
    }

    #[test]
    fn sample_project_simple_template_1() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            let src_pack0 = String::from(r#"
            package pack0;
    
            Group any_bit<n: int> {
                data: Bit(n);
            }
    
            "#);
            let src_pack1 = String::from(r#"
            package pack1;
            use pack0;
    
            bit8 = pack0.any_bit<8>;
            bit8_data = pack0.any_bit<8>.data;
            "#);
    
            let status = project_write.add_package(format!("./pack0.td"), src_pack0);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
            let status = project_write.add_package(format!("./pack1.td"), src_pack1);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
        }
        let evaluator = project.read().unwrap().evaluate_target(format!("bit8_data"), format!("pack1"));
        let evaluator = match evaluator {
            Ok(e) => e,
            Err(e) => {
                println!("{}", e.print());
                return;
            },
        };
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        std::fs::write("./output.json", &json_output).unwrap();
    
        println!("{}", evaluator.read().unwrap().print_evaluation_record());
    }

    #[test]
    fn sample_project_simple_template_2() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            let src_pack0 = String::from(r#"
            package pack0;
    
            Group any_bit<n: int> {
                for i in [0,1,2,3]
                {
                    data: Bit(n);
                }
            }
    
            "#);
            let src_pack1 = String::from(r#"
            package pack1;
            use pack0;
    
            bit8 = pack0.any_bit<8>;
            bit8_data = pack0.any_bit<8>.data;
            "#);
    
            let status = project_write.add_package(format!("./pack0.td"), src_pack0);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
            let status = project_write.add_package(format!("./pack1.td"), src_pack1);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
        }
        let evaluator = project.read().unwrap().evaluate_target(format!("bit8_data"), format!("pack1"));
        let evaluator = match evaluator {
            Ok(e) => e,
            Err(e) => {
                let json_output = project.read().unwrap().get_pretty_json();
                std::fs::write("./output.json", &json_output).unwrap();
                println!("{}", e.print());
                return;
            },
        };
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        std::fs::write("./output.json", &json_output).unwrap();
    
        println!("{}", evaluator.read().unwrap().print_evaluation_record());
    }

    #[test]
    fn sample_project_simple_template_3() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            let src_pack0 = String::from(r#"
            package pack0;
    
            Group any_bit<n: int> {
                for i in [0,1,2,3]
                {
                    data: Bit(n);
                }
            }
    
            "#);
            let src_pack1 = String::from(r#"
            package pack1;
            use pack0;
    
            bit8 = pack0.any_bit<8>;
            bit8_data_index0 = pack0.any_bit<8>.data[0];
            "#);
    
            let status = project_write.add_package(format!("./pack0.td"), src_pack0);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
            let status = project_write.add_package(format!("./pack1.td"), src_pack1);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
        }
        let evaluator = project.read().unwrap().evaluate_target(format!("bit8_data_index0"), format!("pack1"));
        let evaluator = match evaluator {
            Ok(e) => e,
            Err(e) => {
                let json_output = project.read().unwrap().get_pretty_json();
                std::fs::write("./output.json", &json_output).unwrap();
                println!("{}", e.print());
                return;
            },
        };
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        std::fs::write("./output.json", &json_output).unwrap();
    
        println!("{}", evaluator.read().unwrap().print_evaluation_record());
    }

    #[test]
    fn sample_project_simple_template_4() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            let src_pack0 = String::from(r#"
            package pack0;
    
            Union any_bit<n: int> {
                bit_8: Bit(8);
                for i in [1,2,3]
                {
                    data: Bit(i);
                }
            }
    
            "#);
            let src_pack1 = String::from(r#"
            package pack1;
            use pack0;
    
            bit8 = pack0.any_bit<8>;
            bit8_data_index0 = pack0.any_bit<8>.data[0];
            "#);
    
            let status = project_write.add_package(format!("./pack0.td"), src_pack0);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
            let status = project_write.add_package(format!("./pack1.td"), src_pack1);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
        }
        let evaluator = project.read().unwrap().evaluate_target(format!("bit8_data_index0"), format!("pack1"));
        let evaluator = match evaluator {
            Ok(e) => e,
            Err(e) => {
                let json_output = project.read().unwrap().get_pretty_json();
                std::fs::write("./output.json", &json_output).unwrap();
                println!("{}", e.print());
                return;
            },
        };
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        std::fs::write("./output.json", &json_output).unwrap();
    
        println!("{}", evaluator.read().unwrap().print_evaluation_record());
    }

    #[test]
    fn sample_project_simple_template_5() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            let src_pack0 = String::from(r#"
            package pack0;
            
            Group bitn<n: int> {
                bits: Bit(n);
            }
            bit8_stream = Stream(bitn<8>);
            bit16_stream = Stream(bitn<16>);
            
            "#);
            let src_pack1 = String::from(r#"
            package pack1;
            use pack0;
            
            streamlet bypass <logic_type: type> {
                in_port: logic_type in;
                out_port: logic_type out;
            }

            bypass_bit8 = bypass<pack0.bit8_stream>;

            "#);
            
            let status = project_write.add_package(format!("./pack0.td"), src_pack0);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
            let status = project_write.add_package(format!("./pack1.td"), src_pack1);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
        }
        let evaluator = project.read().unwrap().evaluate_target(format!("bypass_bit8"), format!("pack1"));
        let evaluator = match evaluator {
            Ok(e) => e,
            Err(e) => {
                let json_output = project.read().unwrap().get_pretty_json();
                std::fs::write("./output.json", &json_output).unwrap();
                println!("{}", e.print());
                return;
            },
        };
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        std::fs::write("./output.json", &json_output).unwrap();
    
        println!("{}", evaluator.read().unwrap().print_evaluation_record());
    }

    #[test]
    fn sample_project_simple_template_6() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            let src_pack0 = String::from(r#"
            package pack0;
            
            Group bitn<n: int> {
                bits: Bit(n);
            }
            bit8_stream = Stream(bitn<8>);
            bit16_stream = Stream(bitn<16>);
            
            "#);
            let src_pack1 = String::from(r#"
            package pack1;
            use pack0;
            
            streamlet bypass <logic_type: type> {
                in_port: logic_type in;
                out_port: logic_type out;
            }

            impl i_bypass <logic_type: type> of bypass<logic_type> {
                in_port => out_port;
            }

            bypass_bit8 = i_bypass<pack0.bit8_stream>;

            "#);
            
            let status = project_write.add_package(format!("./pack0.td"), src_pack0);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
            let status = project_write.add_package(format!("./pack1.td"), src_pack1);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
        }
        let evaluator = project.read().unwrap().evaluate_target(format!("bypass_bit8"), format!("pack1"));
        let evaluator = match evaluator {
            Ok(e) => e,
            Err(e) => {
                let json_output = project.read().unwrap().get_pretty_json();
                std::fs::write("./output.json", &json_output).unwrap();
                println!("{}", e.print());
                return;
            },
        };
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        std::fs::write("./output.json", &json_output).unwrap();
    
        println!("{}", evaluator.read().unwrap().print_evaluation_record());
    }

    #[test]
    fn sample_project_simple_function_6() {
        let project = Project::new(format!("sample_project"));
        {
            let mut project_write = project.write().unwrap();
    
            let src_pack0 = String::from(r#"
            package pack0;
            
            data = 8;
            
            "#);
            let src_pack1 = String::from(r#"
            package pack1;
            use pack0;
            
            assert(pack0.data == 8);

            "#);
            
            let status = project_write.add_package(format!("./pack0.td"), src_pack0);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
            let status = project_write.add_package(format!("./pack1.td"), src_pack1);
            if status.is_err() {
                panic!("{}", status.err().unwrap().print());
            }
        }
        let evaluator = project.read().unwrap().evaluate_target(format!("data"), format!("pack1"));
        let evaluator = match evaluator {
            Ok(e) => e,
            Err(e) => {
                let json_output = project.read().unwrap().get_pretty_json();
                std::fs::write("./output.json", &json_output).unwrap();
                println!("{}", e.print());
                return;
            },
        };
    
        let json_output = project.read().unwrap().get_pretty_json();
    
        std::fs::write("./output.json", &json_output).unwrap();
    
        println!("{}", evaluator.read().unwrap().print_evaluation_record());
    }
}
