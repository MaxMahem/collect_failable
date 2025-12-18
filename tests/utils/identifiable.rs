use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// Helper type to test if values are overwritten.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Identifiable {
    /// Value of the item, used for eq.
    pub value: i32,
    /// Id of the item. *Not* used for eq.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifiable_eq() {
        let a = Identifiable { value: 1, id: 1 };
        let b = Identifiable { value: 1, id: 2 };
        assert_eq!(a, b);
    }

    #[test]
    fn identifiable_cmp() {
        let a = Identifiable { value: 1, id: 1 };
        let b = Identifiable { value: 2, id: 2 };
        assert!(a < b);
    }

    #[test]
    fn identifiable_hash() {
        let a = Identifiable { value: 1, id: 1 };
        let b = Identifiable { value: 1, id: 2 };

        let mut h1 = std::collections::hash_map::DefaultHasher::new();
        let mut h2 = std::collections::hash_map::DefaultHasher::new();
        a.hash(&mut h1);
        b.hash(&mut h2);

        assert_eq!(a, b);
    }

    #[test]
    fn identifiable_partial_cmp() {
        let a = Identifiable { value: 1, id: 1 };
        let b = Identifiable { value: 1, id: 1 };

        assert_eq!(PartialOrd::partial_cmp(&a, &b), Some(Ordering::Equal));
    }
}
