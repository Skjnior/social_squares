//! Langage commun entre le serveur et le client.
//!
//! Les deux programmes échangent des lignes de texte :
//!
//! - `HELLO <id> <x> <y> <r> <g> <b>` : le serveur présente son carré au client
//! - `MOVE <x> <y>` : le client annonce sa nouvelle position
//! - `GET` : le client demande la position de tous les carrés
//! - `STATE <id> <x> <y> <r> <g> <b> ...` : le serveur répond avec toute la liste

/// Taille de la zone de jeu, identique chez tout le monde.
pub const LARGEUR: f64 = 800.0;
pub const HAUTEUR: f64 = 600.0;
pub const TAILLE: f64 = 48.0;

/// Couleurs partagées, en rouge / vert / bleu de 0 à 255.
/// L'indice du carré choisit une couleur, puis la palette recommence.
pub const COULEURS: [[u8; 3]; 6] = [
    [242, 89, 89],
    [89, 191, 115],
    [89, 140, 242],
    [242, 191, 64],
    [191, 115, 217],
    [64, 217, 217],
];

/// Un carré connu du serveur : identifiant, position, couleur.
#[derive(Clone, Debug)]
pub struct Carre {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub couleur: [u8; 3],
}

/// Garde le carré à l'intérieur de la fenêtre.
pub fn borner(x: f64, y: f64) -> (f64, f64) {
    let max_x = LARGEUR - TAILLE;
    let max_y = HAUTEUR - TAILLE;
    (x.clamp(0.0, max_x), y.clamp(0.0, max_y))
}

/// Place le carré numéro `index` sur une grille, pour qu'ils ne naissent pas tous au même endroit.
pub fn position_initiale(index: usize) -> (f64, f64) {
    let colonne = index % 6;
    let ligne = index / 6;
    borner(80.0 + colonne as f64 * 110.0, 80.0 + ligne as f64 * 100.0)
}

pub fn message_hello(carre: &Carre) -> String {
    format!(
        "HELLO {} {} {} {} {} {}\n",
        carre.id, carre.x, carre.y, carre.couleur[0], carre.couleur[1], carre.couleur[2]
    )
}

pub fn message_move(x: f64, y: f64) -> String {
    format!("MOVE {x} {y}\n")
}

pub fn message_etat(carres: &[Carre]) -> String {
    let mut message = String::from("STATE");
    for carre in carres {
        message.push_str(&format!(
            " {} {} {} {} {} {}",
            carre.id, carre.x, carre.y, carre.couleur[0], carre.couleur[1], carre.couleur[2]
        ));
    }
    message.push('\n');
    message
}

pub fn lire_hello(ligne: &str) -> Option<Carre> {
    let mots: Vec<&str> = ligne.split_whitespace().collect();
    if mots.len() != 7 || mots[0] != "HELLO" {
        return None;
    }
    Some(Carre {
        id: mots[1].parse().ok()?,
        x: mots[2].parse().ok()?,
        y: mots[3].parse().ok()?,
        couleur: [
            mots[4].parse().ok()?,
            mots[5].parse().ok()?,
            mots[6].parse().ok()?,
        ],
    })
}

/// Lit une réponse `STATE`. `None` si la ligne n'a pas la forme attendue.
pub fn lire_etat(ligne: &str) -> Option<Vec<Carre>> {
    let mots: Vec<&str> = ligne.split_whitespace().collect();
    if mots.first().copied()? != "STATE" {
        return None;
    }
    let reste = &mots[1..];
    if reste.len() % 6 != 0 {
        return None;
    }

    let mut carres = Vec::new();
    for bloc in reste.chunks(6) {
        carres.push(Carre {
            id: bloc[0].parse().ok()?,
            x: bloc[1].parse().ok()?,
            y: bloc[2].parse().ok()?,
            couleur: [
                bloc[3].parse().ok()?,
                bloc[4].parse().ok()?,
                bloc[5].parse().ok()?,
            ],
        });
    }
    Some(carres)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exemple() -> Carre {
        Carre {
            id: 1,
            x: 80.5,
            y: 190.0,
            couleur: [9, 8, 7],
        }
    }

    #[test]
    fn hello_aller_retour() {
        let carre = exemple();
        let relu = lire_hello(message_hello(&carre).trim()).expect("HELLO");
        assert_eq!(relu.id, carre.id);
        assert_eq!(relu.x, carre.x);
        assert_eq!(relu.y, carre.y);
        assert_eq!(relu.couleur, carre.couleur);
    }

    #[test]
    fn etat_aller_retour() {
        let carres = vec![
            Carre {
                id: 0,
                x: 1.5,
                y: 2.5,
                couleur: [1, 2, 3],
            },
            exemple(),
        ];
        let relu = lire_etat(message_etat(&carres).trim()).expect("STATE");
        assert_eq!(relu.len(), 2);
        assert_eq!(relu[0].id, 0);
        assert_eq!(relu[1].id, 1);
        assert_eq!(relu[1].y, 190.0);
    }

    #[test]
    fn etat_vide() {
        let relu = lire_etat("STATE").expect("STATE");
        assert!(relu.is_empty());
    }
}
