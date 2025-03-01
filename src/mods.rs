pub struct Mod {
    pub enabled: bool,
    pub name: String,
    pub size: usize
}

impl Mod {
    pub fn new(name: String, size: usize) -> Self {
        Mod {enabled: false, name, size}
    }
}