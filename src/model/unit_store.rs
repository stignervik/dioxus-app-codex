use super::Unit;

#[derive(Debug, Clone)]
pub struct UnitStore {
    units: Vec<Unit>,
    next_id: u64,
}

impl UnitStore {
    pub fn new() -> Self {
        let mut store = Self {
            units: Vec::new(),
            next_id: 0,
        };

        // Always create the Root unit with id = 0
        store.units.push(Unit::new(0, "Root"));
        store.next_id = 1;

        store
    }

    pub fn add_unit(&mut self, name: &str) -> u64 {
        let id = self.next_id;
        self.units.push(Unit::new(id, name));
        self.next_id += 1;
        id
    }

    pub fn get_unit(&self, id: u64) -> Option<&Unit> {
        self.units.iter().find(|unit| unit.id() == id)
    }

    pub fn get_unit_mut(&mut self, id: u64) -> Option<&mut Unit> {
        self.units.iter_mut().find(|unit| unit.id() == id)
    }

    pub fn get_all_units(&self) -> &Vec<Unit> {
        &self.units
    }

    pub fn remove_unit(&mut self, id: u64) -> bool {
        if id == 0 {
            // Don't allow removing the Root unit
            return false;
        }

        if let Some(pos) = self.units.iter().position(|unit| unit.id() == id) {
            self.units.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn root_unit(&self) -> &Unit {
        &self.units[0] // Root is always at index 0
    }

    pub fn count(&self) -> usize {
        // Total units minus 1 (the Root unit)
        self.units.len() - 1
    }

    pub fn is_empty(&self) -> bool {
        self.units.is_empty()
    }

    pub fn clear(&mut self) {
        self.units.clear();
        self.units.push(Unit::new(0, "Root"));
        self.next_id = 1;
    }
}

impl Default for UnitStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_store_has_root_unit() {
        let store = UnitStore::new();

        assert_eq!(store.units.len(), 1);
        assert_eq!(store.next_id, 1);

        let root = store.root_unit();
        assert_eq!(root.id(), 0);
        assert_eq!(root.name(), "Root");
    }

    #[test]
    fn test_default_creates_root_unit() {
        let store = UnitStore::default();

        assert_eq!(store.units.len(), 1);
        let root = store.root_unit();
        assert_eq!(root.id(), 0);
        assert_eq!(root.name(), "Root");
    }

    #[test]
    fn test_add_unit_increments_id() {
        let mut store = UnitStore::new();

        let id1 = store.add_unit("First Unit");
        let id2 = store.add_unit("Second Unit");

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(store.units.len(), 3); // Root + 2 added units
        assert_eq!(store.next_id, 3);
    }

    #[test]
    fn test_get_unit_returns_correct_unit() {
        let mut store = UnitStore::new();
        store.add_unit("Test Unit");

        let unit = store.get_unit(1);
        assert!(unit.is_some());
        assert_eq!(unit.unwrap().name(), "Test Unit");

        let nonexistent = store.get_unit(999);
        assert!(nonexistent.is_none());
    }

    #[test]
    fn test_get_unit_mut_allows_modification() {
        let mut store = UnitStore::new();
        store.add_unit("Original Name");

        {
            let unit = store.get_unit_mut(1);
            assert!(unit.is_some());
            unit.unwrap().set_name("Modified Name");
        }

        let unit = store.get_unit(1);
        assert_eq!(unit.unwrap().name(), "Modified Name");
    }

    #[test]
    fn test_get_all_units_returns_all() {
        let mut store = UnitStore::new();
        store.add_unit("Unit 1");
        store.add_unit("Unit 2");

        let units = store.get_all_units();
        assert_eq!(units.len(), 3); // Root + 2 added
        assert_eq!(units[0].name(), "Root");
        assert_eq!(units[1].name(), "Unit 1");
        assert_eq!(units[2].name(), "Unit 2");
    }

    #[test]
    fn test_remove_unit_success() {
        let mut store = UnitStore::new();
        let id = store.add_unit("To Remove");

        assert_eq!(store.units.len(), 2);

        let removed = store.remove_unit(id);
        assert!(removed);
        assert_eq!(store.units.len(), 1);
        assert!(store.get_unit(id).is_none());
    }

    #[test]
    fn test_remove_nonexistent_unit() {
        let mut store = UnitStore::new();

        let removed = store.remove_unit(999);
        assert!(!removed);
        assert_eq!(store.units.len(), 1); // Still has Root
    }

    #[test]
    fn test_cannot_remove_root_unit() {
        let mut store = UnitStore::new();

        let removed = store.remove_unit(0); // Try to remove Root
        assert!(!removed);
        assert_eq!(store.units.len(), 1);
        assert!(store.get_unit(0).is_some()); // Root still exists
    }

    #[test]
    fn test_root_unit_always_accessible() {
        let mut store = UnitStore::new();
        store.add_unit("Some Unit");

        let root = store.root_unit();
        assert_eq!(root.id(), 0);
        assert_eq!(root.name(), "Root");

        // Root should still be accessible after adding more units
        store.add_unit("Another Unit");
        let root_again = store.root_unit();
        assert_eq!(root_again.id(), 0);
    }

    #[test]
    fn test_id_sequence_after_removals() {
        let mut store = UnitStore::new();

        let id1 = store.add_unit("Unit 1");
        let id2 = store.add_unit("Unit 2");
        let id3 = store.add_unit("Unit 3");

        // Remove middle unit
        store.remove_unit(id2);

        // Next ID should still increment from last used
        let id4 = store.add_unit("Unit 4");
        assert_eq!(id4, 4); // Should be 4, not 2

        // Verify we have Root, Unit 1, Unit 3, Unit 4
        assert_eq!(store.units.len(), 4);
        assert!(store.get_unit(id1).is_some());
        assert!(store.get_unit(id2).is_none());
        assert!(store.get_unit(id3).is_some());
        assert!(store.get_unit(id4).is_some());
    }
}
