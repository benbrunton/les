use style::PaintItem;

pub struct PaintItems<'a> {
    items: Vec<PaintItem>
}

impl <'a> PaintItems<'a> {
    pub fn new(items: Vec<PaintItem>) {
        PaintItems {
            items
        }
    }

    pub fn get_longest_filename_size(&self) -> usize {
    }

    pub fn get_visible(&self) -> PaintItems<'a> {
        let visible_items = Vec::new();

        for i in items {
            if (!i.is_hidden()) {
                visible_items.push(i);
            }
        }

        return PaintItems::new(visible_items);
    }

    pub fn to_vec(&self) -> Vec<PaintItem> {
        return self.items;
    }
}
