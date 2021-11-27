// use crate::query::{IntoQueryParts, Passthrough, QueryBase, QueryFilter,
// QueryModifier};

// /// Wrapper over a `QueryBase`. Applies an include `QueryModifier`.
// pub struct Include<B, I> {
//     base: B,
//     include: I,
// }

// impl<'a, B, I> Include<B, I>
// where
//     B: QueryBase<'a>,
//     I: QueryModifier<'a>,
// {
//     pub(crate) fn new(base: B, include: I) -> Self {
//         Self { base, include }
//     }

//     /// Applies an exclude modifier to the query.
//     pub fn exclude<E>(self, exclude: E) -> IncludeExclude<B, I, E>
//     where
//         E: QueryModifier<'a>,
//     {
//         IncludeExclude::new(self.base, self.include, exclude)
//     }

//     /// Applies a filter to the query.
//     pub fn filter<F>(self, filter: F) -> IncludeExcludeFilter<B, I,
// Passthrough, F>     where
//         F: QueryFilter,
//     {
//         IncludeExcludeFilter::new(self.base, self.include, Passthrough,
// filter)     }
// }

// impl<'a, B, I> IntoQueryParts<'a> for Include<B, I>
// where
//     B: QueryBase<'a>,
//     I: QueryModifier<'a>,
// {
//     type Base = B;
//     type Include = I;
//     type Exclude = Passthrough;
//     type Filter = Passthrough;

//     fn into_query_parts(self) -> (Self::Base, Self::Include, Self::Exclude,
// Self::Filter) {         (self.base, self.include, Passthrough, Passthrough)
//     }
// }

// /// Wrapper over a `QueryBase`. Applies include and exclude `QueryModifier`s.
// pub struct IncludeExclude<B, I, E> {
//     base: B,
//     include: I,
//     exclude: E,
// }

// impl<'a, B, I, E> IncludeExclude<B, I, E>
// where
//     B: QueryBase<'a>,
//     I: QueryModifier<'a>,
//     E: QueryModifier<'a>,
// {
//     pub(crate) fn new(base: B, include: I, exclude: E) -> Self {
//         Self {
//             base,
//             include,
//             exclude,
//         }
//     }

//     /// Applies a filter to the query.
//     pub fn filter<F>(self, filter: F) -> IncludeExcludeFilter<B, I, E, F>
//     where
//         F: QueryFilter,
//     {
//         IncludeExcludeFilter::new(self.base, self.include, self.exclude,
// filter)     }
// }

// impl<'a, B, I, E> IntoQueryParts<'a> for IncludeExclude<B, I, E>
// where
//     B: QueryBase<'a>,
//     I: QueryModifier<'a>,
//     E: QueryModifier<'a>,
// {
//     type Base = B;
//     type Include = I;
//     type Exclude = E;
//     type Filter = Passthrough;

//     fn into_query_parts(self) -> (Self::Base, Self::Include, Self::Exclude,
// Self::Filter) {         (self.base, self.include, self.exclude, Passthrough)
//     }
// }

// /// Wrapper over a `QueryBase`. Applies include and exclude `QueryModifier`s
// as /// well as a `QueryFilter`.
// pub struct IncludeExcludeFilter<B, I, E, F> {
//     base: B,
//     include: I,
//     exclude: E,
//     filter: F,
// }

// impl<'a, B, I, E, F> IncludeExcludeFilter<B, I, E, F>
// where
//     B: QueryBase<'a>,
//     I: QueryModifier<'a>,
//     E: QueryModifier<'a>,
//     F: QueryFilter,
// {
//     pub(crate) fn new(base: B, include: I, exclude: E, filter: F) -> Self {
//         Self {
//             base,
//             include,
//             exclude,
//             filter,
//         }
//     }
// }

// impl<'a, B, I, E, F> IntoQueryParts<'a> for IncludeExcludeFilter<B, I, E, F>
// where
//     B: QueryBase<'a>,
//     I: QueryModifier<'a>,
//     E: QueryModifier<'a>,
//     F: QueryFilter,
// {
//     type Base = B;
//     type Include = I;
//     type Exclude = E;
//     type Filter = F;

//     fn into_query_parts(self) -> (Self::Base, Self::Include, Self::Exclude,
// Self::Filter) {         (self.base, self.include, self.exclude, self.filter)
//     }
// }
