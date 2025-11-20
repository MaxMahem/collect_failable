use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Identifiable {
    pub value: i32,
    pub id: i32,
}

impl PartialEq for Identifiable {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Identifiable {}

impl PartialOrd for Identifiable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Identifiable {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl Hash for Identifiable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
