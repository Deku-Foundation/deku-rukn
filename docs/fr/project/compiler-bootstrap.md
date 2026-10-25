---
title: Stratégie d’amorçage et d’auto-hébergement
description: Une trajectoire fondée sur des preuves, de l’interpréteur Rust à un compilateur écrit en Rukn.
---

# Stratégie d’amorçage et d’auto-hébergement

**Statut : stratégie d’ingénierie ; l’auto-hébergement n’est pas encore implémenté.** Ce document précise l’indépendance visée par Rukn, les prérequis et les moyens de valider chaque étape.

## Décision technique

L’objectif à long terme est que le compilateur maintenu et les outils de base soient principalement écrits en Rukn. Cet objectif se réalise par amorçage progressif, et non en supposant que le premier compilateur peut exister sans aucune base d’exécution antérieure.

Un compilateur doit initialement être exécuté par un programme existant, un assembleur, un compilateur antérieur ou une petite amorce de confiance. Même les langages auto-hébergés ont besoin d’un exécutable de stade zéro. La cible pratique est donc :

- **Implémentation auto-hébergée :** le compilateur et les outils principaux sont écrits en Rukn.
- **Racine d’amorçage documentée :** un compilateur ou interpréteur minimal de stade zéro peut construire le premier compilateur Rukn depuis ses sources.
- **Base de confiance réductible :** réduire, versionner, tester et auditer l’implémentation initiale et les outils externes au fil du temps.
- **Étapes reproductibles :** chaque stade passe le même corpus de conformité ; la reproductibilité binaire est recherchée lorsque le backend et l’environnement le permettent.

Réécrire immédiatement le projet en Rukn ne supprime pas les dépendances. Rukn possède maintenant des blocs, des portées lexicales, l’affectation, des conditions et une boucle while bornée, mais il lui manque encore fonctions, modules, collections et bibliothèque suffisante pour exprimer proprement un compilateur. Il faut stabiliser la sémantique de cette base, puis accroître les capacités du langage par étapes révisées.

## Modèle d’amorçage

    Stade 0 : interpréteur et CLI en Rust
                 |
                 | exécuter le corpus de conformité
                 v
    Stade 1 : prototype de compilateur Rukn écrit en Rust
                 |
                 | compiler les sources du compilateur Rukn
                 v
    Stade 2 : binaire produit par le stade 1
                 |
                 | recompiler les mêmes sources
                 v
    Stade 3 : binaire produit par le stade 2
                 |
                 | comparer tests, sorties et artefacts reproductibles
                 v
    Compilateur Rukn auto-hébergé avec amorce documentée

Il s’agit d’une architecture cible. Le stade 1 et les stades auto-hébergés ultérieurs n’existent pas encore.

## Étapes et critères d’acceptation

### B0 — Rendre le comportement mesurable

- Maintenir une spécification écrite et un corpus machine-readable de conformité CLI.
- Tester les sorties correctes, catégories de diagnostics, codes de sortie, sources mal formées, familles de chiffres Unicode et limites de ressources.
- Exécuter formatage, tests, lint, exemples, build documentaire et fuzzing borné dans la CI.
- Transformer chaque défaut de fuzzing minimisé ou erreur sémantique en test de non-régression permanent.

**Critère de sortie :** aucun changement sémantique ne passe silencieusement ; il modifie un résultat attendu ou fait l’objet d’une révision explicite de la spécification.

### B1 — Définir un noyau capable d’exprimer un compilateur

Avant d’écrire le compilateur en Rukn, spécifier :

- sémantique des expressions, instructions, blocs et portées lexicales ;
- fonctions, paramètres, retours et récursion ;
- système de types, largeurs numériques et débordements, mutabilité et inférence ;
- collections, enregistrements ou types algébriques, chaînes et propagation explicite des erreurs ;
- modules, imports, positions source et format stable des diagnostics ;
- bibliothèque standard minimale pour fichiers, arguments CLI, chemins et code de sortie.

N’ajoutez pas ces fonctionnalités comme de simples correctifs syntaxiques indépendants. Chacune doit avoir une spécification, des exemples valides/invalides, des tests de conformité, des notes de compatibilité et une documentation alignée en arabe, anglais et français. Gardez le noyau initial petit et auditable.

### B2 — Séparer le frontend et une représentation intermédiaire explicite

Divisez l’implémentation en phases testables :

1. décodage source et lexing ;
2. parsing en arbre syntaxique ;
3. résolution des noms et vérification des portées ;
4. vérification des types et validation sémantique ;
5. abaissement vers une représentation intermédiaire typée ;
6. exécution ou génération de code.

La représentation intermédiaire doit être indépendante de la syntaxe arabe de surface et exprimer la sémantique du langage. Ajoutez des dumps structurés et des tests pour chaque phase avant les optimisations.

### B3 — Ajouter une VM bytecode, puis évaluer le code natif

Une VM bytecode est une étape intermédiaire pratique : elle exécute la sortie du compilateur sur plusieurs plateformes et fournit une cible à l’auto-compilateur Rukn. Comparez-la à l’interpréteur d’arbre de référence au moyen du même corpus de conformité.

La génération native est une étape distincte. Évaluez LLVM ou Cranelift comme accélérateurs techniques seulement après la création de l’IR et du corpus de correction. Ces backends peuvent permettre de produire plus tôt des exécutables utiles, mais restent des dépendances ; ils ne prouvent pas l’indépendance de toute la chaîne d’outils. Un backend natif écrit en Rukn et une stratégie objet/édition de liens pourront suivre lorsque les cibles, l’ABI et les tests seront spécifiés.

### B4 — Construire le premier compilateur en Rust

Gardez un compilateur de référence en Rust, petit et compréhensible, pendant l’évolution rapide de la sémantique. Ajoutez une IR versionnée, des règles d’abaissement documentées, des diagnostics conservant les positions source, des tests différentiels entre interpréteur/VM/compilateur, des cibles de fuzzing pour chaque phase et des benchmarks uniquement après validation de la correction.

Ne choisissez pas un backend pour sa seule réputation. Publiez charges de travail, commandes, versions des outils, détails de la cible et résultats reproductibles.

### B5 — Porter le compilateur en Rukn et prouver l’amorçage

Lorsque Rukn peut exprimer le compilateur et la bibliothèque nécessaire :

1. Écrire le compilateur en Rukn.
2. Utiliser le stade 0 pour exécuter ou compiler le premier compilateur Rukn.
3. Faire compiler ses propres sources par ce compilateur afin de produire le stade suivant.
4. Exécuter le corpus de conformité, les tests du compilateur et de vrais builds à chaque stade.
5. Étudier la reproductibilité binaire ; normaliser dates, chemins, ordre et autres entrées non déterministes avant d’interpréter des différences binaires.
6. Documenter les commandes d’amorçage, versions, révisions sources et outils de confiance.

Des binaires identiques sont une preuve utile si le build est déterministe, mais pas une preuve de sécurité complète. « Reflections on Trusting Trust » de Ken Thompson explique pourquoi inspecter le code source du compilateur ne suffit pas à exclure tous les compromis introduits pendant la compilation. La compilation double diversifiée peut être étudiée ultérieurement, sans remplacer les builds reproductibles, la revue de code et la transparence de la chaîne.

### B6 — Réduire progressivement l’amorce de confiance

Après stabilisation de l’auto-hébergement, évaluez la réduction de l’implémentation Rust de stade 0 à un petit compilateur ou interpréteur d’amorçage. Maintenez cette procédure reproductible et entretenue ; ne supprimez pas l’amorce pour revendiquer une pureté de façade. Une chaîne impossible à reconstruire à partir de sources documentées est moins indépendante en pratique.

## Recherche et normes à l’appui

- Le [Rust Compiler Development Guide](https://rustc-dev-guide.rust-lang.org/) documente un processus mature de compilation par étapes.
- Le [tutoriel LLVM Kaleidoscope](https://llvm.org/docs/tutorial/) construit progressivement lexing, parsing, IR, optimisation et production d’exécutables ; il rappelle lui-même qu’il privilégie l’apprentissage plutôt que toutes les pratiques de production.
- [CompCert](https://compcert.org/doc/) est une référence de recherche en correction formellement vérifiée. C’est une piste à étudier à long terme, pas un prérequis réaliste pour le premier compilateur Rukn fonctionnel.
- L’étude de 2026 [Selective Feature-Sensitive Coverage for Conformance Testing of Programming Languages](https://doi.org/10.1145/3808231) motive des tests conçus autour des fonctionnalités et chemins d’exécution, pas seulement du nombre de tests.
- [Ken Thompson, « Reflections on Trusting Trust » (1984)](https://doi.org/10.1145/358198.358210) montre que le compilateur et la chaîne de build font partie de la base de confiance.
- [NIST SP 800-218, Secure Software Development Framework](https://csrc.nist.gov/pubs/sp/800/218/final) recommande une pratique de développement et de chaîne d’outils disciplinée et auditable. Il s’agit de recommandations, pas d’une spécification du langage.

Pour le code source arabe, les références normatives Unicode actuelles comprennent [UAX #31, révision 45](https://www.unicode.org/reports/tr31/), [UAX #15, révision 58](https://www.unicode.org/reports/tr15/) et [UTS #39, révision 34](https://www.unicode.org/reports/tr39/) dans la famille Unicode 18.0. Elles orientent les profils d’identifiants, la normalisation et la sécurité, sans impliquer que Rukn respecte déjà toutes leurs recommandations.

La linguistique informatique arabe peut éclairer la terminologie, la morphologie et le traitement du texte, mais ne remplace ni la sémantique formelle d’un langage, ni les tests de compilateur, ni l’ingénierie d’amorçage. Le langage a besoin de règles formelles que les outils peuvent appliquer de façon cohérente.

## Travail à court terme

1. Maintenir un corpus de conformité explicite et le réviser à chaque changement de comportement.
2. Séparer le fuzzing du parseur de la cible de bout en bout.
3. Stabiliser la sémantique du contrôle de flux existant, puis spécifier types et nombres avant d’ajouter d’autres fonctionnalités.
4. Implémenter et tester fonctions et frontières de modules avant le port du compilateur.
5. Choisir bytecode ou backend natif à partir des preuves de conformité et d’un plan documenté de cible/ABI.

Voir la [feuille de route](/fr/project/roadmap), la [stratégie de test](/fr/reference/testing-strategy) et le [registre des décisions](/fr/project/design-decisions).