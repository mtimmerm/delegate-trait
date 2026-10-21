//! Tests for the `delegate_trait` attribute macro.

use delegate_trait::{DelegateProvider, delegate_trait};

trait Named {
    fn name(&self) -> String;
}

trait Valued<T> {
    fn value(&self) -> T;
}

#[delegate_trait(Named)]
struct Unit;

impl Named for Unit {
    fn name(&self) -> String {
        "unit".into()
    }
}

#[delegate_trait(Named + Send + Sync)]
enum Either {
    Left,
    Right,
}

impl Named for Either {
    fn name(&self) -> String {
        match self {
            Either::Left => "left".into(),
            Either::Right => "right".into(),
        }
    }
}

#[delegate_trait(Valued<T>)]
struct Wrapper<T: Copy> {
    inner: T,
}

impl<T: Copy> Valued<T> for Wrapper<T> {
    fn value(&self) -> T {
        self.inner
    }
}

fn name_of<P: DelegateProvider<Target = dyn Named>>(p: &P) -> String {
    p.delegate_ref().target.name()
}

#[test]
fn simple_struct() {
    assert_eq!(name_of(&Unit), "unit");
}

#[test]
fn enum_with_auto_traits() {
    let target: &(dyn Named + Send + Sync) = Either::Right.delegate_ref().target;
    assert_eq!(target.name(), "right");
    assert_eq!(Either::Left.delegate_ref().target.name(), "left");
}

#[test]
fn generic_trait_and_type() {
    let w = Wrapper { inner: 42u32 };
    let target: &dyn Valued<u32> = w.delegate_ref().target;
    assert_eq!(target.value(), 42);
}
