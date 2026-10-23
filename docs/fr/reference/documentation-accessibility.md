---
title: Accessibilité de la documentation
description: Principes de lisibilité, contraste, clavier et RTL pour le site de documentation Rukn.
---

# Accessibilité de la documentation

Cette page consigne le socle pratique d’accessibilité du site Rukn. Il s’agit d’une liste de contrôle d’ingénierie, et non d’une déclaration de conformité WCAG formelle ni d’un remplacement des essais avec des technologies d’assistance.

## Normes et références

- [WCAG 2.2](https://www.w3.org/TR/WCAG22/) — viser un contraste d’au moins 4,5:1 pour le texte courant et 3:1 pour le grand texte ; prendre en charge le clavier, le focus visible et le redimensionnement.
- [Explication W3C du contraste minimum](https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum.html) — explique pourquoi un faible contraste nuit à la lecture.
- [Unicode UAX #9](https://www.unicode.org/reports/tr9/) — algorithme bidirectionnel pour le texte mêlant arabe et latin.
- [Personnalisation du thème VitePress](https://vitepress.dev/guide/custom-theme) — préférer les points d’extension officiels avant d’ajouter un comportement personnalisé à l’exécution.

## Éléments actuellement implémentés

- Variables de palette distinctes pour les thèmes clair et sombre, avec des couleurs de texte principal et secondaire renforcées.
- Espacement confortable des paragraphes et interlignage généreux, notamment en arabe.
- Liens soulignés, focus clavier visible, tableaux adaptatifs et surfaces lisibles pour le code en ligne.
- Exemples de code de gauche à droite dans les pages arabes RTL.
- Respect de la préférence de réduction des animations et adaptations aux petits écrans.
- Aucun téléchargement de police externe requis ; les polices système servent de repli.

## Liste de vérification manuelle

- [ ] Mesurer le contraste du texte, des liens et du code dans les deux thèmes avec un outil spécialisé.
- [ ] Parcourir tout le site au clavier et vérifier la visibilité du focus ainsi que l’ordre de navigation.
- [ ] Agrandir à 200 % et vérifier que menus, tableaux et exemples restent utilisables.
- [ ] Examiner les pages arabes sur mobile, notamment ponctuation, termes latins, liens et direction du code.
- [ ] Tester au moins une combinaison lecteur d’écran/navigateur avant toute déclaration de conformité.
- [ ] Répéter la revue après chaque mise à jour de VitePress ou changement CSS.

## Décisions de conception

Privilégier la lisibilité plutôt que la densité décorative. Ne pas utiliser la couleur comme seul indicateur d’état, ne pas supprimer le focus natif sans remplacement visible et ne pas confondre build réussi et accessibilité démontrée. Le contraste et le clavier doivent être évalués sur le site rendu.

Dernière révision : 23 octobre 2026.
