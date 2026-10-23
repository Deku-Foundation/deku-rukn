---
title: Unicode et sécurité des identifiants
description: Recherche sur les identifiants arabes, la normalisation, les confusables et le texte bidirectionnel dans Rukn.
---

# Unicode et sécurité des identifiants

**Statut : base lexicale implémentée et questions de sécurité ouvertes. La conformité complète à UAX #31 n’est pas revendiquée.**

Pour un langage conçu avec l’arabe au premier plan, Unicode fait partie de la conception du langage. Rukn utilise maintenant la dépendance verrouillée `unicode-ident 1.0.27` pour les propriétés `XID_Start` et `XID_Continue`, avec le soulignement autorisé volontairement aux deux positions. Cela fournit une base lexicale conforme à l’approche UAX #31 ; normalisation, confusables visuels et diagnostics dédiés aux contrôles bidirectionnels restent à étudier.

## État actuel

- Les sources utilisent UTF-8.
- Un identifiant commence par le souligné ou un caractère `XID_Start`, puis utilise `XID_Continue` ou le souligné via `unicode-ident 1.0.27` verrouillé.
- Les littéraux numériques acceptent les chiffres ASCII, arabo-indiens (٠–٩) et orientaux (۰–۹).
- Les chiffres d’un littéral sont convertis en chiffres ASCII avant l’analyse numérique.
- Les marques combinantes prises en charge peuvent suivre le premier caractère ; les diacritiques arabes peuvent donc apparaître dans un identifiant.
- L’orthographe des noms est conservée. Aucune normalisation ou alerte de confusables n’est actuellement appliquée.
- Les caractères XID suivent les données Unicode intégrées à la dépendance verrouillée ; toute mise à jour exige des tests de conformité et une revue de compatibilité.

C’est une base lexicale plus cohérente, mais pas un profil de sécurité Unicode complet. La normalisation, les confusables, le mélange de scripts et les contrôles invisibles restent des sujets ouverts.

## Axes de recherche

### Profil des identifiants

[UAX #31](https://www.unicode.org/reports/tr31/) présente des propriétés recommandées pour les identifiants. Rukn a adopté `XID_Start` et `XID_Continue` comme base lexicale actuelle, avec une règle explicite pour le soulignement et une implémentation verrouillée. Les tests doivent inclure les marques arabes, les diacritiques, les caractères persans, les variantes de yāʾ/kāf, les caractères hors du plan multilingue de base et les points de code non pris en charge.

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
