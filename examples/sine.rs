#![feature(phase)]

#![allow(uppercase_variables)]

#[phase(syntax,link)]
extern crate lua;
extern crate libc;

mod common;

fn main() {
    let mut L = lua::State::new();
    L.openlibs();
    L.register("sin", my_sin);
    L.register("cos", my_cos);
    L.register("tan", my_tan);
    common::repl(&mut L);
}

lua_extern! {
    unsafe fn my_sin(L: &mut lua::State) -> i32 {
        let input = L.checknumber(1);
        let output = input.sin();
        L.pushnumber(output);
        1
    }

    unsafe fn my_cos(L: &mut lua::State) -> i32 {
        let input = L.checknumber(1);
        let output = input.cos();
        L.pushnumber(output);
        1
    }
}

lua_extern_pub! {
    // this function is marked public
    unsafe fn my_tan(L: &mut lua::State) -> i32 {
        let input = L.checknumber(1);
        let output = input.tan();
        L.pushnumber(output);
        1
    }
}
