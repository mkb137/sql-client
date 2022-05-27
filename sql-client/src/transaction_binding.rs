pub(crate) enum TransactionBinding {
    ImplicitUnbind,
    ExplicitUnbind,
}
pub(crate) struct TransactionBindingKeywords;
impl TransactionBindingKeywords {
    pub const IMPLICIT_UNBIND: &'static str = "Implicit Unbind";
    pub const EXPLICIT_UNBIND: &'static str = "Explicit Unbind";
}
