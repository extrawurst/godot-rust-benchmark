mod bench;

use gdnative::prelude::*;

fn init(handle: InitHandle) {
    handle.add_class::<bench::MyBench>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
