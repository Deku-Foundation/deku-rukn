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
| RUKN-DR-0005 | Opérateurs logiques arabes | Base implémentée dans le brouillon 0.2 ; mots réservés expérimentaux avant stabilisation | Tests des booléens, de l’associativité, de la priorité et du court-circuit |
| RUKN-DR-0006 | Une seule famille de chiffres par littéral | Base expérimentale implémentée dans le brouillon 0.3 ; revue de compatibilité encore ouverte | Tests des familles de chiffres, des séparateurs décimaux, de la position des erreurs et note de migration |
| RUKN-DR-0007 | Taille des chaînes et budget cumulé de copies à l’exécution | Garde-fou implémenté dans le brouillon 0.4 ; comptage conservateur, pas un bac à sable | Tests de copies répétées, concaténation et sortie imprimée ; revoir les limites avant 1.0 et en cas d’intégration |
| RUKN-DR-0008 | Noyau minimal de contrôle et portées lexicales | Base expérimentale implémentée dans le brouillon 0.5 | Cas de conformité pour l’ombrage, l’affectation, les conditions booléennes, les limites de boucle et les blocs mal formés ; revue d’ergonomie avant 1.0 |
| RUKN-DR-0009 | Fonctions nommées, retours et limites de récursion | Base expérimentale implémentée dans le brouillon 0.6 ; fonctions de premier niveau, pas de valeurs de première classe | Tests d’arité, récursion, propagation des retours, absence de capture des variables de l’appelant et limite d’appels ; fermetures et valeurs de fonction différées |

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
