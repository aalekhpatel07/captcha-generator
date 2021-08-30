
use captcha::{gen, Difficulty};
use clap::{Arg, App};
use rand::Rng;
use std::path::Path;
use std::fs;
use indicatif::{ProgressBar};
use rayon::prelude::*;


fn generate_and_save_captcha(output: &str) {

    let mut rng = rand::thread_rng();

    let diff = match rng.gen_range(0 as usize, 3 as usize) {
            0 => {
                Difficulty::Easy
            },
            1 => {
                Difficulty::Medium
            },
            _ => {
                Difficulty::Hard
            }    
        };

    let captcha = gen(diff);

    if let Some((chars, _)) = captcha.as_tuple(){
        let out_file_name = format!("{}.png", chars);
        let out_path = Path::new(output).join(out_file_name);
        fs::create_dir_all(&output).unwrap();
        captcha.save(&out_path).unwrap();
    }
}


fn generate_captchas_to_directory(n: usize, output: &str) {
    
    println!("Generating {} captchas to {}...", n, &output);

    let bar = ProgressBar::new(n as u64);

    (0..n).into_par_iter().for_each(|_| {
        generate_and_save_captcha(&output);
        bar.inc(1);
    });

    bar.finish();
}


fn main() {

    let namespace = App::new("Captcha Generator")
                        .version("1.0")
                        .author("Aalekh P. <aalekh@protectchildren.ca>")
                        .about("Generate Captchas of random difficulty to a directory.")
                        .arg(
                            Arg::with_name("number")
                            .index(1)
                            .required(true)
                            .help("The number of captchas to generate.")
                        )
                        .arg(
                            Arg::with_name("output")
                            .index(2)
                            .help("The path to the directory that will store the generated captchas.")
                            .required(true)
                        )
                        .get_matches();


    let number = namespace
                    .value_of("number")
                    .unwrap_or("0")
                    .parse::<usize>()
                    .unwrap();

    if let Some(output) = namespace.value_of("output") {
        generate_captchas_to_directory(number, &output);
    } else {
        println!("{}", namespace.usage());
    }



}
