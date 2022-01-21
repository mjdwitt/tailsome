use rstest::rstest;
use std::convert::Infallible;
use std::fmt::Debug;
use tailsome::IntoResult;

#[derive(Clone, Debug, PartialEq)]
struct Thing;

#[rstest]
#[case(())]
#[case(42)]
#[case(Thing)]
#[case("a str")]
#[case(Some(Thing))]
fn anything_can_be_ok<T: Clone + Debug + PartialEq>(#[case] anything: T) {
    match anything.clone().into_ok::<Infallible>() {
        Ok(thing) => assert_eq!(thing, anything),
        Err(_) => unreachable!(),
    }
}

#[rstest]
#[case(())]
#[case(42)]
#[case(Thing)]
#[case("a str")]
#[case(Some(Thing))]
fn anything_can_be_err<T: Clone + Debug + PartialEq>(#[case] anything: T) {
    match anything.clone().into_err::<()>() {
        Err(thing) => assert_eq!(thing, anything),
        Ok(_) => unreachable!(),
    }
}
