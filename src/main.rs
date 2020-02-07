use twiggy_parser;
use twiggy_opt;
use twiggy_analyze;

use std::fs;
use std::io::Read;
use std::io;


fn main() {
    let mut file = fs::File::open("../hello-wasm/pkg/hello_wasm_bg.wasm").unwrap();
    let mut data = vec![];
    file.read_to_end(&mut data).unwrap();

    let mut items = twiggy_parser::parse(&data).unwrap();

    let options = twiggy_opt::Top::default();
    let top = twiggy_analyze::top(&mut items, &options).unwrap();

    let mut stdout = io::stdout();
    top.emit_text(&items, &mut stdout).unwrap();
}
