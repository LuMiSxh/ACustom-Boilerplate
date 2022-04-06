use std::fs;

pub struct DepManager {
    pub dev: Vec<String>,
    pub standard: Vec<String>,
}

impl DepManager {
    pub fn new(initial_dev: Vec<String>, initial_standard: Vec<String>) -> Self {
        Self {
            dev: initial_dev,
            standard: initial_standard,
        }
    }

    pub fn add_dev(mut self, dev: String) {
        self.dev.push(dev);
    }

    pub fn add_standard(mut self, standard: String) {
        self.standard.push(standard);
    }

    pub async fn build(self, client: &reqwest::Client, path: &str, url: &str) {
        let mut f: fs::File = fs::File::create(format!("{}package.json", &path)).unwrap();

        let res: String = client.get(url).send().await.unwrap().text().await.unwrap();
    }
}
