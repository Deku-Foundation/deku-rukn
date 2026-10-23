---
title: Stratégie de test et de conformité
description: Plan pratique pour tester le lexer, le parseur, l’évaluateur, la CLI et la documentation de Rukn.
---

# Stratégie de test et de conformité

**Statut : plan d’ingénierie.** Les tests unitaires et les contrôles CI actuels constituent la base. Le fuzzing, le corpus formel de conformité et les seuils de benchmark restent des travaux futurs tant qu’ils ne sont pas implémentés et exécutés dans CI.

## Couches de test

| Couche | Vérifications | Exemples |
| --- | --- | --- |
| Lexer | Types de jetons, conversion des chiffres, positions, caractères invalides | Points-virgules arabes/ASCII, trois familles de chiffres, chaîne non terminée |
| Parseur | Précédence, associativité, terminators, profondeur bornée | `2 + 3 * 4`, parenthèse manquante, expression très imbriquée |
| Évaluateur | Sémantique des valeurs et échecs | Addition de types différents, nom inconnu, division par zéro, résultat non fini |
| CLI | Entrée fichier, stdout/stderr, codes de sortie | Chemin absent, arguments multiples, fichier non UTF-8 |
| Documentation | Liens et exemples exacts | Construire les trois locales et comparer l’exemple au résultat CI |
| Publication | Contrôles reproductibles et compatibilité | Dépendances verrouillées, revue de la spécification, liste de publication |

## Invariants utiles

- Une source UTF-8 de taille limitée doit retourner un résultat ou un diagnostic, sans panique sur une entrée mal formée.
- Les erreurs lexicales et syntaxiques doivent afficher des positions utiles.
- La normalisation numérique conserve la valeur représentée par chaque famille de chiffres acceptée.
- La précédence des opérateurs doit correspondre à la spécification et couvrir des combinaisons mixtes.
- Une opération invalide doit produire un diagnostic plutôt qu’infini ou NaN silencieux.
- Les limites de source, de jetons et de profondeur doivent rejeter les entrées excessives de façon prévisible.
- Chaque exemple documentaire provient d’un fichier réel ou est explicitement présenté comme illustratif.

Un corpus lisible par machine associant source, stdout attendu, catégorie d’erreur et code de sortie faciliterait les tests et la comparaison future de plusieurs implémentations.

## Plan de fuzzing

Le projet [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) et le [Rust Fuzz Book](https://rust-fuzz.github.io/book/cargo-fuzz.html) fournissent une voie pour utiliser libFuzzer avec Rust. Une cible Rukn proposée devrait appeler l’API publique d’analyse ou d’exécution avec du texte UTF-8 arbitraire, puis vérifier les paniques, blocages et contournements des limites.

Séparation recommandée : une cible lexer, une cible parseur pour les imbrications et délimiteurs manquants, une cible de bout en bout, puis des tests permanents pour chaque cas de défaillance réduit.

Le fuzzing n’est pas encore une capacité déclarée comme achevée. Il nécessite un harnais revu, des graines reproductibles et une exécution automatisée. `cargo-fuzz` exige Rust nightly et des outils sanitizer pris en charge ; un workflow distinct peut être ajouté en premier.

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
