---
layout: home
title: Rukn — langage de programmation arabe
description: Découvrez Rukn, un langage de programmation arabe expérimental implémenté en Rust.
hero:
  name: Rukn · ركن
  text: Programmer d’abord en arabe
  tagline: Une expérience linguistique compacte, avec des mots-clés arabes, une sémantique explicite et un interpréteur écrit en Rust.
  image:
    light: /rukn-hero.svg
    dark: /rukn-hero-dark.svg
    alt: Le symbole Rukn à côté d'un exemple en arabe et de sa sortie attendue.
  actions:
    - theme: brand
      text: Exécuter un premier programme
      link: /fr/guide/getting-started
    - theme: alt
      text: Lire la spécification
      link: /fr/reference/specification
    - theme: alt
      text: العربية
      link: /ar/
features:
  - title: Syntaxe arabe en priorité
    details: Mots-clés arabes, identifiants Unicode et chiffres ASCII, arabo-indiens ou persans.
    link: /fr/guide/language
    linkText: Explorer le langage
  - title: Noyau réduit et explicite
    details: Un interpréteur d’arbre en Rust avec nombres, chaînes, booléens, variables, opérateurs et diagnostics clairs.
    link: /fr/reference/architecture
    linkText: Comprendre l’interpréteur
  - title: La recherche avant les promesses
    details: Les capacités existantes, les questions ouvertes et les propositions futures sont documentées séparément.
    link: /fr/project/design-decisions
    linkText: Lire les décisions
---

## Un programme complet

Cet exemple correspond à `examples/hello.rukn`, également vérifié par l’intégration continue.

```rukn
# أول برنامج بلغة ركن
دع الاسم = «العالم»؛
اطبع(«مرحبًا يا » + الاسم)؛
دع الناتج = ٦ * ٧؛
اطبع(الناتج)؛
اطبع(صحيح == خطأ)؛
اطبع(صحيح أو خطأ)؛
```

Résultat attendu :

```text
مرحبًا يا العالم
42
خطأ
صحيح
```

## État du projet

**Expérimental · pré-alpha · non destiné à la production.**

Rukn n’implémente pas encore les fonctions, les boucles, les modules, les collections, un gestionnaire de paquets, une VM bytecode, la compilation native, l’intégration IDE ou l’auto-hébergement. Ce sont des objectifs futurs, pas des fonctionnalités disponibles.

Les priorités immédiates sont une spécification précise, des diagnostics fiables, une politique des identifiants Unicode, des tests de non-régression et une construction reproductible du site.
