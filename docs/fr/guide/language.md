---
title: Guide du langage
description: Syntaxe et sémantique actuellement prises en charge par Rukn.
---

# Guide du langage

Cette page décrit le comportement implémenté, pas toute la vision à long terme.

## Variables et affichage

    دع الاسم = «ليلى»؛
    دع السعر = 12.5؛
    اطبع(الاسم)؛
    اطبع(السعر * 2)؛

Chaque instruction se termine par ؛ ou ;. Les chiffres ASCII, arabo-indiens et persans sont acceptés. Le séparateur décimal est le point ASCII ; le séparateur arabe ٫ n’est pas encore pris en charge.

## Valeurs

| Type | Exemples |
| --- | --- |
| Nombre | 42, ٣.٥ |
| Texte | "hello", «أهلًا» |
| Booléen | صحيح, خطأ |
| Valeur nulle | لاشيء |

## Opérateurs

- + : addition de nombres ou concaténation de deux chaînes.
- - : soustraction ou négation numérique.
- * et / : multiplication et division.
- <, <=, >, >= : comparaison numérique.
- ==, != : égalité et inégalité.
- ! : négation booléenne.

La priorité décroît dans cet ordre : négation, multiplication/division, addition/soustraction, comparaison, égalité. Les parenthèses changent la priorité.

## Commentaires et erreurs

Un commentaire commence par # et se termine à la fin de la ligne. Les variables inconnues, chaînes non fermées, caractères non pris en charge, divisions par zéro et incompatibilités de types produisent un diagnostic avec ligne et colonne. Les conversions implicites sont interdites.


## Identifiants Unicode

Les identifiants utilisent les propriétés Unicode `XID_Start` et `XID_Continue` via la dépendance verrouillée `unicode-ident 1.0.27`, avec le soulignement (`_`) autorisé volontairement en début et en continuation. Cette base lexicale suit [Unicode Standard Annex #31](https://www.unicode.org/reports/tr31/).

Les lettres arabes et persanes sont prises en charge, ainsi que les marques combinantes lorsque le profil XID les autorise après le premier caractère :

```rukn
دع عَلم = ٧؛
اطبع(عَلم)؛
دع پ = ۳؛
اطبع(پ)؛
```

Les noms d’identifiants ne sont **pas normalisés**. Des graphies canoniquement équivalentes ou visuellement similaires ne désignent donc pas nécessairement le même nom. La normalisation, les avertissements de confusables et le traitement des contrôles bidirectionnels invisibles restent des sujets ouverts ; voir [Unicode et sécurité des identifiants](/fr/reference/unicode-and-security). Une mise à jour des données Unicode des identifiants modifie le comportement du langage et doit inclure des tests de conformité.
