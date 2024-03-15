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

