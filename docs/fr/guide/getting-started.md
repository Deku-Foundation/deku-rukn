---
title: Installer et exécuter
description: Compiler l’interpréteur Rukn et exécuter un premier programme arabe.
---

# Installer et exécuter

Rukn s’exécute actuellement depuis les sources avec la chaîne d’outils Rust. Aucune version binaire officielle n’est publiée.

## Prérequis

- Chaîne Rust stable avec prise en charge de l’édition 2024.
- Git.
- Terminal UTF-8 et police prenant en charge l’arabe.

## Compiler et tester

    git clone https://github.com/Deku-Foundation/deku-rukn.git
    cd deku-rukn
    cargo test --locked
    cargo build --release --locked
    cargo run --locked -- examples/hello.rukn
    cargo run --locked -- examples/functions.rukn

Sortie attendue pour `hello.rukn` :

    مرحبًا يا العالم
    42
    خطأ

## Écrire un programme

    # Les commentaires commencent par #
    دع الاسم = «العالم»؛
    اطبع(«مرحبًا يا » + الاسم)؛
    دع العدد = ٦ * ٧؛
    اطبع(العدد)؛

Chaque instruction se termine par ؛ ou ;. Le programme accepte un seul fichier source. Les fonctions nommées, paramètres, retours, récursion et boucles sont pris en charge expérimentalement. Les valeurs de fonction de première classe, fermetures, modules/imports, collections, accès fichiers/réseau depuis le langage, VM bytecode et compilation native ne sont pas encore implémentés.
