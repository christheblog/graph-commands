use std::collections::HashMap;

/// Identifies and represents an attribute id
#[derive(PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord, Debug)]
pub struct AttrId(pub u64);

// An attribute is a name-value pair
pub struct Attr<'a, T>(&'a str, T);

/// Repository for attributes
pub struct AttrRepo<'a, T> {
    attributes: HashMap<AttrId, Attr<'a, T>>,
}

impl<'a, T> AttrRepo<'a, T> {
    pub fn get_by_id(&self, id: &AttrId) -> Option<&Attr<T>> {
        self.attributes.get(id)
    }

    pub fn add_attr(&mut self, id: AttrId, name: &'a str, value: T) -> bool {
        self.attributes.insert(id, Attr(name, value));
        true
    }

    pub fn remove_attr(&mut self, id: &AttrId) -> bool {
        self.attributes.remove(id).is_some()
    }
}
