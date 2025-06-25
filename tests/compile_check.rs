use dyn_compatible::dyn_compatible;
use std::fmt::Display;

#[dyn_compatible(true)]
trait _DynTrait {
    fn instance_method(&self);
}

#[dyn_compatible(false)]
trait _NotDynTrait {
    fn static_method();
    fn instance_method(&self);
}

#[dyn_compatible(true)]
trait _GenericDynTrait<'a, T1, T2: Display, const N: usize> {
    fn instance_method(&self);
}

#[dyn_compatible(false)]
trait _GenericNotDynTrait<'a, T1, T2: Display, const N: usize> {
    fn static_method();
    fn instance_method(&self);
}
