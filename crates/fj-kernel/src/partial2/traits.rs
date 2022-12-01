/// Implemented for objects that a partial object variant exists for
pub trait HasPartial {
    /// The type representing the partial variant of this object
    type Partial;
}

macro_rules! impl_trait {
    ($($full:ident, $partial:ident;)*) => {
        $(
            impl HasPartial for crate::objects::$full {
                type Partial = super::$partial;
            }
        )*
    };
}

impl_trait!(
    Curve, PartialCurve;
    Cycle, PartialCycle;
    Face, PartialFace;
    GlobalCurve, PartialGlobalCurve;
    GlobalEdge, PartialGlobalEdge;
    GlobalVertex, PartialGlobalVertex;
    HalfEdge, PartialHalfEdge;
    Shell, PartialShell;
    Sketch, PartialSketch;
    Solid, PartialSolid;
    Surface, PartialSurface;
    SurfaceVertex, PartialSurfaceVertex;
    Vertex, PartialVertex;
);
