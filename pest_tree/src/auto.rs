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
// for all FromStr implementing types
pest_tree_implementation!(std::net::IpAddr);
pest_tree_implementation!(std::net::SocketAddr);
pest_tree_implementation!(bool);
pest_tree_implementation!(char);

pest_tree_implementation!(f32);
pest_tree_implementation!(f64);

pest_tree_implementation!(i8);
pest_tree_implementation!(i16);
pest_tree_implementation!(i32);
pest_tree_implementation!(i64);
pest_tree_implementation!(i128);
pest_tree_implementation!(isize);

pest_tree_implementation!(u8);
pest_tree_implementation!(u16);
pest_tree_implementation!(u32);
pest_tree_implementation!(u64);
pest_tree_implementation!(u128);
pest_tree_implementation!(usize);

pest_tree_implementation!(std::ffi::OsString);
pest_tree_implementation!(std::net::Ipv4Addr);
pest_tree_implementation!(std::net::Ipv6Addr);
pest_tree_implementation!(std::net::SocketAddrV4);
pest_tree_implementation!(std::net::SocketAddrV6);

pest_tree_implementation!(std::num::NonZeroI8);
pest_tree_implementation!(std::num::NonZeroI16);
pest_tree_implementation!(std::num::NonZeroI32);
pest_tree_implementation!(std::num::NonZeroI64);
pest_tree_implementation!(std::num::NonZeroI128);
pest_tree_implementation!(std::num::NonZeroIsize);

pest_tree_implementation!(std::num::NonZeroU8);
pest_tree_implementation!(std::num::NonZeroU16);
pest_tree_implementation!(std::num::NonZeroU32);
pest_tree_implementation!(std::num::NonZeroU64);
pest_tree_implementation!(std::num::NonZeroU128);
pest_tree_implementation!(std::num::NonZeroUsize);

pest_tree_implementation!(std::path::PathBuf);
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

impl<R: pest::RuleType, T: PestTree<R>> PestTree<R> for Vec<T> {
    fn with_pair(
        pair: pest::iterators::Pair<'_, R>,
        context: Rc<ParsingContext>,
    ) -> Result<Self, TreeError<R>>
    where
        Self: Sized,
    {
        let inner = pair.into_inner();
        let v: Result<Vec<_>, _> = inner
            .map(|pair| T::with_pair(pair, context.clone()))
            .collect();
        v
    }
}
