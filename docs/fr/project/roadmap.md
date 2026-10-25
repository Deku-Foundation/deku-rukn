---
title: Feuille de route
description: Étapes vérifiables pour le langage Rukn et ses outils.
---

# Feuille de route

Les étapes reposent sur des preuves, pas sur des dates promises. Une fonctionnalité est achevée lorsque le code, les tests et la documentation concordent.

## M0 — Fondation du dépôt

- [x] Crate Rust, CLI et premiers tests.
- [x] Mots-clés arabes, identifiants Unicode et diagnostics.
- [x] Grammaire initiale et documentation trilingue.
- [x] CI réussie pour formatage, lint, tests et sortie de l’exemple.
- [x] Processus de décisions de conception examiné et documenté.

## M1 — Noyau du langage

- [x] Limites de taille source, de jetons et de profondeur syntaxique ; rejet des résultats numériques non finis.
- [x] Ajouter une cible de fuzzing de bout en bout, bornée et hebdomadaire, pour l’API publique de l’interpréteur.
- [ ] Ajouter le fuzzing ciblé du parseur.
- [x] Ajouter un corpus CLI machine-readable pour programmes valides et invalides, sortie/fragment d’erreur attendus et code de sortie.
- [x] Adopter un profil lexical `XID_Start` / `XID_Continue` verrouillé, avec adaptation du souligné et tests de régression arabes/persans.
- [x] Normaliser les jetons d’identifiant en NFC avant la reconnaissance des mots-clés et la résolution des noms ; préserver littéraux/commentaires et positions source, avec tests d’équivalence canonique (brouillon 0.9).
- [ ] Définir les avertissements de confusables/scripts mixtes et l’affichage/diagnostic syntaxique du source bidi, avec un corpus de compatibilité.
- [x] Conditions booléennes, blocs, portées lexicales et affectation avec tests de régression.
- [x] Boucle while bornée avec conditions booléennes et budget cumulé par exécution.
- [ ] Sémantique numérique et codes d’erreur destinés aux outils.
- [ ] Diagnostics structurés stables pour les éditeurs.

## M2 — Abstractions

- [x] Fonctions nommées de niveau supérieur, paramètres, retours et récursion, avec limite d’appels et tests de conformité (brouillon 0.6).
- [x] Références de fonctions nommées comme valeurs : affectation, passage en argument, retour, appels d’ordre supérieur et égalité des références (brouillon 0.7).
- [ ] Fermetures et capture de portée lexicale.
- [x] Listes immuables, indexation à base zéro, `طول` et `أضف` immuable, limites de ressources et tests de conformité (brouillon 0.8).
- [ ] Cartes/enregistrements, tranches et API de collections enrichies.
- [ ] Modules, imports et modèle de dépendances.
- [ ] Propagation explicite des erreurs et sémantique des autres formes de boucles.
- [ ] Stabiliser la sémantique numérique et définir une orientation pour le typage statique.
- [ ] Bibliothèque standard avec niveaux de stabilité documentés.

## M3 — Outils

- [ ] Formateur et diagnostics structurés.
- [ ] Language Server Protocol.
- [ ] Coloration syntaxique dans les éditeurs.
- [ ] Manifeste et résolution reproductible des dépendances.
- [ ] Benchmarks et corpus de non-régression.

## M4 — Recherche sur le compilateur et l’amorçage

Voir la [stratégie d’amorçage du compilateur](/fr/project/compiler-bootstrap) pour les étapes et critères d’acceptation.

- [x] Documenter l’objectif d’auto-hébergement, la nécessité d’un stade zéro et la validation par étapes.

- [ ] Représentation intermédiaire stable.
- [ ] Comparaison mesurée de l’interpréteur, de la VM bytecode et de la compilation native avec un corpus commun.
- [ ] ABI et FFI définies avant toute promesse de compatibilité C.
- [ ] Compilateur Rust ciblant une plateforme documentée.
- [ ] Tests d’amorçage reproductible avant l’auto-hébergement.

## Politique de publication

La version 0.1 exigera une CI réussie et une spécification examinée. La version 1.0 exigera en plus une sémantique des modules stable, des politiques de compatibilité et migration, une revue de sécurité, des limites de ressources documentées et une procédure de maintenance. L’auto-hébergement n’est pas une exigence automatique de la version 1.0.
