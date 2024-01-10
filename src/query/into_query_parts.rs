use crate::query::QueryPart;

pub trait IntoQueryParts {
    type Get: QueryPart;
    type Include: QueryPart;
    type Exclude: QueryPart;

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
