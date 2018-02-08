use std::io::{Stdout,stdout};
use std::cmp::min;
use terminal_size::{Width, terminal_size};
use style::paint;
use decorate::Decorate;
use fs::File;

pub trait Print {
    fn print(&self, paths: Vec<File>);
}

pub struct TerminalPrinter<'a> {
    decorator: &'a Decorate<'a>
}

impl<'a> TerminalPrinter<'a> {
    pub fn new(decorator: &'a Decorate) -> TerminalPrinter<'a> {
        TerminalPrinter { decorator }
    }
}

impl<'a> Print for TerminalPrinter<'a> {
    fn print(&self, paths: Vec<File>) {
        for path in paths {

            let paint_rules = self.decorator.get_paint_rules(&path);
            if !paint_rules.is_hidden {
                println!("{}", paint(&paint_rules));
            }
        }
    }
}
