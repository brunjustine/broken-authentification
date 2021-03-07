# Password-Tool

Rust

-----
### Available commands


##### Password entropy

Password entropy measures the unpredictability of a password, and thus the difficulty an attacker will encounter in discovering the password.

Entropy is measured in bits.

example : Si un mot de passe a 40 bits d’entropie, cela signifie qu’il est aussi difficile à deviner qu’un mot de passe aléatoire composé de 40 caractères parmi {0, 1} (40 bits donc). Autrement dit, lors d’une attaque par force brute, il est nécessaire d’évaluer 2⁴⁰ possibilités pour épuiser tous les mots de passes équivalents et être sûr de découvrir le mot de passe.


Leur recommandation est de 29 bits d’entropie au minimum pour un mot de passe standard.


##### Critical Passord

Test if a password is in the most common password file.


##### Help Commmands

```console
cargo run entropy --help
cargo run critical --help
```

-----

### Source
- Data Source :

 	- [10-million-password-list-top-10000.txt](https://github.com/danielmiessler/SecLists/blob/master/Passwords/Common-Credentials/10-million-password-list-top-10000.txt)


- Crates/API :

	- [Rust Clap](https://docs.rs/clap/2.33.0/clap/)
	- [Have I been pwned](https://haveibeenpwned.com/API/v3)

- Documentation :

	- [https://medium.com/@antoine.ansel/mots-de-passe-et-entropie-d5256019d06d](https://medium.com/@antoine.ansel/mots-de-passe-et-entropie-d5256019d06d)



<br>
<br>
Brun Justine <brunjustin@eisti.eu>
Larmarque Marine
