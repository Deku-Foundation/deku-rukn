---
title: Architecture de l’interpréteur
description: Pipeline actuel de Rukn, limites de ressources et décisions techniques ouvertes.
---

# Architecture de l’interpréteur

Rukn est actuellement un petit interpréteur d’arbre écrit en Rust sûr. Il évite volontairement la génération de code natif et les dépendances de runtime externes.

```text
Source UTF-8
    ↓
Lexer — jetons et positions
    ↓
Parseur récursif descendant — arbre syntaxique
    ↓
Évaluateur — environnement et valeurs
    ↓
Sortie collectée ou erreur localisée
```

## Composants

- **Lexer :** ponctuation, identifiants, nombres, chaînes, commentaires et mots-clés arabes.
- **Parseur :** précédence, affectation, blocs, conditions, boucles while bornées, déclarations de fonctions nommées, appels et retours par descente récursive. Les fonctions imbriquées sont rejetées ; nombre de paramètres/arguments et imbrication sont bornés.
- **Évaluateur :** exécution dans l’ordre source avec environnements lexicaux imbriqués. Les fonctions nommées utilisent une table globale et un environnement neuf de paramètres/variables locales ; elles ne capturent pas les variables de l’appelant. Les retours se propagent dans les blocs et conditions ; la récursion est limitée à 128 appels actifs et à 64 arguments par appel. Les conditions de `إذا` et `بينما` doivent être booléennes, avec un budget d’évaluations de boucle par exécution.
- **CLI :** lit un fichier UTF-8, écrit les résultats sur stdout et les erreurs sur stderr avec un code de sortie non nul.
- **Tests :** caractères arabes, chiffres, précédence, erreurs de types, noms/affectations inconnus, portées, conditions, boucles, appels nommés, retours, limite de récursion, sources mal formées et limites de ressources.

## Limites de ressources implémentées

L’implémentation définit des limites pour la taille source, le nombre de jetons, l’imbrication syntaxique et des blocs, la taille d’une chaîne, le budget cumulé d’allocations/copies de chaînes et les évaluations de conditions while. Elle rejette les sources trop grandes, la syntaxe trop imbriquée, les concaténations excessives, les boucles incontrôlées et les dépassements de budget avec des erreurs du langage. Le budget des chaînes compte les copies des valeurs, les concaténations et la sortie collectée. Le budget des boucles autorise au maximum 1 000 000 d’évaluations de conditions while cumulées par exécution. Les opérations numériques rejettent aussi les résultats non finis au lieu de laisser circuler silencieusement l’infini ou NaN.

Ces garde-fous ne constituent pas un bac à sable formel. Ils doivent rester documentés et couverts par des tests de non-régression.

## Limites de sécurité

Le langage n’expose pas d’API fichiers, réseau ou processus et ne charge pas de plugins natifs. La CLI lit le fichier source explicitement fourni. Cela réduit la surface d’attaque sans garantir une isolation formelle.

N’intégrez pas du code non fiable dans un service privilégié multi-tenant sans modèle de menace distinct, isolation au niveau processus, supervision des ressources et revue de la frontière d’intégration.

## Pourquoi Rust d’abord ?

Rust apporte des types énumérés pour les jetons et valeurs, une gestion explicite des erreurs, des garanties de sûreté mémoire par défaut et un écosystème de tests robuste. Rust est le langage d’implémentation, pas une contrainte imposée à la syntaxe ou à la sémantique future.

## Questions ouvertes

Normalisation Unicode, diacritiques arabes, identifiants confondables, types et mutabilité, fonctions de première classe/fermetures, modules, format de paquets, bibliothèque standard, interpréteur ou VM bytecode ou backend natif, ABI/FFI et amorçage du compilateur.

L’auto-hébergement ne doit être étudié qu’après stabilisation de la grammaire, de la bibliothèque standard, des diagnostics, du corpus de conformité et du processus d’amorçage.
