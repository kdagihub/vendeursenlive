
---

# Document de Conception Orientée Domaine (DDD)

**Projet :** VendeursEnLive (Marketplace & Live Commerce)
**Cible :** Marché ivoirien (Mobile-first, PWA)

## 1. Le Langage Omniprésent (Ubiquitous Language)

*Ce vocabulaire doit être utilisé à l'identique dans les discussions métier, les maquettes, et le code source (nom des variables, classes, tables).*

* **Vendeur :** L'utilisateur professionnel qui diffuse un flux vidéo en direct (via TikTok) et propose des articles à la vente. Il est abonné à la plateforme.
* **Spectateur / Client :** L'utilisateur (connecté ou anonyme) qui regarde le Live et initie un processus d'achat.
* **Session Live :** L'instance temporelle regroupant le flux vidéo (URL), le Vendeur, et le catalogue d'articles créés à la volée.
* **Produit Éphémère (ou Article) :** Une entité créée instantanément pendant le Live (via capture photo ou screenshot vidéo), caractérisée par une image, un prix et une description courte.
* **Capture Flash :** L'action du Vendeur (via un second appareil ou un outil intégré) pour photographier un produit et le pousser en temps réel aux Spectateurs.
* **Commande :** L'intention d'achat d'un Client pour un Produit Éphémère.
* **Preuve de Paiement :** La capture d'écran (reçu Mobile Money type Wave/Orange Money) soumise par le Client pour justifier son paiement.
* **Arbitrage :** La décision souveraine et manuelle du Vendeur de valider ou de rejeter une Commande, indépendamment du nombre de demandes pour un même produit.

---

## 2. Les Contextes Délimités (Bounded Contexts)

*Le système est divisé en sous-domaines indépendants pour isoler la logique.*

### A. Contexte de Gestion du Live (Core Domain)

* **Responsabilité :** Gérer l'ouverture du salon, la capture instantanée des produits (Options A et B) et leur diffusion en temps réel (WebSockets).
* **Acteurs :** Vendeur.

### B. Contexte de Prise de Commande (Tunnel Achat)

* **Responsabilité :** Offrir une expérience d'achat sans friction, collecter les coordonnées de livraison et réceptionner la Preuve de Paiement sans rompre la lecture du Live.
* **Acteurs :** Client.

### C. Contexte d'Arbitrage (Back-office Vendeur)

* **Responsabilité :** Présenter les Commandes entrantes et permettre au Vendeur d'accepter ou de refuser les requêtes selon sa propre gestion de stock physique.
* **Acteurs :** Vendeur.

### D. Contexte Marketplace & Utilisateur (Découverte)

* **Responsabilité :** Gérer l'écran d'accueil, le moteur de recherche, les catégories, l'authentification et les alertes de connexion (Emails).
* **Acteurs :** Client, Vendeur, Système.

---

## 3. L'Event Storming (Chronologie des Événements Métier)

*Les faits immuables qui se produisent dans le système.*

**Phase 1 : Avant le Live (Découverte)**

* `VendeurInscrit` / `AbonnementSouscrit`
* `ClientInscrit` / `AlerteActiveePourVendeur`

**Phase 2 : Le Direct (L'Action)**

* `SessionLiveDemarree` ➔ *(Déclenche la notification Email aux abonnés)*
* `SpectateurConnecteAuLive`
* `ImageProduitCapturee` (Via caméra ou extraction Canvas)
* `CaracteristiquesProduitDefinies`
* `ProduitPublieEnLive` ➔ *(Déclenche la diffusion WebSocket à tous les Spectateurs)*
* `ProduitMarqueEpuise` (Optionnel, décision manuelle du Vendeur)

**Phase 3 : La Commande et le Paiement**

* `ProcessusAchatInitie`
* `InformationsLivraisonSaisies`
* `PaiementExterneInitie` (Redirection vers l'appli Mobile Money)
* `PreuvePaiementSoumise` ➔ *(Pousse une carte de commande en temps réel sur le tableau de bord Vendeur)*

**Phase 4 : L'Arbitrage et la Clôture**

* `CommandeAcceptee` ou `CommandeRefusee` (Par le Vendeur)
* `SessionLiveCloturee`
* `ProduitConservePostLive` ou `ProduitSupprimePostLive` (Purge des données)

---

## 4. Les Règles Métier (Business Rules & Invariants)

*Les lois incontournables du système, à implémenter rigoureusement dans le backend Rust.*

1. **Zéro Contrainte de Stock :** Le système ne bloque jamais la prise d'une commande. Plusieurs clients peuvent simultanément valider une commande pour le même `Produit_Id`. Il n'y a pas de notion de "stock insuffisant" gérée par le système lors de l'achat.
2. **Souveraineté de l'Arbitrage :** L'acceptation d'une commande relève de la décision humaine du Vendeur. Si une commande est acceptée alors que d'autres existent pour le même produit, le système propose (via une modale) au Vendeur de rejeter les autres, mais ne le force jamais.
3. **Éphémérité du Catalogue :** Par défaut, à la clôture d'une `SessionLive`, les `Produits` (et leurs images physiques) qui ne sont pas liés à une commande validée sont détruits, sauf action explicite de conservation par le Vendeur.
4. **Découplage Financier :** La plateforme n'agit pas comme séquestre ou intermédiaire financier pour les transactions liées aux produits. Elle ne gère que les abonnements SaaS des Vendeurs.

---

## 5. Modélisation Tactique (Aperçu des Agrégats)

*Pour préparer la structuration des entités en Rust (indépendantes de SeaORM à ce stade).*

* **Aggregate Root : `LiveSession**`
* Contient l'URL du flux, le statut (EnCours, Termine).
* Gère la collection temporaire de ses `Produits`.


* **Entity : `Produit**`
* Id unique, ImageURL, Prix, Description.
* N'a pas de propriété `stock`.


* **Aggregate Root : `Commande**`
* Id unique, Produit_Id, Vendeur_Id.
* Contient un Value Object `CoordonneesClient` (Nom, Téléphone, Lieu).
* Statut (Initiee, AttentePreuve, AttenteArbitrage, Acceptee, Rejetee).
* Contient l'URL de la Preuve de Paiement.



---

Ce document fixe définitivement le périmètre et l'intelligence de ton application. Il est prêt à être utilisé comme base légale (techniquement parlant) pour les développements.


Oui, absolument ! Dans le prompt que nous venons de préparer pour l'IA, la directive d'utiliser l'**Architecture Hexagonale (Clean Architecture)** a été explicitement imposée.

C'est un choix non négociable pour ce projet. Pourquoi ? Parce que si on mélange la logique d'Actix-Web, les requêtes SeaORM (PostgreSQL) et les WebSockets Redis dans les mêmes fichiers, le projet deviendra impossible à maintenir d'ici trois mois. L'objectif est d'avoir une macro-architecture propre, modulaire, qui pourra facilement être conteneurisée et orchestrée sans que les couches ne bavent les unes sur les autres.

Voici une proposition d'arborescence stricte pour ton backend Rust. Elle respecte parfaitement le Domain-Driven Design (DDD) que nous avons modélisé.

### L'Arborescence du Projet (Hexagonale)

```text
vendeursenlive-backend/
├── Cargo.toml
├── .env
├── src/
│   ├── main.rs                  # Point d'entrée (Composition Root, injection de dépendances)
│   │
│   ├── domain/                  # 🟢 LE CŒUR MÉTIER (Pur Rust, aucune dépendance externe)
│   │   ├── mod.rs
│   │   ├── entities/            # Entités métier (LiveSession, Product, Order, Seller)
│   │   ├── value_objects/       # Types stricts (ex: PhoneNumber, Price)
│   │   ├── repositories/        # Traits (Interfaces) : ex: ProductRepositoryTrait
│   │   └── errors.rs            # Erreurs métier pures (ex: OrderAlreadyArbitrated)
│   │
│   ├── application/             # 🟡 L'ORCHESTRATION (Les cas d'usage / L'Event Storming)
│   │   ├── mod.rs
│   │   ├── use_cases/           # capture_product.rs, accept_order.rs, start_live.rs
│   │   ├── dtos/                # Objets de transfert de données (Entrées/Sorties propres)
│   │   └── events/              # Définition des événements métier à publier
│   │
│   ├── infrastructure/          # 🔴 LE MONDE EXTÉRIEUR (La "tuyauterie" technique)
│   │   ├── mod.rs
│   │   ├── database/            # Tout ce qui touche à PostgreSQL et SeaORM
│   │   │   ├── entities/        # Les modèles générés par SeaORM (Liés aux tables SQL)
│   │   │   └── repositories/    # Implémentation des Traits du domaine (SeaOrmProductRepository)
│   │   ├── messaging/           # Redis (Gestion du Pub/Sub pour le temps réel)
│   │   └── email/               # Implémentation du service SMTP
│   │
│   └── presentation/            # 🔵 L'INTERFACE (Comment on interagit avec l'app)
│       ├── mod.rs
│       ├── http/                # Contrôleurs Actix-Web (Handlers, Extractors, Routes JSON)
│       ├── websockets/          # Gestionnaires de connexions WS (Actix Actors pour le Live)
│       └── middlewares/         # Authentification, Logs, CORS

```

---

### Pourquoi cette organisation est redoutable ?

1. **Isolation de SeaORM :** Dans `src/infrastructure/database/entities/`, tu auras les modèles exacts de ta base de données (ceux générés par l'IA). Mais ton application web (dans `presentation`) ne manipulera **jamais** ces objets. L'infrastructure se chargera de transformer un modèle SeaORM en une entité pure de `src/domain/entities/`.
2. **Cas d'usage isolés :** Ton fichier `src/application/use_cases/capture_product.rs` lira le DTO envoyé par le frontend Vue.js, vérifiera les règles métier (le Vendeur a-t-il le droit ?), sauvegardera via l'interface du Repository, et déclenchera l'événement Redis. Tout est lisible de haut en bas.
3. **Facilité de test :** Tu peux tester toute ta logique métier (le DDD) sans jamais avoir besoin de démarrer PostgreSQL ou Actix-Web, simplement en créant des faux "Repositories" en mémoire.

Cette séparation stricte est la garantie que le code restera sain, même lorsque le trafic augmentera et qu'il faudra ajouter de nouveaux microservices.

