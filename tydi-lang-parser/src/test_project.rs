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

        let src_pack0_ptr = Arc::new(RwLock::new(src_pack0.clone()));
        let src_pack1_ptr = Arc::new(RwLock::new(src_pack1.clone()));
        let status = project_write.add_package(format!("./pack0.td"), src_pack0);
        if status.is_err() {
            panic!("{}", status.err().unwrap().print(Some(src_pack0_ptr)));
        }
        let status = project_write.add_package(format!("./pack1.td"), src_pack1);
        if status.is_err() {
            panic!("{}", status.err().unwrap().print(Some(src_pack1_ptr)));
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

        let src_pack0_ptr = Arc::new(RwLock::new(src_pack0.clone()));
        let src_pack1_ptr = Arc::new(RwLock::new(src_pack1.clone()));
        let status = project_write.add_package(format!("./pack0.td"), src_pack0);
        if status.is_err() {
            panic!("{}", status.err().unwrap().print(Some(src_pack0_ptr)));
        }
        let status = project_write.add_package(format!("./pack1.td"), src_pack1);
        if status.is_err() {
            panic!("{}", status.err().unwrap().print(Some(src_pack1_ptr)));
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

        let src_pack0_ptr = Arc::new(RwLock::new(src_pack0.clone()));
        let src_pack1_ptr = Arc::new(RwLock::new(src_pack1.clone()));
        let status = project_write.add_package(format!("./pack0.td"), src_pack0);
        if status.is_err() {
            panic!("{}", status.err().unwrap().print(Some(src_pack0_ptr)));
        }
        let status = project_write.add_package(format!("./pack1.td"), src_pack1);
        if status.is_err() {
            panic!("{}", status.err().unwrap().print(Some(src_pack1_ptr)));
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

        let src_pack0_ptr = Arc::new(RwLock::new(src_pack0.clone()));
        let src_pack1_ptr = Arc::new(RwLock::new(src_pack1.clone()));
        let status = project_write.add_package(format!("./pack0.td"), src_pack0);
        if status.is_err() {
            panic!("{}", status.err().unwrap().print(Some(src_pack0_ptr)));
        }
        let status = project_write.add_package(format!("./pack1.td"), src_pack1);
        if status.is_err() {
            panic!("{}", status.err().unwrap().print(Some(src_pack1_ptr)));
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

        let src_pack0_ptr = Arc::new(RwLock::new(src_pack0.clone()));
        let src_pack1_ptr = Arc::new(RwLock::new(src_pack1.clone()));
        let status = project_write.add_package(format!("./pack0.td"), src_pack0);
        if status.is_err() {
            panic!("{}", status.err().unwrap().print(Some(src_pack0_ptr)));
        }
        let status = project_write.add_package(format!("./pack1.td"), src_pack1);
        if status.is_err() {
            panic!("{}", status.err().unwrap().print(Some(src_pack1_ptr)));
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
        ");

        let src_pack0_ptr = Arc::new(RwLock::new(src_pack0.clone()));
        let src_pack1_ptr = Arc::new(RwLock::new(src_pack1.clone()));
        let status = project_write.add_package(format!("./pack0.td"), src_pack0);
        if status.is_err() {
            panic!("{}", status.err().unwrap().print(Some(src_pack0_ptr)));
        }
        let status = project_write.add_package(format!("./pack1.td"), src_pack1);
        if status.is_err() {
            panic!("{}", status.err().unwrap().print(Some(src_pack1_ptr)));
        }
    }
    let evaluator = project.read().unwrap().evaluate_target(format!("stream_rgb"), format!("pack1")).expect("fail to evaluate");

    let json_output = project.read().unwrap().get_pretty_json();

    println!("{json_output}");
    std::fs::write("./output.json", &json_output).unwrap();

    println!("{}", evaluator.read().unwrap().print_evaluation_record());
}