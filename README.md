# arhiva-info-summary

Un script care sa faca un rezumat pentru toate problemele dintr-o arhiva, cu
ce mai e nevoie ca sa fie completa.

# Utilizare

> arhiva-info-summary [dir]

Comanda aceasta va afisa toate problemele si directoarele din folder-ul dat
ca si argument si va genera in consola un fisier Markdown cu toate lucrurile de 
care e nevoie.

Fiecare problema va avea urmatoarele patru directoare:

* problema/enunt
* problema/teste
* problema/editorial
* problema/surse

Tabelul va arata astfel:

| Nume | Enunt | Teste | Editorial | Surse |
| ---- | ----- | ----- | --------- | ----- |
| problema-1 | Ok | Incomplet | Gol | Gol |
| problema-2 | Gol | Gol | Gol | Gol |

Ok inseamna ca exista fisiere in acel director, deci exista lucruri.

Incomplet inseamna ca exista in directorul problemei un fisier "broken.md"
care il marcheaza ca si incomplet, iar in fisierul acela ar trebui sa fie o
descriere cu ce mai trebuie.

Gol inseamna ca directorul problemei este gol.

Un aspect important este ca un utilizator poate sa isi scrie README-urile lui, iar scriptul
doar va adauga la sfarsit rezumatul, sau il va inlocui daca exista. Important e sa nu existe
in README un "# Generated Summary" deja facut de utilizator, altfel partea de mai jos o sa fie
inlocuita toata. Un alt lucru important, mare grija sa nu schimbe cineva partea din "# Generated Summary",
sa adauge din greseala o litera sau ceva, altfel scriptul o sa mai bage un summary, si o sa devina README-ul 
si mai mare. Deja, de la cum a fost facut scriptul asta, totul o sa iasa README-ul ala enorm.

# Flaguri

```
Usage: arhiva-info-summary [OPTIONS] <FOLDER>

Arguments:
  <FOLDER>  Target folder to summarize

Options:
  -w, --write      Write the markdown into the file
  -t, --table      Display a big table with all the problems
  -o, --overwrite  Overwrite already existing content
  -r, --recursive  Make a README for every directory that contains a problem
  -h, --help       Print help
  -V, --version    Print version
```

Cele mai utile comenzi ar fi:

> arhiva-info-summary -w -r folder

Asta o sa faca recursiv toate directoarele si subdirectoarele si va crea pentru toate un readme.

> arhiva-info-summary -w folder

Asta va face un rezumat doar pentru un director. Se va uita la toate problemele recursiv, dar va crea 
un singur README.
