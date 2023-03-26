// use std::{marker::PhantomData, convert::Infallible, mem::MaybeUninit, ops::ControlFlow};

// pub type SimpleGeneratorState<G:SimpleGenerator>=ControlFlow<
//     G::Return,
//     (G,G::Yield),
// >;

// pub trait SimpleGenerator:Sized {
//     type Yield;
//     type Return;
//     fn step(self)->SimpleGeneratorState<Self>;
//     fn collect<C:FromIterator<Self::Yield>>(self)->(C,Self::Return){
//         let mut self_or_return = ControlFlow::Continue(self);
//         let mut yeilds = std::iter::from_fn(move ||{
//             let ControlFlow::Continue(generator) = self_or_return else{
//                 return None;
//             };
//             match generator.step() {
//                 ControlFlow::Continue((s,y))=>{
//                     self_or_return = ControlFlow::Continue(s);
//                     Some(y)
//                 }
//                 ControlFlow::Break(return_value)=>{
//                     self_or_return = ControlFlow::Break(return_value);
//                     None
//                 }
//             }
//         });
//         let collection:C = C::from_iter(yeilds.by_ref());
//         let return_value = match self_or_return {
//             ControlFlow::Continue(gen)=> loop {
//                 match gen.step() {
//                     ControlFlow::Continue((s,_))=>{
//                         gen = s;
//                     }
//                     ControlFlow::Break(r)=>{
//                         break r
//                     }
//                 }
//             },
//             ControlFlow::Break(r)=>{
//                 r
//             }
//         };
//         (collection,return_value)
//     }
// }

// pub struct Just<R>(pub R);
// impl <R> From<R> for Just<R>{
//     #[inline]
//     fn from(value: R) -> Self {
//         Just(value)
//     }
// }
// impl <R> SimpleGenerator for Just<R> {
//     type Yield=Infallible;
//     type Return=R;
//     #[inline]
//     fn step(self)->SimpleGeneratorState<Self> {
//         ControlFlow::Break(self.0)
//     }
//     #[inline]
//     fn collect<C:FromIterator<Self::Yield>>(self)->(C,Self::Return) {
//         (C::from_iter(std::iter::empty()),self.0)
//     }
// }

// impl <I:Iterator> SimpleGenerator for I {
//     type Yield=I::Item;
//     type Return=();
//     #[inline]
//     fn step(self)->SimpleGeneratorState<Self> {
//         if let Some(item) = self.next(){
//             ControlFlow::Continue((self,item))
//         }else{
//             ControlFlow::Break(())
//         }
//     }
//     #[inline]
//     fn collect<C:FromIterator<Self::Yield>>(self)->(C,Self::Return) {
//         (C::from_iter(self),())
//     }
// }