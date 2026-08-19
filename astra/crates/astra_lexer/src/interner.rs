use lasso::{self, Rodeo, Spur};

pub struct Interner {
    //  think of `Rodeo` as
    //         Rodeo
    //        /     \
    //      "hello" "world"
    //     (Spur(0)) (Spur(1))
    rodeo: Rodeo,
}
impl Interner {
    pub fn new() -> Self {
        Self {
            rodeo: Rodeo::new(),
        }
    }
    pub fn intern(&mut self, string: &str) -> Spur {
        self.rodeo.get_or_intern(string)
    }
    pub fn resolve(&self, symbol: &Spur) -> &str {
        self.rodeo.resolve(symbol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_string_has_same_symbol() {
        let mut interner = Interner::new();
        let a = interner.intern("Hello");
        let b = interner.intern("Hello");
        assert_eq!(a, b);
    }
    #[test]
    fn diff_string_have_different_symbols() {
        let mut interner = Interner::new();

        let a = interner.intern("Hello");
        let b = interner.intern("World");
        assert_ne!(a, b);
    }

    #[test]
    fn symbol_can_be_resolved() {
        let mut interner = Interner::new();
        let symbol = interner.intern("Hello");
        assert_eq!(interner.resolve(&symbol), "Hello");
    }
}
