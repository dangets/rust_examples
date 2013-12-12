

fn main() {
    let (port, chan): (Port<int>, Chan<int>) = stream();

    do spawn {
        chan.send(42);
    }

    println(port.recv().to_str());
}
