# Documentation technique

## Qu'est-ce que TCP ?

Transmission Control Protocol (TCP) est une norme de communication qui permet
aux applicatifs d'echanger des messages sur un reseau.

Lorsque on utilise `TcpListener`, on ecoute des connections (par exemple quand
on va sur un navigateur a l'adresse 127.0.0.1)
`TcpListener` declenche la creation d'un socket et l'associe a un IP/port, puis
l'ecoute.

### Qu'est-ce qu'une socket ?

Un socket est un point de connexion de communication qu'on peut nommer et
utiliser dans un reseau.

## Documentation

- https://doc.rust-lang.org/std/net/struct.TcpListener.html
- https://www.ibm.com/docs/fr/i/7.6.0?topic=communications-socket-programming
