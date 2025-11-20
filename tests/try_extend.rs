use collect_failable::TryExtend;

#[derive(Debug, PartialEq, Eq)]
struct Collection {
    called: bool,
}

const DEFAULT: Collection = Collection { called: false };
const CALLED: Collection = Collection { called: true };

impl TryExtend<i32> for Collection {
    type Error = ();

    fn try_extend_safe<I>(&mut self, _iter: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = i32>,
    {
        self.called = true;
        Ok(())
    }
}

#[test]
fn test_try_extend_default() {
    let mut test = DEFAULT;
    test.try_extend(vec![1, 2, 3]).expect("Should be ok");
    assert_eq!(test, CALLED);
}
