use rstest::rstest;
use std::fmt::Debug;
use tailsome::IntoOption;

#[derive(Clone, Debug, PartialEq)]
struct Thing;

#[rstest]
#[case(())]
#[case(42)]
#[case(Thing)]
#[case("a str")]
#[case(Some(Thing))]
fn anything_can_be_something<T: Clone + Debug + PartialEq>(#[case] anything: T) {
    match anything.clone().into_some() {
        Some(thing) => assert_eq!(thing, anything),
        None => unreachable!(),
    }
}
