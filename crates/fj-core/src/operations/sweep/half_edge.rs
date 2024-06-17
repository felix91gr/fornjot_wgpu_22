use fj_interop::{ext::ArrayExt, Color};
use fj_math::{Point, Scalar, Vector};

use crate::{
    geometry::{CurveBoundary, HalfEdgeGeom},
    operations::{
        build::{BuildCycle, BuildHalfEdge},
        geometry::{UpdateCurveGeometry, UpdateHalfEdgeGeometry},
        insert::Insert,
        presentation::SetColor,
        update::{UpdateCycle, UpdateHalfEdge},
    },
    storage::Handle,
    topology::{Curve, Cycle, Face, HalfEdge, Region, Surface, Vertex},
    Core,
};

use super::{vertex::SweepVertex, SweepCache, SweepSurfacePath};

/// # Sweep a [`HalfEdge`]
///
/// See [module documentation] for more information.
///
/// [module documentation]: super
pub trait SweepHalfEdge {
    /// # Sweep the [`HalfEdge`]
    ///
    /// Returns a face, the result of sweeping the edge, as well as the top edge
    /// of that face, i.e. the edge that is the version of the original edge
    /// that was translated along the sweep path.
    ///
    /// In addition to the usual arguments that many sweep operations require,
    /// some other ones are needed:
    ///
    /// - `end_vertex`, the vertex where the half-edge ends. This is the start
    ///   vertex of the next half-edge in the cycle.
    /// - The `surface` that the half-edge is defined on.
    /// - The `color` of the resulting face, if applicable
    fn sweep_half_edge(
        &self,
        end_vertex: Handle<Vertex>,
        surface: Handle<Surface>,
        color: Option<Color>,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        core: &mut Core,
    ) -> (Face, Handle<HalfEdge>);
}

impl SweepHalfEdge for Handle<HalfEdge> {
    fn sweep_half_edge(
        &self,
        end_vertex: Handle<Vertex>,
        surface: Handle<Surface>,
        color: Option<Color>,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        core: &mut Core,
    ) -> (Face, Handle<HalfEdge>) {
        let path = path.into();

        let half_edge_geom = *core.layers.geometry.of_half_edge(self);
        let curve_geom = core
            .layers
            .geometry
            .of_curve(self.curve())
            .unwrap()
            .local_on(&surface)
            .unwrap()
            .clone();
        let surface_geom = *core.layers.geometry.of_surface(&surface);
        let surface =
            curve_geom
                .path
                .sweep_surface_path(&surface_geom, path, core);

        // Next, we need to define the boundaries of the face. Let's start with
        // the global vertices and edges.
        let (vertices, curves) = {
            let [a, b] = [self.start_vertex().clone(), end_vertex];
            let (curve_up, c) = b.clone().sweep_vertex(cache, core);
            let (curve_down, d) = a.clone().sweep_vertex(cache, core);

            (
                [a, b, c, d],
                [
                    Some(self.curve().clone()),
                    Some(curve_up),
                    None,
                    Some(curve_down),
                ],
            )
        };

        // Let's figure out the surface coordinates of the edge vertices.
        let surface_points = {
            let [a, b] = half_edge_geom.boundary.inner;

            [
                [a.t, Scalar::ZERO],
                [b.t, Scalar::ZERO],
                [b.t, Scalar::ONE],
                [a.t, Scalar::ONE],
            ]
            .map(Point::from)
        };
        let surface_points_next = {
            let mut points = surface_points;
            points.rotate_left(1);
            points
        };

        // Now, the boundaries of each edge.
        let boundaries = {
            let [a, b] = half_edge_geom.boundary.inner;
            let [c, d] = [0., 1.].map(|coord| Point::from([coord]));

            [[a, b], [c, d], [b, a], [d, c]]
        };

        let mut exterior = Cycle::empty();

        // Armed with all of that, we're ready to create the edges.
        let [_edge_bottom, _edge_up, edge_top, _edge_down] = boundaries
            .zip_ext(surface_points)
            .zip_ext(surface_points_next)
            .zip_ext(vertices)
            .zip_ext(curves)
            .map(|((((boundary, start), end), start_vertex), curve)| {
                let boundary = CurveBoundary { inner: boundary };

                let curve = curve
                    .unwrap_or_else(|| Curve::new().insert(core))
                    .make_line_on_surface(
                        [start, end],
                        Some(boundary),
                        surface.clone(),
                        &mut core.layers.geometry,
                    );

                let half_edge = HalfEdge::unjoined(core)
                    .update_start_vertex(|_, _| start_vertex, core)
                    .update_curve(|_, _| curve.clone(), core)
                    .insert(core)
                    .set_geometry(
                        HalfEdgeGeom {
                            path: core
                                .layers
                                .geometry
                                .of_curve(&curve)
                                .expect(
                                    "Curve geometry was just defined in same \
                                    function",
                                )
                                .local_on(&surface)
                                .expect(
                                    "Curve geometry was just defined in same \
                                    function",
                                )
                                .path,
                            boundary,
                        },
                        &mut core.layers.geometry,
                    );

                exterior = exterior.add_half_edges([half_edge.clone()], core);

                half_edge
            });

        let exterior = exterior.insert(core);
        let region = Region::new(exterior, []).insert(core);

        if let Some(color) = color {
            region.set_color(color, core);
        }

        let face = Face::new(surface, region);

        (face, edge_top)
    }
}
