use style::PaintItem;
use std::slice::Iter;
use std::cmp::max;

pub struct PaintItems {
    items: Vec<PaintItem>,
}

impl PaintItems {
    pub fn new(items: Vec<PaintItem>) -> PaintItems {
        PaintItems {
            items
        }
    }

    pub fn get_longest_filename_size(&self) -> usize {
        self.items.iter().fold(0, |mx, ref i| max(i.label.len(), mx))
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get(&self, index: usize) -> Option<&PaintItem> {
        if index >= self.items.len() || index < 0 {
            None
        } else {
            Some(&self.items[index])
        }
    }

    pub fn get_visible(&self) -> PaintItems {
        let mut visible_items: Vec<PaintItem> = Vec::new();

        for i in &self.items {
            if !i.is_hidden {
                visible_items.push(i.clone());
            }
        }

        return PaintItems::new(visible_items);
    }

    pub fn iter(&self) -> Iter<PaintItem> {
        return self.items.iter();
    }
}

#[cfg(test)]
mod tests {
    use paintitems::*;
    use style::PaintItem;

    #[test]
    fn get_longest_filename_size_returns_correct_size() {
        let mut items: Vec<PaintItem> = Vec::new();
        items.push(PaintItem{
            label: "LICENSE".to_string(),
            is_bold: false,
            is_underline: false,
            is_dimmed: false,
            is_hidden: false,
            colour: None,
            icon: None
        });
        items.push(PaintItem{
            label: "file211.rs".to_string(),
            is_bold: false,
            is_underline: false,
            is_dimmed: false,
            is_hidden: false,
            colour: None,
            icon: None
        });

        let items = PaintItems::new(items);
        assert_eq!(items.get_longest_filename_size(), 10);
    }

    #[test]
    fn get_visible_returns_visible_items() {
        let mut items: Vec<PaintItem> = Vec::new();
        items.push(PaintItem{
            label: "LICENSE".to_string(),
            is_bold: false,
            is_underline: false,
            is_dimmed: false,
            is_hidden: false,
            colour: None,
            icon: None
        });
        items.push(PaintItem{
            label: ".hiddenfile211.rs".to_string(),
            is_bold: false,
            is_underline: false,
            is_dimmed: false,
            is_hidden: true,
            colour: None,
            icon: None
        });

        let items = PaintItems::new(items);
        match items.get(0) {
            Some(ref i) => {
                assert_eq!(i.label, "LICENSE");
            },
            None => {
                panic!("Failed to retrieve item from item list");
            }
        };
        assert_eq!(items.get_visible().len(), 1);
    }
}
