fn main() {
    // TODO: limit to only real combinations
    let ssns = (000..=999)
        .flat_map(|a| std::iter::repeat(a)
            .map(|a| {
                let mut s = String::with_capacity(11);
                s.push_str(&format!("{:0>3}", a));
                s
            }) // create strings with enough capacity to avoid extra allocations
            .zip(00..=99)
            .map(|ab| format!("{}-{:0>2}", ab.0, ab.1))
        )
        .flat_map(|b| std::iter::repeat(b)
            .zip(0000..=9999)
            .map(|bc| format!("{}-{:0>4}", bc.0, bc.1))
        );
    
    // example: grab the first 50
    dbg!(ssns.take(50).collect::<Vec<_>>());
}
