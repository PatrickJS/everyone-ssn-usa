use std::iter::repeat;

fn main() {
    // TODO: limit to only real combinations

    let ssns = (000..=999)
        .flat_map(|a| repeat(a).zip(00..=99))
        .flat_map(|ab| repeat(ab).zip(0000..=9999))
        .map(|abc| format!("{:0>3}-{:0>2}-{:0>4}", abc.0.0, abc.0.1, abc.1));


    dbg!(ssns.take(50).collect::<Vec<_>>());
}
