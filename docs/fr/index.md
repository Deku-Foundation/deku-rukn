---
title: Rukn — un langage de programmation arabe
description: Découvrez le langage expérimental Rukn et son interpréteur écrit en Rust.
---

# Programmer devrait pouvoir se lire naturellement en arabe.

Rukn (ركن) est un langage de programmation expérimental conçu avec l’arabe comme langue de première classe. La première implémentation est un petit interpréteur écrit en Rust. L’objectif initial est volontairement limité : une syntaxe explicite, des erreurs compréhensibles et des tests pour chaque fonctionnalité implémentée.

[Exécuter votre premier programme](/fr/guide/getting-started){.VPButton .medium .alt}
[Lire la spécification du langage](/fr/reference/specification){.VPButton .medium}

## Fonctionnalités disponibles

- Mots-clés arabes et identifiants Unicode.
- Chiffres arabo-indiens, persans et ASCII.
- Nombres, chaînes, booléens et valeur de type nul.
- Variables, opérations arithmétiques, comparaisons et égalité.
- Parenthèses, instructions terminées par un point-virgule et diagnostics arabes.
- Bibliothèque Rust, exécutable en ligne de commande et tests unitaires.

## Fonctionnalités absentes

Les fonctions, boucles, types définis par l’utilisateur, modules, gestionnaire de paquets, machine virtuelle à bytecode, compilateur natif, intégration IDE et auto-hébergement ne sont pas encore implémentés. Ce sont des étapes futures, pas des capacités actuelles.

## Premier programme

```rukn
# examples/hello.rukn
دع الاسم = «العالم»؛
اطبع(«مرحبًا يا » + الاسم)؛
دع الناتج = ٦ * ٧؛
اطبع(الناتج)؛
```

Résultat attendu :

```text
مرحبًا يا العالم
42
```

## Principes de conception

1. **L’arabe d’abord, sans exclusivité.** Les mots-clés et messages sont arabes ; les chiffres ASCII et les opérateurs usuels restent pris en charge.
2. **Une grammaire réduite avant une longue liste de fonctionnalités.** La spécification et les tests guident le développement.
3. **Des erreurs utiles.** Les diagnostics indiquent la ligne, la colonne et la cause.
4. **Une sémantique explicite.** La première version n’effectue pas de conversion implicite de types.
5. **Un développement public.** Les fonctionnalités réalisées sont distinguées des propositions.

## État du projet

**Expérimental, avant l’alpha.** Rukn ne convient pas à la production ; la syntaxe pourra évoluer avant la spécification 1.0.
