use std::io::{Stdout,stdout};
use std::cmp::min;
use terminal_size::{Width, terminal_size};
use style::paint;
use decorate::Decorate;
use fs::File;
use paintitems::PaintItems;

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

    fn calculate_columns(&self, inodes: &PaintItems) -> usize {
        let inodes_count = inodes.len();
        let terminal_size = terminal_size();
        if let Some((Width(w), _)) = terminal_size {
            let max_term_cols = (w as f32 / inodes.get_longest_filename_size() as f32).round() as usize;
            println!("max_term_cols: {}", max_term_cols);
            max_term_cols
            //if inodes_count < max_term_cols {
            //    return 1;
            //} else {
            //    return max_term_cols;
            //}
        } else {
            1
        }
    }

    // get the list of all inodes and the max columns
    fn print_rows(&self, items: &PaintItems, max_cols: usize) {
        // divide total inodes by max cols and that's your `rows_per_col`
        let num_inodes = items.len();
        let mut rows_per_col = if num_inodes < max_cols {
            1
        } else {
            ((num_inodes / max_cols) as f32).round() as usize
        };
        if num_inodes % max_cols != 0 {
            //rows_per_col = rows_per_col + 1;
        }
        println!("{}", num_inodes);
        println!("{}", max_cols);
        println!("{}", rows_per_col);

        // iterate through the array of inodes in steps of `rows_per_col`
        // on each iteration print indent() + paint(path). when your current
        // path index > items print "\n" and proceed to the next row
        //let max_name_length = items.get_longest_filename_size();

        if rows_per_col == 1 {
            let mut indentation = 0;
            for item in items.iter() {
                self.print_inode_str(paint(item), indentation);
                indentation = 3;
            }
        } else {
            // TODO -- Put in separate function
            let max_column_width = self.get_column_width(&items);
            for row_num in 0..rows_per_col {
                let mut current_file_idx = row_num;
                let mut indentation = 0;
                let mut prev_item_length = 0;
                loop {
                    let item = items.get(current_file_idx);
                    match item {
                        Some(ref it) => {
                            self.print_inode_str(paint(it), indentation);

                            prev_item_length = it.label.len();
                            indentation = self.get_item_indentation(prev_item_length, max_column_width);
                            //println!("Position: {}", indentation);
                            current_file_idx = current_file_idx + rows_per_col;
                            //println!("Current File Index: {}", current_file_idx);
                        },
                        None => break
                    }
                }

                print!("\n");
            }
        }

        println!('\n');
    }

    fn get_item_indentation(&self, prev_item_length: usize, column_width: usize) -> usize {
        column_width - prev_item_length - 2
    }

    fn get_column_width(&self, items: &PaintItems) -> usize {
        // need to retrieve the highest filename size. That +3 spaces will become your column width
        // then you determine how much to indent based on a subtraction of the previous item from 
        // the max column width.
        items.get_longest_filename_size() + 3
    }

    fn print_inode_str(&self, inode: String, indentation: usize) {
        let spacing = " ".repeat(indentation);
        print!("{}{}", spacing, inode);
    }

    //fn indent() {
    //        let _ = writeln(self.writer, "{}", self.painter.paint(path));

    //}
}

impl<'a> Print for TerminalPrinter<'a> {
    fn print(&self, paths: Vec<File>) {
        let mut painted_entries = Vec::new();
        for path in paths {
            painted_entries.push(self.decorator.get_paint_rules(&path));
        }
        let paint_items = PaintItems::new(painted_entries);
        let mut visible_items = paint_items.get_visible();
        self.print_rows(&visible_items, self.calculate_columns(&visible_items));
    }
}
