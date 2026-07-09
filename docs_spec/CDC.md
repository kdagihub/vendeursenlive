
---

# Cahier des Charges Fonctionnel (CDC) : VendeursEnLive

## 1. Vision et Objectifs du Produit

VendeursEnLive est une Progressive Web App (PWA) de Live Commerce destinée au marché ivoirien. Elle agit comme un hub central reliant les flux vidéo en direct (issus de TikTok) à un tunnel de commande instantané et sans friction. L'objectif est de supprimer la charge mentale liée à la gestion des stocks pour les vendeurs informels et de fluidifier l'acte d'achat par Mobile Money pour les clients.

## 2. Profils Utilisateurs (Personas)

* **Le Vendeur :** Professionnel ou particulier diffusant en direct. Il travaille dans l'urgence, souvent avec deux téléphones. Il refuse la lourdeur administrative (pas de gestion de stock). Il a besoin d'une interface ultra-réactive pour capturer des articles à la volée et valider des commandes manuellement.
* **Le Client :** Utilisateur mobile cherchant à acheter rapidement ce qu'il voit à l'écran. Il déteste les processus d'inscription longs et les allers-retours sur WhatsApp. Il paie via Mobile Money (Wave, Orange Money) ou à la livraison.

---

## 3. Arborescence et Navigation Globale (PWA Mobile)

L'application utilise un **Bottom Navigation Menu** (barre de navigation inférieure) qui s'adapte au rôle de l'utilisateur connecté.

**Menu Client (Acheteur) :**

* **Explorer :** Page d'accueil, recherche et mur des Lives.
* **Favoris :** Liste des vendeurs suivis et gestion des alertes.
* **Mes Commandes :** Suivi des achats et upload des reçus de paiement.
* **Profil :** Paramètres et appel à l'action "Devenir Vendeur".

**Menu Vendeur (Abonné) :**

* **Explorer :** Veille concurrentielle sur le mur des Lives.
* **Tableau de Bord :** Lancement de session, gestion de l'abonnement SaaS, statistiques.
* **Commandes :** File d'attente globale et historique d'arbitrage.
* **Boutique :** Configuration du lien Wave/OM, nom de la boutique, logo.

---

## 4. Parcours Fonctionnel : Le Vendeur

### 4.1. Inscription et Onboarding

* Le vendeur s'inscrit via un formulaire simple (Nom de boutique, Téléphone, Mot de passe).
* Le système active automatiquement une période d'essai de 14 jours.
* Le vendeur renseigne ses méthodes de paiement acceptées (Lien Wave, Numéro OM, ou Paiement à la livraison).

### 4.2. Lancement d'une Session Live

* Le vendeur accède à son Tableau de Bord.
* Il colle l'URL de partage de son Live TikTok en cours.
* Il clique sur "Démarrer la session".
* Le système génère un environnement en direct et envoie un email d'alerte à tous ses abonnés.

### 4.3. Interface de Capture (En Direct)

L'écran est optimisé pour le mode paysage ou portrait, divisé en deux zones :

* **Zone Supérieure (La Capture) :** Un bouton massif "📸 Capturer". Un clic lance un compte à rebours (3s). Le vendeur saisit ensuite un "Prix" et une courte "Description" (ex: Pointure 38).
* **Zone Inférieure (La Réception) :** Un flux d'affichage des "Cartes de Commandes" entrantes en temps réel.

### 4.4. Arbitrage des Commandes

* Le vendeur voit une carte avec le nom du client, le produit désiré et le statut (ex: *Preuve soumise*).
* Il dispose de deux boutons frontaux : "Accepter" et "Rejeter".
* **Règle d'arbitrage :** S'il clique sur "Accepter" pour un produit ayant généré de multiples commandes, une modale interactive s'ouvre : *"Produit très demandé (X autres commandes). Conserver les autres en attente ou les rejeter ?"*
* Le vendeur clôture sa session. Le système lui propose de purger les photos des produits non vendus.

---

## 5. Parcours Fonctionnel : Le Client

### 5.1. Découverte (Écran d'Accueil)

* **Catalogue des Lives :** Affichage sous forme de grille des vendeurs actuellement en direct (Badge rouge clignotant).
* **Recherche et Filtres :** Barre de recherche textuelle et pastilles de catégories (Mode, Électronique, Abidjan, etc.).

### 5.2. Interface du Live (Le Tunnel d'Achat)

* Le flux vidéo TikTok joue en continu dans la partie supérieure de l'écran.
* **Zone Centrale :** Affichage du "Produit à la Une" (celui que le vendeur vient de capturer), mis à jour en temps réel (WebSockets).
* **Bouton d'Action :** Un gros bouton "Commander maintenant".
* **Zone Inférieure :** Un carrousel défilant affichant l'historique des produits présentés depuis le début de la session.

### 5.3. Processus de Commande

* Le client clique sur "Commander". Un tiroir (Bottom Sheet) monte sur l'écran.
* Il saisit (ou retrouve) ses informations : Nom, Téléphone, Commune/Lieu de livraison.
* Il sélectionne sa méthode de paiement parmi celles offertes par le vendeur.
* S'il choisit Mobile Money, un bouton le redirige vers l'application de paiement (ex: lien Wave).
* De retour sur l'application VendeursEnLive, l'interface affiche un bouton pour uploader la capture d'écran du reçu.
* La commande passe en statut "En attente d'arbitrage".

---

## 6. Exigences UI/UX et Ergonomie

### 6.1. Identité Visuelle

* Couleurs dominantes : Contraste fort (Rouge corail pour l'urgence du direct, Violet sombre/Noir pour le socle technologique).
* Typographie : Sans-serif moderne, optimisée pour la lisibilité sur petits écrans.

### 6.2. Adaptabilité et Thèmes

* **Mobile-First :** L'interface entière est pensée pour une manipulation au pouce (zones de clic larges).
* **Mode Sombre / Mode Clair :** L'application bascule automatiquement selon les préférences du système de l'utilisateur, avec un bouton de forçage manuel (crucial pour l'économie de batterie du vendeur).

### 6.3. Comportements d'Interface

* **Non-Interruption :** Le lecteur vidéo du direct ne doit jamais être rechargé ou masqué par des pop-ups bloquantes. Toutes les interactions d'achat se font par-dessus ou en dessous du flux.
* **Feedback Visuel :** Toute action (Capture, Commande, Upload) génère une micro-animation ou un message de confirmation (Toast) discret.