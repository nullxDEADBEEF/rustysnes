mod mapping;

pub struct Cartridge {
    title: String,
    data: Vec<u8>
}

impl Cartridge {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            data: Vec::new(),
        }
    }

    pub fn load(&mut self, path: &Path) -> Result<(), &str> {
        self.data = fs::read_path(path).unwrap();
        println!("{:?} loaded.", path);

        self.parse_cartridge_header();

        println!("TITLE: {}", self.title)
    }

    pub fn read_byte(addr: u16) -> u8 {
        self.data[addr as usize]
    }

    fn parse_cartridge_header(&mut self) {
        self.parse_title();
    }

    fn parse_title(&mut self) {
        for addr in 0xFFC0..=0xFFD5 {
            self.title.push(self.read_byte(addr) as char);
        }
    }
}