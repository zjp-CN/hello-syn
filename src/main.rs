fn main() {
    hello_impl::seq!(N {
        let _succeeded = 1;
    });

    hello_impl::seq_simpler!(N {
        let _simpler = 0;
    });

    // hello_impl::seq_failed!(N {
    //     let _failed = 0;
    // });
}
