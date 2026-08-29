/// use PtTracing::error($msg) to record and then panic
/// Will generate:
/// <$crate::PtTracing as $crate::traits::TorPtCommunicator>::error($msg).unwrap();
/// panic!("{}", $msg);
#[macro_export]
macro_rules! rec_panic {
    ($msg:expr) => {
        #[allow(unwrap_used)]
        <$crate::PtTracing as $crate::traits::TorPtCommunicator>::error($msg).unwrap();
        panic!("{}", $msg);
    };
}
