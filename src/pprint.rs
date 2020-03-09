use std::env::current_dir;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Duration;

use crate::detail::{LanguageDetail, SumDetail};

pub struct PrettyPrinter {}

impl PrettyPrinter {
    pub fn terminal(languages: Vec<LanguageDetail>, sum: SumDetail, elapsed: Duration) {
        println!("{:>12.4} secs", elapsed.as_secs_f64());
        println!("┌────────────────────────────────────────────────────────────────┐");
        println!(
            "| {:<14}{:>12}{:>12}{:>12}{:>12} |",
            "Language", "Size", "Code", "Comment", "Blank"
        );
        println!("├────────────────────────────────────────────────────────────────┤");

        for detail in languages {
            println!(
                "| {:<14}{:>12}{:>12}{:>12}{:>12} |",
                detail.language,
                bytes_to_size(detail.bytes as f64),
                detail.code,
                detail.comment,
                detail.blank,
            );
        }

        println!("├────────────────────────────────────────────────────────────────┤");
        println!(
            "| {:<14}{:>12}{:>12}{:>12}{:>12} |",
            "Sum",
            bytes_to_size(sum.bytes as f64),
            sum.code,
            sum.comment,
            sum.blank
        );
        println!("└────────────────────────────────────────────────────────────────┘");
    }

    // TODO
    pub fn markdown(languages: Vec<LanguageDetail>, sum: SumDetail, elapsed: Duration) {
        let mut filename = current_dir().unwrap();
        filename.push("total.md");

        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .append(false)
            .open(filename)
            .unwrap();
        let mut template = format!("{:.4} secs\n\n", elapsed.as_secs_f64());

        template.push_str("| Language      |        Code |     Comment |       Blank |\n");
        template.push_str("----------------|-------------|-------------|--------------\n");
        for detail in languages {
            template.push_str(&format!(
                "| {:<13} | {:>11} | {:>11} | {:>11} |\n",
                detail.language, detail.code, detail.comment, detail.blank
            ));
        }
        template.push_str(&format!(
            "| {:<13} | {:>11} | {:>11} | {:>11} |\n",
            "Sum", sum.code, sum.comment, sum.blank
        ));

        file.write_all(template.as_bytes()).unwrap();
    }
}

const SIZES: [&str; 9] = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

fn bytes_to_size(bytes: f64) -> String {
    let k = 1024_f64;
    if bytes <= 1_f64 {
        return format!("{:.2} B", bytes);
    }
    let i = (bytes.ln() / k.ln()) as i32;
    format!("{:.2} {}", bytes / k.powi(i), SIZES[i as usize])
}
