// TODO Copyright Header

#![crate_name="util"]
#![crate_type="rlib"]

#![no_std]
#![feature(phase, globs, macro_rules, asm, if_let, default_type_params, unsafe_destructor, tuple_indexing)]
#![doc(html_logo_url = "https://avatars.io/gravatar/d0ad9c6f37bb5aceac2d7ac95ba82607?size=large",
       html_favicon_url="https://avatars.io/gravatar/d0ad9c6f37bb5aceac2d7ac95ba82607?size=small")]

#[phase(link, plugin)] extern crate core;
#[phase(link, plugin)] extern crate base;
#[phase(link, plugin)] extern crate collections;
#[phase(link, plugin)] extern crate mm;
extern crate alloc;
extern crate libc;

pub fn init_stage1() {
    lru_cache::init_stage1();
    pinnable_cache::init_stage1();
}

pub fn init_stage2() {
    lru_cache::init_stage2();
    pinnable_cache::init_stage2();
}

pub fn init_stage3() {
    lru_cache::init_stage3();
    pinnable_cache::init_stage3();
    // TODO Mark we now have a pid.
}

pub mod format;
pub mod lru_cache;
pub mod pinnable_cache;

mod list_node;
mod key_ref {
    use core::mem::transmute;
    use core::ptr::*;
    use core::prelude::*;
    /// A struct used as the key for our map so we can get the key back out without trouble.
    struct KeyRef<K> { k: *const K, }
    impl<K> KeyRef<K> {
        pub fn new(v: &K) -> KeyRef<K> { unsafe { KeyRef { k: transmute(v), } } }
        pub fn as_ref<'a>(&'a self) -> &'a K { unsafe { self.k.as_ref().expect("LRU-cache key ref should never be null") } }
    }

    impl<K: PartialEq>  PartialEq  for KeyRef<K> {
        fn eq(&self, o: &KeyRef<K>)  -> bool { self.as_ref().eq( o.as_ref()) }
    }
    impl<K: PartialOrd> PartialOrd for KeyRef<K> {
        fn partial_cmp(&self, o: &KeyRef<K>) -> Option<Ordering> { self.as_ref().partial_cmp(o.as_ref()) }
    }
    impl<K: Eq>  Eq  for KeyRef<K> { }
    impl<K: Ord> Ord for KeyRef<K> {
        fn cmp(&self, o: &KeyRef<K>) -> Ordering { self.as_ref().cmp(o.as_ref()) }
    }
}

mod std {
    pub use core::clone;
    pub use core::cmp;
    pub use core::fmt;
    pub use core::num;
    pub use core::option;
    pub use collections::hash;
}