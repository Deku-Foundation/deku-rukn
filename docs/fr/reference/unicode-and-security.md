---
title: Unicode et sécurité des identifiants
description: Recherche sur les identifiants arabes, la normalisation, les confusables et le texte bidirectionnel dans Rukn.
---

# Unicode et sécurité des identifiants

**Statut : note de recherche. Cette page n’adopte pas de nouvelle politique des identifiants.**

Pour un langage conçu avec l’arabe au premier plan, Unicode fait partie de la conception du langage. Le lexer accepte actuellement les caractères alphabétiques selon la propriété de Rust `char::is_alphabetic` au début d’un identifiant, puis les caractères alphabétiques ou numériques, avec le soulignement. La version actuelle ne définit pas de normalisation, de détection des confusables visuels ni de politique spécifique aux contrôles bidirectionnels.

## État actuel

- Les sources utilisent UTF-8.
- Un identifiant commence par un soulignement ou un caractère alphabétique selon Rust ; les caractères numériques sont aussi permis ensuite.
- Les littéraux numériques acceptent les chiffres ASCII, arabo-indiens (٠–٩) et orientaux (۰–۹).
- Les chiffres d’un littéral sont convertis en chiffres ASCII avant l’analyse numérique.
- Le lexer n’emploie pas encore le profil `XID_Start` / `XID_Continue` et ne normalise pas les noms d’identifiants.

Il s’agit d’un prototype, pas d’une spécification Unicode complète.

## Axes de recherche

### Profil des identifiants

[UAX #31](https://www.unicode.org/reports/tr31/) présente des propriétés recommandées pour les identifiants. Rukn devrait évaluer un profil fondé sur `XID_Start` et `XID_Continue`, avec une règle documentée pour le soulignement. Les tests doivent inclure les marques arabes, les diacritiques, les caractères persans, les variantes de yāʾ/kāf, les caractères hors du plan multilingue de base et les points de code non pris en charge.

### Normalisation

[UAX #15](https://www.unicode.org/reports/tr15/) définit les formes de normalisation. Une décision doit préciser si deux écritures canoniquement équivalentes désignent le même identifiant, à quel moment la normalisation intervient et comment les diagnostics conservent l’écriture source. Les chaînes de texte ne doivent pas être normalisées simplement parce que les identifiants le sont. Toute modification exige des tests de compatibilité et une note de migration.

### Confusables et scripts mélangés

[UTS #39](https://www.unicode.org/reports/tr39/) décrit la détection des caractères visuellement confondables et des identifiants mélangeant plusieurs scripts. Un futur linter pourrait produire des avertissements exploitables. Interdire tout caractère latin rendrait l’interopérabilité inutilement difficile ; le profil exact reste à décider.

### Contrôles bidirectionnels

[UAX #9](https://www.unicode.org/reports/tr9/) décrit l’algorithme bidirectionnel. Le langage et ses outils doivent définir comment détecter et afficher les contrôles directionnels invisibles dans le code, les commentaires, les chaînes et les diagnostics, sans altérer l’affichage légitime de l’arabe.

## Processus proposé

1. Rédiger des exemples valides, invalides et ambigus.
2. Comparer un profil UAX #31, un profil arabe plus restrictif et le comportement actuel.
3. Ajouter des tests pour les diacritiques, paires normalisées, scripts mélangés, contrôles directionnels, chiffres et colonnes.
4. Évaluer la compatibilité avant toute modification de la résolution des noms.
5. Consigner la décision et la version Unicode choisie.

## Critères avant une version stable

- Grammaire normative des identifiants et version Unicode documentées.
- Corpus de conformité partagé entre la spécification et l’implémentation.
- Normalisation et confusables testés et documentés.
- Affichage des contrôles invisibles revu dans les éditeurs, terminaux et diagnostics.
- Notes de compatibilité et de migration pour chaque modification significative.

## Sources

- [UAX #31 — Unicode Identifiers and Syntax](https://www.unicode.org/reports/tr31/)
- [UAX #15 — Unicode Normalization Forms](https://www.unicode.org/reports/tr15/)
- [UTS #39 — Unicode Security Mechanisms](https://www.unicode.org/reports/tr39/)
- [UAX #9 — Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/)

Ces normes guident la recherche ; elles ne signifient pas que Rukn y est actuellement conforme.
