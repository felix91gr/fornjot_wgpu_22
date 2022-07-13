use fj_math::{Line, Point, Scalar, Vector};

use crate::objects::{Curve, Surface};

/// Test intersection between two surfaces
pub fn surface_surface(a: &Surface, b: &Surface) -> Option<Curve<3>> {
    // Algorithm from Real-Time Collision Detection by Christer Ericson. See
    // section 5.4.4, Intersection of Two Planes.

    let a_parametric = PlaneParametric::extract_from_surface(a);
    let b_parametric = PlaneParametric::extract_from_surface(b);

    let a = PlaneConstantNormal::from_parametric_plane(&a_parametric);
    let b = PlaneConstantNormal::from_parametric_plane(&b_parametric);

    let direction = a.normal.cross(&b.normal);

    let denom = direction.dot(&direction);
    if denom == Scalar::ZERO {
        // Comparing `denom` against zero looks fishy. It's probably better to
        // compare it against an epsilon value, but I don't know how large that
        // epsilon should be.
        //
        // I'll just leave it like that, until we had the opportunity to collect
        // some experience with this code.
        // - @hannobraun
        return None;
    }

    let origin = (b.normal * a.distance - a.normal * b.distance)
        .cross(&direction)
        / denom;
    let origin = Point { coords: origin };

    let curve_global = Curve::Line(Line { origin, direction });

    Some(curve_global)
}

/// A plane in parametric form
struct PlaneParametric {
    pub origin: Point<3>,
    pub u: Vector<3>,
    pub v: Vector<3>,
}

impl PlaneParametric {
    pub fn extract_from_surface(surface: &Surface) -> Self {
        let Surface::SweptCurve(surface) = surface;
        let line = match surface.curve {
            Curve::Line(line) => line,
            _ => todo!("Only plane-plane intersection is currently supported."),
        };

        Self {
            origin: line.origin,
            u: line.direction,
            v: surface.path,
        }
    }
}

/// A plane in constant-normal form
struct PlaneConstantNormal {
    pub distance: Scalar,
    pub normal: Vector<3>,
}

impl PlaneConstantNormal {
    /// Extract a plane in constant-normal form from a `Surface`
    ///
    /// Panics, if the given `Surface` is not a plane.
    pub fn from_parametric_plane(plane: &PlaneParametric) -> Self {
        // Convert plane from parametric form to three-point form.
        let a = plane.origin;
        let b = plane.origin + plane.u;
        let c = plane.origin + plane.v;

        // Convert plane from three-point form to constant-normal form. See
        // Real-Time Collision Detection by Christer Ericson, section 3.6, Planes
        // and Halfspaces.
        let normal = (b - a).cross(&(c - a)).normalize();
        let distance = normal.dot(&a.coords);

        PlaneConstantNormal { distance, normal }
    }
}

#[cfg(test)]
mod tests {
    use fj_math::Transform;

    use crate::objects::{Curve, Surface};

    use super::surface_surface;

    #[test]
    fn plane_plane() {
        let xy = Surface::xy_plane();
        let xz = Surface::xz_plane();

        assert_eq!(surface_surface(&xy, &xy), None);
        assert_eq!(
            surface_surface(
                &xy,
                &xy.transform(&Transform::translation([0., 0., 1.]))
            ),
            None,
        );
        assert_eq!(surface_surface(&xy, &xz), Some(Curve::x_axis()));
    }
}
