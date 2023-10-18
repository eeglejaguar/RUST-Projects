

    #![allow(unused)]
    fn main()->Result<(),Box<dyn >> {
        let result = std::fs::read_to_string("test.txt").unwrap();
        let content= match result {
            Ok(content) => { content }
            Err(error) => { panic!("Oh noes: {}", error); }
        };
        println!("{}",content);
    }

