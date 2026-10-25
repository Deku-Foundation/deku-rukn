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

Chaque instruction se termine par ؛ ou ;. Les chiffres ASCII, arabo-indiens et persans sont acceptés. Le séparateur décimal peut être le point ASCII ou le séparateur arabe ٫ (U+066B) ; il doit être suivi d’au moins un chiffre. Tous les chiffres d’un même nombre doivent appartenir à une seule famille ; mélanger les familles dans un littéral est une erreur.

## Affectation et portées lexicales

Utilisez `=` dans une instruction pour modifier une variable existante. Affecter un nom non défini est une erreur. En cas de noms imbriqués, l’affectation met à jour la liaison existante la plus proche, en partant de la portée intérieure.

Les blocs entre accolades créent des portées lexicales. Une variable déclarée dans un bloc n’est plus accessible une fois le bloc terminé. Redéclarer le même nom dans un bloc imbriqué masque la liaison extérieure ; le redéclarer dans la même portée remplace la liaison de cette portée.

```rukn
دع الاسم = «خارج»؛
{
    دع الاسم = «داخل»؛
    الاسم = «معدّل»؛
    اطبع(الاسم)؛
}
اطبع(الاسم)؛
```

Sortie attendue :

```text
معدّل
خارج
```

## Conditions et boucles

`إذا` exécute un bloc si la condition vaut `صحيح` ; `وإلا` permet de définir le bloc alternatif. `بينما` répète son bloc tant que la condition vaut `صحيح`. Les conditions doivent produire des booléens ; les nombres et chaînes ne sont pas convertis implicitement en booléens.

```rukn
دع العد = ٠؛
بينما (العد < ٣) {
    اطبع(العد)؛
    العد = العد + ١؛
}
إذا (العد == ٣) {
    اطبع(«انتهى»)؛
} وإلا {
    اطبع(«لم ينتهِ»)؛
}
```

Sortie attendue :

```text
0
1
2
انتهى
```

L’interpréteur arrête une exécution après 1 000 000 d’évaluations cumulées de conditions `بينما`. Cette limite de ressources n’est ni une garantie de temps d’exécution ni un bac à sable complet.

## Fonctions nommées, paramètres et retours

Une fonction se déclare avec le mot-clé arabe `دالة`, un nom, une liste de paramètres séparés par des virgules ASCII `,` et un corps entre accolades. L’appel utilise le nom suivi de parenthèses ; le nombre d’arguments doit correspondre au nombre de paramètres.

```rukn
دالة جمع(أ, ب) {
    أرجع أ + ب؛
}

دالة عاملي(ن) {
    إذا (ن <= ١) {
        أرجع ١؛
    }
    أرجع ن * عاملي(ن - ١)؛
}

اطبع(جمع(٢٠, ٢٢))؛
اطبع(عاملي(٥))؛
```

Sortie attendue :

```text
42
120
```

`أرجع expression؛` retourne une valeur. `أرجع؛` retourne `لاشيء`, et une fonction qui se termine sans retour explicite retourne également `لاشيء`. Un retour hors fonction est une erreur. Dans cette version, les fonctions ne peuvent être déclarées qu’au niveau supérieur ; la récursion est limitée à 32 appels actifs dans l’interpréteur arborescent actuel et chaque liste accepte au plus 64 paramètres/arguments.

Un nom de fonction peut aussi être utilisé comme valeur : l’affecter à une variable, le passer à une autre fonction ou le retourner. Les appels utilisent encore un identifiant ; stockez donc une fonction retournée dans une variable avant de l’appeler. Les références ne capturent pas les variables locales de l’appelant. Fermetures, fonctions anonymes, déclarations imbriquées, modules et typage statique restent à réaliser.

```rukn
دالة جمع(أ, ب) { أرجع أ + ب؛ }
دالة تطبيق(عملية, أ, ب) { أرجع عملية(أ, ب)؛ }
دع العملية = جمع؛
اطبع(العملية(٢, ٣))؛
اطبع(تطبيق(جمع, ٤, ٥))؛
```

Sortie attendue : `5` puis `9`.

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
- و : ET logique, exige deux booléens ; si le côté gauche vaut خطأ, le côté droit n'est pas évalué.
- أو : OU logique, exige deux booléens ; si le côté gauche vaut صحيح, le côté droit n'est pas évalué.

La priorité décroît dans cet ordre : négation, multiplication/division, addition/soustraction, comparaison, égalité, و, puis أو. Les opérateurs logiques sont associatifs à gauche et utilisent le court-circuit. L'opérande gauche doit produire un booléen. L'opérande droit n'est évalué — et son type n'est vérifié à l'exécution — que si nécessaire ; un opérande ignoré n'est pas évalué ni vérifié. Aucune conversion implicite en booléen n'existe. Les parenthèses changent la priorité.

    اطبع(صحيح أو خطأ و خطأ)؛ # صحيح : و est prioritaire sur أو
    اطبع(خطأ و (١ / ٠ == ٠))؛ # خطأ ; la division n'est pas évaluée

## Commentaires et erreurs

Un commentaire commence par # et se termine à la fin de la ligne. Les variables inconnues, chaînes non fermées, caractères non pris en charge, divisions par zéro et incompatibilités de types produisent un diagnostic avec ligne et colonne. Les conversions implicites sont interdites.


## Fonctionnalités pas encore implémentées

Les fermetures, fonctions anonymes, chaînage direct d’un résultat d’appel, collections, types définis par l’utilisateur, modules, syntaxe explicite de gestion des erreurs, système de types statique et ABI stable ne sont pas encore implémentés. Les références de fonctions nommées, appels d’ordre supérieur, paramètres, retours, récursion, conditions, blocs, portées lexicales, affectations et boucles `بينما` bornées sont pris en charge expérimentalement. Consultez la spécification et la feuille de route.


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
