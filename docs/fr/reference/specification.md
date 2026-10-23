---
title: Spécification du langage — brouillon
description: Grammaire et sémantique actuelles de Rukn.
---

# Spécification du langage Rukn

**État : brouillon 0.1.** Ce document décrit uniquement l’interpréteur actuel.

## Règles lexicales

- Les fichiers source sont en UTF-8.
- Un identifiant commence par un souligné ou un caractère alphabétique Unicode ; les caractères suivants peuvent aussi être numériques.
- Mots réservés : دع, اطبع, صحيح, خطأ, لاشيء.
- Les nombres acceptent les chiffres ASCII, arabo-indiens et persans ; le point ASCII sert de séparateur décimal.
- Les chaînes utilisent des guillemets doubles ou les guillemets arabes «texte».
- Les commentaires commencent par #.
- Les instructions se terminent par ؛ ou ;.

## Grammaire abrégée

    program      = { statement } ;
    statement    = declaration | print_statement | expression_statement ;
    declaration  = "دع" identifier "=" expression terminator ;
    print_statement = "اطبع" [ "(" ] expression [ ")" ] terminator ;
    expression   = equality ;
    equality     = comparison { ("==" | "!=") comparison } ;
    comparison   = term { (">" | ">=" | "<" | "<=") term } ;
    term         = factor { ("+" | "-") factor } ;
    factor       = unary { ("*" | "/") unary } ;
    unary        = ("!" | "-") unary | primary ;
    primary      = number | string | "صحيح" | "خطأ" | "لاشيء"
                 | identifier | "(" expression ")" ;

## Évaluation

Les nombres utilisent f64. L’opérateur + accepte deux nombres ou deux chaînes, sans conversion implicite. Les comparaisons exigent deux nombres. Une variable inconnue et la division par zéro sont des erreurs. Les instructions sont évaluées dans l’ordre du fichier.

La normalisation Unicode, la politique contre les identifiants visuellement confondables, les portées lexicales, le système de types statiques et les limites de ressources restent à définir.
