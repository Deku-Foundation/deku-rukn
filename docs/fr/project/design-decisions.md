---
title: Registre des décisions et recherches
description: Processus de décision pour la syntaxe, la sécurité, la compatibilité et l’architecture de Rukn.
---

# Registre des décisions et recherches

Un langage nécessite un registre de décisions, pas seulement une feuille de route. Cette page suit les sujets susceptibles de modifier la syntaxe, la portabilité, la sécurité, la compatibilité et l’architecture future du compilateur.

**Étiquettes d’état**
- **Base implémentée :** visible dans le code et les tests actuels.
- **Recherche ouverte :** plusieurs options sont encore évaluées.
- **Différé :** volontairement reporté jusqu’à ce qu’un prérequis existe.
- **Accepté :** décision examinée, conséquences documentées.
- **Remplacé :** une décision plus récente la remplace.

## Registre actuel

| ID | Sujet | État | Preuves nécessaires |
| --- | --- | --- | --- |
| RUKN-DR-0001 | Profil d’identifiants Unicode et normalisation | Base XID implémentée ; normalisation ouverte | `unicode-ident` verrouillé et tests arabes ; politique de normalisation/confusables et migration encore ouvertes |
| RUKN-DR-0002 | Politique des confusables et contrôles bidirectionnels | Recherche ouverte ; les points de code rejetés sont affichés | Modèle de menace, chaînes/commentaires, exemples éditeur/terminal et faux positifs |
| RUKN-DR-0003 | Interpréteur d’arbre, VM bytecode ou backend natif | Différé | Sémantique stable, corpus commun, benchmarks reproductibles |
| RUKN-DR-0004 | Amorçage et auto-hébergement | Différé | Parseur stable, bibliothèque standard, tests du compilateur et amorçage reproductible |

Ces entrées sont des sujets de recherche, pas une autorisation d’implémenter une option particulière.

## Processus de revue

1. Décrire le problème et les personnes concernées.
2. Présenter au moins deux options crédibles, dont le maintien du comportement actuel.
3. Documenter les conséquences sur la compatibilité, la sécurité, le coût, les outils et la maintenance.
4. Ajouter des exemples minimaux et des tests qui distinguent les options.
5. Citer les normes et recherches externes, avec les hypothèses restant à vérifier.
6. Ouvrir une issue ou une pull request révisable ; passer à **Accepté** uniquement après revue.
7. Ajouter une note de migration si la décision modifie les programmes acceptés ou le comportement observable.

## Modèle de décision

```md
# RUKN-DR-NNNN — Titre court

- Statut : Recherche ouverte | Accepté | Rejeté | Remplacé
- Date :
- Réviseurs :

## Contexte
Quel problème résolvons-nous ?

## Options
Quelles alternatives crédibles ont été évaluées ?

## Décision
Qu’est-ce qui est choisi et qu’est-ce qui reste ouvert ?

## Conséquences
Compatibilité, diagnostics, performance, sécurité, maintenance.

## Preuves et tests
Normes, recherches, benchmarks reproductibles et cas de conformité.
```

## Priorités

La prochaine revue utile concerne le profil Unicode, décrit dans [Unicode et sécurité des identifiants](/fr/reference/unicode-and-security). Le choix du backend doit attendre un corpus commun de conformité. L’auto-hébergement est un objectif de recherche à long terme, pas un raccourci pour éviter la stabilisation de la sémantique et des tests.
