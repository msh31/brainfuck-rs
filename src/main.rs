fn main() {
    println!("Hello, world!");

    //what the fuck is this shit
    let mut victor = vec![1, 2, 3];
    victor.push(4);

    let victoria = Vec::from([1, 2, 3, 4]);
    assert_eq!(victor, victoria);
}
