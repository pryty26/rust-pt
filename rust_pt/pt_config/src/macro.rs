use derive_deftly::define_derive_deftly;

define_derive_deftly! {
    build_with_check for struct: 
    impl $ttype {
        $tvis fn build( $( $fname: $ftype ,)) -> Self {

        }
    }
}