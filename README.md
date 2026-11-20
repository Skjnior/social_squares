# Projet « mini_jeu_serveur » - Programmation bas niveau

**Cours :** Programmation bas niveau (UPEC), Master 1 Logiciels sûrs  
**Séance :** 14 - mise en réseau du programme de carrés  
**Outil :** Rust, TCP (`std::net`) et [Piston](https://www.piston.rs/) pour la fenêtre  
**Programmes :** `server` et `client`

Le détail de **chaque ligne** de `Cargo.toml`, `src/lib.rs`, `src/server.rs` et `src/client.rs` est dans [EXPLICATIONS.md](EXPLICATIONS.md). Les numéros de ligne de ce fichier correspondent au code.

## 1. Objectif

Le résultat demandé est **deux programmes**.

Le **serveur** ne dessine rien. Il garde un ensemble de positions de carrés, vide au démarrage. Il attend des connexions. À chaque client :

1. il crée un carré de plus ;
2. il lance un **thread** qui reste en contact avec ce client.

Ce thread reçoit la nouvelle position quand le carré du client bouge, et répond quand le client demande la position de **tous** les carrés.

Le **client** reçoit l’adresse IP et le port du serveur. Il ouvre une fenêtre Piston, demande la liste des carrés déjà présents et les affiche. Quand son propre carré bouge, il prévient le serveur.

Chaque fenêtre est un joueur. Trois joueurs, ce sont trois terminaux qui lancent `client`, plus le terminal du serveur.

Le serveur de cette version écoute sur **127.0.0.1** et le port **1234**, écrits en dur, comme dans l’exemple du cours. Le client, lui, reçoit l’adresse et le port en arguments.

## 2. Prérequis

- [Rust](https://www.rust-lang.org/tools/install), avec `cargo`
- Sous Linux : de quoi ouvrir une fenêtre OpenGL (Glutin, Wayland ou X11). C’est en général déjà installé sur une machine de bureau.

## 3. Lancer le projet

Ouvre **un terminal par programme**. Le serveur d’abord, sinon le client s’arrête tout de suite avec « impossible de joindre le serveur ».

Terminal 1, le serveur :

```bash
cd "/home/junior/Documents/UPEC/COURS/PG bas Niveau/Mes QCM de Revision/projet/mini_jeu_serveur"
cargo run --bin server
```

Le terminal reste occupé et affiche :

```text
Serveur en écoute sur 127.0.0.1:1234
```

Terminal 2, le premier joueur :

```bash
cd "/home/junior/Documents/UPEC/COURS/PG bas Niveau/Mes QCM de Revision/projet/mini_jeu_serveur"
cargo run --bin client -- 127.0.0.1 1234
```

Répète la même commande dans un terminal 3, puis un terminal 4, pour avoir trois fenêtres. Le serveur écrit alors `Client 0 connecté`, `Client 1 connecté`, `Client 2 connecté`.

Compilation seule, sans lancer :

```bash
cargo build --release
cargo run --release --bin server
cargo run --release --bin client -- 127.0.0.1 1234
```

Vérifier seulement le format des messages :

```bash
cargo test
```

### Commandes au clavier

| Touche | Effet |
| --- | --- |
| Flèches | Déplacer **son** carré. Il faut d’abord cliquer la fenêtre. |
| Échap | Fermer cette fenêtre. Le carré disparaît côté serveur. |
| Ctrl+C dans le terminal du serveur | Arrêter le serveur. Les clients affichent « connexion avec le serveur perdue ». |

Le carré entouré de blanc est celui de cette fenêtre. Les autres carrés sont ceux des autres clients. Chaque joueur a une couleur fixe, choisie par le serveur.

### Si ça ne démarre pas

- `impossible d'écouter sur 127.0.0.1:1234` : un serveur tourne déjà. Un seul suffit. Arrête l’ancien avec Ctrl+C dans son terminal.
- `impossible de joindre le serveur` : le serveur n’est pas lancé, ou l’adresse / le port ne sont pas `127.0.0.1` et `1234`.
- `usage : client <adresse IP> <port>` : il manque un des deux arguments.
- La fenêtre ne réagit pas aux flèches : clique dedans. Une seule fenêtre reçoit le clavier à la fois.

## 4. Ce qui se passe entre les programmes

```text
client 0                         serveur                         client 1
fenêtre Piston                   pas de fenêtre                  fenêtre Piston
    |                                |                                |
    |-------- connexion TCP -------->|                                |
    |<----------- HELLO 0 -----------|                                |
    |------------- GET ------------->|                                |
    |<------------ STATE ------------|                                |
    |                                |<-------- connexion TCP --------|
    |                                |----------- HELLO 1 ----------->|
    |                                |<------------- GET -------------|
    |                                |------------ STATE ------------>|
    |                                |                                |
    |  flèche : le carré bouge       |                                |
    |------------- MOVE ------------>|  met à jour le carré 0         |
    |------------- GET ------------->|                                |
    |<------------ STATE ------------|                                |
    |                                |<------------- GET -------------|
    |                                |------------ STATE ------------>|
    |                                |     le carré 0 a bougé ici     |
```

Le client demande la liste environ **10 fois par seconde** (`GET`). Il n’envoie `MOVE` que si sa position a vraiment changé.

Exemple de messages, une ligne chacun :

```text
HELLO 0 80 80 242 89 89
MOVE 150.5 90
GET
STATE 0 150.5 90 242 89 89 1 190 80 89 191 115
```

| Message | Qui l’envoie | Contenu |
| --- | --- | --- |
| `HELLO id x y r g b` | serveur, à la connexion | le carré qui vient d’être créé pour ce client |
| `MOVE x y` | client | nouvelle position de **son** carré |
| `GET` | client | « envoie-moi tout le monde » |
| `STATE` puis des groupes `id x y r g b` | serveur, réponse à `GET` | toutes les positions connues |

`r g b` est la couleur, trois entiers de 0 à 255. `STATE` seul veut dire « personne n’est connecté ».

## 5. Fichiers

```text
mini_jeu_serveur/
├── Cargo.toml          deux binaires : server et client
├── src/lib.rs          le format des messages, partagé par les deux
├── src/server.rs       écoute, threads, positions
└── src/client.rs       fenêtre, clavier, envoi et réception
```

`Cargo.toml` déclare les deux programmes avec `[[bin]]`. La bibliothèque `mini_jeu_serveur` (`src/lib.rs`) n’est pas un troisième programme : c’est le code commun, pour que le client et le serveur écrivent exactement les mêmes lignes.

Piston n’est utilisé que par le client. Le serveur ne dépend que de la bibliothèque standard.

## 6. `src/lib.rs` - le langage commun

`Carre` est une position connue des deux côtés : numéro, `x`, `y`, couleur.

`LARGEUR` (800), `HAUTEUR` (600) et `TAILLE` (48) décrivent la même zone chez tout le monde. `borner` empêche un carré de sortir de cette zone. Le serveur l’applique aussi quand il reçoit un `MOVE`, pour ne pas enregistrer une position hors fenêtre.

`position_initiale` pose le carré numéro `index` sur une grille de 6 colonnes. `Monde::position_libre` dans le serveur cherche la première case de cette grille où personne ne se trouve, pour éviter d’empiler deux carrés au même endroit.

Les fonctions `message_*` fabriquent une ligne prête à envoyer, avec `\n` à la fin. `lire_hello` et `lire_etat` font le chemin inverse. Elles rendent `None` si la ligne n’a pas la forme attendue : un mauvais message est ignoré, il ne fait pas tomber le programme.

Les trois tests en bas de fichier vérifient qu’une ligne écrite puis relue redonne le même carré. `cargo test` les lance.

## 7. `src/server.rs` - un thread par client

C’est le schéma du chapitre 21.2 du Rust Book : `TcpListener::bind`, puis `for connexion in ecoute.incoming()`, puis `thread::spawn` pour traiter cette connexion sans bloquer l’attente des suivantes.

```text
main
  ├── Monde vide, partagé avec Arc<Mutex<Monde>>
  ├── bind 127.0.0.1:1234
  └── pour chaque connexion
        ├── ajouter un carré (id, position libre, couleur)
        └── thread gerer_client
              ├── HELLO
              └── boucle : MOVE ou GET, jusqu’à la déconnexion
```

`Monde` contient `prochain_id` et `carres`. L’identifiant ne recule jamais : si le client 0 part et qu’un nouveau arrive, il reçoit 1, 2, 3… et non pas de nouveau 0. La couleur est `COULEURS[id % 6]`, donc la palette de six couleurs recommence au septième joueur.

Pourquoi `Arc<Mutex<Monde>>` :

- chaque thread doit pouvoir **garder** le monde après la fin de `main` qui a lancé le `spawn` : `Arc` partage la propriété, avec un compteur ;
- `Mutex` fait attendre les autres threads pendant qu’un thread lit ou modifie la liste. `lock()` prend le verrou, il se relâche à la fin du bloc ;
- `Arc::clone` avant `thread::spawn` donne au thread sa propre copie du pointeur. Le `move` de la closure emporte cette copie, pas le `Arc` du `main`.

`gerer_client` appelle `dialogue`, puis retire le carré avec `retain` quoi qu’il arrive. Fermer la fenêtre retire donc le joueur de la liste. Les autres le voient disparaître à leur prochain `GET`.

`dialogue` coupe la connexion en deux avec `try_clone` : un `BufReader` lit des lignes, un `BufWriter` écrit. `set_nodelay(true)` envoie les petits messages tout de suite, sans attendre d’en regrouper plusieurs dans un paquet TCP. `flush` après chaque écriture vide le tampon, sinon `GET` pourrait rester coincé dans le `BufWriter` et le client attendrait la réponse.

`read_line` qui renvoie 0 signifie que le client a fermé. La boucle s’arrête proprement.

`traiter_ligne` ne connaît que deux messages :

- `MOVE x y` : met à jour **uniquement** le carré de ce thread, après `borner` ;
- `GET` : construit la ligne `STATE` de tous les carrés.

Le verrou est relâché **avant** `write_all`. On ne garde pas `Mutex` pendant l’envoi réseau : un client lent ne bloque pas les autres threads plus longtemps que la copie de la liste.

Une ligne inconnue, ou un `MOVE` mal formé, est ignorée. Le thread continue.

## 8. `src/client.rs` - la fenêtre et le réseau

```text
main
  ├── lire_adresse          argv : IP puis port
  ├── connecter
  │     ├── TcpStream::connect
  │     ├── lire HELLO
  │     ├── envoyer GET, lire le premier STATE
  │     └── thread de lecture des STATE suivants
  └── fenetre               boucle Piston
        ├── press / release → flèches
        ├── update          → déplacer, MOVE, GET
        └── render          → dessiner
```

`lire_adresse` exige exactement deux arguments, et un port qui tient dans un `u16`. L’exemple du cours est `127.0.0.1 1234`.

`connecter` fait la poignée de main **avant** d’ouvrir la fenêtre, pour connaître son numéro et voir les joueurs déjà là. Le délai de lecture de 5 secondes ne sert que pour ce début : si le serveur ne répond pas, on n’attend pas indéfiniment. Ensuite `set_read_timeout(None)` : le thread de lecture a le droit de rester bloqué sur `read_line` jusqu’au prochain `STATE`.

Ce thread existe parce que la boucle Piston ne doit pas s’arrêter pour attendre le réseau. Il ne fait qu’une chose : lire une ligne, et si c’est un `STATE`, remplacer la liste partagée. La fenêtre, de son côté, est la seule à écrire sur le socket. Lire d’un côté et écrire de l’autre est possible parce que `try_clone` a donné deux accès à la même connexion.

La liste reçue est un `Arc<Mutex<Vec<Carre>>>`, le même outil que sur le serveur : le thread réseau écrit, le dessin lit.

`Joueur` est l’état local. Les quatre booléens `haut`, `bas`, `gauche`, `droite` valent vrai tant que la flèche est enfoncée. `appliquer_touche` les met à jour. `deplacer` ajoute `VITESSE * dt` (220 pixels par seconde, multipliés par le temps écoulé depuis la frame précédente, fourni par Piston) puis appelle `borner`.

`mettre_a_jour` envoie `MOVE` seulement si `x` ou `y` s’éloigne de plus de 0,01 pixel de la dernière position envoyée. Toutes les 0,1 s (`PERIODE_DEMANDE`), il envoie `GET`. Si un envoi échoue, la fonction rend `false` : la fenêtre affiche « connexion avec le serveur perdue » et continue à dessiner, sans réessayer.

`vue` construit ce qu’il faut dessiner. Les autres carrés viennent du dernier `STATE`. Le carré local utilise `joueur.x` et `joueur.y`, pas la position renvoyée par le serveur. Sinon le déplacement attendrait l’aller-retour réseau et le carré semblerait en retard sur les flèches.

`dessiner` efface la fenêtre en gris sombre, trace un rectangle blanc un peu plus grand derrière **son** carré, puis le carré coloré. La couleur 0–255 est divisée par 255 pour Piston, qui attend des composantes entre 0 et 1. `exit_on_esc(true)` ferme la fenêtre sur Échap, ce qui ferme le socket, ce qui fait sortir le thread du serveur de `read_line`.

## 9. Correspondance avec l’énoncé

| Demande | Où c’est fait |
| --- | --- |
| Ensemble de positions, vide au départ | `Monde { carres: Vec::new() }` dans `main` du serveur |
| Le serveur n’affiche pas le jeu | `server.rs` n’ouvre pas de fenêtre. Il écrit seulement « connecté » / « parti » dans le terminal |
| Attendre les connexions | `ecoute.incoming()` |
| Un carré de plus à chaque connexion | `Monde::ajouter` |
| Un thread par client | `thread::spawn` dans la boucle du serveur |
| Le client indique que son carré bouge | `MOVE` dans `mettre_a_jour` |
| Le client redemande toutes les positions | `GET` toutes les 0,1 s, et une fois juste après `HELLO` |
| Le client reçoit IP et port | `lire_adresse` |
| Fenêtre Piston et affichage de tous les carrés | `fenetre` et `dessiner` |
| Adresse et port d’exemple | `127.0.0.1:1234` en dur dans `TcpListener::bind` |

## 10. Limites de cette version

Le cours demandait de ne pas traiter toutes les erreurs dès la première version. Ici, une ligne illisible est ignorée, et une coupure ferme le thread du client. Il n’y a pas de reconnexion automatique : on relance `client`.

Le serveur n’accepte le jeu que sur la machine locale (`127.0.0.1`). Pour jouer depuis une autre machine, il faudrait changer l’adresse dans `TcpListener::bind` et donner au client l’adresse IP de la machine du serveur.

Au-delà de six joueurs, les couleurs recommencent. Les cases de départ sont prévues pour une trentaine de carrés ; ensuite `position_initiale` peut reposer un nouveau carré sur le bord de la fenêtre.
