use fj_math::Point;

use crate::{
    geometry::{CurveBoundary, HalfEdgeGeometry, SurfacePath},
    objects::HalfEdge,
    operations::insert::Insert,
    storage::Handle,
    Core,
};

/// Update the geometry of a [`HalfEdge`]
pub trait UpdateHalfEdgeGeometry {
    /// Update the path of the edge
    #[must_use]
    fn update_path(
        &self,
        update: impl FnOnce(SurfacePath) -> SurfacePath,
        core: &mut Core,
    ) -> Self;

    /// Update the boundary of the edge
    #[must_use]
    fn update_boundary(
        &self,
        update: impl FnOnce(CurveBoundary<Point<1>>) -> CurveBoundary<Point<1>>,
        core: &mut Core,
    ) -> Self;
}

impl UpdateHalfEdgeGeometry for Handle<HalfEdge> {
    fn update_path(
        &self,
        update: impl FnOnce(SurfacePath) -> SurfacePath,
        core: &mut Core,
    ) -> Self {
        let path = update(self.path());

        let half_edge = HalfEdge::new(
            path,
            self.boundary(),
            self.curve().clone(),
            self.start_vertex().clone(),
        )
        .insert(core);

        core.layers
            .geometry
            .define_half_edge(half_edge.clone(), HalfEdgeGeometry { path });

        half_edge
    }

    fn update_boundary(
        &self,
        update: impl FnOnce(CurveBoundary<Point<1>>) -> CurveBoundary<Point<1>>,
        core: &mut Core,
    ) -> Self {
        HalfEdge::new(
            self.path(),
            update(self.boundary()),
            self.curve().clone(),
            self.start_vertex().clone(),
        )
        .insert(core)
    }
}
