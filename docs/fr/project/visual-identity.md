---
title: Identité visuelle et système de conception
description: Logos SVG de Rukn, palette, accessibilité et règles éditoriales au sein de Deku Foundation.
---

# Identité visuelle et système de conception

Rukn est un projet de recherche sur un langage au sein de Deku Foundation. Son identité doit appartenir visuellement à la fondation tout en conservant un symbole et un vocabulaire propres. L’objectif est une identité technique, sobre et centrée sur la documentation, pas une marque séparée qui contredit celle de la fondation.

## Logos officiels

![Signature Rukn pour un fond clair](/rukn-wordmark.svg)

![Signature Rukn pour un fond sombre](/rukn-wordmark-inverse.svg)

| Fichier | Usage |
| --- | --- |
| `/rukn-mark.svg` | Symbole carré, favicon et petite icône |
| `/rukn-wordmark.svg` | Signature complète sur fond clair ou neutre |
| `/rukn-wordmark-inverse.svg` | Signature sur fond vert foncé |

Les fichiers SVG restent modifiables et utilisent des tracés vectoriels ainsi que des polices système de secours. Aucun téléchargement de police ou d’image externe n’est nécessaire. Les fichiers contiennent un titre et une description accessibles. Dans l’en-tête du site, associez le symbole au nom visible « Rukn · ركن » plutôt que de communiquer le nom uniquement par une icône.

## Concept

Le symbole combine une arche/un angle et une base stable. La géométrie évoque le sens de « rukn » (ركن), un pilier ou un élément d’appui, tandis qu’un petit point doré rappelle un jeton ou une ponctuation de langage. Il rejoint la direction architecturale et végétale de la fondation sans réutiliser son logo.

Le symbole est une métaphore visuelle, pas une affirmation qu’un compilateur complet, un écosystème de paquets ou l’auto-hébergement existent déjà.

## Couleurs

Les valeurs sont alignées avec les tokens du site de Deku Foundation.

| Token | Hex | Usage |
| --- | --- | --- |
| Vert profond | `#12372A` | Fond principal sombre et fond du symbole |
| Vert le plus sombre | `#0B271D` | Surfaces les plus sombres |
| Vert feuille | `#27734F` | Actions et liens sur fond clair |
| Vert dense | `#18563A` | Texte et actions verts à contraste élevé |
| Vert secondaire | `#7DBB81` | Accents secondaires |
| Menthe claire | `#BFE6A6` | Accent sur fond sombre |
| Papier | `#F5F7F2` | Fond clair principal |
| Papier secondaire | `#EDF2E9` | Surface alternative |
| Blanc cassé | `#F0F7EA` | Texte du logo et du texte sur fond sombre |
| Texte secondaire | `#58675E` | Texte secondaire sur fond clair |
| Or | `#D5AF66` | Accent limité, jamais seul indicateur d’état |

N’utilisez pas la menthe claire pour du petit texte sur blanc ni le texte secondaire sur un fond sombre sans vérifier le contraste. Le contrôle de tokens du site protège quelques paires de couleurs ; il ne constitue pas un audit WCAG complet.

## Typographie

Privilégiez les polices système avec des alternatives adaptées à l’arabe, telles que Noto Sans Arabic et Tahoma. Le site n’embarque ni ne télécharge de fichiers de police. Le texte technique arabe doit rester lisible à côté des identifiants latins, chemins et extraits de code ; les blocs de code restent orientés de gauche à droite.

## Mise en page et comportement

- Mettre le contenu avant la décoration : paragraphes courts, titres explicites, lignes lisibles et prochaine étape claire.
- Garder une architecture d’information équivalente en anglais, arabe et français ; le sens et le statut du projet comptent plus que la traduction mot à mot.
- Les pages arabes doivent utiliser la langue arabe et la direction RTL. Le code source et les sorties terminal restent LTR.
- Préserver un indicateur de focus visible, la navigation au clavier et le respect de la préférence de réduction des animations.
- Distinguer les fonctionnalités implémentées, les propositions de recherche et les travaux différés aussi par le texte, jamais uniquement par la couleur.
- Préférer les actifs locaux et l’amélioration progressive ; ne pas ajouter de suivi, de média en lecture automatique ou de chargement de polices externes pour la décoration.

## Statut produit et règles éditoriales

Rukn est **expérimental et pré-alpha**. Employer « recherche », « prototype », « prévu » ou « question ouverte » selon la situation. Ne pas le présenter comme prêt pour la production, auto-hébergé, complet, formellement sécurisé ou doté d’une ABI stable sans code, preuves et décision de publication validée.

Une fonctionnalité n’est implémentée que lorsque le code, les tests et la spécification concordent. Une page de recherche ou une ligne de feuille de route ne prouve pas l’implémentation.

## Processus de modification

Pour modifier le symbole, les tokens ou les composants globaux :

1. Garder la source SVG modifiable dans `docs/public/`.
2. Actualiser le favicon et les métadonnées si le nom du fichier change.
3. Vérifier les thèmes clair/sombre, mobile, RTL et les états de focus.
4. Exécuter `npm ci --no-audit --no-fund` puis `npm run build` depuis `docs/`.
5. Demander une revue visuelle et conserver le résultat de compilation dans la pull request.

## Références de conception et normes

- [Site Deku Foundation](https://deku-foundation.github.io/) — palette commune et direction visuelle.
- [WCAG 2.2](https://www.w3.org/TR/WCAG22/) — exigences et conseils d’accessibilité.
- [W3C SVG Accessibility API Mappings](https://www.w3.org/TR/svg-aam-1.0/) — noms et descriptions accessibles des SVG.
- [W3C SVG 2](https://www.w3.org/TR/SVG2/) — modèle SVG.

Ces références orientent l’implémentation ; elles ne signifient pas que le site est certifié dans son ensemble.
