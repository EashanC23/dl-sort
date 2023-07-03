extern crate dirs;

pub struct ConfigData {
    folders: Option<Vec<String>>,
}

impl ConfigData {
    pub fn config_read() {
        let config_dir = dirs::home_dir().unwrap().push(".config/dl_sort");
        println!("{:?}", config_dir);
    }
    // add code here
}
