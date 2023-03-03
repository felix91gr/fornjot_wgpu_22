use fj_math::Point;
use itertools::Itertools;

use crate::{
    geometry::surface::SurfaceGeometry,
    objects::{HalfEdge, Objects},
    partial::{Partial, PartialCycle},
    services::Service,
};

use super::{HalfEdgeBuilder, ObjectArgument};

/// Builder API for [`PartialCycle`]
pub trait CycleBuilder {
    /// Add a new half-edge to the cycle
    ///
    /// Creates a half-edge and adds it to the cycle. The new half-edge is
    /// connected to the front vertex of the last half-edge , and the back
    /// vertex of the first edge, making sure the half-edges actually form a
    /// cycle.
    ///
    /// If this is the first half-edge being added, it is connected to itself,
    /// meaning its front and back vertices are the same.
    fn add_half_edge(&mut self) -> Partial<HalfEdge>;

    /// Update cycle as a polygon from the provided points
    fn update_as_polygon_from_points<O, P>(
        &mut self,
        points: O,
        objects: &mut Service<Objects>,
    ) -> O::SameSize<Partial<HalfEdge>>
    where
        O: ObjectArgument<P>,
        P: Into<Point<2>>;

    /// Connect the cycles to the provided half-edges
    ///
    /// Assumes that the provided half-edges, once translated into local
    /// equivalents of this cycle, form a cycle themselves.
    ///
    /// Returns the local equivalents of the provided half-edges.
    fn connect_to_closed_edges<O>(
        &mut self,
        edges: O,
        surface: &SurfaceGeometry,
    ) -> O::SameSize<Partial<HalfEdge>>
    where
        O: ObjectArgument<Partial<HalfEdge>>;

    /// Infer the positions of all vertices, if necessary
    fn infer_vertex_positions_if_necessary(
        &mut self,
        surface: &SurfaceGeometry,
    );
}

impl CycleBuilder for PartialCycle {
    fn add_half_edge(&mut self) -> Partial<HalfEdge> {
        let half_edge = Partial::new();
        self.half_edges.push(half_edge.clone());
        half_edge
    }

    fn update_as_polygon_from_points<O, P>(
        &mut self,
        points: O,
        _: &mut Service<Objects>,
    ) -> O::SameSize<Partial<HalfEdge>>
    where
        O: ObjectArgument<P>,
        P: Into<Point<2>>,
    {
        let mut start_positions = Vec::new();
        let half_edges = points.map(|point| {
            start_positions.push(point.into());
            self.add_half_edge()
        });

        for ((start, end), half_edge) in start_positions
            .into_iter()
            .circular_tuple_windows()
            .zip(&mut self.half_edges)
        {
            half_edge.write().update_as_line_segment(start, end);
        }

        half_edges
    }

    fn connect_to_closed_edges<O>(
        &mut self,
        edges: O,
        surface: &SurfaceGeometry,
    ) -> O::SameSize<Partial<HalfEdge>>
    where
        O: ObjectArgument<Partial<HalfEdge>>,
    {
        edges.map_with_prev(|other, prev| {
            let mut this = self.add_half_edge();
            this.write().update_from_other_edge(&other, &prev, surface);
            this
        })
    }

    fn infer_vertex_positions_if_necessary(
        &mut self,
        surface: &SurfaceGeometry,
    ) {
        for (mut half_edge, next_half_edge) in
            self.half_edges.iter().cloned().circular_tuple_windows()
        {
            let next_vertex = next_half_edge.read().start_vertex.clone();
            half_edge
                .write()
                .infer_vertex_positions_if_necessary(surface, next_vertex);
        }
    }
}
