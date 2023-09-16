//! Implementations for basic types used by the `convert(auto)` conversion attribute.

use super::*;
/// Used internally to quickly implement PestTree<R> for types implementing FromStr.
/// Blanket implementations cannot be used because Rust doesn't support specialization yet.
/// Using one would mean that [`Box<T>`] can't have its own separate implementation.
macro_rules! pest_tree_implementation {
    ($typ:ty) => {
        impl<R: pest::RuleType> PestTree<R> for $typ {
            fn with_pair(
                pair: pest::iterators::Pair<'_, R>,
                context: Rc<ParsingContext>,
            ) -> Result<Self, TreeError<R>>
            where
                Self: Sized,
            {
                let res = pair.as_str().parse::<$typ>();
                if let Ok(v) = res {
                    return Ok(v);
                } else {
                    return Err(StringConversionError::from_str_conversion_error::<$typ>(
                        pair, context,
                    ));
                }
            }
        }
    };
}
pest_tree_implementation!(u8);
pest_tree_implementation!(u16);
pest_tree_implementation!(u32);
pest_tree_implementation!(u64);
pest_tree_implementation!(u128);
pest_tree_implementation!(i8);
pest_tree_implementation!(i16);
pest_tree_implementation!(i32);
pest_tree_implementation!(i64);
pest_tree_implementation!(i128);
pest_tree_implementation!(String);

impl<R: pest::RuleType, T: PestTree<R>> PestTree<R> for Box<T> {
    fn with_pair(
        pair: pest::iterators::Pair<'_, R>,
        context: Rc<ParsingContext>,
    ) -> Result<Self, TreeError<R>>
    where
        Self: Sized,
    {
        let res = T::with_pair(pair.clone(), context.clone());
        if let Ok(v) = res {
            Ok(Box::new(v))
        } else {
            Err(BoxConversionError::from_type::<T>(pair, context))
        }
    }
}

impl<R: pest::RuleType, T: PestTree<R>> PestTree<R> for Option<T> {
    fn with_pair(
        pair: pest::iterators::Pair<'_, R>,
        context: Rc<ParsingContext>,
    ) -> Result<Self, TreeError<R>>
    where
        Self: Sized,
    {
        let res = T::with_pair(pair, context);
        Ok(res.ok())
    }
}
