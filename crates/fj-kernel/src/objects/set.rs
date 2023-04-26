use std::collections::{btree_set, BTreeSet};

use super::{
    BehindHandle, Cycle, Face, GlobalEdge, HalfEdge, Object, Surface, Vertex,
};

/// A graph of objects and their relationships
pub struct ObjectSet {
    inner: BTreeSet<Object<BehindHandle>>,
}

impl IntoIterator for ObjectSet {
    type Item = Object<BehindHandle>;
    type IntoIter = btree_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

trait InsertIntoSet {
    fn insert_into_set(&self, objects: &mut ObjectSet);
}

impl InsertIntoSet for Cycle {
    fn insert_into_set(&self, objects: &mut ObjectSet) {
        for half_edge in self.half_edges() {
            objects.inner.insert(half_edge.clone().into());
            half_edge.insert_into_set(objects);
        }
    }
}

impl InsertIntoSet for Face {
    fn insert_into_set(&self, objects: &mut ObjectSet) {
        objects.inner.insert(self.surface().clone().into());
        self.surface().insert_into_set(objects);

        objects.inner.insert(self.exterior().clone().into());
        self.exterior().insert_into_set(objects);

        for interior in self.interiors() {
            objects.inner.insert(interior.clone().into());
        }
        for interior in self.interiors() {
            interior.insert_into_set(objects);
        }
    }
}

impl InsertIntoSet for GlobalEdge {
    fn insert_into_set(&self, _: &mut ObjectSet) {}
}

impl InsertIntoSet for HalfEdge {
    fn insert_into_set(&self, objects: &mut ObjectSet) {
        objects.inner.insert(self.start_vertex().clone().into());
        self.start_vertex().insert_into_set(objects);

        objects.inner.insert(self.global_form().clone().into());
        self.global_form().insert_into_set(objects);
    }
}

impl InsertIntoSet for Surface {
    fn insert_into_set(&self, _: &mut ObjectSet) {}
}

impl InsertIntoSet for Vertex {
    fn insert_into_set(&self, _: &mut ObjectSet) {}
}
