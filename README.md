# server-rust

## Objectif

Creer un serveur HTTP en utilisant Rust.

- On cree un porte d'entree sur la machine (127.0.0.1:8080)
- On ecoute les entrees de maniere evenementiel
- On intercepte une connexion
- On lit la requete (quelle methode, quelle version HTTP...)
- On cree la reponse HTTP
- On envoie la reponse
- Tout ca est fait en continue
