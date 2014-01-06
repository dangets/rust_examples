
fn main() {
    let args = std::os::args();

    // args is ~[~str]

    for arg in args.iter() {
        println(*arg);
    }
}
