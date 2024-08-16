use std::fs;
use std::io::Write;
use std::iter::repeat;

fn main() {
    let folder_path = "rust-ssn-batches";
    fs::remove_dir_all(folder_path).ok();
    fs::create_dir(folder_path).unwrap();

    // TODO: limit to only real combinations
    let mut ssns = (000..=999)
        .flat_map(|a| repeat(a).zip(00..=99))
        .flat_map(|ab| repeat(ab).zip(0000..=9999))
        .map(|abc| format!("{:0>3}-{:0>2}-{:0>4}\n", abc.0.0, abc.0.1, abc.1));
    // it would be nicer to use .intersperse() to add the newline, but it's nightly only

    // write batche of 4 million to each file
    for c in b'a'..=b'z' {
        let c = c as char;
        let path = format!("{}/batch-{}.txt", folder_path, c);
        println!("writing {}", path);
        let mut f = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .unwrap_or_else(|_| panic!("failed to open {} for writing", path));

        // TODO: should we write this in chunks so we don't have to allocate 44 megabytes of memory?
        let batch = ssns.by_ref().take(4_000_000);
        f.write_all(batch.collect::<String>().as_bytes())
            .unwrap_or_else(|_| panic!("failed while writing to {}", path));
    }
}
