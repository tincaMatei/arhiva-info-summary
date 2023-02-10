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
