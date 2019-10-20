extern crate eosio_cli;

// use eosio_cli::Cmd;
// use futures::future::{self, Future};

use std::process::Command;
use syn::visit::{self, Visit};
use syn::{File, ItemImpl};

struct FnVisitor<'ast> {
    functions: Vec<&'ast ItemImpl>,
}

impl<'ast> Visit<'ast> for FnVisitor<'ast> {
    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        self.functions.push(node);
        visit::visit_item_impl(self, node);
    }
}

fn main() {
    // let app = eosio_cli::app().get_matches();

    let mut cmd = Command::new("cargo");
    let out = cmd
        .args(&[
            "+nightly",
            "rustc",
            "-p",
            "hello",
            "--profile=check",
            "--",
            "-Zunstable-options",
            "--pretty=expanded",
        ])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let syntax_tree: File = syn::parse_str(&stdout).unwrap();
    let mut visitor = FnVisitor {
        functions: Vec::new(),
    };
    visitor.visit_file(&syntax_tree);
    for f in visitor.functions {
        println!("Thing: {:#?}", f.trait_);
    }
    println!("Done");
}
