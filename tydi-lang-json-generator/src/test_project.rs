#[allow(unused_imports)]
use tydi_lang_parser::tydi_memory_representation::Project;
#[allow(unused_imports)]
use crate::generate_json_representation_from_tydi_project;


#[test]
fn sample_project_rgb() {
    let project = Project::new(format!("sample_project"));
    {
        let mut project_write = project.write().unwrap();

        let src_pack0 = String::from(r#"
        package pack0;

        bit_8 = Bit(8);
        bit_8_copy = Bit(8);

        Group rgb {
            r : bit_8;
            g : bit_8;
            b : bit_8_copy;
        }

        "#);
        let src_pack1 = format!("
        package pack1;
        use pack0;

        stream_rgb = Stream(pack0.rgb, d=2, u=pack0.bit_8);
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

    let code_structure = project.read().unwrap().get_pretty_json();
    std::fs::write("./code_structure.json", &code_structure).unwrap();

    let json_output = generate_json_representation_from_tydi_project(project.clone(), format!("stream_rgb"), format!("pack1")).expect("fail to generate json");
    std::fs::write("./json_output.json", &json_output).unwrap();
    println!("{}", json_output);
}

#[test]
fn sample_project_union_0() {
    let project = Project::new(format!("sample_project"));
    {
        let mut project_write = project.write().unwrap();

        let src_pack0 = String::from(r#"
        package pack0;

        bit_4 = Bit(4);
        bit_8 = Bit(8);
        bit_16 = Bit(16);

        Union size {
            small : bit_4;
            mid : bit_8;
            large : bit_16;
        }

        "#);
        let src_pack1 = format!("
        package pack1;
        use pack0;

        stream_size = Stream(pack0.size);
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
    project.read().unwrap().evaluate_target(format!("stream_size"), format!("pack1")).expect("fail to evaluate");

    let code_structure = project.read().unwrap().get_pretty_json();
    std::fs::write("./code_structure.json", &code_structure).unwrap();

    let json_output = generate_json_representation_from_tydi_project(project.clone(), format!("stream_size"), format!("pack1")).expect("fail to generate json");
    std::fs::write("./json_output.json", &json_output).unwrap();
    println!("{}", json_output);
}

#[test]
fn sample_project_streamlet_impl_0() {
    let project = Project::new(format!("sample_project"));
    {
        let mut project_write = project.write().unwrap();

        let src_pack0 = String::from(r#"
        package pack0;

        bit_4 = Bit(4);
        bit_8 = Bit(8);
        bit_16 = Bit(16);

        #this is an union size#
        Union size {
            small : bit_4;
            mid : bit_8;
            large : bit_16;
        }

        stream_size = Stream(size);

        "#);
        let src_pack1 = String::from(r#"
        package pack1;
        use pack0;

        #this is a streamlet#
        streamlet bypass_s {
            # this is port_in #
            port_in: pack0.stream_size in;
            
            # this is port_out #
            port_out: pack0.stream_size out;
        }

        #this is an implementation#
        impl bypass_i_inner of bypass_s {
            self.port_in => self.port_out;
        }

        impl bypass_i of bypass_s {
            # this instance is used to test using an implementation without template expansion #
            instance test_inst(bypass_i_inner);

            # ports on self have "opposite" direction #
            self.port_in => test_inst.port_in;
            test_inst.port_out => self.port_out;
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
    project.read().unwrap().evaluate_target(format!("bypass_i"), format!("pack1")).expect("fail to evaluate");

    let code_structure = project.read().unwrap().get_pretty_json();
    std::fs::write("./code_structure.json", &code_structure).unwrap();

    let json_output = generate_json_representation_from_tydi_project(project.clone(), format!("bypass_i"), format!("pack1")).expect("fail to generate json");
    std::fs::write("./json_output.json", &json_output).unwrap();
    println!("{}", json_output);
}


#[test]
fn sample_project_stdlib_0() {
    let project = Project::new(format!("sample_project"));
    {
        let mut project_write = project.write().unwrap();

        let src_pack0 = String::from(r#"
        package pack0;

        bit_4 = Bit(4);
        bit_8 = Bit(8);
        bit_16 = Bit(16);

        Union size {
            small : bit_4;
            mid : bit_8;
            large : bit_16;
        }

        stream_size = Stream(size);

        "#);
        let src_pack1 = String::from(r#"
        package pack1;
        use pack0;

        streamlet bypass_s<bypass_type: type> {
            port_in: bypass_type in;
            port_out: bypass_type out;
        }

        impl bypass_i<bypass_type: type> of bypass_s<bypass_type> @NoTemplateExpansion @External {
            
        }

        impl bypass_i_stream_size of bypass_s <pack0.stream_size> {
            instance bypass_inst(bypass_i<pack0.stream_size>);

            self.port_in => bypass_inst.port_in;
            bypass_inst.port_out => self.port_out;
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
    project.read().unwrap().evaluate_target(format!("bypass_i_stream_size"), format!("pack1")).expect("fail to evaluate");

    let code_structure = project.read().unwrap().get_pretty_json();
    std::fs::write("./code_structure.json", &code_structure).unwrap();

    let json_output = generate_json_representation_from_tydi_project(project.clone(), format!("bypass_i_stream_size"), format!("pack1")).expect("fail to generate json");
    std::fs::write("./json_output.json", &json_output).unwrap();
    println!("{}", json_output);
}


#[test]
fn sample_project_nested_stream_0() {
    let project = Project::new(format!("sample_project"));
    {
        let mut project_write = project.write().unwrap();

        let src_pack0 = String::from(r#"
        package pack0;

        chars = Stream(Bit(8),t=2.0,d=1,c=1); /*synchronicity: Sync*/
        
        Group timestamped_group {
            time: Bit(64);
            message: chars;
        }
        
        timestamped_message = Stream(timestamped_group,t=2.0,d=1,c=1 /*synchronicity: Sync*/ ); 
        
        streamlet child {
            timestamped_message_in: timestamped_message in;
            timestamped_message_out: timestamped_message out;
        }
        
        streamlet example {
            timestamped_message_in: timestamped_message in;
            timestamped_message_out: timestamped_message out;
        }
        
        impl ChildImpl of child {}
        
        impl ExampleImpl of example @External {
            instance a(ChildImpl);
            instance b(ChildImpl);
            self.timestamped_message_in => a.timestamped_message_in;
            a.timestamped_message_out => b.timestamped_message_in;
            b.timestamped_message_out => self.timestamped_message_out;
            data = 10;
        }

        entry = ExampleImpl.data;

        "#);
        let src_pack1 = String::from(r#"
        package pack1;
        use pack0;



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
    project.read().unwrap().evaluate_target(format!("entry"), format!("pack0")).expect("fail to evaluate");

    let code_structure = project.read().unwrap().get_pretty_json();
    std::fs::write("./code_structure.json", &code_structure).unwrap();

    let json_output = generate_json_representation_from_tydi_project(project.clone(), format!("ExampleImpl"), format!("pack0")).expect("fail to generate json");
    std::fs::write("./json_output.json", &json_output).unwrap();
    println!("{}", json_output);
}


#[test]
fn sample_project_comment_bug() {
    let project = Project::new(format!("sample_project"));
    {
        let mut project_write = project.write().unwrap();

        let src_pack0 = String::from(r#"
        package pack0;

        chars = Stream(Bit(8) /*synchronicity: Sync*/ ,c=8 /*synchronicity: Sync*/ ); 
        "#);
        let src_pack1 = String::from(r#"
        package pack1;
        use pack0;



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
    std::fs::write("./code_structure_before_evaluation.json", &project.read().unwrap().get_pretty_json()).unwrap();

    project.read().unwrap().evaluate_target(format!("chars"), format!("pack0")).expect("fail to evaluate");

    let code_structure = project.read().unwrap().get_pretty_json();
    std::fs::write("./code_structure.json", &code_structure).unwrap();

    let json_output = generate_json_representation_from_tydi_project(project.clone(), format!("chars"), format!("pack0")).expect("fail to generate json");
    std::fs::write("./json_output.json", &json_output).unwrap();
    println!("{}", json_output);
}

#[test]
fn sample_project_impl_of_template() {
    let project = Project::new(format!("sample_project"));
    {
        let mut project_write = project.write().unwrap();

        let src_pack0 = String::from(r#"
        package pack0;
    
        Group t {
            a: Bit(8);
        }
        t_stream = Stream(t);

        streamlet bypass_s {
            port_in: t_stream in;
            port_out: t_stream out;
        }

        impl bypass_i of bypass_s {
            port_in => port_out;
        }

        impl bypass_external<inner: impl of bypass_s> of bypass_s {
            instance i(inner);
            port_in => i.port_in;
            i.port_out => port_out;
        }

        start = bypass_external<bypass_i>;
        "#);
        let src_pack1 = String::from(r#"
        package pack1;
        use pack0;

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
    std::fs::write("./code_structure_before_evaluation.json", &project.read().unwrap().get_pretty_json()).unwrap();

    project.read().unwrap().evaluate_target(format!("start"), format!("pack0")).expect("fail to evaluate");

    let code_structure = project.read().unwrap().get_pretty_json();
    std::fs::write("./code_structure.json", &code_structure).unwrap();

    let json_output = generate_json_representation_from_tydi_project(project.clone(), format!("start"), format!("pack0")).expect("fail to generate json");
    std::fs::write("./json_output.json", &json_output).unwrap();
    println!("{}", json_output);
}

#[test]
fn casper_wrong_error_line_bug() {
    let project = Project::new(format!("sample_project"));
    {
        let mut project_write = project.write().unwrap();

        let src_pack0 = String::from(r#"
            package snappy;

            byte = Bit(8);
            
            byte_stream = Stream(byte, t=8.0, d=1, c=1);
            
            streamlet decompressor {
                # Compressed data #
                co: byte_stream in;
                # Decompressed data #
                de: byte_stream out;
            }
            
            impl VhSnUnzipUnbufferedWrap of decompressor {}
            
            base_d = 2;
            
            JSONStream = Stream(
                byte,
                throughput = 4.0,
                dimension = base_d,
                synchronicity = "Sync",
                complexity = 7
            );
            
            IntParserStream = Stream(
                Bit(64),
                throughput = 1.0,
                dimension = base_d+2,
                synchronicity = "Sync",
                complexity = 2
            );
            
            BoolParserStream = Stream(
                Bit(1),
                throughput = 1.0,
                dimension = base_d,
                synchronicity = "Sync",
                complexity = 2
            );
            
            streamlet schema_0_parser_0_top {
                input: JSONStream in;
                output_int_parser_L4_00_inst: IntParserStream out;
                output_bool_parser_L2_00_inst: BoolParserStream out;
                output_int_parser_L4_01_inst: IntParserStream out;
            }
            
            impl schema_0_parser_0_top_com of schema_0_parser_0_top {}
            
            
            streamlet TydiDemoTop_Interface {
                # Compressed data #
                input: byte_stream in;
            }
            
            impl TydiDemoTop of TydiDemoTop_Interface {
                instance decompressor(VhSnUnzipUnbufferedWrap);
                instance json_parser(schema_0_parser_0_top_com);
            
                input => decompressor.co;
                decompressor.de => json_parser.input;
            }
        
        "#);
        let src_pack1 = String::from(r#"
        package pack1;
        use pack0;

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
    std::fs::write("./code_structure_before_evaluation.json", &project.read().unwrap().get_pretty_json()).unwrap();

    project.read().unwrap().evaluate_target(format!("TydiDemoTop"), format!("snappy")).expect("fail to evaluate");

    let code_structure = project.read().unwrap().get_pretty_json();
    std::fs::write("./code_structure.json", &code_structure).unwrap();

    let json_output = generate_json_representation_from_tydi_project(project.clone(), format!("TydiDemoTop"), format!("snappy")).expect("fail to generate json");
    std::fs::write("./json_output.json", &json_output).unwrap();
    println!("{}", json_output);
}

#[test]
fn duplicator_and_voider() {
    let project = Project::new(format!("sample_project"));
    {
        let mut project_write = project.write().unwrap();

        let src_pack0 = String::from(r#"
            package pack;
            use std;

            bit8_stream = Stream(Bit(8));

            streamlet test_s {
                in_port: bit8_stream in;
                out_port: bit8_stream out;
            }

            impl test_i of test_s @External {

            }

            impl top of test_s {
                instance c0(test_i);
                instance c1(test_i);
                instance c2(test_i);
                in_port => c0.in_port;
                c0.out_port => c1.in_port;
                c0.out_port => c2.in_port;
                c1.out_port => out_port;
            }

            top2 = std.duplicator_i<Stream(Bit(8)), 3>;
        "#);
        let src_pack1 = String::from(r#"
        package std;
        streamlet void_s<type_in: type> {
            input_port: type_in in;
        }
          
        impl void_i<type_in: type> of void_s<type_in> @External {
            
        }

        streamlet duplicator_s<type_in: type, N: int> {
            input_port: type_in in;
            for i in range(N) {
                output_port: type_in out;
            }
        }
        
        impl duplicator_i<type_in: type, N: int> of duplicator_s<type_in, N> @External {
            
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
    std::fs::write("./code_structure_before_evaluation.json", &project.read().unwrap().get_pretty_json()).unwrap();

    project.read().unwrap().evaluate_target(format!("top"), format!("pack")).expect("fail to evaluate");

    let code_structure = project.read().unwrap().get_pretty_json();
    std::fs::write("./code_structure.json", &code_structure).unwrap();

    let json_output = generate_json_representation_from_tydi_project(project.clone(), format!("top"), format!("pack")).expect("fail to generate json");
    std::fs::write("./json_output.json", &json_output).unwrap();
    println!("{}", json_output);
}

#[test]
fn duplicator_and_voider2() {
    let project = Project::new(format!("sample_project"));
    {
        let mut project_write = project.write().unwrap();

        let src_pack0 = String::from(r#"
            package pack;
            use std;
            
            bit8_stream = Stream(Bit(8));
            
            streamlet test_s {
                in_port: bit8_stream in;
                out_port: bit8_stream out;
            }
            
            impl test_i of test_s @External {
            
            }
            
            impl top of test_s {
                for i in range(3) {
                    instance c(test_i);
                }
                
                in_port => c[0].in_port;
                c[0].out_port => c[1].in_port;
                c[0].out_port => c[2].in_port;
                c[1].out_port => out_port;
            }
            
            top2 = std.duplicator_i<Stream(Bit(8)), 3>;
        "#);
        let src_pack1 = String::from(r#"
        package std;
        streamlet void_s<type_in: type> {
            input_port: type_in in;
        }
          
        impl void_i<type_in: type> of void_s<type_in> @External {
            
        }

        streamlet duplicator_s<type_in: type, N: int> {
            input_port: type_in in;
            for i in range(N) {
                output_port: type_in out;
            }
        }
        
        impl duplicator_i<type_in: type, N: int> of duplicator_s<type_in, N> @External {
            
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
    std::fs::write("./code_structure_before_evaluation.json", &project.read().unwrap().get_pretty_json()).unwrap();

    project.read().unwrap().evaluate_target(format!("top"), format!("pack")).expect("fail to evaluate");

    let code_structure = project.read().unwrap().get_pretty_json();
    std::fs::write("./code_structure.json", &code_structure).unwrap();

    let json_output = generate_json_representation_from_tydi_project(project.clone(), format!("top"), format!("pack")).expect("fail to generate json");
    std::fs::write("./json_output.json", &json_output).unwrap();
    println!("{}", json_output);
}

#[test]
fn stream_template_in_streamlet() {
    let project = Project::new(format!("sample_project"));
    {
        let mut project_write = project.write().unwrap();

        let src_pack0 = String::from(r#"
            package pack;
            use std;
            
            bit8_stream = Stream(Bit(8));
            streamlet test_s <N : int> {
                in_port: bit8_stream in;
                out_port: bit8_stream out;
                bitn_stream = Stream(Bit(N));
            }

            streamlet test_ss {
                in_port: test_s<1>.bitn_stream in;
                out_port: test_s<1>.bitn_stream out;
            }
            
            impl test_ii of test_ss {
                
            }
            
        "#);
        let src_pack1 = String::from(r#"
        package std;
        streamlet void_s<type_in: type> {
            input_port: type_in in;
        }
          
        impl void_i<type_in: type> of void_s<type_in> @External {
            
        }

        streamlet duplicator_s<type_in: type, N: int> {
            input_port: type_in in;
            for i in range(N) {
                output_port: type_in out;
            }
        }
        
        impl duplicator_i<type_in: type, N: int> of duplicator_s<type_in, N> @External {
            
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
    std::fs::write("./code_structure_before_evaluation.json", &project.read().unwrap().get_pretty_json()).unwrap();

    project.read().unwrap().evaluate_target(format!("test_ii"), format!("pack")).expect("fail to evaluate");

    let code_structure = project.read().unwrap().get_pretty_json();
    std::fs::write("./code_structure.json", &code_structure).unwrap();

    let json_output = generate_json_representation_from_tydi_project(project.clone(), format!("test_ii"), format!("pack")).expect("fail to generate json");
    std::fs::write("./json_output.json", &json_output).unwrap();
    println!("{}", json_output);
}

#[test]
fn casper_json_example() {
    let project = Project::new(format!("sample_project"));
    {
        let mut project_write = project.write().unwrap();

        let src_pack0 = String::from(r#"
        package student_schema_parser;

        streamlet t <d: int> {
        JSONStream = Stream(
            Bit(8),
            throughput = 4.0,
            dimension = d,
            synchronicity = "Sync",
            complexity = 8
        );
        
        IntParserStream = Stream(
            Bit(64),
            throughput = 1.0,
            dimension = d,
            synchronicity = "Sync",
            complexity = 2
        );
        
        RecordParserStream = Stream(
            Bit(9),
            throughput = 4.0,
            dimension = d,
            synchronicity = "Sync",
            complexity = 8
        );
        
        MatcherMatchStream = Stream(
            Bit(1),
            throughput = 4.0,
            dimension = 1,
            synchronicity = "Sync",
            complexity = 8
        );
        
        MatcherStrStream = Stream(
            Bit(8),
            throughput = 4.0,
            dimension = 1,
            synchronicity = "Sync",
            complexity = 8
        );
        }
        
        streamlet string_parser_L1_00 {
            EPC = 4;
            NESTING_LEVEL = 1;
        
            input: t<NESTING_LEVEL+1>.JSONStream in;
            output: t<NESTING_LEVEL+1>.JSONStream out;
        }
        
        impl string_parser_L1_00_impl of string_parser_L1_00 @External { }
        
        streamlet student_number_matcher_L1_00 {
            BPC = 4;
        
            input: t<0>.MatcherStrStream in;
            output: t<0>.MatcherMatchStream out;
        }
        
        impl student_number_matcher_L1_00_impl of student_number_matcher_L1_00 @External { }
        
        streamlet key_parser_L1_00 {
            EPC = 4;
            OUTER_NESTING_LEVEL = 1;
        
            input: t<OUTER_NESTING_LEVEL+1>.RecordParserStream in;
            matcher_str: t<0>.MatcherStrStream out;
            matcher_match: t<0>.MatcherMatchStream in;
            output: t<OUTER_NESTING_LEVEL+1>.JSONStream out;
        }
        
        impl key_parser_L1_00_impl of key_parser_L1_00 @External { }
        
        streamlet string_parser_L1_01 {
            EPC = 4;
            NESTING_LEVEL = 1;
        
            input: t<NESTING_LEVEL+1>.JSONStream in;
            output: t<NESTING_LEVEL+1>.JSONStream out;
        }
        
        impl string_parser_L1_01_impl of string_parser_L1_01 @External { }
        
        streamlet name_matcher_L1_00 {
            BPC = 4;
        
            input: t<0>.MatcherStrStream in;
            output: t<0>.MatcherMatchStream out;
        }
        
        impl name_matcher_L1_00_impl of name_matcher_L1_00 @External { }
        
        streamlet key_parser_L1_01 {
            EPC = 4;
            OUTER_NESTING_LEVEL = 1;
        
            input: t<OUTER_NESTING_LEVEL+1>.RecordParserStream in;
            matcher_str: t<0>.MatcherStrStream out;
            matcher_match: t<0>.MatcherMatchStream in;
            output: t<OUTER_NESTING_LEVEL+1>.JSONStream out;
        }
        
        impl key_parser_L1_01_impl of key_parser_L1_01 @External { }
        
        streamlet string_parser_L1_02 {
            EPC = 4;
            NESTING_LEVEL = 1;
        
            input: t<NESTING_LEVEL+1>.JSONStream in;
            output: t<NESTING_LEVEL+1>.JSONStream out;
        }
        
        impl string_parser_L1_02_impl of string_parser_L1_02 @External { }
        
        streamlet birthdate_matcher_L1_00 {
            BPC = 4;
        
            input: t<0>.MatcherStrStream in;
            output: t<0>.MatcherMatchStream out;
        }
        
        impl birthdate_matcher_L1_00_impl of birthdate_matcher_L1_00 @External { }
        
        streamlet key_parser_L1_02 {
            EPC = 4;
            OUTER_NESTING_LEVEL = 1;
        
            input: t<OUTER_NESTING_LEVEL+1>.RecordParserStream in;
            matcher_str: t<0>.MatcherStrStream out;
            matcher_match: t<0>.MatcherMatchStream in;
            output: t<OUTER_NESTING_LEVEL+1>.JSONStream out;
        }
        
        impl key_parser_L1_02_impl of key_parser_L1_02 @External { }
        
        streamlet string_parser_L1_03 {
            EPC = 4;
            NESTING_LEVEL = 1;
        
            input: t<NESTING_LEVEL+1>.JSONStream in;
            output: t<NESTING_LEVEL+1>.JSONStream out;
        }
        
        impl string_parser_L1_03_impl of string_parser_L1_03 @External { }
        
        streamlet study_start_matcher_L1_00 {
            BPC = 4;
        
            input: t<0>.MatcherStrStream in;
            output: t<0>.MatcherMatchStream out;
        }
        
        impl study_start_matcher_L1_00_impl of study_start_matcher_L1_00 @External { }
        
        streamlet key_parser_L1_03 {
            EPC = 4;
            OUTER_NESTING_LEVEL = 1;
        
            input: t<OUTER_NESTING_LEVEL+1>.RecordParserStream in;
            matcher_str: t<0>.MatcherStrStream out;
            matcher_match: t<0>.MatcherMatchStream in;
            output: t<OUTER_NESTING_LEVEL+1>.JSONStream out;
        }
        
        impl key_parser_L1_03_impl of key_parser_L1_03 @External { }
        
        streamlet study_end_matcher_L1_00 {
            BPC = 4;
        
            input: t<0>.MatcherStrStream in;
            output: t<0>.MatcherMatchStream out;
        }
        
        impl study_end_matcher_L1_00_impl of study_end_matcher_L1_00 @External { }
        
        streamlet key_parser_L1_04 {
            EPC = 4;
            OUTER_NESTING_LEVEL = 1;
        
            input: t<OUTER_NESTING_LEVEL+1>.RecordParserStream in;
            matcher_str: t<0>.MatcherStrStream out;
            matcher_match: t<0>.MatcherMatchStream in;
            output: t<OUTER_NESTING_LEVEL+1>.JSONStream out;
        }
        
        impl key_parser_L1_04_impl of key_parser_L1_04 @External { }
        
        streamlet string_parser_L1_04 {
            EPC = 4;
            NESTING_LEVEL = 1;
        
            input: t<NESTING_LEVEL+1>.JSONStream in;
            output: t<NESTING_LEVEL+1>.JSONStream out;
        }
        
        impl string_parser_L1_04_impl of string_parser_L1_04 @External { }
        
        streamlet study_matcher_L1_00 {
            BPC = 4;
        
            input: t<0>.MatcherStrStream in;
            output: t<0>.MatcherMatchStream out;
        }
        
        impl study_matcher_L1_00_impl of study_matcher_L1_00 @External { }
        
        streamlet key_parser_L1_05 {
            EPC = 4;
            OUTER_NESTING_LEVEL = 1;
        
            input: t<OUTER_NESTING_LEVEL+1>.RecordParserStream in;
            matcher_str: t<0>.MatcherStrStream out;
            matcher_match: t<0>.MatcherMatchStream in;
            output: t<OUTER_NESTING_LEVEL+1>.JSONStream out;
        }
        
        impl key_parser_L1_05_impl of key_parser_L1_05 @External { }
        
        streamlet string_parser_L1_05 {
            EPC = 4;
            NESTING_LEVEL = 1;
        
            input: t<NESTING_LEVEL+1>.JSONStream in;
            output: t<NESTING_LEVEL+1>.JSONStream out;
        }
        
        impl string_parser_L1_05_impl of string_parser_L1_05 @External { }
        
        streamlet email_matcher_L1_00 {
            BPC = 4;
        
            input: t<0>.MatcherStrStream in;
            output: t<0>.MatcherMatchStream out;
        }
        
        impl email_matcher_L1_00_impl of email_matcher_L1_00 @External { }
        
        streamlet key_parser_L1_06 {
            EPC = 4;
            OUTER_NESTING_LEVEL = 1;
        
            input: t<OUTER_NESTING_LEVEL+1>.RecordParserStream in;
            matcher_str: t<0>.MatcherStrStream out;
            matcher_match: t<0>.MatcherMatchStream in;
            output: t<OUTER_NESTING_LEVEL+1>.JSONStream out;
        }
        
        impl key_parser_L1_06_impl of key_parser_L1_06 @External { }
        
        streamlet string_parser_L3_00 {
            EPC = 4;
            NESTING_LEVEL = 3;
        
            input: t<NESTING_LEVEL+1>.JSONStream in;
            output: t<NESTING_LEVEL+1>.JSONStream out;
        }
        
        impl string_parser_L3_00_impl of string_parser_L3_00 @External { }
        
        streamlet course_code_matcher_L3_00 {
            BPC = 4;
        
            input: t<0>.MatcherStrStream in;
            output: t<0>.MatcherMatchStream out;
        }
        
        impl course_code_matcher_L3_00_impl of course_code_matcher_L3_00 @External { }
        
        streamlet key_parser_L3_00 {
            EPC = 4;
            OUTER_NESTING_LEVEL = 3;
        
            input: t<OUTER_NESTING_LEVEL+1>.RecordParserStream in;
            matcher_str: t<0>.MatcherStrStream out;
            matcher_match: t<0>.MatcherMatchStream in;
            output: t<OUTER_NESTING_LEVEL+1>.JSONStream out;
        }
        
        impl key_parser_L3_00_impl of key_parser_L3_00 @External { }
        
        streamlet string_parser_L3_01 {
            EPC = 4;
            NESTING_LEVEL = 3;
        
            input: t<NESTING_LEVEL+1>.JSONStream in;
            output: t<NESTING_LEVEL+1>.JSONStream out;
        }
        
        impl string_parser_L3_01_impl of string_parser_L3_01 @External { }
        
        streamlet course_name_matcher_L3_00 {
            BPC = 4;
        
            input: t<0>.MatcherStrStream in;
            output: t<0>.MatcherMatchStream out;
        }
        
        impl course_name_matcher_L3_00_impl of course_name_matcher_L3_00 @External { }
        
        streamlet key_parser_L3_01 {
            EPC = 4;
            OUTER_NESTING_LEVEL = 3;
        
            input: t<OUTER_NESTING_LEVEL+1>.RecordParserStream in;
            matcher_str: t<0>.MatcherStrStream out;
            matcher_match: t<0>.MatcherMatchStream in;
            output: t<OUTER_NESTING_LEVEL+1>.JSONStream out;
        }
        
        impl key_parser_L3_01_impl of key_parser_L3_01 @External { }
        
        streamlet string_parser_L3_02 {
            EPC = 4;
            NESTING_LEVEL = 3;
        
            input: t<NESTING_LEVEL+1>.JSONStream in;
            output: t<NESTING_LEVEL+1>.JSONStream out;
        }
        
        impl string_parser_L3_02_impl of string_parser_L3_02 @External { }
        
        streamlet exam_date_matcher_L3_00 {
            BPC = 4;
        
            input: t<0>.MatcherStrStream in;
            output: t<0>.MatcherMatchStream out;
        }
        
        impl exam_date_matcher_L3_00_impl of exam_date_matcher_L3_00 @External { }
        
        streamlet key_parser_L3_02 {
            EPC = 4;
            OUTER_NESTING_LEVEL = 3;
        
            input: t<OUTER_NESTING_LEVEL+1>.RecordParserStream in;
            matcher_str: t<0>.MatcherStrStream out;
            matcher_match: t<0>.MatcherMatchStream in;
            output: t<OUTER_NESTING_LEVEL+1>.JSONStream out;
        }
        
        impl key_parser_L3_02_impl of key_parser_L3_02 @External { }
        
        streamlet int_parser_L4_00 {
            EPC = 4;
            NESTING_LEVEL = 3;
            BITWIDTH = 64;
        
            input: t<NESTING_LEVEL+1>.JSONStream in;
            output: t<NESTING_LEVEL>.IntParserStream out;
        }
        
        impl int_parser_L4_00_impl of int_parser_L4_00 @External { }
        
        streamlet grade_matcher_L3_00 {
            BPC = 4;
        
            input: t<0>.MatcherStrStream in;
            output: t<0>.MatcherMatchStream out;
        }
        
        impl grade_matcher_L3_00_impl of grade_matcher_L3_00 @External { }
        
        streamlet key_parser_L3_03 {
            EPC = 4;
            OUTER_NESTING_LEVEL = 3;
        
            input: t<OUTER_NESTING_LEVEL+1>.RecordParserStream in;
            matcher_str: t<0>.MatcherStrStream out;
            matcher_match: t<0>.MatcherMatchStream in;
            output: t<OUTER_NESTING_LEVEL+1>.JSONStream out;
        }
        
        impl key_parser_L3_03_impl of key_parser_L3_03 @External { }
        
        streamlet record_parser_L3_00 {
            EPC = 4;
            OUTER_NESTING_LEVEL = 3;
            INNER_NESTING_LEVEL = 0;
        
            input: t<OUTER_NESTING_LEVEL>.JSONStream in;
            output: t<OUTER_NESTING_LEVEL+1>.RecordParserStream out;
        }
        
        impl record_parser_L3_00_impl of record_parser_L3_00 @External { }
        
        streamlet array_parser_L2_00 {
            EPC = 4;
            OUTER_NESTING_LEVEL = 2;
            INNER_NESTING_LEVEL = 1;
        
            input: t<OUTER_NESTING_LEVEL>.JSONStream in;
            output: t<OUTER_NESTING_LEVEL+1>.JSONStream out;
        }
        
        impl array_parser_L2_00_impl of array_parser_L2_00 @External { }
        
        streamlet exams_matcher_L1_00 {
            BPC = 4;
        
            input: t<0>.MatcherStrStream in;
            output: t<0>.MatcherMatchStream out;
        }
        
        impl exams_matcher_L1_00_impl of exams_matcher_L1_00 @External { }
        
        streamlet key_parser_L1_07 {
            EPC = 4;
            OUTER_NESTING_LEVEL = 1;
        
            input: t<OUTER_NESTING_LEVEL+1>.RecordParserStream in;
            matcher_str: t<0>.MatcherStrStream out;
            matcher_match: t<0>.MatcherMatchStream in;
            output: t<OUTER_NESTING_LEVEL+1>.JSONStream out;
        }
        
        impl key_parser_L1_07_impl of key_parser_L1_07 @External { }
        
        streamlet record_parser_L1_00 {
            EPC = 4;
            OUTER_NESTING_LEVEL = 1;
            INNER_NESTING_LEVEL = 2;
        
            input: t<OUTER_NESTING_LEVEL>.JSONStream in;
            output: t<OUTER_NESTING_LEVEL+1>.RecordParserStream out;
        }
        
        impl record_parser_L1_00_impl of record_parser_L1_00 @External { }
        
        streamlet top {
            input: t<1>.JSONStream in;
            output_string_parser_L1_00_inst: t<2>.JSONStream out;
            output_string_parser_L1_01_inst: t<2>.JSONStream out;
            output_string_parser_L1_02_inst: t<2>.JSONStream out;
            output_string_parser_L1_03_inst: t<2>.JSONStream out;
            output_key_parser_L1_04_inst: t<2>.JSONStream out;
            output_string_parser_L1_04_inst: t<2>.JSONStream out;
            output_string_parser_L1_05_inst: t<2>.JSONStream out;
            output_string_parser_L3_00_inst: t<4>.JSONStream out;
            output_string_parser_L3_01_inst: t<4>.JSONStream out;
            output_string_parser_L3_02_inst: t<4>.JSONStream out;
            output_int_parser_L4_00_inst: t<3>.IntParserStream out;
        }
        
        impl top_impl of top {
            string_parser_L1_00_inst = string_parser_L1_00;
            student_number_matcher_L1_00_inst = student_number_matcher_L1_00;
            key_parser_L1_00_inst = key_parser_L1_00;
            string_parser_L1_01_inst = string_parser_L1_01;
            name_matcher_L1_00_inst = name_matcher_L1_00;
            key_parser_L1_01_inst = key_parser_L1_01;
            string_parser_L1_02_inst = string_parser_L1_02;
            birthdate_matcher_L1_00_inst = birthdate_matcher_L1_00;
            key_parser_L1_02_inst = key_parser_L1_02;
            string_parser_L1_03_inst = string_parser_L1_03;
            study_start_matcher_L1_00_inst = study_start_matcher_L1_00;
            key_parser_L1_03_inst = key_parser_L1_03;
            study_end_matcher_L1_00_inst = study_end_matcher_L1_00;
            key_parser_L1_04_inst = key_parser_L1_04;
            string_parser_L1_04_inst = string_parser_L1_04;
            study_matcher_L1_00_inst = study_matcher_L1_00;
            key_parser_L1_05_inst = key_parser_L1_05;
            string_parser_L1_05_inst = string_parser_L1_05;
            email_matcher_L1_00_inst = email_matcher_L1_00;
            key_parser_L1_06_inst = key_parser_L1_06;
            string_parser_L3_00_inst = string_parser_L3_00;
            course_code_matcher_L3_00_inst = course_code_matcher_L3_00;
            key_parser_L3_00_inst = key_parser_L3_00;
            string_parser_L3_01_inst = string_parser_L3_01;
            course_name_matcher_L3_00_inst = course_name_matcher_L3_00;
            key_parser_L3_01_inst = key_parser_L3_01;
            string_parser_L3_02_inst = string_parser_L3_02;
            exam_date_matcher_L3_00_inst = exam_date_matcher_L3_00;
            key_parser_L3_02_inst = key_parser_L3_02;
            int_parser_L4_00_inst = int_parser_L4_00;
            grade_matcher_L3_00_inst = grade_matcher_L3_00;
            key_parser_L3_03_inst = key_parser_L3_03;
            record_parser_L3_00_inst = record_parser_L3_00;
            array_parser_L2_00_inst = array_parser_L2_00;
            exams_matcher_L1_00_inst = exams_matcher_L1_00;
            key_parser_L1_07_inst = key_parser_L1_07;
            record_parser_L1_00_inst = record_parser_L1_00;
        
            input => record_parser_L1_00_inst.input;
            student_number_matcher_L1_00_inst.output => key_parser_L1_00_inst.matcher_match;
            key_parser_L1_00_inst.matcher_str => student_number_matcher_L1_00_inst.input;
            key_parser_L1_00_inst.output => string_parser_L1_00_inst.input;
            name_matcher_L1_00_inst.output => key_parser_L1_01_inst.matcher_match;
            key_parser_L1_01_inst.matcher_str => name_matcher_L1_00_inst.input;
            key_parser_L1_01_inst.output => string_parser_L1_01_inst.input;
            birthdate_matcher_L1_00_inst.output => key_parser_L1_02_inst.matcher_match;
            key_parser_L1_02_inst.matcher_str => birthdate_matcher_L1_00_inst.input;
            key_parser_L1_02_inst.output => string_parser_L1_02_inst.input;
            study_start_matcher_L1_00_inst.output => key_parser_L1_03_inst.matcher_match;
            key_parser_L1_03_inst.matcher_str => study_start_matcher_L1_00_inst.input;
            key_parser_L1_03_inst.output => string_parser_L1_03_inst.input;
            study_end_matcher_L1_00_inst.output => key_parser_L1_04_inst.matcher_match;
            key_parser_L1_04_inst.matcher_str => study_end_matcher_L1_00_inst.input;
            study_matcher_L1_00_inst.output => key_parser_L1_05_inst.matcher_match;
            key_parser_L1_05_inst.matcher_str => study_matcher_L1_00_inst.input;
            key_parser_L1_05_inst.output => string_parser_L1_04_inst.input;
            email_matcher_L1_00_inst.output => key_parser_L1_06_inst.matcher_match;
            key_parser_L1_06_inst.matcher_str => email_matcher_L1_00_inst.input;
            key_parser_L1_06_inst.output => string_parser_L1_05_inst.input;
            course_code_matcher_L3_00_inst.output => key_parser_L3_00_inst.matcher_match;
            key_parser_L3_00_inst.matcher_str => course_code_matcher_L3_00_inst.input;
            key_parser_L3_00_inst.output => string_parser_L3_00_inst.input;
            course_name_matcher_L3_00_inst.output => key_parser_L3_01_inst.matcher_match;
            key_parser_L3_01_inst.matcher_str => course_name_matcher_L3_00_inst.input;
            key_parser_L3_01_inst.output => string_parser_L3_01_inst.input;
            exam_date_matcher_L3_00_inst.output => key_parser_L3_02_inst.matcher_match;
            key_parser_L3_02_inst.matcher_str => exam_date_matcher_L3_00_inst.input;
            key_parser_L3_02_inst.output => string_parser_L3_02_inst.input;
            grade_matcher_L3_00_inst.output => key_parser_L3_03_inst.matcher_match;
            key_parser_L3_03_inst.matcher_str => grade_matcher_L3_00_inst.input;
            key_parser_L3_03_inst.output => int_parser_L4_00_inst.input;
            record_parser_L3_00_inst.output => key_parser_L3_00_inst.input;
            record_parser_L3_00_inst.output => key_parser_L3_01_inst.input;
            record_parser_L3_00_inst.output => key_parser_L3_02_inst.input;
            record_parser_L3_00_inst.output => key_parser_L3_03_inst.input;
            array_parser_L2_00_inst.output => record_parser_L3_00_inst.input;
            exams_matcher_L1_00_inst.output => key_parser_L1_07_inst.matcher_match;
            key_parser_L1_07_inst.matcher_str => exams_matcher_L1_00_inst.input;
            key_parser_L1_07_inst.output => array_parser_L2_00_inst.input;
            record_parser_L1_00_inst.output => key_parser_L1_00_inst.input;
            record_parser_L1_00_inst.output => key_parser_L1_01_inst.input;
            record_parser_L1_00_inst.output => key_parser_L1_02_inst.input;
            record_parser_L1_00_inst.output => key_parser_L1_03_inst.input;
            record_parser_L1_00_inst.output => key_parser_L1_04_inst.input;
            record_parser_L1_00_inst.output => key_parser_L1_05_inst.input;
            record_parser_L1_00_inst.output => key_parser_L1_06_inst.input;
            record_parser_L1_00_inst.output => key_parser_L1_07_inst.input;
            string_parser_L1_00_inst.output => output_string_parser_L1_00_inst;
            string_parser_L1_01_inst.output => output_string_parser_L1_01_inst;
            string_parser_L1_02_inst.output => output_string_parser_L1_02_inst;
            string_parser_L1_03_inst.output => output_string_parser_L1_03_inst;
            key_parser_L1_04_inst.output => output_key_parser_L1_04_inst;
            string_parser_L1_04_inst.output => output_string_parser_L1_04_inst;
            string_parser_L1_05_inst.output => output_string_parser_L1_05_inst;
            string_parser_L3_00_inst.output => output_string_parser_L3_00_inst;
            string_parser_L3_01_inst.output => output_string_parser_L3_01_inst;
            string_parser_L3_02_inst.output => output_string_parser_L3_02_inst;
            int_parser_L4_00_inst.output => output_int_parser_L4_00_inst;
        }
        
        "#);
        let src_pack1 = String::from(r#"
        package std;
        streamlet void_s<type_in: type> {
            input_port: type_in in;
        }
          
        impl void_i<type_in: type> of void_s<type_in> @External {
            
        }

        streamlet duplicator_s<type_in: type, N: int> {
            input_port: type_in in;
            for i in range(N) {
                output_port: type_in out;
            }
        }
        
        impl duplicator_i<type_in: type, N: int> of duplicator_s<type_in, N> @External {
            
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
    std::fs::write("./code_structure_before_evaluation.json", &project.read().unwrap().get_pretty_json()).unwrap();

    project.read().unwrap().evaluate_target(format!("top_impl"), format!("student_schema_parser")).expect("fail to evaluate");

    let code_structure = project.read().unwrap().get_pretty_json();
    std::fs::write("./code_structure.json", &code_structure).unwrap();

    let json_output = generate_json_representation_from_tydi_project(project.clone(), format!("top_impl"), format!("student_schema_parser")).expect("fail to generate json");
    std::fs::write("./json_output.json", &json_output).unwrap();
    println!("{}", json_output);
}