use refinement::{Predicate, Refinement};

#[derive(PartialEq, Clone, Debug)]
struct NonEmptyStringPredicate;

impl Predicate<String> for NonEmptyStringPredicate {
    fn test(x: &String) -> bool {
        !x.is_empty()
    }
}

type NonEmptyString = Refinement<String, NonEmptyStringPredicate>;

#[test]
fn create_good_value() {
    let x = NonEmptyString::new(String::from("Hello"));
    assert_eq!(Some(String::from("Hello")), x.map(|x| x.to_inner().clone()))
}

#[test]
fn create_bad_value() {
    let x = NonEmptyString::new(String::from(""));
    assert_eq!(None, x);
}

#[test]
fn clone_equality() {
    let x = NonEmptyString::new(String::from("Hello")).unwrap();
    assert_eq!(x, x.clone())
}

#[cfg(feature = "serde")]
#[test]
fn serde_pass() {
    #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
    struct TestSerde {
        value: NonEmptyString,
    }

    let x = TestSerde {
        value: NonEmptyString::new(String::from("Hello")).unwrap(),
    };

    let json = serde_json::to_string(&x).unwrap();

    let y = serde_json::from_str::<TestSerde>(&json).unwrap();

    assert_eq!(x, y)
}

#[cfg(feature = "serde")]
#[test]
fn serde_fail() {
    #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
    struct NoRefinement {
        value: String,
    }

    #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
    struct WithRefinement {
        value: NonEmptyString,
    }

    let x = NoRefinement {
        value: String::from(""),
    };

    let json = serde_json::to_string(&x).unwrap();

    serde_json::from_str::<WithRefinement>(&json).unwrap_err();
}
