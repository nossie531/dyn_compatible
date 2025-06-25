//! Tests that should be compile errors.

/// Argument type missmatching.
///
/// ```compile_fail
/// # use dyn_compatible::prelude::*;
/// #
/// #[dyn_compatible(42)]
/// trait MyTrait {
///     // nop.
/// }
/// ```
fn _arg_type_missmatch() {
    // nop.
}

/// Argument count lack.
///
/// ```compile_fail
/// # use dyn_compatible::prelude::*;
/// #
/// #[dyn_compatible()]
/// trait MyTrait {
///     // nop.
/// }
/// ```
fn _arg_count_lack() {
    // nop.
}

/// Argument count over.
///
/// ```compile_fail
/// # use dyn_compatible::prelude::*;
/// #
/// #[dyn_compatible(true, true)]
/// trait MyTrait {
///     // nop.
/// }
/// ```
fn _arg_count_over() {
    // nop.
}

/// Illegal target.
///
/// ```compile_fail
/// # use dyn_compatible::prelude::*;
/// #
/// #[dyn_compatible(true)]
/// struct MyType {
///     // nop.
/// }
/// ```
fn _illegal_target() {
    // nop.
}

/// Dyn compatible trait has illegal method.
///
/// ```compile_fail
/// # use dyn_compatible::prelude::*;
/// #
/// #[dyn_compatible(true)]
/// trait MyTrait {
///     fn test();
/// }
/// ```
fn _dyn_has_illegal_method() {
    // nop.
}

/// Not dyn compatible trait use as dyn compatible.
///
/// ```compile_fail
/// # use dyn_compatible::prelude::*;
/// #
/// #[dyn_compatible(false)]
/// trait MyTrait {
///     // nop.
/// }
///
/// fn test(_arg: &dyn MyTrait) {
///     // nop.
/// }
/// ```
fn _not_dyn_as_dyn() {
    // nop.
}
