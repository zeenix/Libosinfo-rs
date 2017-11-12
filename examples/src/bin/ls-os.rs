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

fn main() {
    let loader = libosinfo::Loader::new();
    loader.process_default_path().unwrap();
    let db = loader.get_db().unwrap();

    let os_list = db.get_os_list().unwrap();
    for i in 0..os_list.get_length() {
        let os: libosinfo::Product = os_list.get_nth(i).unwrap().downcast().unwrap();
        println!("{}", os.get_name().unwrap());
    }
}
