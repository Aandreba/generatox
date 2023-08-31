use generatox::generator;

#[generator]
fn hold() {
    for i in 0..10 {
        yield i;
    }
    return 2;
}
