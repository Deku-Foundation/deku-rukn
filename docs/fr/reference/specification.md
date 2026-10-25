---
title: Spécification du langage — brouillon
description: Grammaire et sémantique actuelles de Rukn.
---

# Spécification du langage Rukn

**État : brouillon 0.8.** Ce document décrit uniquement l’interpréteur actuel ; il ne spécifie pas un futur langage 1.0.

## Règles lexicales

- Les fichiers source sont en UTF-8.
- Un identifiant commence par un souligné ou un caractère Unicode `XID_Start`; les caractères suivants utilisent `XID_Continue` ou le souligné, via la dépendance verrouillée `unicode-ident 1.0.27` (UAX #31). Les marques combinantes autorisées peuvent suivre le premier caractère.
- Les identifiants ne sont pas normalisés ; la détection des confusables et des contrôles bidirectionnels n’est pas implémentée.
- La notation scientifique ne fait pas partie de la grammaire numérique actuelle.
- Mots réservés : دع, اطبع, دالة, أرجع, إذا, وإلا, بينما, صحيح, خطأ, لاشيء, و, أو.
- Les nombres acceptent les chiffres ASCII, arabo-indiens et persans. Le point ASCII ou le séparateur décimal arabe ٫ (U+066B) est accepté et doit être suivi d’au moins un chiffre ; les séparateurs répétés sont invalides. Tous les chiffres d’un même littéral doivent appartenir à la même famille ; un mélange est rejeté au premier chiffre incompatible.
- Les chaînes utilisent des guillemets doubles ou les guillemets arabes «texte». Les retours de ligne physiques LF et CR sont interdits dans une chaîne, y compris après une barre oblique inverse.
- Les commentaires commencent par #.
- Les instructions se terminent par ؛ ou ;.

## Grammaire abrégée

    program      = { statement } ;
    statement    = function_declaration | return_statement | declaration
                 | assignment | print_statement | if_statement | while_statement
                 | block | expression_statement ;
    function_declaration = "دالة" identifier "(" [ parameter_list ] ")" block ;
    parameter_list = identifier { "," identifier } ;
    return_statement = "أرجع" [ expression ] terminator ;
    declaration  = "دع" identifier "=" expression terminator ;
    assignment   = identifier "=" expression terminator ;
    print_statement = "اطبع" ( "(" expression ")" | expression ) terminator ;
    if_statement = "إذا" "(" expression ")" block [ "وإلا" block ] ;
    while_statement = "بينما" "(" expression ")" block ;
    block        = "{" { statement } "}" ;
    expression_statement = expression terminator ;
    expression   = logical_or ;
    logical_or   = logical_and { "أو" logical_and } ;
    logical_and  = equality { "و" equality } ;
    equality     = comparison { ("==" | "!=") comparison } ;
    comparison   = term { (">" | ">=" | "<" | "<=") term } ;
    term         = factor { ("+" | "-") factor } ;
    factor       = unary { ("*" | "/") unary } ;
    unary        = ("!" | "-") unary | postfix ;
    postfix      = primary { "[" expression "]" } ;
    primary      = number | string | list_literal | "صحيح" | "خطأ" | "لاشيء"
                 | call_expression | identifier | "(" expression ")" ;
    list_literal  = "[" [ argument_list ] "]" ;
    call_expression = identifier "(" [ argument_list ] ")" ;
    argument_list = expression { "," expression } ;

## Évaluation

- Les nombres utilisent les valeurs Rust f64.
- L’opérateur + accepte deux nombres ou deux chaînes, sans conversion implicite.
- Les comparaisons exigent deux nombres.
- L’égalité compare les valeurs et leurs variantes.
- و et أو sont associatifs à gauche et utilisent le court-circuit. L'opérande gauche doit produire un booléen. L'opérande droit n'est évalué — et son type n'est vérifié à l'exécution — que si nécessaire ; un opérande ignoré n'est ni évalué ni vérifié. Aucune conversion implicite en booléen n'est effectuée.
- دع déclare une liaison dans la portée lexicale courante et remplace une liaison du même nom dans cette même portée.
- Les blocs entre accolades créent une portée lexicale. La résolution et l’affectation recherchent la liaison existante la plus proche en remontant les portées ; une liaison locale ne survit pas à la sortie du bloc.
- L’affectation d’un nom non défini est une erreur. L’affectation est une instruction, pas une expression.
- Les conditions de إذا et بينما doivent produire un booléen ; aucune conversion implicite de vérité n’est effectuée.
- Les conditions de boucle sont évaluées dans l’ordre source. Au-delà de 1 000 000 évaluations cumulées de conditions while, l’exécution est interrompue pour borner les boucles incontrôlées.
- Les fonctions se déclarent avec `دالة nom(paramètre, ...)` et sont autorisées uniquement au niveau supérieur dans cette version. Leurs noms sont enregistrés avant l’exécution des instructions ; les appels anticipés et la récursion mutuelle sont donc pris en charge. Les noms dupliqués sont rejetés à l’analyse. Lorsqu’aucune variable lexicale ne masque le nom, une référence nommée de fonction est une valeur qui peut être affectée, passée comme argument ou retournée. Les références au même nom de fonction sont égales. Les appels utilisent la forme `nom(...)` : un alias peut être appelé, mais les résultats d’appel arbitraires ne peuvent pas encore être chaînés directement sans variable intermédiaire. Les références ne capturent pas d’environnement ; les fermetures et déclarations de fonctions imbriquées restent non prises en charge.
- Les arguments sont évalués de gauche à droite dans l’environnement de l’appelant. L’arité doit correspondre exactement ; au plus 64 paramètres/arguments sont admis.
- Le corps d’une fonction s’exécute avec l’environnement global et un nouvel environnement de paramètres/variables locales. Il ne capture pas les variables locales de l’appelant ; les fermetures et déclarations de fonctions imbriquées ne sont pas prises en charge. Les affectations globales persistent après l’appel.
- `أرجع expression؛` retourne une valeur ; `أرجع؛` retourne `لاشيء`. Une fonction qui termine sans retour explicite retourne aussi `لاشيء`. Un retour hors fonction produit une erreur.
- Les appels récursifs sont pris en charge avec une profondeur active maximale de 128 ; la profondeur d’expression est bornée indépendamment dans chaque cadre de fonction.
- Les littéraux liste utilisent des crochets et des expressions séparées par des virgules, par exemple `[١, «texte», صحيح]`. Les listes sont immuables et peuvent contenir des types différents. L’indexation commence à zéro et exige un entier fini et non négatif. Un index de liste retourne un élément ; un index de chaîne retourne une valeur Unicode scalaire sous forme de chaîne. Un index hors limites est une erreur.
- La fonction intégrée `طول(valeur)` accepte une chaîne ou une liste et retourne un nombre. La longueur et l’indexation des chaînes comptent les valeurs Unicode scalaires, pas les graphèmes. `أضف(liste, valeur)` retourne une nouvelle liste contenant l’élément ajouté sans modifier la liste initiale. Ces fonctions intégrées peuvent être passées comme valeurs ; leurs noms sont réservés.
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
| Taille d’une chaîne à l’exécution | 1 048 576 octets UTF-8 (1 Mio) |
| Budget cumulé d’allocations/copies de chaînes par exécution | 16 777 216 octets (16 Mio) |
| Évaluations des conditions while par exécution | 1 000 000 |
| Profondeur active des appels de fonction | 128 |
| Paramètres / arguments de fonction | 64 |
| Éléments par liste | 16 384 |
| Budget cumulé des éléments de listes créés/copiés par exécution | 1 000 000 |
| Profondeur des listes imbriquées | 64 |

Le budget des chaînes compte de manière conservative les copies de valeurs chaîne, les noms de références de fonction, les concaténations, la sortie collectée et les noms/chaînes copiés pendant un ajout à une liste. Le budget des collections compte les éléments créés ou copiés, y compris chaque ajout immuable. Ce sont des budgets cumulatifs, pas des mesures exactes du tas vivant. Les entrées dépassant ces bornes sont rejetées avec une erreur du langage. Ces limites d’implémentation peuvent évoluer avant une version stable ; elles ne constituent pas un bac à sable formel ni une garantie contre toutes les formes de déni de service.

## Lacunes connues

La grammaire n’est pas encore générée depuis une définition formelle. La normalisation Unicode, la politique sur les identifiants visuellement confondables, les liaisons immuables, un système de types statique, les fermetures, les enregistrements/cartes, les modules, une IR typée, une VM bytecode et un corpus commun aux futures implémentations interpréteur/VM/compilateur restent des travaux ouverts avant une version stable.
