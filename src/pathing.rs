pub struct Bundle {
    project_name: String,
    pub root_path: String,
    pub bundle_path: String,
    pub public_path: String,
    pub is_ts: bool,
    pub with_src: bool,
}

impl Bundle {
    // Creating a Bundle
    pub fn new(p_name: &str) -> Self {
        Self {
            project_name: String::from(p_name),
            root_path: format!("./{}/", &p_name),
            bundle_path: format!("./{}/", &p_name),
            public_path: format!("./{}/public/", &p_name),
            is_ts: true,
            with_src: true,
        }
    }

    pub fn set_src(self, src: bool) -> Self {
        let bp: String;
        if src {
            bp = format!("./{}/src/", &self.project_name)
        } else {
            bp = format!("./{}/", &self.project_name)
        }

        Self {
            project_name: self.project_name,
            root_path: self.root_path,
            bundle_path: bp,
            public_path: self.public_path,
            is_ts: self.is_ts,
            with_src: src,
        }
    }

    pub fn set_ts(self, ts: bool) -> Self {
        Self {
            project_name: self.project_name,
            root_path: self.root_path,
            bundle_path: self.bundle_path,
            public_path: self.public_path,
            is_ts: ts,
            with_src: self.with_src,
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
