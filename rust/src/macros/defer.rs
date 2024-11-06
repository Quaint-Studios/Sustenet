pub struct ScopeCall<F: FnMut()> {
    pub c: F
}
impl<F: FnMut()> Drop for ScopeCall<F> {
    fn drop(&mut self) {
        (self.c)();
    }
}
 
#[macro_export]
macro_rules! defer {
    ($e:expr) => (
        let _scope_call = $crate::macros::defer::ScopeCall { c: || -> () { $e; } };
    )
}