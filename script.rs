use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn generate_and_save_batch(start: u32, end: u32, file_name: &str) -> std::io::Result<()> {
    let mut combinations = Vec::new();
    for a in start..=end {
        for b in 0..10 {
            for c in 0..10 {
                for d in 0..10 {
                    for e in 0..10 {
                        for f in 0..10 {
                            for g in 0..10 {
                                let combination = format!(
                                    "{:02}{:01}-{:02}-{:01}{:01}{:01}{:01}",
                                    a, b, c, d, e, f, g
                                );
                                combinations.push(combination);
                            }
                        }
                    }
                }
            }
        }
    }

    let mut file = File::create(file_name)?;
    file.write_all(combinations.join("\n").as_bytes())?;
    println!("Batch saved to {}", file_name);
    Ok(())
}

fn generate_batches() -> std::io::Result<()> {
    let folder_path = "./all-ssn-batches";
    fs::create_dir_all(folder_path)?;

    let batch_ranges = vec![
        (0, 3, "batch-a.txt"),
        (4, 7, "batch-b.txt"),
        (8, 11, "batch-c.txt"),
        (12, 15, "batch-d.txt"),
        (16, 19, "batch-e.txt"),
        (20, 23, "batch-f.txt"),
        (24, 27, "batch-g.txt"),
        (28, 31, "batch-h.txt"),
        (32, 35, "batch-i.txt"),
        (36, 39, "batch-j.txt"),
        (40, 43, "batch-k.txt"),
        (44, 47, "batch-l.txt"),
        (48, 51, "batch-m.txt"),
        (52, 55, "batch-n.txt"),
        (56, 59, "batch-o.txt"),
        (60, 63, "batch-p.txt"),
        (64, 67, "batch-q.txt"),
        (68, 71, "batch-r.txt"),
        (72, 75, "batch-s.txt"),
        (76, 79, "batch-t.txt"),
        (80, 83, "batch-u.txt"),
        (84, 87, "batch-v.txt"),
        (88, 91, "batch-w.txt"),
        (92, 95, "batch-x.txt"),
        (96, 99, "batch-y.txt"),
        (100, 103, "batch-z.txt"),
    ];

    for (start, end, file_name) in batch_ranges {
        let full_path = Path::new(folder_path).join(file_name);
        generate_and_save_batch(start, end, full_path.to_str().unwrap())?;
    }

    Ok(())
}

fn main() {
    if let Err(e) = generate_batches() {
        eprintln!("Error: {}", e);
    }
}
