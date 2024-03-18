use super::SurfacePath;

/// The geometry of a half-edge
#[derive(Copy, Clone)]
pub struct HalfEdgeGeometry {
    /// # The path of the half-edge
    ///
    /// ## Implementation Note
    ///
    /// Currently, all curve-related geometry is defined locally, in terms of
    /// the surface that the curve is on (or purely in 2D, if there is no
    /// surface associated with this geometry). However, curves exist globally,
    /// independently of surfaces. Half-edges in multiple surfaces can refer to
    /// the same curve, and in fact, that is the whole reason for their
    /// existence as a topological object.
    ///
    /// This contradiction, globally defined curves but locally defined curve
    /// geometry, is the reason that this curve geometry is defined right here,
    /// associated with a locally existing half-edge. (And, I might add,
    /// redundantly so, as multiple half-edges within the same surface context
    /// can refer to the same curve.)
    ///
    /// Instead, it should be possible to define curve geometry *either* locally
    /// or globally. Then that respective definition can be associated with the
    /// curve (and possibly, in addition, a surface). How exactly that is going
    /// to work is up in the air.
    ///
    /// The point of all this exposition is to clarify that this field doesn't
    /// really belong here. It exists here for practical reasons that are,
    /// hopefully, temporary.
    pub path: SurfacePath,
}
