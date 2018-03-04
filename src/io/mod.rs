use std::io::{Stdout,stdout};
use std::cmp::{min,max};
use terminal_size::{Width, terminal_size};
use style::paint;
use decorate::Decorate;
use fs::File;
use paintitems::PaintItems;

// Leading and trailing spaces plus icon
const DEFAULT_INDENTATION: usize = 3;

pub trait Print {
    fn print(&self, paths: Vec<File>);
}

pub struct TerminalPrinter<'a> {
    decorator: &'a Decorate<'a>,
    list_display: bool
}

impl<'a> TerminalPrinter<'a> {
    pub fn new(decorator: &'a Decorate, list_display: bool) -> TerminalPrinter<'a> {
        TerminalPrinter { decorator, list_display }
    }

    fn calculate_columns(&self, inodes: &PaintItems) -> usize {
        let inodes_count = inodes.len();
        let terminal_size = terminal_size();
        let max_filename_size = inodes.get_longest_filename_size();

        if let Some((Width(width), _)) = terminal_size {
            // Calculate how many columns would be rendered if we printed all columns back to back
            let max_term_cols_no_spacing = (width as f32 / max_filename_size as f32).floor() as usize;
            // Calculate the actual terminal width available after subtracting all the spacing
            // between columns
            let width_with_spaces = width as usize - (max_term_cols_no_spacing * DEFAULT_INDENTATION) - 1;

            max((width_with_spaces as f32 / max_filename_size as f32).floor() as usize, 1)
        } else {
            1
        }
    }

    fn print_rows(&self, items: &PaintItems, max_cols: usize) {
        let num_inodes = items.len();
        let mut rows_per_col = if num_inodes < max_cols {
            1
        } else {
            ((num_inodes / max_cols) as f32).floor() as usize
        };

        if num_inodes % max_cols != 0 {
            rows_per_col = rows_per_col + 1;
        }

        if rows_per_col == 1 {
            let mut indentation = 0;
            for item in items.iter() {
                self.indent(indentation);
                self.print_inode_str(paint(item));
                indentation = DEFAULT_INDENTATION;
            }
        } else {
            self.print_table(&items, max_cols, rows_per_col);
        }
    }

    fn print_table(&self, items: &PaintItems, max_cols: usize, max_rows: usize) {
        let max_column_width = self.get_column_width(&items);
        for row_num in 0..max_rows {
            let mut current_file_idx = row_num;
            let mut indentation = 0;
            let mut prev_item_length = 0;

            for _ in 0..max_cols {
                let item = items.get(current_file_idx);
                match item {
                    Some(ref it) => {
                        self.indent(indentation);
                        self.print_inode_str(paint(it));

                        prev_item_length = it.label.len();
                        indentation = self.get_item_indentation(prev_item_length, max_column_width);
                        current_file_idx = current_file_idx + max_rows;
                    },
                    None => break
                }
            }

            print!("\n");
        }
    }

    // TODO -- Detect if item is going to be rendered with an icon and adjust indentation
    // accordingly.
    fn get_item_indentation(&self, prev_item_length: usize, column_width: usize) -> usize {
        column_width - prev_item_length - 2
    }

    fn get_column_width(&self, items: &PaintItems) -> usize {
        items.get_longest_filename_size() + DEFAULT_INDENTATION
    }

    fn print_inode_str(&self, inode: String) {
        print!("{}", inode);
    }

    fn indent(&self, indentation: usize) {
        print!("{}", " ".repeat(indentation));
    }
}

impl<'a> Print for TerminalPrinter<'a> {
    fn print(&self, paths: Vec<File>) {
        let mut painted_entries = Vec::new();
        for path in paths {
            painted_entries.push(self.decorator.get_paint_rules(&path));
        }
        let paint_items = PaintItems::new(painted_entries);
        let mut visible_items = paint_items.get_visible();
        if self.list_display {
            self.print_rows(&visible_items, 1);
        } else {
            self.print_rows(&visible_items, self.calculate_columns(&visible_items));
        }
    }
}
