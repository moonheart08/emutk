use emutk_core::{
    bus::{Bus, SimpleBusError},
};
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PSL(u32);
pub struct VAXCPU<B>
    where B: Bus<SimpleBusError, ()>
{
    gpr: [u32;16],
    psl: PSL,
    #[doc(hidden)]
    __bus: std::marker::PhantomData<B>,
}

impl<B> VAXCPU<B>
    where B: Bus<SimpleBusError, ()>
{

}