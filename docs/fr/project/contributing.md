---
title: Contribuer
description: Proposer et réviser des changements pour Rukn.
---

# Contribuer

Rukn est un projet de recherche précoce. Les changements petits et testables sont préférables aux branches volumineuses.

## Avant de coder

1. Consultez les issues et pull requests existantes.
2. Ouvrez une discussion avant de modifier la syntaxe ou la sémantique.
3. Décrivez le comportement attendu et les cas d’échec.
4. Évitez de mélanger des changements sans rapport.

## Chaque changement de comportement doit inclure

- Des tests pour les cas valides et invalides.
- Une mise à jour de la documentation arabe et, si nécessaire, des versions anglaise et française.
- Une justification des changements de syntaxe.
- Aucun Rust non sécurisé : le crate l’interdit actuellement.
- Une CI réussie sur le commit soumis.

Les exemples doivent fonctionner avec l’interpréteur actuel. Les idées futures doivent être présentées comme des propositions, et non comme des capacités déjà disponibles.

## Flux de travail des mainteneurs

Les mainteneurs effectuent les changements courants sur `dev`. Après réussite des contrôles requis et revue du diff, avancez `main` directement vers le commit `dev` vérifié ; n’utilisez pas de pull request pour cette promotion. Les contributeurs externes peuvent proposer des changements depuis un fork pour révision dans `dev`.

## Style documentaire

Fondez les affirmations sur l’interpréteur, les tests et la spécification actuels. Présentez les fonctions futures comme des propositions et gardez les pages arabe, française et anglaise cohérentes. Suivez le [standard rédactionnel technique de Deku Foundation](https://github.com/Deku-Foundation/.github/blob/main/EDITORIAL_STYLE.md).
