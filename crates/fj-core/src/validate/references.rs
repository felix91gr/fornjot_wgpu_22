use std::collections::HashMap;

use crate::storage::Handle;
use crate::topology::{Cycle, Face, HalfEdge, Region, Shell};

#[derive(Default)]
pub struct ReferenceCounter<T, U>(HashMap<Handle<T>, Vec<Handle<U>>>);

impl<T, U> ReferenceCounter<T, U> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add_reference(
        &mut self,
        referenced: Handle<T>,
        reference: Handle<U>,
    ) {
        self.0
            .entry(referenced)
            .and_modify(|references| references.push(reference.clone()))
            .or_insert(vec![reference]);
    }

    pub fn get_multiples(&self) -> Vec<MultipleReferences<T, U>> {
        self.0
            .iter()
            .filter(|(_, references)| references.len() > 1)
            .map(|(referenced, references)| MultipleReferences {
                referenced: referenced.clone(),
                references: references.to_vec(),
            })
            .collect()
    }
}

/// Find errors and convert to [`crate::validate::ValidationError`]
#[macro_export]
macro_rules! validate_references {
    ($errors:ident, $error_ty:ty;$($counter:ident, $err:ident;)*) => {
        $(
            $counter.get_multiples().iter().for_each(|multiple| {
                let reference_error = ObjectNotExclusivelyOwned::$err { references: multiple.clone() };
                $errors.push(Into::<$error_ty>::into(reference_error).into());
            });
        )*
    };
}

/// Object that should be exclusively owned by another, is not
///
/// Some objects are expected to be "owned" by a single other object. This means
/// that only one reference to these objects must exist within the topological
/// object graph.
#[derive(Clone, Debug, thiserror::Error)]
pub enum ObjectNotExclusivelyOwned {
    /// [`Region`] referenced by more than one [`Face`]
    #[error(
        "[`Region`] referenced by more than one [`Face`]\n{references:#?}"
    )]
    Region {
        references: MultipleReferences<Region, Face>,
    },
    /// [`Face`] referenced by more than one [`Shell`]
    #[error("[`Face`] referenced by more than one [`Shell`]\n{references:#?}")]
    Face {
        references: MultipleReferences<Face, Shell>,
    },
    /// [`HalfEdge`] referenced by more than one [`Cycle`]
    #[error(
        "[`HalfEdge`] referenced by more than one [`Cycle`]\n{references:#?}"
    )]
    HalfEdge {
        references: MultipleReferences<HalfEdge, Cycle>,
    },
    /// [`Cycle`] referenced by more than one [`Region`]
    #[error(
        "[`Cycle`] referenced by more than one [`Region`]\n{references:#?}"
    )]
    Cycle {
        references: MultipleReferences<Cycle, Region>,
    },
}

pub struct MultipleReferences<T, U> {
    referenced: Handle<T>,
    references: Vec<Handle<U>>,
}

use std::fmt::Debug;

impl<T: Debug, U: Debug> Debug for MultipleReferences<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} referenced by {:?}",
            self.referenced, self.references
        )
    }
}

impl<T, U> Clone for MultipleReferences<T, U> {
    fn clone(&self) -> Self {
        Self {
            referenced: self.referenced.clone(),
            references: self.references.to_vec(),
        }
    }
}
