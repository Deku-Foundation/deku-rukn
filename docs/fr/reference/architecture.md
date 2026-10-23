---
title: Architecture de l’interpréteur
description: Composants actuels de Rukn et limites de sécurité.
---

# Architecture de l’interpréteur

La première version est un petit interpréteur d’arbre, sans Rust non sécurisé ni génération de code natif.

    Source UTF-8
        ↓
    Analyse lexicale — jetons et positions
        ↓
    Analyse syntaxique — arbre d’instructions et d’expressions
        ↓
    Évaluation — environnement et valeurs
        ↓
    Sortie collectée ou erreur localisée

## Composants

- **Analyseur lexical :** ponctuation, identifiants, nombres, chaînes, commentaires et mots-clés arabes.
- **Analyseur syntaxique :** descente récursive et précédence des opérateurs.
- **Évaluateur :** exécution dans l’ordre, avec nombres, chaînes, booléens et valeur nulle.
- **CLI :** lit un fichier UTF-8, écrit les résultats sur stdout et les erreurs sur stderr.
- **Tests :** arabe, chiffres, précédence, erreurs de types et sources mal formées.

## Limites de sécurité

Le langage ne fournit pas encore d’API fichiers, réseau ou processus et ne charge pas de modules natifs. Cela réduit la surface d’attaque sans constituer une garantie formelle d’isolation.

Il manque encore des limites de taille source, nombre de jetons, profondeur syntaxique et durée d’exécution. Une entrée profondément imbriquée peut épuiser la pile. N’exécutez pas de code Rukn non fiable dans un service privilégié.

## Décisions ouvertes

Normalisation Unicode, politique des identifiants, système de types, fonctions, modules, gestion des erreurs, format de paquets, machine virtuelle ou compilation native, ABI et stratégie d’auto-hébergement. L’auto-hébergement ne doit commencer qu’après stabilisation de la grammaire, de la bibliothèque, des diagnostics et des tests d’amorçage.
