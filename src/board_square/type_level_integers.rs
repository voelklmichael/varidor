pub trait IntegerTrait<Type>: Sized {
    type PreviousType: IntegerTrait<Type>;
    const SIZE: Type;
    fn new() -> Self;
}

pub struct Usize0 {}
impl IntegerTrait<usize> for Usize0 {
    type PreviousType = Self;
    const SIZE: usize = 0;
    fn new() -> Self {
        Usize0 {}
    }
}

use std::marker::PhantomData;
pub struct UsizeNext<PreviousType> {
    _phantom_data: PhantomData<PreviousType>,
}
impl<PreviousType: IntegerTrait<usize>> IntegerTrait<usize> for UsizeNext<PreviousType> {
    type PreviousType = PreviousType;
    const SIZE: usize = <PreviousType as IntegerTrait<usize>>::SIZE + 1;
    fn new() -> Self {
        UsizeNext {
            _phantom_data: PhantomData,
        }
    }
}

pub type Usize1 = UsizeNext<Usize0>;
pub type Usize2 = UsizeNext<Usize1>;
pub type Usize3 = UsizeNext<Usize2>;
pub type Usize4 = UsizeNext<Usize3>;
pub type Usize5 = UsizeNext<Usize4>;
