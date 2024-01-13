use std::hash::Hash;

#[derive(Debug)]
pub struct SymTable<K, T> {
    pub scope: usize,
    pub symbols: Vec<std::collections::HashMap<K, T>>,
}

impl<K, T> SymTable<K, T>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        SymTable {
            scope: 0,
            symbols: vec![std::collections::HashMap::new()],
        }
    }
    pub fn add(&mut self, key: K, item: T) {
        match self.symbols.get_mut(self.scope) {
            Some(map) => map.insert(key, item),
            None => panic!(),
        };
    }

    pub fn get(&self, key: K) -> Option<&T> {
        return match self.symbols.get(self.scope) {
            Some(map) => map.get(&key),
            None => panic!(),
        };
    }

    pub fn new_scope(&mut self) {
        self.scope += 1;
        self.symbols.push(std::collections::HashMap::new());
    }

    pub fn leave_scope(&mut self) {
        self.scope -= 1;
        self.symbols.pop();
    }
}
