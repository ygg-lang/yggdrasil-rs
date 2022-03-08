// mod character;
// mod compare;
// use core::marker::PhantomData;
// use std::{error::Error, fmt::Debug};
// mod input_stream;
//
// use crate::{ErrorMessage, Failed, Failure, IResult, Success};
//
// impl Failed {
//     /// Tests if the result is Incomplete
//     pub fn is_incomplete(&self) -> bool {
//         if let Failed::Incomplete(_) = self { true } else { false }
//     }
//
//     /// Applies the given function to the inner error
//     pub fn map<F>(self, f: F) -> Failed
//     where
//         F: FnOnce(ErrorMessage) -> ErrorMessage,
//     {
//         match self {
//             Failed::Incomplete(n) => Failed::Incomplete(n),
//             Failed::Fatal(t) => Failed::Fatal(f(t)),
//             Failed::Fail(t) => Failed::Fail(f(t)),
//         }
//     }
// }
//
// /// All nom parsers implement this trait
// pub trait Parser<I, O> {
//     /// A parser takes in input type, and returns a `Result` containing
//     /// either the remaining input and the output value, or an error
//     fn parse(&mut self, input: I) -> IResult<I, O>;
//
//     /// Maps a function over the result of a parser
//     fn map<G, O2>(self, g: G) -> Map<Self, G, O>
//     where
//         G: Fn(O) -> O2,
//         Self: Sized,
//     {
//         Map { f: self, g, phantom: PhantomData }
//     }
//
//     /// Creates a second parser from the output of the first one, then apply over the rest of the input
//     fn flat_map<G, H, O2>(self, g: G) -> FlatMap<Self, G, O>
//     where
//         G: FnMut(O) -> H,
//         H: Parser<I, O2>,
//         Self: Sized,
//     {
//         FlatMap { f: self, g, phantom: PhantomData }
//     }
//
//     /// Applies a second parser over the output of the first one
//     fn and_then<G, O2>(self, g: G) -> AndThen<Self, G, O>
//     where
//         G: Parser<O, O2>,
//         Self: Sized,
//     {
//         AndThen { f: self, g, phantom: PhantomData }
//     }
//
//     /// Applies a second parser after the first one, return their results as a tuple
//     fn and<G, O2>(self, g: G) -> And<Self, G>
//     where
//         G: Parser<I, O2>,
//         Self: Sized,
//     {
//         And { f: self, g }
//     }
//
//     /// Applies a second parser over the input if the first one failed
//     fn or<G>(self, g: G) -> Or<Self, G>
//     where
//         G: Parser<I, O>,
//         Self: Sized,
//     {
//         Or { f: self, g }
//     }
//
//     /// automatically converts the parser's output and error values to another type, as long as they
//     /// implement the `From` trait
//     fn into<O2: From<O>>(self) -> Into<Self, O, O2>
//     where
//         Self: Sized,
//     {
//         Into { f: self, phantom_out1: PhantomData, phantom_out2: PhantomData }
//     }
// }
//
// impl<'a, I, O, F> Parser<I, O> for F
// where
//     F: FnMut(I) -> IResult<I, O> + 'a,
// {
//     fn parse(&mut self, i: I) -> IResult<I, O> {
//         self(i)
//     }
// }
//
// impl<'a, I, O> Parser<I, O> for Box<dyn Parser<I, O> + 'a> {
//     fn parse(&mut self, input: I) -> IResult<I, O> {
//         (**self).parse(input)
//     }
// }
//
// /// Implementation of `Parser::map`
// pub struct Map<F, G, O1> {
//     f: F,
//     g: G,
//     phantom: PhantomData<O1>,
// }
//
// impl<'a, I, O1, O2, F: Parser<I, O1>, G: Fn(O1) -> O2> Parser<I, O2> for Map<F, G, O1> {
//     fn parse(&mut self, i: I) -> IResult<I, O2> {
//         let (i, o) = self.f.parse(i)?;
//         Success(i, (self.g)(o))
//     }
// }
//
// /// Implementation of `Parser::flat_map`
// pub struct FlatMap<F, G, O1> {
//     f: F,
//     g: G,
//     phantom: PhantomData<O1>,
// }
//
// impl<'a, I, O1, O2, F: Parser<I, O1>, G: Fn(O1) -> H, H: Parser<I, O2>> Parser<I, O2> for FlatMap<F, G, O1> {
//     fn parse(&mut self, i: I) -> IResult<I, O2> {
//         let (i, o1) = self.f.parse(i)?;
//         (self.g)(o1).parse(i)
//     }
// }
//
// /// Implementation of `Parser::and_then`
// #[cfg_attr(nightly, warn(rustdoc::missing_doc_code_examples))]
// pub struct AndThen<F, G, O1> {
//     f: F,
//     g: G,
//     phantom: PhantomData<O1>,
// }
//
// impl<'a, I, O1, O2, F: Parser<I, O1>, G: Parser<O1, O2>> Parser<I, O2> for AndThen<F, G, O1> {
//     fn parse(&mut self, i: I) -> IResult<I, O2> {
//         let (i, o1) = self.f.parse(i)?;
//         let (_, o2) = self.g.parse(o1)?;
//         Success(i, o2)
//     }
// }
//
// /// Implementation of `Parser::and`
// #[cfg_attr(nightly, warn(rustdoc::missing_doc_code_examples))]
// pub struct And<F, G> {
//     f: F,
//     g: G,
// }
//
// impl<'a, I, O1, O2, F: Parser<I, O1>, G: Parser<I, O2>> Parser<I, (O1, O2)> for And<F, G> {
//     fn parse(&mut self, i: I) -> IResult<I, (O1, O2)> {
//         let (i, o1) = self.f.parse(i)?;
//         let (i, o2) = self.g.parse(i)?;
//         Success(i, (o1, o2))
//     }
// }
//
// /// Implementation of `Parser::or`
// #[cfg_attr(nightly, warn(rustdoc::missing_doc_code_examples))]
// pub struct Or<F, G> {
//     f: F,
//     g: G,
// }
//
// impl<'a, I: Clone, O, F: Parser<I, O>, G: Parser<I, O>> Parser<I, O> for Or<F, G> {
//     fn parse(&mut self, i: I) -> IResult<I, O> {
//         match self.f.parse(i.clone()) {
//             Failure(Failed::Fail(_)) => match self.g.parse(i) {
//                 Failure(Failed::Fail(e2)) => Failure(Failed::Fail(e2)),
//                 res => res,
//             },
//             res => res,
//         }
//     }
// }
//
// /// Implementation of `Parser::into`
// #[cfg_attr(nightly, warn(rustdoc::missing_doc_code_examples))]
// pub struct Into<F, O1, O2: From<O1>> {
//     f: F,
//     phantom_out1: PhantomData<O1>,
//     phantom_out2: PhantomData<O2>,
// }
//
// impl<'a, I: Clone, O1, O2: From<O1>, F: Parser<I, O1>> Parser<I, O2> for Into<F, O1, O2> {
//     fn parse(&mut self, i: I) -> IResult<I, O2> {
//         match self.f.parse(i) {
//             Success(i, o) => Success(i, o.into()),
//             Failure(Failed::Fail(e)) => Failure(Failed::Fail(e)),
//             Failure(Failed::Fatal(e)) => Failure(Failed::Fatal(e)),
//             Failure(Failed::Incomplete(e)) => Failure(Failed::Incomplete(e)),
//         }
//     }
// }
