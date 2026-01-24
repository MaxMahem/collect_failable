use collect_failable::errors::Collision;

#[derive(Debug, Clone, Copy)]
pub struct CollisionData<T, const B: usize, const A: usize> {
    pub base: [T; B],
    pub add: [T; A],
    pub collide_pos: usize,
}

impl<T: Copy, const B: usize, const A: usize> CollisionData<T, B, A> {
    pub fn colliding_item(self) -> T {
        self.add[self.collide_pos]
    }

    pub fn colliding_error(self) -> Collision<T> {
        Collision::new(self.colliding_item())
    }

    pub fn collected(self) -> impl Iterator<Item = T> {
        self.add.into_iter().take(self.collide_pos)
    }

    pub fn remaining(self) -> impl Iterator<Item = T> {
        self.add.into_iter().skip(self.collide_pos + 1)
    }
}

#[allow(unused_macros)]
macro_rules! test_try_collect_collision {
    ($name:ident, $type:ty, $data:expr) => {
        #[test]
        fn $name() {
            let err = <$type>::try_from_iter($data.add).expect_err("should collide");

            assert_eq!(err.collected, $data.collected().collect::<$type>());
            assert_eq!(err.error, $data.colliding_error());
            assert_eq!(err.error.item(), Some(&$data.colliding_item()));
            assert_eq!(err.iterator.clone().collect::<Vec<_>>(), $data.remaining().collect::<Vec<_>>());

            assert_eq_unordered!(
                err.into_iter().collect::<Vec<_>>(),
                $data.add.into_iter().collect::<Vec<_>>(),
                "all added items should be recovered"
            );
        }
    };
}

#[allow(unused_macros)]
macro_rules! test_try_extend_safe_collision {
    ($name:ident, $type:ty, $data:expr) => {
        #[test]
        fn $name() {
            let mut collection = <$type>::from($data.base);

            let err = collection.try_extend_safe($data.add).expect_err("should collide");

            assert_eq!(collection, <$type>::from($data.base), "collection should be unchanged");

            assert_eq!(err.collected, $data.collected().collect::<$type>());
            assert_eq!(err.error, $data.colliding_error());
            assert_eq!(err.error.item(), Some(&$data.colliding_item()));
            assert_eq!(err.iterator.clone().collect::<Vec<_>>(), $data.remaining().collect::<Vec<_>>());

            assert_eq_unordered!(
                $data.add.into_iter().collect::<Vec<_>>(),
                err.into_iter().collect::<Vec<_>>(),
                "all added items should be recovered"
            );
        }
    };
}

#[allow(unused_macros)]
macro_rules! test_try_extend_collision {
    ($name:ident, $type:ty, $data:expr) => {
        #[test]
        fn $name() {
            let mut collection = <$type>::from($data.base);

            let err = collection.try_extend($data.add).expect_err("should collide");

            // collection can be mutated, no check on final state vs initial here

            assert!(err.collected.is_empty(), "collected in error should be empty for try_extend");
            assert_eq!(err.error, $data.colliding_error());
            assert_eq!(err.error.item(), Some(&$data.colliding_item()));
            assert_eq!(err.iterator.clone().collect::<Vec<_>>(), $data.remaining().collect::<Vec<_>>());

            let err_content: Vec<_> = err.into_iter().collect();
            assert_eq_unordered!(
                err_content.clone(),
                $data.remaining().chain(std::iter::once($data.colliding_item())).collect::<Vec<_>>(),
                "iter should contain all iterated items (remaining + colliding)"
            );

            assert_eq_unordered!(
                std::iter::chain(err_content, collection).collect::<Vec<_>>(),
                std::iter::chain($data.base, $data.add).collect::<Vec<_>>(),
                "all items should be recovered (error content + partial collection = original total)"
            );
        }
    };
}

#[allow(unused_imports)]
pub(crate) use test_try_collect_collision;
#[allow(unused_imports)]
pub(crate) use test_try_extend_collision;
#[allow(unused_imports)]
pub(crate) use test_try_extend_safe_collision;
