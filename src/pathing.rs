pub mod bundling {

    pub struct Bundle {
        pub root_path: String,
        pub bundle_path: String,
        pub public_path: String,
        pub is_ts: bool,
    }

    impl Bundle {
        // Creating a Bundle
        pub fn new(p_name: &str, ts: bool, src: bool) -> Bundle {
            let bp: String;
            if src {
                bp = format!("./{}/src/", &p_name)
            } else {
                bp = format!("./{}/", &p_name)
            }

            let bundle: Bundle = Bundle {
                root_path: format!("./{}/", &p_name),
                bundle_path: bp,
                public_path: format!("./{}/public/", &p_name),
                is_ts: ts,
            };
            bundle
        }
    }
}

pub mod utilities {
    use std::fs;
    use std::io::Write;
    use std::process;

    pub fn create_index(path: String, ts: &bool) {
        let mut f: fs::File;
        if *ts {
            f = fs::File::create(format!("{}/index.ts", path)).unwrap();
        } else {
            f = fs::File::create(format!("{}/index.js", path)).unwrap();
        }
        match f.write_all(b"export {};") {
            Err(reason) => {
                println!("{:?}", reason);
                process::exit(1)
            }
            Ok(_) => {}
        }
    }
}
