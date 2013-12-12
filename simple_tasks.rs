

fn print_hello() {
    println("Hello");
}


fn main() {
    do 100.times {
        // the two below are equivalent
        spawn(print_hello);

        //do spawn {
        //    println("Hello");
        //}
    }
}
