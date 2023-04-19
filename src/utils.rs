pub struct Symbol {
    pub name: String,
    pub address: u64,
    pub size: u64,
}

pub struct SymbolMap {
    pub symbols: Vec<Symbol>,
}

impl SymbolMap {
    pub fn new() -> SymbolMap {
        SymbolMap { symbols: Vec::new() }
    }

    pub fn add_symbol(&mut self, name: String, address: u64, size: u64) {
        self.symbols.push(Symbol {
            name: name,
            address: address,
            size: size,
        });
    }

    pub fn find_symbol(&self, address: u64) -> Option<&Symbol> {
        for symbol in &self.symbols {
            if symbol.address <= address && address < symbol.address + symbol.size {
                return Some(symbol);
            }
        }
        None
    }
}