//! Serveur des positions de carrés.
//!
//! Il n'ouvre pas de fenêtre. Il écoute sur 127.0.0.1:1234 (codé en dur,
//! comme dans l'exemple du cours). À chaque client :
//! 1. on ajoute un carré dans l'ensemble partagé ;
//! 2. on lance un thread qui reste en contact avec ce client.
//!
//! Lancer : `cargo run --bin server`

use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use mini_jeu_serveur::{borner, message_etat, message_hello, position_initiale, Carre, COULEURS};

/// L'ensemble des carrés. `Arc` le partage entre les threads,
/// `Mutex` garantit qu'un seul thread le modifie à la fois.
struct Monde {
    prochain_id: usize,
    carres: Vec<Carre>,
}

impl Monde {
    fn ajouter(&mut self) -> usize {
        let id = self.prochain_id;
        self.prochain_id += 1;
        let (x, y) = self.position_libre();
        let carre = Carre {
            id,
            x,
            y,
            couleur: COULEURS[id % COULEURS.len()],
        };
        self.carres.push(carre);
        id
    }

    /// Première case de la grille où personne ne se trouve encore.
    /// Utile quand un client se reconnecte : on ne le pose pas sur un carré déjà là.
    fn position_libre(&self) -> (f64, f64) {
        for index in 0..30 {
            let (x, y) = position_initiale(index);
            let occupee = self
                .carres
                .iter()
                .any(|carre| (carre.x - x).abs() < 1.0 && (carre.y - y).abs() < 1.0);
            if !occupee {
                return (x, y);
            }
        }
        position_initiale(self.carres.len())
    }
}

fn main() {
    let monde = Arc::new(Mutex::new(Monde {
        prochain_id: 0,
        carres: Vec::new(),
    }));

    // Même schéma que le chapitre 21.2 : bind, puis une boucle sur incoming().
    let ecoute =
        TcpListener::bind("127.0.0.1:1234").expect("impossible d'écouter sur 127.0.0.1:1234");
    println!("Serveur en écoute sur 127.0.0.1:1234");

    for connexion in ecoute.incoming() {
        let connexion = match connexion {
            Ok(flux) => flux,
            Err(erreur) => {
                eprintln!("connexion ignorée : {erreur}");
                continue;
            }
        };

        let id = monde.lock().unwrap().ajouter();
        let monde = Arc::clone(&monde);

        thread::spawn(move || {
            gerer_client(connexion, monde, id);
        });
    }
}

/// Un thread = un client. On lit ses messages jusqu'à ce qu'il parte.
fn gerer_client(flux: TcpStream, monde: Arc<Mutex<Monde>>, id: usize) {
    println!("Client {id} connecté");
    if let Err(erreur) = dialogue(flux, &monde, id) {
        eprintln!("Client {id} : {erreur}");
    }
    monde.lock().unwrap().carres.retain(|carre| carre.id != id);
    println!("Client {id} parti");
}

fn dialogue(flux: TcpStream, monde: &Arc<Mutex<Monde>>, id: usize) -> std::io::Result<()> {
    // Les messages sont courts : on les envoie tout de suite, sans les regrouper.
    let _ = flux.set_nodelay(true);
    let mut lecture = BufReader::new(flux.try_clone()?);
    let mut ecriture = BufWriter::new(flux);

    let hello = {
        let monde = monde.lock().unwrap();
        let carre = monde
            .carres
            .iter()
            .find(|carre| carre.id == id)
            .expect("le carré vient d'être créé");
        message_hello(carre)
    };
    ecriture.write_all(hello.as_bytes())?;
    ecriture.flush()?;

    let mut ligne = String::new();
    loop {
        ligne.clear();
        // 0 octet : le client a fermé la connexion.
        if lecture.read_line(&mut ligne)? == 0 {
            return Ok(());
        }
        traiter_ligne(ligne.trim(), id, monde, &mut ecriture)?;
    }
}

fn traiter_ligne(
    ligne: &str,
    id: usize,
    monde: &Arc<Mutex<Monde>>,
    ecriture: &mut BufWriter<TcpStream>,
) -> std::io::Result<()> {
    if let Some(reste) = ligne.strip_prefix("MOVE ") {
        if let Some((x, y)) = lire_xy(reste) {
            let (x, y) = borner(x, y);
            let mut monde = monde.lock().unwrap();
            if let Some(carre) = monde.carres.iter_mut().find(|carre| carre.id == id) {
                carre.x = x;
                carre.y = y;
            }
        }
    } else if ligne == "GET" {
        // On copie le texte pendant le verrou, puis on écrit sur le réseau après.
        // Le verrou n'est pas gardé pendant l'envoi.
        let message = {
            let monde = monde.lock().unwrap();
            message_etat(&monde.carres)
        };
        ecriture.write_all(message.as_bytes())?;
        ecriture.flush()?;
    }
    Ok(())
}

fn lire_xy(texte: &str) -> Option<(f64, f64)> {
    let mut mots = texte.split_whitespace();
    let x = mots.next()?.parse().ok()?;
    let y = mots.next()?.parse().ok()?;
    Some((x, y))
}
