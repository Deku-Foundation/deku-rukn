---
title: Feuille de route
description: Étapes vérifiables pour le langage Rukn et ses outils.
---

# Feuille de route

Les étapes reposent sur des preuves et des tests, pas sur des dates promises.

## M0 — Fondation du dépôt

- [x] Crate Rust, entrée CLI et premiers tests.
- [x] Mots-clés arabes, identifiants Unicode et diagnostics.
- [x] Grammaire initiale et documentation arabe, française et anglaise.
- [ ] CI réussie pour formatage, lint, tests et exemple.
- [ ] Processus de décisions de conception documenté.

## M1 — Noyau du langage

- [ ] Limites de taille source, jetons et profondeur.
- [ ] Fuzzing de l’analyseur et corpus de programmes invalides.
- [ ] Normalisation Unicode et identifiants visuellement confondables.
- [ ] Conditions, blocs et portées après revue sémantique.
- [ ] Politique numérique et codes d’erreur stables.

## M2 — Abstractions

- [ ] Fonctions et valeurs de retour.
- [ ] Collections et itération.
- [ ] Modules, imports et dépendances.
- [ ] Gestion explicite des erreurs et ressources.
- [ ] Bibliothèque standard avec niveaux de stabilité documentés.

## M3 — Outils

- [ ] Formateur et diagnostics structurés.
- [ ] Language Server Protocol.
- [ ] Coloration syntaxique dans les éditeurs.
- [ ] Manifeste et résolution reproductible des dépendances.
- [ ] Benchmarks et corpus de non-régression.

## M4 — Compilateur et amorçage

- [ ] Représentation intermédiaire stable.
- [ ] Comparaison mesurée de l’interpréteur, de la VM bytecode et de la compilation native.
- [ ] ABI et FFI définies avant toute promesse de compatibilité C.
- [ ] Compilateur Rust ciblant une plateforme documentée.
- [ ] Tests d’amorçage reproductible avant l’auto-hébergement.

La version 1.0 exigera une sémantique des modules stable, une politique de compatibilité, une revue de sécurité et un processus de maintenance. L’auto-hébergement n’est pas automatiquement une exigence de 1.0.
