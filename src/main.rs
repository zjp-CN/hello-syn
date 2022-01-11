fn main() {
    hello_impl::seq!(N {
        let _succeeded = 1;
    });

    hello_impl::seq_failed!(N {
        let _failed = 0;
    });
}
