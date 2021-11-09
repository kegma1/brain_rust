fn main() {
    // let prog = "<<<>>><++--..,,[[]]] This is a comment";
    let _prog = ">><<+>+>+>[[].]";

    brain_rust::runtime::Runtime::new(_prog).run()
}
