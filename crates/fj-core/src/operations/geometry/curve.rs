use crate::{
    geometry::{Geometry, LocalCurveGeom, SurfacePath},
    layers::Layer,
    storage::Handle,
    topology::{Curve, Surface},
};

/// Update the geometry of a [`Curve`]
pub trait UpdateCurveGeometry {
    /// Define the geometry as a path on a surface
    fn make_path_on_surface(
        self,
        path: SurfacePath,
        surface: Handle<Surface>,
        geometry: &mut Layer<Geometry>,
    ) -> Self;
}

impl UpdateCurveGeometry for Handle<Curve> {
    fn make_path_on_surface(
        self,
        path: SurfacePath,
        surface: Handle<Surface>,
        geometry: &mut Layer<Geometry>,
    ) -> Self {
        geometry.define_curve(self.clone(), surface, LocalCurveGeom { path });

        self
    }
}