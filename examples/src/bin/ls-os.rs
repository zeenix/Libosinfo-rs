// Copyright (C) 2017 Zeeshan Ali <zeeshanak@gnome.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate libosinfo;
extern crate glib;

use glib::prelude::*;
use libosinfo::prelude::*;

use std::env;

fn main() {
    let loader = libosinfo::Loader::new();
    loader.process_default_path().unwrap();
    let db = loader.get_db().unwrap();

    let os_list = db.get_os_list().unwrap();
    let args: Vec<_> = env::args().collect();
    for e in os_list.get_elements() {
        let os: libosinfo::Product = e.downcast().unwrap();
        let name = os.get_name().unwrap();

        if args.len() > 1 && !name.contains(&args[1]) {
            continue;
        }

        println!("{}", name);
        if let Some(v) = os.get_vendor() {
            println!("\tvendor: {}", v);
        }
        if let Some(c) = os.get_codename() {
            println!("\tcodename: {}", c);
        }
        if let Some(r) = os.get_release_date_string() {
            println!("\tReleased: {}", r);
        }
        println!("");
    }
}
