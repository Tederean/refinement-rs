use refinement::{Predicate, Refinement};

#[derive(PartialEq, Debug)]
struct LessThanTenPredicate;

impl Predicate<i32> for LessThanTenPredicate {
    fn test(x: &i32) -> bool {
        *x < 10
    }
}

type LessThanTen = Refinement<i32, LessThanTenPredicate>;

#[test]
fn add() {
    let x = LessThanTen::new(5).unwrap();
    let y = LessThanTen::new(5).unwrap();
    let _z = x + y;
}

#[cfg(feature = "serde")]
#[test]
fn serde_pass() {
    #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
    struct TestSerde {
        value: LessThanTen,
    }

    let x = TestSerde {
        value: LessThanTen::new(5).unwrap(),
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
        value: i32,
    }

    #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
    struct WithRefinement {
        value: LessThanTen,
    }

    let x = NoRefinement { value: 20 };

    let json = serde_json::to_string(&x).unwrap();

    serde_json::from_str::<WithRefinement>(&json).unwrap_err();
}
