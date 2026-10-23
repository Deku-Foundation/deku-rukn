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
- **Parseur :** précédence par descente récursive et profondeur de syntaxe bornée.
- **Évaluateur :** exécution dans l’ordre source ; les types incompatibles et résultats numériques non finis produisent un diagnostic.
- **CLI :** lit un fichier UTF-8, écrit les résultats sur stdout et les erreurs sur stderr avec un code de sortie non nul.
- **Tests :** caractères arabes, conversion des chiffres, précédence, erreurs de types, noms inconnus, sources mal formées et limites de ressources.

## Limites de ressources implémentées

L’implémentation définit des limites pour la taille source, le nombre de jetons et la profondeur du parseur. Elle rejette les sources ou expressions dépassant ces limites avec des erreurs du langage. Les opérations numériques rejettent aussi les résultats non finis au lieu de laisser circuler silencieusement l’infini ou NaN.

Ces garde-fous ne constituent pas un bac à sable formel. Ils doivent rester documentés et couverts par des tests de non-régression.

## Limites de sécurité

Le langage n’expose pas d’API fichiers, réseau ou processus et ne charge pas de plugins natifs. La CLI lit le fichier source explicitement fourni. Cela réduit la surface d’attaque sans garantir une isolation formelle.

N’intégrez pas du code non fiable dans un service privilégié multi-tenant sans modèle de menace distinct, isolation au niveau processus, supervision des ressources et revue de la frontière d’intégration.

## Pourquoi Rust d’abord ?

Rust apporte des types énumérés pour les jetons et valeurs, une gestion explicite des erreurs, des garanties de sûreté mémoire par défaut et un écosystème de tests robuste. Rust est le langage d’implémentation, pas une contrainte imposée à la syntaxe ou à la sémantique future.

## Questions ouvertes

Normalisation Unicode, diacritiques arabes, identifiants confondables, mots réservés, types et mutations, portées, fonctions, modules, format de paquets, stabilité de la bibliothèque, interpréteur ou VM bytecode ou backend natif, ABI/FFI et amorçage du compilateur.

L’auto-hébergement ne doit être étudié qu’après stabilisation de la grammaire, de la bibliothèque standard, des diagnostics, du corpus de conformité et du processus d’amorçage.
