# Chaque ligne du projet

Ce fichier suit le code dans l’ordre, avec les mêmes numéros de ligne que les sources. Les lignes vides ne font que séparer les blocs : elles n’exécutent rien.

Le README explique comment lancer le projet. Ici, on lit le code.

---

## `Cargo.toml`

```1:19:mini_jeu_serveur/Cargo.toml
[package]
name = "mini_jeu_serveur"
version = "0.1.0"
edition = "2021"
description = "Carrés partagés en réseau : un serveur TCP et un client Piston"

[[bin]]
name = "server"
path = "src/server.rs"

[[bin]]
name = "client"
path = "src/client.rs"

[dependencies]
piston = "1.0.0"
piston2d-graphics = "0.45.0"
pistoncore-glutin_window = "0.73.1"
piston2d-opengl_graphics = "0.89.0"
```

- **1** - Début de la fiche d’identité du projet Cargo.
- **2** - Nom de la bibliothèque commune. C’est ce nom qu’on écrit dans `use mini_jeu_serveur::...`.
- **3** - Numéro de version du projet, pas celui de Rust.
- **4** - Édition du langage. `2021` fixe la syntaxe reconnue par le compilateur.
- **5** - Phrase descriptive, affichée par Cargo, sans effet sur l’exécution.
- **7** - Déclare un programme exécutable, en plus de la bibliothèque `src/lib.rs` qui existe par convention.
- **8** - Le programme s’appelle `server`. On le lance avec `cargo run --bin server`.
- **9** - Son code est `src/server.rs`, qui contient son propre `main`.
- **11 à 13** - Deuxième programme, `client`, dont le `main` est dans `src/client.rs`.
- **15** - Liste des bibliothèques externes. Seul le client s’en sert. Le serveur n’utilise que la bibliothèque standard.
- **16** - Piston : boucle d’événements, clavier, temps entre deux images.
- **17** - Fonctions de dessin (`rectangle`, `clear`) et déplacement du repère (`trans`).
- **18** - Création de la fenêtre du système.
- **19** - Dessin OpenGL branché sur Piston.

---

## `src/lib.rs`

Le client et le serveur incluent ce fichier. Un message écrit ici est lu de la même façon des deux côtés.

### Lignes 1 à 8 - présentation du fichier

- **1** - `//!` documente le module entier, pas une fonction.
- **3** - Annonce que la suite décrit le dialogue en texte.
- **5** - `HELLO` : première ligne envoyée par le serveur. `id` est le numéro du carré, `x` et `y` son coin haut-gauche, `r g b` sa couleur de 0 à 255.
- **6** - `MOVE` : le client envoie sa nouvelle position.
- **7** - `GET` : le client demande la liste complète. La ligne ne contient rien d’autre.
- **8** - `STATE` : la réponse. Après le mot, les carrés sont collés les uns après les autres, six nombres chacun.

### Lignes 10 à 13 - la zone de jeu

- **10** - Commentaire de la constante qui suit.
- **11** - Largeur commune, 800 pixels. `pub` la rend utilisable dans les autres fichiers. `f64` est un nombre à virgule.
- **12** - Hauteur commune, 600 pixels.
- **13** - Côté d’un carré, 48 pixels. `x` et `y` désignent le coin haut-gauche, pas le centre.

### Lignes 15 à 24 - les couleurs

- **15 et 16** - Commentaire : trois octets rouge, vert, bleu. Le numéro du joueur choisit une case, puis on recommence au début.
- **17** - Tableau de 6 couleurs. Chaque couleur est un tableau de 3 octets.
- **18** - Rouge.
- **19** - Vert.
- **20** - Bleu.
- **21** - Jaune.
- **22** - Violet.
- **23** - Cyan.
- **24** - Fin du tableau.

### Lignes 26 à 33 - la structure `Carre`

- **26** - Commentaire : ce qu’un carré contient.
- **27** - `Clone` permet de copier un carré. `Debug` permet de l’afficher pendant un débogage.
- **28** - Début de la structure publique.
- **29** - Numéro unique, attribué par le serveur. `usize` est un entier positif.
- **30** - Position horizontale du coin haut-gauche.
- **31** - Position verticale. En écran, `y` augmente vers le bas.
- **32** - Couleur rouge, vert, bleu.
- **33** - Fin de la structure.

### Lignes 36 à 40 - `borner`

- **35** - Commentaire : ramener le carré dans la fenêtre.
- **36** - Reçoit une position et renvoie la position corrigée, sous forme d’un couple `(x, y)`.
- **37** - Le `x` le plus grand autorisé. On retire la taille du carré, sinon son bord droit sortirait.
- **38** - Le `y` le plus grand autorisé, même raisonnement.
- **39** - `clamp` force chaque valeur entre 0 et ce maximum. Cette ligne est le résultat de la fonction : en Rust, la dernière expression est renvoyée, sans écrire `return`.
- **40** - Fin de `borner`.

### Lignes 42 à 47 - `position_initiale`

- **42** - Commentaire : répartir les carrés sur une grille au moment de leur création.
- **43** - `index` est le numéro de case demandé, pas forcément l’identifiant du joueur.
- **44** - Colonne : le reste de la division par 6. Les cases 0, 1, 2, 3, 4, 5 sont sur la première rangée.
- **45** - Ligne : le quotient de la division par 6. La case 6 passe à la rangée suivante.
- **46** - Coin haut-gauche : marge de 80 pixels, puis 110 pixels de plus par colonne et 100 par rangée. `as f64` convertit l’entier en nombre à virgule, sinon l’addition avec `80.0` est refusée.
- **47** - `borner` garantit que ce point reste dans la fenêtre. Fin de la fonction.

### Lignes 49 à 54 - `message_hello`

- **49** - Fabrique la ligne `HELLO` à partir d’un carré. `&Carre` emprunte le carré sans le copier. Le résultat est une `String`.
- **50** - `format!` construit le texte. La chaîne continue à la ligne suivante.
- **51** - Le modèle. `\n` termine la ligne : `read_line` de l’autre côté s’arrête à ce caractère.
- **52** - Les six valeurs dans l’ordre des `{}`.
- **53** - Fin de l’appel `format!`.
- **54** - Fin de la fonction. La `String` construite est le résultat.

### Lignes 56 à 58 - `message_move`

- **56** - Fabrique `MOVE` à partir de deux nombres, sans passer par un `Carre` entier : le client connaît déjà son numéro, le serveur aussi grâce à la connexion.
- **57** - `{x}` et `{y}` insèrent les deux arguments. Le `\n` est obligatoire pour que l’autre côté voie la fin du message.
- **58** - Fin de la fonction.

### Lignes 60 à 70 - `message_etat`

- **60** - Fabrique `STATE` pour une liste de carrés. `&[Carre]` est une tranche : on peut lui passer le vecteur du serveur.
- **61** - Le message commence par le mot `STATE`, sans saut de ligne pour l’instant.
- **62** - Parcourt tous les carrés, dans l’ordre du vecteur.
- **63** - Ajoute du texte à la fin de `message`. `&format!(...)` prête la chaîne temporaire à `push_str`.
- **64** - Un espace, puis les six champs. L’espace sépare le mot `STATE` du premier nombre, puis chaque carré du suivant.
- **65** - Les six valeurs de ce carré.
- **66** - Fin du modèle de format.
- **67** - Fin de `push_str`.
- **68** - Fin de la boucle.
- **69** - Ajoute le saut de ligne une seule fois, après tous les carrés. Une liste vide donne donc la ligne `STATE\n`.
- **70** - Fin de la fonction. `message` est le résultat.

### Lignes 72 à 87 - `lire_hello`

- **72** - Fait l’inverse de `message_hello`. `Option` vaut `Some(carre)` si la ligne est bonne, `None` sinon. `&str` est du texte emprunté, pas une copie.
- **73** - Découpe la ligne sur les espaces et les retours à la ligne, puis stocke les mots dans un vecteur.
- **74** - Il faut exactement 7 mots, et le premier doit être `HELLO`. Sinon ce n’est pas ce message.
- **75** - On sort avec `None`.
- **76** - Fin du `if`.
- **77** - On construit le carré. `Some` enveloppe le résultat.
- **78** - `parse` transforme le texte en nombre. `.ok()?` transforme une erreur de parse en `None` et sort de la fonction.
- **79** - Abscisse.
- **80** - Ordonnée.
- **81** - Début du tableau couleur.
- **82 à 84** - Rouge, vert, bleu.
- **85** - Fin du tableau couleur.
- **86** - Fin du `Carre`.
- **87** - Fin de la fonction.

### Lignes 89 à 114 - `lire_etat`

- **89** - Commentaire : `None` si la forme est mauvaise.
- **90** - Même idée que `lire_hello`, mais le résultat est une liste, parce que `STATE` porte plusieurs carrés.
- **91** - Découpe en mots.
- **92** - Regarde le premier mot sans le retirer de la liste. `?` rend `None` si la liste est vide.
- **93** - Fin du test.
- **94** - Tout ce qui suit le mot `STATE`.
- **95** - Chaque carré occupe 6 mots. Si le compte n’est pas un multiple de 6, la ligne est incomplète.
- **96** - On refuse la ligne.
- **97** - Fin du test.
- **99** - Liste résultat, vide au départ.
- **100** - `chunks(6)` groupe les mots six par six.
- **101** - Ajoute un carré.
- **102 à 104** - Numéro, `x`, `y`.
- **105 à 109** - Couleur.
- **110** - Fin du carré ajouté.
- **111** - Fin de la boucle.
- **112** - Renvoie la liste, éventuellement vide si la ligne était `STATE` tout seul.
- **113** - Fin de `Some`.
- **114** - Fin de la fonction.

### Lignes 116 à 162 - tests

Ces fonctions ne sont compilées que pour `cargo test`. Elles ne font pas partie du serveur ni du client.

- **116** - Le module `tests` n’existe que quand on lance les tests.
- **117** - Début du module.
- **118** - `super` désigne `lib.rs` : les tests peuvent appeler `Carre`, `message_hello`, etc.
- **120** - Fabrique un carré fixe, réutilisé par les tests.
- **121 à 127** - Numéro 1, position `(80.5, 190)`, couleur `(9, 8, 7)`.
- **129** - Nom du premier test.
- **130** - Début du test.
- **131** - Prend l’exemple.
- **132** - Écrit la ligne `HELLO`, enlève le `\n` avec `trim`, puis relit. `expect` arrête le test si la lecture échoue.
- **133 à 136** - Vérifie que les quatre champs lus sont ceux de départ. `couleur` compare les trois octets d’un coup.
- **137** - Fin du test.
- **139** - Deuxième test, sur `STATE`.
- **141 à 149** - Une liste de deux carrés : un carré écrit sur place, puis `exemple()`.
- **150** - Écrit `STATE`, relit.
- **151** - Il y a bien deux carrés.
- **152 et 153** - Leurs numéros sont dans le même ordre.
- **154** - L’ordonnée du second est celle de `exemple`.
- **155** - Fin du test.
- **157** - Troisième test : une liste vide.
- **158** - Début.
- **159** - `STATE` sans aucun nombre est une liste valide et vide.
- **160** - `assert!` échoue si la condition est fausse.
- **161** - Fin du test.
- **162** - Fin du module de tests.

---

## `src/server.rs`

Ce fichier est un programme. Il a son propre `main`. Il n’ouvre pas de fenêtre.

### Lignes 1 à 15 - en-tête et imports

- **1** - Documentation du fichier serveur.
- **3** - Il n’affiche pas le jeu. L’adresse et le port sont fixés dans le code, comme dans l’exemple du cours.
- **4 à 6** - Les deux actions à chaque connexion : ajouter un carré, lancer un thread.
- **8** - Rappel de la commande de lancement.
- **10** - `BufRead` apporte `read_line`. `BufReader` lit par lignes. `BufWriter` accumule le texte avant l’envoi. `Write` apporte `write_all` et `flush`.
- **11** - `TcpListener` attend les connexions. `TcpStream` est une connexion déjà établie.
- **12** - `Arc` partage une valeur entre threads. `Mutex` empêche deux threads de la modifier en même temps.
- **13** - `thread::spawn` crée un thread.
- **15** - Fonctions et types définis dans `lib.rs`. Le serveur ne redéfinit pas le format des messages.

### Lignes 17 à 54 - `Monde`

- **17 et 18** - Commentaire : pourquoi `Arc` et `Mutex` entoureront cette structure.
- **19** - `Monde` reste privé au serveur. Le client n’a pas à connaître `prochain_id`.
- **20** - Prochain numéro à donner. Il augmente toujours, même si un joueur part.
- **21** - Les carrés encore connectés.
- **22** - Fin de la structure.
- **24** - Bloc des méthodes de `Monde`.
- **25** - Ajoute un carré et renvoie son numéro. `&mut self` : la méthode modifie le monde.
- **26** - Réserve le numéro actuel.
- **27** - Le suivant prendra le numéro d’après.
- **28** - Cherche une case de la grille encore libre.
- **29 à 34** - Construit le carré. `id % COULEURS.len()` retombe entre 0 et 5 : la palette recommence.
- **35** - L’ajoute à la fin de la liste.
- **36** - Renvoie le numéro au `main`, qui le donnera au thread.
- **37** - Fin de `ajouter`.
- **39 et 40** - Commentaire : à la reconnexion, on ne pose pas le nouveau carré sur quelqu’un qui n’a pas bougé.
- **41** - Renvoie une position libre.
- **42** - Essaie les 30 premières cases de la grille.
- **43** - Calcule le point correspondant à cette case.
- **44 à 47** - `any` répond vrai si un carré déjà présent est à moins d’un pixel de ce point. On compare `x` et `y`.
- **48** - Case libre : on la renvoie tout de suite.
- **49** - Fin du `if`.
- **50** - Fin de la boucle. Si on arrive ici, les 30 cases sont prises.
- **51** - Secours : une position calculée d’après le nombre de carrés déjà là.
- **52** - Fin de `position_libre`.
- **53** - Fin du bloc `impl`.

### Lignes 56 à 83 - `main`

- **56** - Point d’entrée du programme `server`.
- **57 à 60** - Le monde naît vide. `Mutex::new` le protège. `Arc::new` permet d’en donner une copie à chaque thread.
- **62** - Commentaire : même forme que le serveur du chapitre 21.2.
- **63 et 64** - Écoute uniquement sur la machine locale, port 1234. `expect` arrête le programme avec ce message si le port est déjà pris.
- **65** - Affiche une ligne pour montrer que l’attente a commencé. Ce n’est pas la fenêtre de jeu.
- **67** - `incoming` bloque jusqu’à une nouvelle connexion, puis redonne la suivante. La boucle ne se termine pas toute seule.
- **68 à 74** - Si l’acceptation échoue, on écrit l’erreur et on passe à la connexion suivante. `continue` ne crée ni carré ni thread.
- **76** - Sous le verrou, ajoute le carré. Le verrou est relâché à la fin de l’instruction. `id` est un entier, il sera copié dans le thread.
- **77** - Nouvelle copie du `Arc` pour le thread. Celle de `main` reste en place pour les connexions suivantes.
- **79** - Crée le thread. `move` lui donne la connexion, la copie du monde et l’identifiant : ces valeurs ne doivent plus être utilisées dans `main`.
- **80** - Le thread appelle `gerer_client` et s’arrête quand cette fonction se termine.
- **81** - Fin de la closure.
- **82** - Fin de la boucle. On retourne attendre un client.
- **83** - Fin de `main`. En pratique on n’atteint cette ligne qu’en arrêtant le processus.

### Lignes 85 à 93 - `gerer_client`

- **85** - Commentaire : un thread suit un seul client jusqu’au bout.
- **86** - Reçoit la connexion, le monde partagé et le numéro du carré de ce client.
- **87** - Trace dans le terminal.
- **88 à 90** - Lance le dialogue. S’il s’arrête à cause d’une erreur d’entrée-sortie, on l’affiche. Une fermeture normale n’affiche pas d’erreur.
- **91** - Retire de la liste le carré dont le numéro est celui de ce thread. Les autres carrés restent.
- **92** - Trace le départ.
- **93** - Fin du thread. La connexion est fermée parce que `flux` n’existe plus.

### Lignes 95 à 122 - `dialogue`

- **95** - Tient la conversation. `Result` permet d’utiliser `?` : en cas d’erreur réseau, on sort de la fonction avec cette erreur.
- **96** - Commentaire : ne pas attendre d’avoir beaucoup de texte avant d’envoyer.
- **97** - `set_nodelay(true)` désactive le regroupement des petits paquets. Le `_` ignore l’erreur éventuelle de cette option : le dialogue peut continuer sans elle.
- **98** - `try_clone` duplique l’accès à la même connexion. Le clone sert à lire, l’original servira à écrire. `?` sort si le clone échoue.
- **99** - L’original est emballé pour écrire par lignes complètes.
- **101 à 108** - Sous verrou, retrouve le carré qui vient d’être créé et fabrique `HELLO`. `expect` arrête ce thread si le carré a disparu, ce qui ne doit pas arriver ici.
- **109** - Envoie les octets de la ligne.
- **110** - Force l’envoi immédiat. Sans `flush`, la ligne peut rester dans le tampon et le client attend `HELLO`.
- **112** - Tampon réutilisé pour chaque ligne reçue.
- **113** - Boucle jusqu’à la déconnexion.
- **114** - Vide le tampon. `read_line` ajoute à la chaîne ; sans `clear`, les anciennes lignes resteraient devant.
- **115** - Commentaire : zéro octet signifie « l’autre a fermé ».
- **116 et 117** - `read_line` attend une ligne. Si elle renvoie 0, on termine par `Ok(())` : ce n’est pas une erreur, le client est parti normalement.
- **118** - `?` propage une vraie erreur de lecture.
- **119** - Envoie la ligne, sans espaces autour, à `traiter_ligne`.
- **120** - Fin de la boucle. On n’arrive ici qu’en recommençant un tour. La sortie normale est le `return` de la ligne 117. Une erreur sort par `?` aux lignes 116 ou 119.
- **121** - Fin de `dialogue`.

### Lignes 124 à 157 - `traiter_ligne` et `lire_xy`

- **124 à 129** - Reçoit le texte, le numéro du client, le monde, et l’écrivain pour répondre. Le verrou n’est pas pris ici : chaque branche le prend seulement quand elle touche à la liste.
- **130** - `strip_prefix("MOVE ")` enlève le début `MOVE ` et rend le reste. Si la ligne ne commence pas ainsi, on va au `else if`.
- **131** - Tente de lire deux nombres dans le reste.
- **132** - Ramène la position dans la fenêtre avant de l’enregistrer.
- **133** - Prend le verrou pour modifier la liste.
- **134 à 136** - Cherche le carré de ce thread et remplace `x` et `y`. Un client ne peut pas déplacer le carré d’un autre : le numéro vient du thread, pas du message.
- **137** - Fin du `if let` sur le carré.
- **138** - Fin du verrou et du `if let` sur les nombres.
- **139** - L’autre message reconnu est exactement `GET`.
- **140 et 141** - Commentaire : copier le texte pendant le verrou, envoyer après l’avoir relâché.
- **142 à 145** - Sous verrou, fabrique `STATE` avec tous les carrés. À la fin du bloc, le verrou tombe. `message` est une `String` indépendante de la liste.
- **146 et 147** - Envoie cette ligne et vide le tampon.
- **148** - Fin du `GET`.
- **149** - Une ligne qui n’est ni `MOVE` ni `GET` arrive ici et n’est pas une erreur. On continue le dialogue.
- **150** - Fin de `traiter_ligne`.
- **152** - Lit deux nombres. `Option` : `None` si le texte est mauvais, par exemple `MOVE pas un nombre`.
- **153** - Découpe le reste sur les espaces. L’itérateur rend les mots un par un.
- **154** - Premier mot converti en `f64`. `?` sort avec `None` s’il manque ou s’il n’est pas un nombre.
- **155** - Deuxième mot, même traitement.
- **156** - Les deux nombres lus.
- **157** - Fin de `lire_xy`.

---

## `src/client.rs`

Ce fichier est l’autre programme. Il ouvre la fenêtre et parle au serveur.

### Lignes 1 à 31 - en-tête, imports, constantes

- **1** - Documentation du client.
- **3 à 5** - Rôle : adresse et port, fenêtre, demande de la liste, signal quand son carré bouge.
- **7 à 9** - Commande, touches, et le contour blanc qui désigne son carré.
- **11** - Mêmes outils de lecture et d’écriture que le serveur.
- **12** - Seulement `TcpStream` : le client ne fait pas `bind`, il se connecte.
- **13** - `process` sert à quitter avec un code d’erreur si les arguments ou la connexion sont mauvais.
- **14** - La liste des autres carrés sera partagée entre le thread qui lit et la fenêtre qui dessine.
- **15** - Le thread de lecture.
- **16** - `Duration` sert au délai de la poignée de main.
- **18** - Fenêtre Glutin, renommée `Window` pour écrire plus court.
- **19** - `Transformed` apporte `.trans(x, y)`, qui décale le dessin.
- **20** - `GlGraphics` dessine. `OpenGL` choisit la version d’OpenGL.
- **21** - Réglages de la boucle d’événements et la boucle elle-même.
- **22 et 23** - Traits et types des événements clavier, dessin et mise à jour. Un trait doit être importé pour que sa méthode (`.press_args()`, `.render_args()`, `.update_args()`) soit disponible.
- **24** - `WindowSettings` décrit la fenêtre avant de la créer.
- **25 à 27** - Ce dont le client a besoin dans `lib.rs` : bornes, lecture des messages, fabrication de `MOVE`, type `Carre`, et les trois tailles.
- **29** - Vitesse, 220 pixels par seconde. Elle reste dans le client : le serveur ne déplace personne, il enregistre la position qu’on lui envoie.
- **30** - Commentaire du rythme des `GET`.
- **31** - Dix demandes par seconde. `0.1` est en secondes, la même unité que `dt` donné par Piston.

### Lignes 33 à 51 - `Joueur` et `main`

- **33** - État du joueur de cette fenêtre. Il n’est pas envoyé tel quel : seuls `x` et `y` partent dans `MOVE`.
- **34** - Numéro reçu dans `HELLO`.
- **35 et 36** - Position locale, mise à jour à chaque frame sans attendre le serveur.
- **37** - Couleur reçue dans `HELLO`, pour dessiner le même carré que les autres voient.
- **38 à 41** - Une flèche est enfoncée ou non. On ne regarde pas une seule touche ponctuelle : tant que la flèche reste en bas, le carré continue d’avancer.
- **42 et 43** - Dernière position réellement envoyée. Sert à ne pas renvoyer `MOVE` quand rien n’a changé.
- **44** - Temps accumulé depuis le dernier `GET`.
- **45** - Fin de `Joueur`.
- **47** - Point d’entrée du programme `client`.
- **48** - Lit l’adresse sur la ligne de commande.
- **49** - Se connecte. Récupère l’écrivain, la liste partagée et le joueur.
- **50** - Ouvre la fenêtre et ne revient que lorsqu’elle se ferme.
- **51** - Fin de `main`. Le programme s’arrête, ce qui coupe la connexion.

### Lignes 53 à 61 - `lire_adresse`

- **53** - Renvoie une adresse de la forme `127.0.0.1:1234`.
- **54** - `args` contient le nom du programme puis les arguments. `skip(1)` enlève le nom. `collect` fabrique la liste des arguments seuls.
- **55** - Il faut deux arguments, et le second doit être un port entre 0 et 65535. `parse::<u16>()` échoue sinon.
- **56 et 57** - Explique l’usage sur la sortie d’erreur.
- **58** - Quitte avec le code 1. On ne continue pas avec une adresse fausse.
- **59** - Fin du test.
- **60** - Recolle l’adresse IP et le port avec `:`, la forme attendue par `TcpStream::connect`.
- **61** - Fin de la fonction.

### Lignes 63 à 133 - `connecter`

- **63 et 64** - Commentaire : se connecter, lire son carré, puis laisser un thread recevoir les listes pendant que la fenêtre tourne.
- **65** - Le triplet renvoyé : de quoi écrire, la liste partagée, le joueur local.
- **66 à 70** - Tente la connexion. Si elle échoue, affiche l’adresse et le rappel pour lancer le serveur, puis quitte. La fenêtre n’est pas ouverte.
- **71** - Même option que le serveur : envoyer les petits messages tout de suite.
- **72 et 73** - Pendant la poignée de main, une lecture qui dépasse 5 secondes est une erreur. On évite de bloquer pour toujours si le serveur ne parle pas.
- **75** - Commentaire : deux accès à la même connexion, un pour chaque sens.
- **76** - Le clone est le côté lecture.
- **77** - L’original est le côté écriture. Il restera dans la fenêtre.
- **79** - Tampon de la première ligne.
- **80 et 81** - Lit `HELLO`. Zéro octet, ou une erreur de lecture, veut dire que le serveur a fermé. On quitte.
- **82** - Fin du test.
- **84 à 87** - Interprète `HELLO`. Si la ligne n’a pas la bonne forme, on affiche ce qui a été reçu et on quitte.
- **89** - Commentaire : demander la liste avant la fenêtre, pour que les autres carrés soient déjà connus.
- **90** - Envoie `GET`. Le résultat vrai/faux est ignoré ici : l’échec sera visible juste après, parce que la lecture ne recevra rien de valable.
- **91** - Vide le tampon avant la réponse.
- **92** - Lit la première ligne `STATE`. `.ok()` transforme une erreur en « pas de ligne ».
- **93** - Décode la liste. Si la ligne est mauvaise, on démarre avec une liste vide : son propre carré sera quand même dessiné plus tard.
- **94** - Commentaire : le délai ne sert plus.
- **95 à 98** - `get_mut` rend le socket caché dans le lecteur. On enlève le délai. Le thread suivant a le droit d’attendre le prochain `STATE` aussi longtemps qu’il faut.
- **100** - Commentaire : le thread bloque sur la lecture, la fenêtre reste libre de dessiner.
- **101** - La liste est placée sous `Arc<Mutex<...>>` pour que les deux threads la touchent.
- **102** - Copie du `Arc` pour le thread. La fenêtre gardera `carres`.
- **103** - Lance le thread. `move` lui donne le lecteur et sa copie de la liste.
- **104** - Tampon des lignes suivantes.
- **105** - Boucle jusqu’à la fin de la connexion.
- **106** - Vide le tampon avant chaque lecture.
- **107** - Lit une ligne.
- **108** - Zéro octet ou une erreur : on quitte le thread. La fenêtre peut continuer avec la dernière liste connue.
- **109 à 112** - Si la ligne est un `STATE`, on remplace toute la liste. Le verrou est pris seulement le temps de l’affectation.
- **113** - Fin du `match`.
- **114** - Fin de la boucle du thread.
- **115** - Fin de la closure.
- **117** - Fin du `spawn`. Le thread tourne déjà pendant qu’on construit `Joueur`.
- **118 à 129** - Recopie les champs de `HELLO` dans le joueur. Les quatre directions sont relâchées. La position déjà envoyée est la position de départ : tant qu’on ne bouge pas, aucun `MOVE` n’est utile. Le compteur de `GET` repart de zéro.
- **130** - Fin de la structure.
- **131** - Affiche le numéro dans le terminal, pour reconnaître la fenêtre.
- **132** - Renvoie les trois valeurs à `main`.
- **133** - Fin de `connecter`.

### Lignes 135 à 140 - `envoyer`

- **135** - Écrit une ligne déjà terminée par `\n`. Renvoie `true` si l’envoi a marché.
- **136 et 137** - `write_all` met les octets dans le tampon. `flush` les envoie vraiment. Si l’un des deux échoue, on rend `false`.
- **138** - Fin du test d’échec.
- **139** - Les deux opérations ont réussi.
- **140** - Fin de `envoyer`.

### Lignes 142 à 177 - `fenetre`

- **142** - Reçoit l’écrivain, la liste partagée et le joueur. `mut` : la boucle modifie l’écrivain et le joueur.
- **143** - Version d’OpenGL demandée à la fenêtre. 3.2 est celle attendue par cette version de Piston.
- **144** - Titre de la fenêtre, avec le numéro du joueur.
- **145** - Décrit une fenêtre de 800 par 600. `as u32` convertit les `f64` de `lib.rs` en entiers, le type demandé par la taille de fenêtre.
- **146** - Lui donne la version d’OpenGL choisie.
- **147** - Échap ferme la fenêtre. Piston s’en charge : on n’écrit pas de test sur la touche Échap.
- **148** - Crée la fenêtre.
- **149** - Si la création échoue, arrête le programme avec ce message.
- **151** - Outil de dessin branché sur la même version d’OpenGL.
- **152** - Tant que ce booléen est vrai, on envoie `MOVE` et `GET`.
- **153** - La boucle d’événements, avec les réglages par défaut de Piston. Elle produit des appuis, des mises à jour et des demandes de dessin.
- **155** - Tant que la fenêtre fournit un événement. Quand elle se ferme, `next` finit par rendre `None` et la boucle s’arrête.
- **156 et 157** - Si l’événement est un appui clavier, on met le booléen de cette flèche à `true`.
- **158 et 159** - Au relâchement, on le met à `false`.
- **161** - Un événement `update` arrive régulièrement. `args.dt` est le temps, en secondes, depuis le précédent.
- **162** - On ne parle au serveur que si la connexion est encore considérée comme bonne.
- **163** - Déplace, envoie `MOVE` si besoin, envoie `GET` si le délai est atteint. Le booléen renvoyé devient le nouvel état de `reseau_ok`.
- **164 à 166** - Au premier échec, on l’écrit une fois. Le test `if !reseau_ok` ne sera plus vrai aux frames suivantes, parce qu’on est passé dans le `else`.
- **167** - Fin du cas « réseau encore bon ».
- **168 et 169** - Réseau perdu : on continue de bouger le carré local, mais on n’envoie plus rien.
- **170** - Fin du `update`.
- **172** - Un événement `render` demande de redessiner.
- **173** - Copie la liste sous verrou. Le verrou est relâché tout de suite. Le dessin, plus lent, utilisera la copie.
- **174** - `vue` remplace la position de notre carré par la position locale, puis `dessiner` trace le tout.
- **175** - Fin du `render`.
- **176** - Fin de la boucle. On attend l’événement suivant.
- **177** - Fin de `fenetre`, donc fin du programme quand `main` se termine juste après.

### Lignes 179 à 207 - touches et déplacement

- **179** - Met à jour un seul booléen. `appuyee` vaut `true` à l’appui et `false` au relâchement.
- **180** - Compare la touche reçue aux quatre flèches.
- **181 à 184** - Chaque flèche écrit dans son booléen.
- **185** - Toute autre touche est ignorée.
- **186** - Fin du `match`.
- **187** - Fin de `appliquer_touche`.
- **189** - Avance le joueur pendant `dt` secondes.
- **190** - Déplacement horizontal de cette frame, nul au départ.
- **191** - Déplacement vertical de cette frame, nul au départ.
- **192** - La flèche gauche est enfoncée.
- **193** - On retire `220 * dt` pixels à `x`.
- **194** - La flèche droite est enfoncée.
- **195** - On ajoute les mêmes pixels à `x`. Si gauche et droite sont enfoncées ensemble, les deux contributions s’annulent.
- **196** - La flèche haut est enfoncée.
- **197** - On retire à `y`. L’axe de l’écran va vers le bas, donc le carré monte.
- **198** - La flèche bas est enfoncée.
- **199** - On ajoute à `y`. Haut et bas ensemble s’annulent aussi.
- **200** - Fin du `if` de la flèche bas.
- **201** - `borner` empêche de sortir. Le résultat est décomposé dans `x` et `y`.
- **202** - Enregistre l’abscisse bornée.
- **203** - Enregistre l’ordonnée bornée.
- **204** - Fin de `deplacer`.

### Lignes 209 à 231 - `mettre_a_jour`

- **209** - Commentaire : bouger, prévenir, redemander la liste.
- **210** - Reçoit le joueur à modifier, le temps de la frame, et l’écrivain. Renvoie `false` si le réseau casse.
- **211** - Applique les flèches à la position locale d’abord. L’affichage n’attend pas la réponse du serveur.
- **213** - Début du calcul : la position a-t-elle changé ?
- **214** - Vrai si `x` ou `y` s’écarte de plus de 0,01 pixel de la dernière position envoyée. Le seuil évite un `MOVE` pour un écart de calcul minuscule.
- **215** - S’il y a eu un mouvement, envoie `MOVE`. Le `!` teste l’échec de l’envoi.
- **216** - Échec : on sort tout de suite avec `false`. La position « déjà envoyée » n’est pas mise à jour.
- **217** - Fin de ce `if`.
- **218** - L’envoi a réussi, ou il n’y avait rien à envoyer. On n’entre dans le bloc que si la position a changé.
- **219** - Retient l’abscisse comme dernière abscisse envoyée.
- **220** - Retient l’ordonnée.
- **221** - Fin de ce `if`.
- **223** - Ajoute le temps de cette frame au compteur de `GET`.
- **224** - Le compteur a atteint un dixième de seconde.
- **225** - On le remet à zéro pour recommencer l’attente.
- **226** - Envoie `GET`.
- **227** - Échec de cet envoi : on rend `false`.
- **228** - Fin du `if` d’échec.
- **229** - Fin du test des 0,1 s.
- **230** - Cette frame s’est bien passée.
- **231** - Fin de `mettre_a_jour`.

### Lignes 233 à 249 - `vue`

- **233 et 234** - Commentaire : les autres viennent du serveur, le nôtre garde sa position locale pour ne pas attendre l’aller-retour.
- **235** - Reçoit la dernière liste reçue et le joueur. Fabrique la liste à dessiner.
- **236** - Début de la liste copiée.
- **237** - Part de la liste reçue du serveur.
- **238** - Parcourt chaque carré.
- **239** - Garde ceux dont le numéro n’est pas le nôtre. Celui du serveur peut avoir une fraction de seconde de retard.
- **240** - Clone chaque carré gardé, pour que la nouvelle liste possède ses propres valeurs.
- **241** - Termine la construction de la liste.
- **242** - Ajoute notre carré à la fin.
- **243** - Son numéro.
- **244** - Son abscisse locale, pas celle du dernier `STATE`.
- **245** - Son ordonnée locale.
- **246** - Sa couleur, reçue dans `HELLO`.
- **247** - Fin du carré ajouté.
- **248** - Trie par numéro, pour que l’ordre de dessin ne change pas d’une frame à l’autre.
- **249** - Cette liste est le résultat de la fonction.
- **250** - Fin de `vue`.

### Lignes 251 à 276 - `dessiner`

- **251** - Reçoit l’outil OpenGL, la zone à redessiner, les carrés de `vue`, et le numéro local pour le contour blanc.
- **252** - Importe `clear` et `rectangle` seulement pour cette fonction.
- **254** - `draw` ouvre un dessin sur la zone visible de la fenêtre. La closure reçoit le contexte `c` et l’outil `gl`.
- **255** - Efface tout en gris sombre. Les quatre nombres sont rouge, vert, bleu et opacité, entre 0 et 1.
- **257** - Un passage par carré.
- **258** - Décale le repère au coin du carré. On dessine ensuite comme si le carré était en `(0, 0)`.
- **259** - Le contour blanc ne concerne que notre numéro.
- **260** - Début du rectangle blanc.
- **261** - Couleur blanche, opaque.
- **262** - Carré de contour : il commence 4 pixels au-dessus à gauche, et son côté vaut la taille du carré plus 8 pixels.
- **263** - Il est dessiné au même endroit que le carré.
- **264** - Outil OpenGL qui reçoit le dessin.
- **265** - Fin de l’appel `rectangle`.
- **266** - Fin du `if` du contour.
- **267** - Début de la couleur du carré, entre 0 et 1.
- **268** - Rouge, divisé par 255.
- **269** - Vert.
- **270** - Bleu.
- **271** - Opacité 1 : le carré n’est pas transparent.
- **272** - Fin du tableau de couleur.
- **273** - Dessine le carré coloré par-dessus le contour, à la taille normale. Le blanc ne reste visible qu’autour.
- **274** - Fin de la boucle sur les carrés.
- **275** - Fin de la closure `draw`.
- **276** - Fin de `dessiner`, et fin du fichier.
