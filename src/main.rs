use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::error::Error;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    // Créer un client HTTP
    let client = Client::new();

    println!("Entrez votre nom d'utilisateur Letterboxd: ");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username)?;

    let letterboxd_url = format!("https://letterboxd.com/{}/films/by/date/", username.trim());

    let response = client.get(&letterboxd_url).send()?;
    let body = response.text()?;
    let document = Html::parse_document(&body);

    // Définir le sélecteur pour cibler tous les éléments avec l'attribut data-film-slug
    let title_selector = Selector::parse("[data-film-slug]").unwrap();
    let elements = document.select(&title_selector);

    // Parcourir les éléments sélectionnés et écrire la valeur de l'attribut data-film-slug dans le fichier CSV
    for (index, element) in elements.enumerate() {
        // Extraire la valeur de l'attribut data-film-slug
        let film_slug = element.value().attr("data-film-slug").unwrap_or_default();

        // Retirer la date de la fin du titre du film
        let mut film_name_fixed = film_slug;
        if let Some(index_dernier_tiret) = film_slug.rfind('-') {
            if let Some(char_after_dash) = film_slug.chars().nth(index_dernier_tiret + 1) {
                if char_after_dash == '1' || char_after_dash == '2' {
            film_name_fixed = &film_slug[..index_dernier_tiret];
        }
    }
}
        // Construire l'URL de film-grab
        let film_grab_url = format!("https://film-grab.com/bwg_gallery/{}", film_name_fixed);

        println!("URL de film-grab pour film: {}", film_grab_url);

        // Faire une requête GET à l'URL de film-grab
        let response_shot = client.get(&film_grab_url).send()?;
        let body_shot = response_shot.text()?;

        // Parse le contenu HTML de la page film-grab
        let document_shot = Html::parse_document(&body_shot);

        // Définir le sélecteur pour cibler les éléments img
        let img_selector = Selector::parse("img[data-original]").unwrap();

        // Sélectionner tous les éléments img correspondants
        let img_elements = document_shot.select(&img_selector);

        // Créer un dossie Gallery qui regroupera le dossier de chaque film indépendamment
        let gallery_dir = Path::new("Gallery");
        fs::create_dir_all(&gallery_dir)?;

        // Créer un dossier a l'intérieur du dossier Galery pour chaque film
        let film_dir = gallery_dir.join(film_name_fixed);
        fs::create_dir_all(&film_dir)?;



        // Télécharger chaque image
        for (img_index, img_element) in img_elements.enumerate() {
            // Extraire l'URL de l'image
            if let Some(img_src) = img_element.value().attr("data-original") {
                // Télécharger l'image
                let mut image_response = client.get(img_src).send()?;
                let mut image_data = Vec::new();
                image_response.read_to_end(&mut image_data)?;

                // Écrire l'image dans un fichier
                let img_path = film_dir.join(format!("{}_{}.jpg", film_name_fixed, img_index + 1));
                let mut img_file = File::create(&img_path)?;
                img_file.write_all(&image_data)?;
                println!("Image téléchargée: {:?}", img_path);
            }
        }
    }

    println!("Toutes les images ont été téléchargées avec succès.");

    Ok(())
}
