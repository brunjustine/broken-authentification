# Password-Tool

Rust

-----
### Password entropy

Password entropy measures the unpredictability of a password, and thus the difficulty an attacker will encounter in discovering the password.

Entropy is measured in bits.

example : Si un mot de passe a 40 bits d’entropie, cela signifie qu’il est aussi difficile à deviner qu’un mot de passe aléatoire composé de 40 caractères parmi {0, 1} (40 bits donc). Autrement dit, lors d’une attaque par force brute, il est nécessaire d’évaluer 2⁴⁰ possibilités pour épuiser tous les mots de passes équivalents et être sûr de découvrir le mot de passe.


Leur recommandation est de 29 bits d’entropie au minimum pour un mot de passe standard.

### Data Source

https://github.com/danielmiessler/SecLists/blob/master/Passwords/Common-Credentials/10-million-password-list-top-10000.txt

### Crates

Rust Clap - https://docs.rs/clap/2.33.0/clap/
-----

### Source

https://medium.com/@antoine.ansel/mots-de-passe-et-entropie-d5256019d06d

Brun Justine
