pub mod atm_repository_impls;
pub mod bank_account_repository_impls;
mod error;
pub mod transactions;

pub use error::InfraError;

#[cfg(test)]
mod test_utils {
    use std::fmt::Debug;

    use ddd_cqrs_core::Aggregate;

    /// idでソートして比較
    pub(crate) fn assert_aggregates_eq<T: Aggregate + Debug>(
        actual: &mut Vec<T>,
        expected: &mut Vec<T>,
    ) {
        actual.sort_by_key(|aggregate| aggregate.id());
        expected.sort_by_key(|aggregate| aggregate.id());

        assert_eq!(actual, expected);
    }
}
