---
title: Bibliographie technique
description: Normes primaires et références d’ingénierie pour un langage de programmation arabe robuste.
---

# Bibliographie technique

**Révisée le 23 octobre 2026.** Cette liste est un point de départ sélectionné, pas une affirmation que Rukn est conforme à toutes les normes citées. Privilégiez les normes primaires et la documentation officielle, et consignez version et date d’accès lorsqu’une décision technique en dépend.

## 1. Texte arabe, identifiants et sécurité Unicode

| Source | Sujet | État dans Rukn |
| --- | --- | --- |
| [UAX #31 — Unicode Identifiers and Syntax](https://www.unicode.org/reports/tr31/) | Propriétés de début/continuation des identifiants et profils | Base de recherche ; le lexer utilise actuellement les propriétés de caractères de Rust sans profil UAX #31 déclaré |
| [UAX #15 — Unicode Normalization Forms](https://www.unicode.org/reports/tr15/) | NFC/NFKC et équivalence | Question ouverte ; les identifiants ne sont pas normalisés |
| [UTS #39 — Unicode Security Mechanisms](https://www.unicode.org/reports/tr39/) | Détection de confusables et mélange de scripts | Question ouverte ; aucun détecteur spécialisé n’existe |
| [UAX #9 — Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/) | Affichage correct des textes LTR/RTL mixtes | Important pour le code source, les diagnostics, la documentation et les futurs éditeurs |
| [UTS #55 — Unicode Source Code Handling](https://www.unicode.org/reports/tr55/) | Recommandations Unicode pour le code source | Référence recommandée avant la stabilisation des identifiants et diagnostics |

Les publications Unicode 18.0 actuelles comprennent UAX #31 révision 45 (1 septembre 2026), UAX #15 révision 58 (12 août 2026), UTS #39 révision 34 (27 août 2026) et UAX #9 révision 52 (1 septembre 2026). Utilisez les liens « dernière version » ci-dessus plutôt que des tableaux copiés depuis d’anciens articles. Ces versions ont été vérifiées le 23 octobre 2026.

### Questions à résoudre

1. Quelles propriétés Unicode exactes sont acceptées et quelle version fait autorité ?
2. Deux écritures canoniquement équivalentes désignent-elles la même variable ? Quand la normalisation s’applique-t-elle ?
3. Quel comportement adopter pour les marques arabes, variantes persanes, contrôles de jointure et identifiants mêlant arabe et latin ?
4. Les confusables suspects produisent-ils un avertissement, une erreur ou un résultat de lint facultatif ?
5. Comment rendre visibles les contrôles bidirectionnels invisibles dans les éditeurs, diagnostics de terminal et revues de code ?

Utilisez des paires de programmes valides/invalides et un corpus de conformité versionné. Ne normalisez pas silencieusement les chaînes utilisateur comme effet secondaire de la normalisation des identifiants.

## 2. Sémantique du langage et implémentation Rust

- [The Rust Reference](https://doc.rust-lang.org/reference/) — référence du langage Rust utilisé pour l’implémentation.
- [The Rust Book](https://doc.rust-lang.org/book/) — ownership, erreurs, tests et structure de projet.
- [The Cargo Book](https://doc.rust-lang.org/cargo/) — dépendances, workspaces, lockfiles et builds reproductibles.
- [Rust Compiler Development Guide](https://rustc-dev-guide.rust-lang.org/) — architecture de compilateur, diagnostics et tests.
- [LLVM Documentation](https://llvm.org/docs/) — référence primaire pour évaluer un backend LLVM.
- [Cranelift](https://cranelift.dev/) — alternative de génération de code à évaluer si la compilation native devient justifiée.

Rukn utilise actuellement un lexer écrit à la main, un parseur récursif descendant et un évaluateur d’arbre. Ne choisissez pas LLVM, Cranelift ou une VM bytecode avant de disposer d’une sémantique et d’un corpus de correction permettant une comparaison équitable.

## 3. Tests, fuzzing et performance

- [Rust Compiler Development Guide : Fuzzing](https://rustc-dev-guide.rust-lang.org/fuzzing.html) — détecter et minimiser les défauts de compilateur.
- [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) et le [Rust Fuzz Book](https://rust-fuzz.github.io/book/cargo-fuzz.html) — fuzzing des frontières d’analyse et d’évaluation.
- [Criterion.rs](https://github.com/criterion-rs/criterion.rs) — microbenchmarks statistiques pour Rust.
- [Documentation des tests Rust](https://doc.rust-lang.org/book/ch11-00-testing.html) — tests unitaires, d’intégration et documentation.

Séquence suggérée :

1. Garder les tests de comportement normal et d’entrée invalide.
2. Créer un corpus machine-readable avec source, sortie ou classe d’erreur et code de sortie attendus.
3. Fuzzer lexer, parseur et exécution, puis transformer chaque défaut minimisé en test de non-régression.
4. Ne mesurer les performances qu’en validant d’abord la correction contre le corpus.
5. Publier charges, matériel, version du compilateur, commandes et variance de chaque comparaison.

Le fuzzing et les benchmarks ne sont pas des capacités achevées tant qu’ils ne sont pas ajoutés et exécutés régulièrement dans le dépôt.

## 4. Éditeurs et interopérabilité

- [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) — contrat de communication entre éditeur et outil de langage.
- [Documentation des grammaires TextMate](https://macromates.com/manual/en/language_grammars) — point de départ pour la coloration.
- [UTS #55 — Unicode Source Code Handling](https://www.unicode.org/reports/tr55/) — recommandations pour les environnements de programmation.

Définissez des codes de diagnostic et plages source stables avant de créer un serveur de langage. Un formateur et une grammaire d’éditeur peuvent arriver plus tôt s’ils respectent les mêmes règles que la spécification.

## 5. Documentation et déploiement

- [Documentation VitePress](https://vitepress.dev/) et [guide de déploiement](https://vitepress.dev/guide/deploy) — build, chemins de base, routage, recherche et GitHub Pages.
- [WCAG 2.2](https://www.w3.org/TR/WCAG22/) — contraste, utilisation au clavier et comportement de l’interface.
- [SVG 2](https://www.w3.org/TR/SVG2/) et [SVG Accessibility API Mappings](https://www.w3.org/TR/svg-aam-1.0/) — images vectorielles et noms accessibles.

Le site de documentation reste sur la branche stable VitePress 1.6.4. La page des releases présente des versions VitePress 2.0 alpha, pas de version 2.0 stable lors de cette revue. Le passage à une version alpha n’est donc pas justifié pour un workflow de publication. Réévaluez après une version stable et une migration testée.

## Discipline des décisions et des sources

Chaque modification issue de la recherche doit enregistrer la source et sa version, le problème, les alternatives, les risques de compatibilité, les tests et le statut de la décision. Une référence ne prouve pas que l’implémentation suit ses recommandations.

Statuts suggérés : **Implémenté**, **Proposé**, **Recherche ouverte**, **Différé**, **Accepté**, **Rejeté / remplacé**.

Pour les questions actives, consultez le [registre de décisions](/fr/project/design-decisions), [Unicode et sécurité](/fr/reference/unicode-and-security) et la [stratégie de test](/fr/reference/testing-strategy).
