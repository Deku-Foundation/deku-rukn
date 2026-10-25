---
title: Indépendance de Rust et conception de l’auto-hébergement
description: Architecture fondée sur la recherche pour l’arabe, Unicode, l’amorçage du compilateur et des critères mesurables.
---

# Indépendance de Rust et conception de l’auto-hébergement

**Statut : proposition d’architecture pour le langage expérimental ; l’auto-hébergement n’est pas une fonctionnalité implémentée.**

## 1. Définir précisément l’indépendance

Trois objectifs doivent être suivis séparément :

1. **Indépendance du langage :** la spécification définit le comportement observable sans dépendre des types Rust, de ses règles d’évaluation ou d’un framework de compilation particulier.
2. **Indépendance d’implémentation :** interpréteur, VM, compilateur, formateur et tests peuvent être réalisés sans exiger Rust. L’implémentation actuelle n’atteint pas encore cet objectif.
3. **Indépendance d’amorçage :** une machine propre peut reconstruire la chaîne d’outils à partir d’une graine documentée et auditable, sans binaire de compilateur opaque. Cela dépasse l’auto-hébergement et nécessite une stratégie stage-0 explicite.

Un compilateur qui se compile lui-même n’est pas nécessairement correct, ne supprime pas sa graine d’amorçage et ne prouve pas qu’il est plus rapide que C, Rust ou Python.

## 2. État actuel du dépôt

L’implémentation est un interpréteur d’arbre expérimental écrit en Rust. Il prend en charge les mots-clés arabes, les identifiants Unicode via unicode-ident, nombres et chaînes, portées lexicales, conditions, boucles bornées, fonctions nommées et références de fonction, listes immuables, indexation et fonctions intégrées طول et أضف. Le corpus CLI et la CI visent à préserver le comportement observable.

Il manque encore des capacités essentielles à un compilateur capable de se construire : modules/imports, API standard de fichiers, modèle stable d’erreurs/résultats, cartes/enregistrements, IR typée, bytecode, VM, générateur de code natif, format de paquets et règles FFI/ABI stables. Ne pas présenter VM, compilation native ou auto-hébergement comme implémentés sans artefacts exécutables et tests.

Après le retour de la CI au vert, la première tâche structurelle est de séparer le code en modules testables (lexer, jetons/diagnostics, parseur/AST, valeurs, évaluateur, fonctions intégrées et CLI), sans changer la sémantique. Le corpus de conformité doit rester identique avant et après ce refactoring.

## 3. Résultats de recherche influant sur la conception

### Langage arabe : davantage qu’une traduction des mots-clés

L’article ARABLAN de 1995 proposait un langage arabe à visée pédagogique. L’article Alf..Eih de 2019 décrit une analyse lexicale, syntaxique et sémantique suivie d’une traduction du code arabe en C++. La synthèse Arabic Programming Language Evolution—A Survey de 2018 répertorie des tentatives antérieures et leurs limites. Ces travaux sont des précédents historiques utiles, mais ils ne démontrent pas une chaîne d’outils arabe moderne, performante et auto-hébergée.

Une étude ACM Learning @ Scale de 2017 a analysé des données longitudinales de plus de 15 000 utilisateurs de Scratch dans cinq pays. Elle rapporte que les novices utilisant des mots-clés et un environnement localisés dans la langue principale de leur pays apprenaient plus vite de nouveaux concepts que ceux utilisant une interface en anglais. C’est un argument en faveur de la localisation dans un contexte éducatif ; ce n’est pas une preuve qu’une syntaxe arabe augmente à elle seule le débit du compilateur, les performances des binaires produits ou la productivité professionnelle.

Un préprint de 2025 décrit PWCT2 comme un langage visuel auto-hébergé bilingue arabe/anglais avec un langage textuel d’implémentation. Il constitue une étude de cas moderne, pas une preuve indépendante et reproduite de performance.

**Conséquence de conception :** Rukn doit être arabe d’abord dans sa syntaxe, ses diagnostics, sa documentation et ses outils, tout en exposant une sémantique stable aux éditeurs, systèmes de build, débogueurs et interfaces FFI. La terminologie doit être relue par des programmeurs et des spécialistes de langue ; les chaînes littérales restent exactes et la compilation ne dépend pas d’une traduction automatique.

### Unicode et le sens droite-à-gauche influent sur la correction et la sécurité

Le scanner utilise les propriétés XID d’Unicode : bon point de départ, mais pas une politique complète de code source Unicode.

- [UAX #31 — Unicode Identifiers and Syntax](https://unicode.org/reports/tr31/) définit les profils d’identifiants et la normalisation. NFC est la référence habituelle pour les identifiants sensibles à la casse.
- [UTS #55 — Unicode Source Code Handling](https://www.unicode.org/reports/tr55/) traite du réordonnancement bidirectionnel, de la présentation trompeuse du source, des identifiants confondables et des diagnostics. Il recommande un affichage conscient de la structure syntaxique plutôt qu’un paragraphe RTL ordinaire.
- [UAX #9 — Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/) définit l’algorithme d’affichage du texte mixte ; la version actuelle publiée est Unicode 18.0.0, datée de septembre 2026.

**Profil à définir avant 1.0 :** versionner les données Unicode ; définir l’équivalence NFC des identifiants sans normaliser les chaînes littérales ; tester les identifiants arabes canoniquement équivalents et les collisions ; documenter les marques combinatoires, le tatweel, les lettres arabes/persanes, les chiffres arabo-indiens/persans, les séparateurs décimaux et les familles de chiffres ; rejeter ou diagnostiquer les contrôles directionnels dans les jetons exécutables ; fournir un rendu syntaxique et des avertissements de confusion/mélange de scripts. Le comportement doit être fixé par des tests avant toute modification du lexer. Ne pas écrire une table de normalisation manuelle incomplète.

### IR et code natif comme backends optionnels

Le [tutoriel LLVM pour langages](https://llvm.org/docs/tutorial/) suit le chemin du lexer au parseur, à l’IR LLVM, au flot de contrôle, à l’optimisation, au code natif et aux informations de débogage. [MLIR](https://mlir.llvm.org/docs/LangRef/) offre plusieurs niveaux d’abstraction et des transformations vérifiables. Ce sont des options d’implémentation, pas la définition de la sémantique de Rukn ni des dépendances obligatoires de sa spécification portable.

**Architecture proposée :** spécification et tests indépendants du backend ; définir d’abord un format de bytecode Rukn versionné et vérifié ; réaliser un vérificateur et une VM de référence ; puis ajouter les backends natifs. LLVM/MLIR peut rester un backend optionnel d’optimisation/génération, tandis que le bytecode portable fournit une base stable pour la CI et les constructions multiplateformes. Aucun backend n’est annoncé comme pris en charge avant d’avoir passé le corpus et les tests de cible/ABI correspondants.

### L’amorçage est une chaîne de preuves

Le [manuel GNU Mes](https://www.gnu.org/software/mes/manual/mes.html) documente les approches stage-0 et bootstrap depuis les sources. Les recherches sur [Diverse Double-Compiling (DDC)](https://arxiv.org/abs/1004.5534) expliquent comment un compilateur indépendant peut aider à vérifier que le binaire correspond à son source, sous des hypothèses explicites. La reproductibilité est utile, mais elle ne prouve pas à elle seule la fiabilité du source.

Il faut donc documenter la graine initiale, sa provenance, les commandes de build, les empreintes, l’architecture cible et les limites. Un exécutable initial est nécessaire pour lancer tout compilateur auto-hébergé ; l’auto-hébergement n’élimine pas la graine. Le but est de réduire, documenter, reproduire puis remplacer la dépendance d’amorçage à Rust, et non de prétendre que la graine peut être évitée.

## 4. Architecture recommandée

1. **Contrat du langage :** spécification révisée, représentation définie des jetons/AST, catégories d’erreurs stables, sémantique des limites de ressources et corpus machine-readable.
2. **Implémentation de référence :** conserver temporairement l’interpréteur Rust comme oracle de comparaison différentielle pendant la stabilisation.
3. **Modules et bibliothèque standard :** imports déterministes, lecture/écriture de fichiers, chemins, arguments CLI, octets, cartes/enregistrements et valeurs d’erreur explicites ; toute E/S passe par une petite interface hôte documentée.
4. **Bytecode Rukn portable (RBC) :** en-tête versionné, opcodes indépendants de la cible, pool de constantes, table de positions source, vérificateur, décodeur borné et sérialisation déterministe. Ne pas sérialiser la disposition mémoire de Rust ni des structures AST internes.
5. **VM :** exécuter le bytecode vérifié avec des règles définies pour pile, appels, allocations, erreurs et limites de ressources. Une VM Rust est acceptable temporairement ; le contrat du bytecode ne dépend pas de Rust.
6. **Compilateur auto-hébergé :** implémenter lexer/parseur, contrôles sémantiques et génération RBC en Rukn. L’exécuter d’abord dans l’interpréteur de référence et comparer résultats/diagnostics.
7. **Backend natif :** après stabilisation du bytecode et de la conformité, abaisser une IR vérifiée vers LLVM IR ou un backend autonome. Préserver les positions de debug, symboles déterministes et spécifications de cible/ABI.
8. **Outils :** formateur, serveur de langage, rendu RTL conscient de la syntaxe, débogueur et gestionnaire de paquets partagent les mêmes métadonnées linguistiques et sources.

## 5. Étapes d’amorçage et critères de sortie

| Étape | Livrable | Critère de sortie |
|---|---|---|
| B0 | Stabiliser source et corpus | Format, tests, intégration, Clippy, documentation et fuzz borné verts ; tests positifs/négatifs par fonctionnalité |
| B1 | Modulariser l’interpréteur et figer langue/Unicode | Aucun changement sémantique non révisé ; tests et politique de version pour normalisation/confusables |
| B2 | Modules et bibliothèque standard minimale | Compiler plusieurs fichiers ; imports déterministes ; erreurs E/S explicites et bornées |
| B3 | RBC v1, vérificateur et VM de référence | Rejeter le bytecode malformé sans danger ; vérificateur et VM passent le même corpus |
| B4 | Compilateur écrit en Rukn | Compiler des programmes représentatifs et son propre source via l’interpréteur ; comparer sortie et diagnostics |
| B5 | Reconstruction auto-hébergée | Les compilateurs issus des versions N et N+1 ont un comportement équivalent et des artefacts identiques lorsque le format est déterministe |
| B6 | Publication courante sans dépendance obligatoire à Rust | CI de release fonctionne sans Rust dans les dépendances requises ; graine reproductible et procédure de retour conservées |
| B7 | Backend natif et assurance du bootstrap | Tests cible/ABI, exécution différentielle, builds déterministes et comparaison d’amorçage diversifiée documentées et automatisées |

Ne supprimez pas l’implémentation Rust après B5. Rendez-la d’abord non nécessaire au chemin de publication ordinaire (B6), puis conservez un mécanisme réduit et auditable d’amorçage/récupération et vérifiez la chaîne indépendamment. La suppression de tout code Rust du dépôt est une décision de maintenance ultérieure, pas la définition de l’auto-hébergement.

## 6. Les performances doivent être mesurées

Aucune donnée ne justifie actuellement d’affirmer que Rukn est globalement plus rapide que C, Rust ou Python. Débit de l’interpréteur, temps de compilation, vitesse du code produit, mémoire et productivité sont des métriques différentes.

Créer une suite versionnée avant toute revendication : débit du lexer/parseur, boucles numériques, récursion, listes (y compris ajout immuable), chaînes, chargement de modules et durée d’auto-compilation. Comparer algorithmes, données, E/S, options d’optimisation et échauffement équivalents. Enregistrer CPU, OS, versions/outils, options, allocations, mémoire maximale et révision source. Répéter plusieurs fois et publier médiane et dispersion ; ne pas faire des microbenchmarks instables une porte CI. La correction est le premier critère, les performances viennent après la parité sémantique.

## 7. Références de recherche

- ARABLAN, *Design of an Arabic Programming Language* (1995) : [ScienceDirect](https://www.sciencedirect.com/science/article/pii/0096055195000062).
- Razaq et al., *Designing and Implementing an Arabic Programming Language for Teaching Pupils* (2019) : [DOI 10.35741/issn.0258-2724.54.3.11](https://doi.org/10.35741/issn.0258-2724.54.3.11). Le travail décrit une traduction de l’arabe vers C++.
- Al-Motwakel et al., *Arabic Programming Language Evolution—A Survey* (2018) : [copie de l’article](https://www.academia.edu/39953345/Arabic_Programming_Language_Evolution-A_Survey). Il s’agit d’une synthèse secondaire ; vérifier les sources primaires pour les détails.
- Dasgupta et Hill, *Learning to Code in Localized Programming Languages* (ACM Learning @ Scale 2017) : [DOI 10.1145/3051457.3051464](https://doi.org/10.1145/3051457.3051464).
- Fayed, *Dual-Language General-Purpose Self-Hosted Visual Language and new Textual Programming Language for Applications* (prépublication 2025) : [arXiv:2509.20426](https://arxiv.org/abs/2509.20426).
- Unicode : [UAX #31](https://unicode.org/reports/tr31/), [UTS #55](https://www.unicode.org/reports/tr55/), [UAX #9](https://www.unicode.org/reports/tr9/).
- Infrastructure de compilation : [tutoriel LLVM](https://llvm.org/docs/tutorial/), [MLIR](https://mlir.llvm.org/docs/LangRef/), [CompCert](https://compcert.org/doc/).
- Amorçage et confiance : [manuel GNU Mes](https://www.gnu.org/software/mes/manual/mes.html), [thèse DDC](https://arxiv.org/abs/1004.5534).

Voir aussi la [stratégie d’amorçage](/fr/project/compiler-bootstrap), la [spécification](/fr/reference/specification), la [stratégie de test](/fr/reference/testing-strategy) et la [feuille de route](/fr/project/roadmap).
