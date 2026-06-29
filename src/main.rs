use std::io;

//  Enums

// Le genre d'un livre.
enum Genre {
    Roman,
    Science,
    Informatique,
    Histoire,
    Autre,
}

// Le statut d'un livre dans la bibliothèque.
enum Statut {
    Disponible,
    Emprunte,
}

//  Trait

trait Affichable {
    fn afficher(&self);
}

//  Structure

struct Livre {
    titre: String,
    auteur: String,
    annee: u32,
    pages: u32,
    genre: Genre,
    statut: Statut,
}

// Implémentation du trait Affichable pour le type Livre.
impl Affichable for Livre {
    fn afficher(&self) {
        // On traduit le genre en texte lisible avec un match.
        let genre_texte = match self.genre {
            Genre::Roman => "Roman",
            Genre::Science => "Science",
            Genre::Informatique => "Informatique",
            Genre::Histoire => "Histoire",
            Genre::Autre => "Autre",
        };

        // statut
        let statut_texte = match self.statut {
            Statut::Disponible => "Disponible",
            Statut::Emprunte => "Emprunté",
        };

        println!("Titre : {}", self.titre);
        println!("Auteur : {}", self.auteur);
        println!("Année : {}", self.annee);
        println!("Pages : {}", self.pages);
        println!("Genre : {}", genre_texte);
        println!("Statut : {}", statut_texte);
    }
}

//  Lecture des entrées utilisateur

fn lire_entree() -> String {
    let mut entree = String::new();
    io::stdin()
        .read_line(&mut entree)
        .expect("Erreur lors de la lecture de la ligne");

    entree.trim().to_string()
}

fn lire_nombre(message: &str) -> u32 {
    loop {
        println!("{}", message);

        let entree = lire_entree();

        match entree.parse::<u32>() {
            Ok(valeur) => return valeur,
            Err(_) => println!("Ce n'est pas un choix valide !"),
        }
    }
}

//  Menu et fonctionnalités

fn afficher_menu() {
    println!("");
    println!("============= Gestionnaire de bibliothèque ===");
    println!("");
    println!("1. Afficher tous les livres");
    println!("2. Ajouter un livre");
    println!("3. Rechercher un livre par titre");
    println!("4. Modifier le statut d'un livre");
    println!("5. Afficher les statistiques");
    println!("6. Quitter");
    println!("");
}

// Affiche tous les livres de la collection
fn afficher_tous_les_livres(livres: &Vec<Livre>) {
    if livres.is_empty() {
        println!("Aucun livre dans la collection.");
        return;
    }

    println!("----- Liste des livres ---");

    for (index, livre) in livres.iter().enumerate() {
        println!("");
        println!("Livre numéro {}", index + 1);
        livre.afficher();
    }
}

// choisir un genre
fn choisir_genre() -> Genre {
    println!("Choisissez un genre :");
    println!("1. Roman");
    println!("2. Science");
    println!("3. Informatique");
    println!("4. Histoire");
    println!("5. Autre");

    let choix = lire_entree();

    match choix.as_str() {
        "1" => Genre::Roman,
        "2" => Genre::Science,
        "3" => Genre::Informatique,
        "4" => Genre::Histoire,
        _ => Genre::Autre,
    }
}

//  choisir un statut
fn choisir_statut() -> Statut {
    println!("Choisissez un statut :");
    println!("1. Disponible");
    println!("2. Emprunté");

    let choix = lire_entree();

    match choix.as_str() {
        "2" => Statut::Emprunte,
        _ => Statut::Disponible,
    }
}

// Ajoute un livre
fn ajouter_livre(livres: &mut Vec<Livre>) {
    println!("Titre du livre :");
    let titre = lire_entree();

    println!("Auteur du livre :");
    let auteur = lire_entree();

    let annee = lire_nombre("Année de publication :");
    let pages = lire_nombre("Nombre de pages :");

    let genre = choisir_genre();
    let statut = choisir_statut();

    let livre = Livre {
        titre,
        auteur,
        annee,
        pages,
        genre,
        statut,
    };

    livres.push(livre);

    println!("Livre ajouté à la collection.");
}

// Recherche un livre à partir de son titre
fn rechercher_livre(livres: &Vec<Livre>) {
    println!("Entrez un mot du titre à rechercher :");

    let recherche = lire_entree();
    let recherche = recherche.to_lowercase();

    let mut trouve = false;

    for livre in livres {
        let titre_minuscule = livre.titre.to_lowercase();

        if titre_minuscule.contains(&recherche) {
            println!("");
            livre.afficher();
            trouve = true;
        }
    }

    if !trouve {
        println!("Aucun livre ne correspond à cette recherche.");
    }
}

// Modifie le statut d'un livre
fn modifier_statut(livres: &mut Vec<Livre>) {
    if livres.is_empty() {
        println!("Aucun livre à modifier.");
        return;
    }

    afficher_tous_les_livres(livres);

    let numero = lire_nombre("Numéro du livre à changer :");

    if numero == 0 || numero > livres.len() as u32 {
        println!("Numéro invalide.");
        return;
    }

    let index = (numero - 1) as usize;
    let livre = &mut livres[index];

    // inverse le statut avec un match
    livre.statut = match livre.statut {
        Statut::Disponible => Statut::Emprunte,
        Statut::Emprunte => Statut::Disponible,
    };

    println!("Statut modifié.");
}

//  Statistiques

// Retourne un tuple
fn calculer_statistiques(livres: &Vec<Livre>) -> (usize, u32, f64) {
    let nombre_livres = livres.len();

    let mut total_pages = 0;

    for livre in livres {
        total_pages += livre.pages;
    }

    // non division par zéro
    let moyenne = if nombre_livres == 0 {
        0.0
    } else {
        total_pages as f64 / nombre_livres as f64
    };

    (nombre_livres, total_pages, moyenne)
}

// statistiques de la collection.
fn afficher_statistiques(livres: &Vec<Livre>) {
    let (nombre_livres, total_pages, moyenne) = calculer_statistiques(livres);

    let mut disponibles = 0;
    let mut empruntes = 0;

    for livre in livres {
        match livre.statut {
            Statut::Disponible => disponibles += 1,
            Statut::Emprunte => empruntes += 1,
        }
    }

    println!("--- Statistiques ---");
    println!("Nombre total de livres : {}", nombre_livres);
    println!("Nombre total de pages : {}", total_pages);
    println!("Moyenne de pages par livre : {:.1}", moyenne);
    println!("Livres disponibles : {}", disponibles);
    println!("Livres empruntés : {}", empruntes) ;
}

//  Données de départ

// collection initiale
fn livres_de_depart() -> Vec<Livre> {
    let mut livres: Vec<Livre> = Vec::new();

    livres.push(Livre {
        titre: String::from("Courage : The Joy of Living Dangerously"),
        auteur: String::from("Osho"),
        annee: 1943,
        pages: 96,
        genre: Genre::Roman,
        statut: Statut::Disponible,
    });

    livres.push(Livre {
        titre: String::from("Programmation Rust"),
        auteur: String::from("Steve Klabnik"),
        annee: 2019,
        pages: 526,
        genre: Genre::Informatique,
        statut: Statut::Emprunte,
    });

    livres.push(Livre {
        titre: String::from("Sapiens"),
        auteur: String::from("Yuval Noah Harari"),
        annee: 2011,
        pages: 443,
        genre: Genre::Histoire,
        statut: Statut::Disponible,
    });

    livres.push(Livre {
        titre: String::from("Tao: The Pathless Path"),
        auteur: String::from("Osho"),
        annee: 2002,
        pages: 192,
        genre: Genre::Autre,
        statut: Statut::Disponible,
    });

    livres
}

fn main() {
    // La collection
    let mut livres = livres_de_depart();

    loop {
        afficher_menu();

        let choix = lire_entree();

        match choix.as_str() {
            "1" => afficher_tous_les_livres(&livres),
            "2" => ajouter_livre(&mut livres),
            "3" => rechercher_livre(&livres),
            "4" => modifier_statut(&mut livres),
            "5" => afficher_statistiques(&livres),
            "6" => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide."),
        }
    }
}