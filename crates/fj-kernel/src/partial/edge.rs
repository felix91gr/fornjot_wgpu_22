use fj_math::{Point, Scalar};

use crate::{
    objects::{
        Curve, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge, Surface, Vertex,
    },
    stores::{Handle, Stores},
};

use super::{MaybePartial, PartialCurve};

/// A partial [`HalfEdge`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct PartialHalfEdge {
    /// The curve that the [`HalfEdge`] is defined in
    pub curve: Option<MaybePartial<Curve>>,

    /// The vertices that bound this [`HalfEdge`] in the [`Curve`]
    pub vertices: Option<[MaybePartial<Vertex>; 2]>,

    /// The global form of the [`HalfEdge`]
    ///
    /// Can be computed by [`PartialHalfEdge::build`], if not available.
    pub global_form: Option<MaybePartial<GlobalEdge>>,
}

impl PartialHalfEdge {
    /// Update the partial half-edge with the given curve
    pub fn with_curve(mut self, curve: impl Into<MaybePartial<Curve>>) -> Self {
        self.curve = Some(curve.into());
        self
    }

    /// Update the partial half-edge with the given vertices
    pub fn with_vertices(
        mut self,
        vertices: [impl Into<MaybePartial<Vertex>>; 2],
    ) -> Self {
        self.vertices = Some(vertices.map(Into::into));
        self
    }

    /// Update the partial half-edge with the given global form
    pub fn with_global_form(
        mut self,
        global_form: impl Into<MaybePartial<GlobalEdge>>,
    ) -> Self {
        self.global_form = Some(global_form.into());
        self
    }

    /// Update partial half-edge as a circle, from the given radius
    pub fn as_circle_from_radius(
        mut self,
        surface: Surface,
        radius: impl Into<Scalar>,
    ) -> Self {
        let curve = Curve::partial()
            .with_surface(surface)
            .as_circle_from_radius(radius);

        let vertices = {
            let [a_curve, b_curve] =
                [Scalar::ZERO, Scalar::TAU].map(|coord| Point::from([coord]));

            let global_vertex = GlobalVertex::partial()
                .from_curve_and_position(curve.clone(), a_curve);

            [a_curve, b_curve].map(|point_curve| {
                Vertex::partial()
                    .with_position(point_curve)
                    .with_curve(curve.clone())
                    .with_global_form(global_vertex.clone())
            })
        };

        self.curve = Some(curve.into());
        self.vertices = Some(vertices.map(Into::into));

        self
    }

    /// Update partial half-edge as a line segment, from the given points
    pub fn as_line_segment_from_points(
        mut self,
        surface: Surface,
        points: [impl Into<Point<2>>; 2],
    ) -> Self {
        let curve = PartialCurve {
            global_form: self.extract_global_curve(),
            ..PartialCurve::default()
        }
        .with_surface(surface)
        .as_line_from_points(points);

        let vertices = [0., 1.].map(|position| {
            Vertex::partial()
                .with_position([position])
                .with_curve(curve.clone())
        });

        self.curve = Some(curve.into());
        self.vertices = Some(vertices.map(Into::into));

        self
    }

    /// Update partial half-edge as a line segment, reusing existing vertices
    pub fn as_line_segment(mut self) -> Self {
        let [from, to] = self
            .vertices
            .clone()
            .expect("Can't infer line segment without vertices");
        let [from_surface, to_surface] = [&from, &to].map(|vertex| {
            vertex
                .surface_form()
                .expect("Can't infer line segment without two surface vertices")
        });

        let surface = from_surface
            .surface()
            .copied()
            .or_else(|| to_surface.surface().copied())
            .expect("Can't infer line segment without a surface");
        let points = [&from_surface, &to_surface].map(|vertex| {
            vertex
                .position()
                .expect("Can't infer line segment without surface position")
        });

        let curve = PartialCurve {
            global_form: self.extract_global_curve(),
            ..PartialCurve::default()
        }
        .with_surface(surface)
        .as_line_from_points(points);

        let vertices = [from, to].map(|vertex| match vertex {
            MaybePartial::Partial(vertex) => {
                let vertex = vertex.with_curve(curve.clone());
                MaybePartial::from(vertex)
            }
            _ => vertex,
        });

        self.curve = Some(curve.into());
        self.vertices = Some(vertices);

        self
    }

    /// Build a full [`HalfEdge`] from the partial half-edge
    pub fn build(self, stores: &Stores) -> HalfEdge {
        let curve = self
            .curve
            .expect("Can't build `HalfEdge` without curve")
            .into_full(stores);
        let vertices = self
            .vertices
            .expect("Can't build `HalfEdge` without vertices")
            .map(|vertex| vertex.into_full(stores));

        let global_form = self
            .global_form
            .unwrap_or_else(|| {
                GlobalEdge::partial()
                    .from_curve_and_vertices(&curve, &vertices)
                    .into()
            })
            .into_full(stores);

        HalfEdge::new(curve, vertices, global_form)
    }

    fn extract_global_curve(
        &self,
    ) -> Option<MaybePartial<Handle<GlobalCurve>>> {
        fn extract_global_curve_from_curve(
            partial: &PartialHalfEdge,
        ) -> Option<MaybePartial<Handle<GlobalCurve>>> {
            partial.curve.as_ref()?.global_form()
        }

        fn extract_global_curve_from_global_form(
            partial: &PartialHalfEdge,
        ) -> Option<MaybePartial<Handle<GlobalCurve>>> {
            Some(partial.global_form.as_ref()?.curve()?.clone().into())
        }

        extract_global_curve_from_curve(self)
            .or_else(|| extract_global_curve_from_global_form(self))
    }
}

impl From<HalfEdge> for PartialHalfEdge {
    fn from(half_edge: HalfEdge) -> Self {
        Self {
            curve: Some(half_edge.curve().clone().into()),
            vertices: Some(half_edge.vertices().clone().map(Into::into)),
            global_form: Some(half_edge.global_form().clone().into()),
        }
    }
}

/// A partial [`GlobalEdge`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct PartialGlobalEdge {
    /// The curve that the [`GlobalEdge`] is defined in
    ///
    /// Must be provided before [`PartialGlobalEdge::build`] is called.
    pub curve: Option<Handle<GlobalCurve>>,

    /// The vertices that bound the [`GlobalEdge`] in the curve
    ///
    /// Must be provided before [`PartialGlobalEdge::build`] is called.
    pub vertices: Option<[GlobalVertex; 2]>,
}

impl PartialGlobalEdge {
    /// Update partial global edge from the given curve and vertices
    pub fn from_curve_and_vertices(
        mut self,
        curve: &Curve,
        vertices: &[Vertex; 2],
    ) -> Self {
        self.curve = Some(curve.global_form().clone());
        self.vertices =
            Some(vertices.clone().map(|vertex| *vertex.global_form()));

        self
    }

    /// Build a full [`GlobalEdge`] from the partial global edge
    pub fn build(self, _: &Stores) -> GlobalEdge {
        let curve = self
            .curve
            .expect("Can't build `GlobalEdge` without `GlobalCurve`");
        let vertices = self
            .vertices
            .expect("Can't build `GlobalEdge` without vertices");

        GlobalEdge::new(curve, vertices)
    }
}

impl From<GlobalEdge> for PartialGlobalEdge {
    fn from(global_edge: GlobalEdge) -> Self {
        Self {
            curve: Some(global_edge.curve().clone()),
            vertices: Some(*global_edge.vertices()),
        }
    }
}
