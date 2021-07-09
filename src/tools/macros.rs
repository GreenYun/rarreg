macro_rules! gf_2_15_const_arr {
    ($($x:literal),+ $(,)?) => (
        [$($crate::tools::field::GaloisField2p15{num:$x}),+]
    );
}
