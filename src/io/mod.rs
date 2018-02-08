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

    fn calculate_columns(&self, inodes: &Vec<File>) -> usize {
        let inodes_count = inodes.len();
        let terminal_size = terminal_size();
        if let Some((Width(w), _)) = terminal_size {
            let max_term_cols = (w as f32 / self.get_max_filename_size(inodes) as f32).round() as usize; // 32 is the average filename
            if inodes_count < max_term_cols {
                return 1;
            } else {
                return max_term_cols;
            }
        } else {
            return 1;
        }
    }

    fn get_max_filename_size(&self, paths: &Vec<File>) -> usize {
        if paths.len() == 0 {
            return 32;
        }

        let mut max_filename_size = 0;
        for p in paths {
            if p.get_label().len() > max_filename_size {
                max_filename_size = p.get_label().len();
            }
        }

        return max_filename_size;
    }

    // get the list of all inodes and the max columns
    fn get_rows(&self, paths: &Vec<File>, max_cols: usize) {
        // divide total inodes by max cols and that's your `rows_per_col`
        let num_inodes = paths.len();
        let mut rows_per_col = ((num_inodes / max_cols) as f32).round() as usize;
        println!("{}", num_inodes);
        println!("{}", max_cols);
        println!("{}", rows_per_col);
        if num_inodes % max_cols != 0 {
            rows_per_col = rows_per_col + 1;
        }

        // iterate through the array of inodes in steps of `rows_per_col`
        // on each iteration print indent() + paint(path). when your current
        // path index > paths print "\n" and proceed to the next row
        for row in 0..rows_per_col {
            let mut current_file_idx = row;
            let mut pos = 0;
            loop {
                let path = paths.get(current_file_idx);
                match path {
                    Some(ref p) => {
                        let paint_rules = self.decorator.get_paint_rules(p);
                        if !paint_rules.is_hidden {
                            self.print_inode_str(paint(&paint_rules), pos);
                        }

                        pos = pos + 32; // max_name_length
                        current_file_idx = current_file_idx + rows_per_col;
                    },
                    None => break
                }
            }

            println!('\n');
        }
    }

    fn print_inode_str(&self, inode: String, position: usize) {
        let spacing = " ".repeat(position);
        println!("{}{}", spacing, inode);
    }

    //fn indent() {
    //        let _ = writeln(self.writer, "{}", self.painter.paint(path));

    //}
}

impl<'a> Print for TerminalPrinter<'a> {
    fn print(&self, paths: Vec<File>) {
        self.get_rows(&paths, self.calculate_columns(&paths));
    }
}
