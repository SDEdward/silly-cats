use rand::prelude::IndexedRandom;
use rand::rng;
use reqwest::blocking::get; // For HTTP GET requests
use std::fs;
use std::io::{self, Write};
use std::path::Path; // For creating a random number generator
fn main() {
    println!("Welcome to Silly Cats!");

    // Specify the URL to the directory containing your cat pictures
    let cat_pics_url = "https://raw.githubusercontent.com/SDEdward/silly-cats/main/"; // Adjust based on your repo structure

    print!("Please enter the directory where you want to save the cat pic: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is shown

    let mut dir_path = String::new();
    io::stdin().read_line(&mut dir_path).unwrap();
    let cleaned_dir_path = dir_path.trim();

    // List of cat picture filenames (expand this based on your repo)
    let cat_files = [
        "cat1.png",
        "cat2.png",
        "cat3.png",
        "cat3.png",
        "cat4.png",
        "cat5.png",
        "cat6.png",
        "cat7.png",
        "cat8.png",
        "cat9.png",
        "cat10.png",
        "cat11.png",
        "cat12.png",
        "cat13.png",
    ]; // Make sure these exist in your repo

    // Select a random cat picture
    let random_cat_file = cat_files
        .choose(&mut rng())
        .expect("Failed to choose a random cat file");
    let image_url = format!("{}{}", cat_pics_url, random_cat_file);

    // Generate a filename to save the cat pic
    let mut n = 1;
    let mut file_path;

    loop {
        file_path = format!("{}/sillycat{}.jpg", cleaned_dir_path, n);
        if !Path::new(&file_path).exists() {
            break;
        }
        n += 1;
    }

    // Download the cat picture
    match download_image(&image_url, &file_path) {
        Ok(_) => println!(
            "Successfully downloaded and saved the cat pic at: {}",
            file_path
        ),
        Err(e) => println!("Failed to download the cat pic: {}", e),
    }
}

// Function to download an image
fn download_image(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = get(url)?; // Simplified error handling
    let bytes = response.bytes()?;
    fs::write(file_path, bytes)?;
    Ok(())
}
