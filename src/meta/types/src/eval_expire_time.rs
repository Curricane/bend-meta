/// Evaluate and returns the absolute expire time.
pub trait EvalExpireTime {
    /// Evaluate and returns the absolute expire time in millisecond since 1970.
    ///
    /// If there is no expire time, return u64::MAX.
    fn eval_expire_at_ms(&self) -> u64;
}

impl<T> EvalExpireTime for &T
where T: EvalExpireTime
{
    fn eval_expire_at_ms(&self) -> u64 {
        EvalExpireTime::eval_expire_at_ms(*self)
    }
}

impl<T> EvalExpireTime for Option<T>
where T: EvalExpireTime
{
    fn eval_expire_at_ms(&self) -> u64 {
        self.as_ref()
            .map(|m| m.eval_expire_at_ms())
            .unwrap_or(u64::MAX)
    }
}
