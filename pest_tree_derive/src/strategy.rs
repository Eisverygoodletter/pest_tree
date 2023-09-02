use super::*;
/**
 * Strategies for converting a [`pest::Pairs`] to a struct.
 */
pub(crate) enum StructStrategy {
    /**
     * Directly match the whole struct with a string.
     */
    Direct,
    /**
     * Match each struct member sequentially, consuming a pair for each.
     */
    Sequential,
    /**
     * Attempts to directly convert the whole Pairs span (as a string) into every member.
     * This should be used with #[pest_tree(converter(my_function_here))] so that custom converters
     * may be used.
     */
    DirectConvert,
}
/// Strategies for converting a pest::Pairs to an Enum.
pub(crate) enum EnumStrategy {
    /**
     * Match different values of the enum, depending on the pair.
     */
    Conditional,
}
