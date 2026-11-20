//! Client graphique.
//!
//! Il reçoit l'adresse IP et le port du serveur, ouvre une fenêtre Piston,
//! demande régulièrement la liste des carrés, et prévient le serveur
//! quand son propre carré bouge.
//!
//! Lancer : `cargo run --bin client -- 127.0.0.1 1234`
//! Flèches : déplacer son carré. Échap : quitter.
//! Le carré entouré de blanc est le sien.

use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use glutin_window::GlutinWindow as Window;
use graphics::Transformed;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

use mini_jeu_serveur::{
    borner, lire_etat, lire_hello, message_move, Carre, HAUTEUR, LARGEUR, TAILLE,
};

const VITESSE: f64 = 220.0;
/// Le client redemande toutes les positions dix fois par seconde.
const PERIODE_DEMANDE: f64 = 0.1;

struct Joueur {
    id: usize,
    x: f64,
    y: f64,
    couleur: [u8; 3],
    haut: bool,
    bas: bool,
    gauche: bool,
    droite: bool,
    x_envoye: f64,
    y_envoye: f64,
    attente_demande: f64,
}

fn main() {
    let adresse = lire_adresse();
    let (ecriture, carres, joueur) = connecter(&adresse);
    fenetre(ecriture, carres, joueur);
}

fn lire_adresse() -> String {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 2 || args[1].parse::<u16>().is_err() {
        eprintln!("usage : client <adresse IP> <port>");
        eprintln!("exemple : client 127.0.0.1 1234");
        process::exit(1);
    }
    format!("{}:{}", args[0], args[1])
}

/// Se connecte, lit le carré qui nous est attribué, puis lance un thread
/// qui reçoit les listes de positions pendant que la fenêtre tourne.
fn connecter(adresse: &str) -> (BufWriter<TcpStream>, Arc<Mutex<Vec<Carre>>>, Joueur) {
    let flux = TcpStream::connect(adresse).unwrap_or_else(|_| {
        eprintln!("impossible de joindre le serveur à {adresse}");
        eprintln!("le serveur est-il lancé ?  cargo run --bin server");
        process::exit(1);
    });
    let _ = flux.set_nodelay(true);
    flux.set_read_timeout(Some(Duration::from_secs(5)))
        .expect("délai de lecture");

    // Un flux pour lire, un flux pour écrire : ce sont les deux sens de la même connexion.
    let mut lecture = BufReader::new(flux.try_clone().expect("clonage du flux"));
    let mut ecriture = BufWriter::new(flux);

    let mut ligne = String::new();
    if lecture.read_line(&mut ligne).unwrap_or(0) == 0 {
        eprintln!("le serveur a fermé la connexion");
        process::exit(1);
    }
    let hello = lire_hello(ligne.trim()).unwrap_or_else(|| {
        eprintln!("message inattendu du serveur : {}", ligne.trim());
        process::exit(1);
    });

    // Première demande, avant d'ouvrir la fenêtre : on veut déjà voir les autres.
    envoyer(&mut ecriture, "GET\n");
    ligne.clear();
    lecture.read_line(&mut ligne).ok();
    let deja_la = lire_etat(ligne.trim()).unwrap_or_default();
    // Le délai ne servait qu'à la poignée de main. Ensuite on attend les vraies réponses.
    lecture
        .get_mut()
        .set_read_timeout(None)
        .expect("délai de lecture");

    // Ensuite le thread se bloque sur read_line, et la fenêtre reste libre.
    let carres = Arc::new(Mutex::new(deja_la));
    let carres_thread = Arc::clone(&carres);
    thread::spawn(move || {
        let mut ligne = String::new();
        loop {
            ligne.clear();
            match lecture.read_line(&mut ligne) {
                Ok(0) | Err(_) => break,
                Ok(_) => {
                    if let Some(liste) = lire_etat(ligne.trim()) {
                        *carres_thread.lock().unwrap() = liste;
                    }
                }
            }
        }
    });

    let joueur = Joueur {
        id: hello.id,
        x: hello.x,
        y: hello.y,
        couleur: hello.couleur,
        haut: false,
        bas: false,
        gauche: false,
        droite: false,
        x_envoye: hello.x,
        y_envoye: hello.y,
        attente_demande: 0.0,
    };
    println!("Connecté, je suis le carré {}", joueur.id);
    (ecriture, carres, joueur)
}

fn envoyer(ecriture: &mut BufWriter<TcpStream>, message: &str) -> bool {
    if ecriture.write_all(message.as_bytes()).is_err() || ecriture.flush().is_err() {
        return false;
    }
    true
}

fn fenetre(mut ecriture: BufWriter<TcpStream>, carres: Arc<Mutex<Vec<Carre>>>, mut joueur: Joueur) {
    let opengl = OpenGL::V3_2;
    let titre = format!("carrés - joueur {}", joueur.id);
    let mut window: Window = WindowSettings::new(titre, [LARGEUR as u32, HAUTEUR as u32])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .expect("impossible de créer la fenêtre");

    let mut gl = GlGraphics::new(opengl);
    let mut reseau_ok = true;
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Keyboard(touche)) = e.press_args() {
            appliquer_touche(&mut joueur, touche, true);
        }
        if let Some(Button::Keyboard(touche)) = e.release_args() {
            appliquer_touche(&mut joueur, touche, false);
        }
        if let Some(args) = e.update_args() {
            if reseau_ok {
                reseau_ok = mettre_a_jour(&mut joueur, args.dt, &mut ecriture);
                if !reseau_ok {
                    eprintln!("connexion avec le serveur perdue");
                }
            } else {
                deplacer(&mut joueur, args.dt);
            }
        }
        if let Some(args) = e.render_args() {
            let liste = carres.lock().unwrap().clone();
            dessiner(&mut gl, &args, &vue(&liste, &joueur), joueur.id);
        }
    }
}

fn appliquer_touche(joueur: &mut Joueur, touche: Key, appuyee: bool) {
    match touche {
        Key::Up => joueur.haut = appuyee,
        Key::Down => joueur.bas = appuyee,
        Key::Left => joueur.gauche = appuyee,
        Key::Right => joueur.droite = appuyee,
        _ => {}
    }
}

fn deplacer(joueur: &mut Joueur, dt: f64) {
    let mut dx = 0.0;
    let mut dy = 0.0;
    if joueur.gauche {
        dx -= VITESSE * dt;
    }
    if joueur.droite {
        dx += VITESSE * dt;
    }
    if joueur.haut {
        dy -= VITESSE * dt;
    }
    if joueur.bas {
        dy += VITESSE * dt;
    }
    let (x, y) = borner(joueur.x + dx, joueur.y + dy);
    joueur.x = x;
    joueur.y = y;
}

/// Bouge le carré local, prévient le serveur, et redemande toute la liste de temps en temps.
fn mettre_a_jour(joueur: &mut Joueur, dt: f64, ecriture: &mut BufWriter<TcpStream>) -> bool {
    deplacer(joueur, dt);

    let a_bouge =
        (joueur.x - joueur.x_envoye).abs() > 0.01 || (joueur.y - joueur.y_envoye).abs() > 0.01;
    if a_bouge && !envoyer(ecriture, &message_move(joueur.x, joueur.y)) {
        return false;
    }
    if a_bouge {
        joueur.x_envoye = joueur.x;
        joueur.y_envoye = joueur.y;
    }

    joueur.attente_demande += dt;
    if joueur.attente_demande >= PERIODE_DEMANDE {
        joueur.attente_demande = 0.0;
        if !envoyer(ecriture, "GET\n") {
            return false;
        }
    }
    true
}

/// Les autres carrés viennent du serveur. Le nôtre utilise la position locale,
/// pour ne pas attendre le prochain aller-retour réseau.
fn vue(distants: &[Carre], joueur: &Joueur) -> Vec<Carre> {
    let mut liste: Vec<Carre> = distants
        .iter()
        .filter(|carre| carre.id != joueur.id)
        .cloned()
        .collect();
    liste.push(Carre {
        id: joueur.id,
        x: joueur.x,
        y: joueur.y,
        couleur: joueur.couleur,
    });
    liste.sort_by_key(|carre| carre.id);
    liste
}

fn dessiner(gl: &mut GlGraphics, args: &RenderArgs, carres: &[Carre], moi: usize) {
    use graphics::{clear, rectangle};

    gl.draw(args.viewport(), |c, gl| {
        clear([0.12, 0.12, 0.16, 1.0], gl);

        for carre in carres {
            let transform = c.transform.trans(carre.x, carre.y);
            if carre.id == moi {
                rectangle(
                    [1.0, 1.0, 1.0, 1.0],
                    rectangle::square(-4.0, -4.0, TAILLE + 8.0),
                    transform,
                    gl,
                );
            }
            let couleur = [
                carre.couleur[0] as f32 / 255.0,
                carre.couleur[1] as f32 / 255.0,
                carre.couleur[2] as f32 / 255.0,
                1.0,
            ];
            rectangle(couleur, rectangle::square(0.0, 0.0, TAILLE), transform, gl);
        }
    });
}
