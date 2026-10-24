---
title: Spécification du langage — brouillon
description: Grammaire et sémantique actuelles de Rukn.
---

# Spécification du langage Rukn

**État : brouillon 0.3.** Ce document décrit uniquement l’interpréteur actuel ; il ne spécifie pas un futur langage 1.0.

## Règles lexicales

- Les fichiers source sont en UTF-8.
- Un identifiant commence par un souligné ou un caractère Unicode `XID_Start`; les caractères suivants utilisent `XID_Continue` ou le souligné, via la dépendance verrouillée `unicode-ident 1.0.27` (UAX #31). Les marques combinantes autorisées peuvent suivre le premier caractère.
- Les identifiants ne sont pas normalisés ; la détection des confusables et des contrôles bidirectionnels n’est pas implémentée.
- La notation scientifique ne fait pas partie de la grammaire numérique actuelle.
- Mots réservés : دع, اطبع, صحيح, خطأ, لاشيء, و, أو.
- Les nombres acceptent les chiffres ASCII, arabo-indiens et persans. Le point ASCII ou le séparateur décimal arabe ٫ (U+066B) est accepté et doit être suivi d’au moins un chiffre ; les séparateurs répétés sont invalides. Tous les chiffres d’un même littéral doivent appartenir à la même famille ; un mélange est rejeté au premier chiffre incompatible.
- Les chaînes utilisent des guillemets doubles ou les guillemets arabes «texte». Les retours de ligne physiques LF et CR sont interdits dans une chaîne, y compris après une barre oblique inverse.
- Les commentaires commencent par #.
- Les instructions se terminent par ؛ ou ;.

## Grammaire abrégée

    program      = { statement } ;
    statement    = declaration | print_statement | expression_statement ;
    declaration  = "دع" identifier "=" expression terminator ;
    print_statement = "اطبع" [ "(" ] expression [ ")" ] terminator ;
    expression   = logical_or ;
    logical_or   = logical_and { "أو" logical_and } ;
    logical_and  = equality { "و" equality } ;
    equality     = comparison { ("==" | "!=") comparison } ;
    comparison   = term { (">" | ">=" | "<" | "<=") term } ;
    term         = factor { ("+" | "-") factor } ;
    factor       = unary { ("*" | "/") unary } ;
    unary        = ("!" | "-") unary | primary ;
    primary      = number | string | "صحيح" | "خطأ" | "لاشيء"
                 | identifier | "(" expression ")" ;

## Évaluation

- Les nombres utilisent les valeurs Rust f64.
- L’opérateur + accepte deux nombres ou deux chaînes, sans conversion implicite.
- Les comparaisons exigent deux nombres.
- L’égalité compare les valeurs et leurs variantes.
- و et أو sont associatifs à gauche et utilisent le court-circuit. L'opérande gauche doit produire un booléen. L'opérande droit n'est évalué — et son type n'est vérifié à l'exécution — que si nécessaire ; un opérande ignoré n'est ni évalué ni vérifié. Aucune conversion implicite en booléen n'est effectuée.
- Une variable inconnue et la division par zéro sont des erreurs.
- Les résultats numériques non finis produisent un diagnostic.
- Les instructions sont évaluées dans l’ordre source et la sortie est collectée.

## Limites de ressources implémentées

L’implémentation applique actuellement les limites suivantes :

| Ressource | Limite actuelle |
| --- | ---: |
| Taille de la source | 1 048 576 octets (1 Mio) |
| Nombre de jetons lexicaux | 16 384 |
| Profondeur d’imbrication syntaxique | 64 niveaux |

Les entrées dépassant ces bornes sont rejetées avec une erreur du langage. Ces valeurs sont des limites d’implémentation susceptibles d’évoluer avant une version stable. Elles ne constituent pas un bac à sable formel ni une garantie contre toutes les formes de déni de service.

## Lacunes connues

La grammaire n’est pas encore générée depuis une définition formelle. La normalisation Unicode, la politique sur les identifiants visuellement confondables, les portées lexicales, les liaisons immuables, un système de types statique, une VM bytecode et un corpus de conformité machine-readable restent des travaux ouverts avant une version stable.
