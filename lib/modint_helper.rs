mod modint_helper {
    use ac_library::modint::*;

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct ModIntId<const ID: u32>;

    impl<const ID: u32> Id for ModIntId<ID> {
        fn companion_barrett() -> &'static Barrett {
            static BARRETT: Barrett = Barrett::new(2);
            &BARRETT
        }
    }

    pub type DynModInt<const ID: u32> = DynamicModInt<ModIntId<ID>>;

    pub const fn gen_id(s: &str) -> u32 {
        const FNV_OFFSET_BASIS_32: u32 = 2166136261;
        const FNV_PRIME_32: u32 = 16777619;
        let bytes = s.as_bytes();
        let mut hash: u32 = FNV_OFFSET_BASIS_32;
        let mut i: usize = 0;
        while i < s.len() {
            hash = (FNV_PRIME_32 * hash) ^ (bytes[i] as u32);
            i += 1;
        }
        hash
    }

    #[macro_export]
    macro_rules! new_mod {
        () => {};
        ($x: ident $(, $xs: ident)*) => {
            new_mod!($x; stringify!($x));
            new_mod!($($xs),*);
        };
        ($x: ident; $id: expr) => {
            pub type $x = DynamicModInt<modint_helper::ModIntId<{modint_helper::gen_id($id)}>>;
        };
    }
}