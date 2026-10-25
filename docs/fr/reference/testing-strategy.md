---
title: Stratégie de test et de conformité
description: Plan pratique pour tester le lexer, le parseur, l’évaluateur, la CLI et la documentation de Rukn.
---

# Stratégie de test et de conformité

**Statut : base d’ingénierie avec fuzzing borné et corpus de conformité CLI.** Les tests unitaires et les contrôles CI standard restent les validations principales. Le test d’intégration sans dépendance supplémentaire exécute le corpus de 32 cas contre la vraie CLI, en vérifiant stdout, code de sortie et fragment de diagnostic. Le corpus couvre l’ombrage lexical, l’affectation, if/else, while, les appels nommés, les appels anticipés, la récursion mutuelle, les retours, les erreurs d’arité/de définition en double, les fonctions inconnues, les blocs mal formés et les limites de ressources. Un workflow distinct lance une campagne libFuzzer de 60 secondes pour les pull requests qui modifient le code source ou le répertoire fuzz, chaque semaine et manuellement. Le fuzzing ciblé du parseur, les campagnes longues, les assertions structurées de diagnostics et les benchmarks restent à réaliser.

## Couches de test

| Couche | Vérifications | Exemples |
| --- | --- | --- |
| Lexer | Types de jetons, conversion des chiffres, positions, caractères invalides | Points-virgules arabes/ASCII, trois familles de chiffres, chaîne non terminée |
| Parseur | Précédence, associativité, blocs, terminators et profondeur bornée | `2 + 3 * 4`, parenthèse ou accolade manquante, blocs et expressions très imbriqués |
| Évaluateur | Sémantique des valeurs et échecs | Types incompatibles, nom/affectation inconnus, ombrage lexical, conditions booléennes, boucles bornées, appels nommés, propagation des retours, limites d’arité et de récursion, division par zéro, résultats non finis, budget de copies de chaînes |
| CLI | Entrée fichier, stdout/stderr, codes de sortie | Chemin absent, arguments multiples, fichier non UTF-8 |
| Documentation | Liens et exemples exacts | Construire les trois locales et comparer l’exemple au résultat CI |
| Publication | Contrôles reproductibles et compatibilité | Dépendances verrouillées, revue de la spécification, liste de publication |

## Invariants utiles

- Une source UTF-8 de taille limitée doit retourner un résultat ou un diagnostic, sans panique sur une entrée mal formée.
- Les erreurs lexicales et syntaxiques doivent afficher des positions utiles.
- La normalisation numérique conserve la valeur représentée par chaque famille de chiffres acceptée.
- La précédence des opérateurs doit correspondre à la spécification et couvrir des combinaisons mixtes.
- Une opération invalide doit produire un diagnostic plutôt qu’infini ou NaN silencieux.
- Les limites de source, de jetons, d’imbrication des blocs, de taille d’une chaîne, du budget de copies et des évaluations de conditions while rejettent les entrées excessives de façon prévisible.
- L’affectation modifie la liaison lexicale existante la plus proche ; les noms locaux ne fuient pas vers la portée extérieure.
- Les conditions if/while exigent des booléens et n’utilisent pas de conversion implicite de vérité.
- Les arguments sont évalués de gauche à droite, l’arité doit correspondre, la récursion respecte la limite d’appels actifs et les fonctions ne capturent pas les variables locales de l’appelant.
- Le retour doit se propager dans les blocs/conditions et un retour au niveau supérieur doit produire un diagnostic.
- Chaque exemple documentaire provient d’un fichier réel ou est explicitement présenté comme illustratif.

Le corpus actuel constitue le premier contrat de compatibilité exécutable de la CLI. Étendez-le à chaque nouvelle fonctionnalité, catégorie de diagnostic ou limite de ressources ; conservez un format indépendant de l’interpréteur actuel afin que de futures VM et implémentations de compilateur puissent le partager.

## Plan de fuzzing

Le projet [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) et le [Rust Fuzz Book](https://rust-fuzz.github.io/book/cargo-fuzz.html) décrivent l’usage de libFuzzer avec Rust. Rukn possède maintenant une cible de bout en bout dans `fuzz/fuzz_targets/run_source.rs` : elle transforme des octets arbitraires en UTF-8 valide en remplaçant les séquences invalides, puis appelle l’API publique `rukn::run`. Un diagnostic du langage est un résultat valide ; une panique ou un signalement sanitizer est un échec. Cette cible teste l’API d’interprétation, pas la lecture CLI de fichiers non UTF-8, couverte par des tests unitaires séparés.

Séparation recommandée : une cible lexer, une cible parseur pour les imbrications et délimiteurs manquants, une cible de bout en bout, puis des tests permanents pour chaque cas de défaillance réduit.

Le workflow `Fuzz smoke` épingle `cargo-fuzz 0.13.2` et exécute `cargo +nightly fuzz run run_source -- -max_total_time=60 -timeout=5 -max_len=16384`. Il se déclenche sur les pull requests touchant le code source ou les fichiers fuzz, chaque semaine et manuellement. La durée bornée maîtrise le coût CI ; elle ne constitue pas une preuve de sécurité et ne remplace pas les campagnes plus longues. Chaque panique minimisée ou résultat incorrect doit devenir un test de non-régression ; une cible parseur distincte et un corpus de conformité restent à ajouter.

## Benchmarks et recherche sur le compilateur

Les décisions de performance doivent attendre la stabilité de la sémantique et des programmes représentatifs. Mesurez séparément lexing, parsing, exécution, démarrage, mémoire et entrées générant de nombreuses erreurs. Comparez l’interpréteur d’arbre à une VM bytecode ou à un backend natif seulement avec un corpus de conformité commun.

Publiez le code des benchmarks, l’environnement, les versions des outils, les critères de correction et les commandes reproductibles. Ne choisissez pas un backend uniquement par réputation.

## Vérifications du dépôt

À la racine :

```sh
cargo fmt --all -- --check
cargo test --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo run --locked -- examples/hello.rukn
```

Dans `docs/` :

```sh
npm ci
npm run build
```

## Pour aller plus loin

- [Rust Compiler Development Guide : fuzzing](https://rustc-dev-guide.rust-lang.org/fuzzing.html)
- [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz)
- [Guide de compilation et de déploiement VitePress](https://vitepress.dev/guide/deploy)

Un plan de test ne signifie pas que chaque test proposé est déjà implémenté.
