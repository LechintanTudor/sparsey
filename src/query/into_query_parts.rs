use crate::query::QueryPart;

/// Helper trait for decomposing queries into their underlying parts.
pub trait IntoQueryParts {
    /// The component views from which to return components.
    type Get: QueryPart;

    /// The component views that act as an "include filter".
    type Include: QueryPart;

    /// The component views that act as an "exclude filter".
    type Exclude: QueryPart;

    /// Splits the query into its underlying parts.
    #[must_use]
    fn into_query_parts(self) -> (Self::Get, Self::Include, Self::Exclude);
}

impl<Q> IntoQueryParts for Q
where
    Q: QueryPart,
{
    type Get = Self;
    type Include = ();
    type Exclude = ();

    fn into_query_parts(self) -> (Self::Get, Self::Include, Self::Exclude) {
        (self, (), ())
    }
}
