#[macro_export]
macro_rules! generate_set {
    ($id:ident, $t: ty, $id_set_func:ident) => {
        #[allow(dead_code)]
        pub fn $id_set_func(&mut self, target: $t) {
            self.$id = target;
        }
    };
}

#[macro_export]
macro_rules! generate_get {
    ($id:ident, $t: ty, $id_get_fun:ident) => {
        #[allow(dead_code)]
        pub fn $id_get_fun(& self) -> $t {
            return self.$id.clone();
        }
    };
}

#[macro_export]
macro_rules! generate_access {
    ($id:ident, $t: ty, $id_get_fun:ident, $id_set_func:ident) => {
        generate_set!($id, $t, $id_set_func);
        generate_get!($id, $t, $id_get_fun);
    };
}

#[macro_export]
macro_rules! generate_set_in_arc_rwlock {
    ($id:ident, $t: ty, $id_set_func:ident) => {
        pub fn $id_set_func(&mut self, target: $t) {
            self.$id = Arc::new(RwLock::new(target));
        }
    };
}