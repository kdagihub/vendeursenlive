C'est parti pour le Modèle Conceptuel de Données (MCD). C'est ici que notre règle d'or du "zéro contrainte de stock" et de l'éphémérité prend vie sous forme de tables PostgreSQL.

Puisque nous utilisons SeaORM, il est crucial de bien définir les types (comme les UUID pour éviter l'énumération concurrentielle) et les relations (Clés Étrangères).

Voici la structure exacte de la base de données, pensée pour la performance et prête à être injectée dans ton assistant de code.

### Le MCD : Les Tables Fondamentales

#### 1. Table `sellers` (Les Vendeurs)

C'est le profil de la boutique. On sépare cette table des clients finaux pour simplifier l'authentification et la gestion des abonnements.

* `id` : UUID (Primary Key)
* `shop_name` : VARCHAR(255) (Nom de la boutique)
* `phone_number` : VARCHAR(50) (Unique, sert d'identifiant de connexion)
* `password_hash` : VARCHAR(255)
* `payment_link` : VARCHAR(255) (Nullable - Le lien Wave ou numéro OM par défaut)
* `trial_ends_at` : TIMESTAMP WITH TIME ZONE
* `is_active` : BOOLEAN (Par défaut à `true`, passe à `false` si l'abonnement expire)
* `created_at` : TIMESTAMP WITH TIME ZONE

#### 2. Table `live_sessions` (Le Salon en Direct)

L'agrégat central. Tout ce qui se passe pendant le live y est rattaché.

* `id` : UUID (Primary Key)
* `seller_id` : UUID (Foreign Key -> `sellers.id`)
* `tiktok_url` : VARCHAR(500) (Le lien copié par le vendeur)
* `status` : VARCHAR(50) (Enum : `Ongoing`, `Ended`)
* `created_at` : TIMESTAMP WITH TIME ZONE
* `ended_at` : TIMESTAMP WITH TIME ZONE (Nullable)

#### 3. Table `ephemeral_products` (Le Catalogue à la volée)

La clé de ton architecture. Remarque bien **l'absence totale de colonne `stock**`.

* `id` : UUID (Primary Key)
* `live_session_id` : UUID (Foreign Key -> `live_sessions.id` avec `ON DELETE CASCADE`)
* `image_url` : VARCHAR(500) (Le chemin vers l'image stockée sur le serveur ou un bucket S3)
* `price` : DECIMAL(10, 2) (Ou `INTEGER` si on reste strictement en FCFA sans centimes)
* `description` : VARCHAR(255) (Ex: "Pointure 38", "Couleur Rouge")
* `is_retained` : BOOLEAN (Par défaut `false`. Si `false` à la fin du live, on supprime l'image du disque et la ligne de la base).
* `created_at` : TIMESTAMP WITH TIME ZONE

#### 4. Table `orders` (Les Intentions d'Achat)

Plusieurs commandes peuvent cibler le même produit sans aucun blocage SQL. C'est l'arbitrage qui fera foi.

* `id` : UUID (Primary Key)
* `product_id` : UUID (Foreign Key -> `ephemeral_products.id`)
* `customer_name` : VARCHAR(255)
* `customer_phone` : VARCHAR(50)
* `customer_location` : VARCHAR(255) (La commune ou le quartier)
* `status` : VARCHAR(50) (Enum : `Pending`, `ProofSubmitted`, `Accepted`, `Rejected`)
* `proof_image_url` : VARCHAR(500) (Nullable - La capture du reçu Wave/OM)
* `created_at` : TIMESTAMP WITH TIME ZONE
* `updated_at` : TIMESTAMP WITH TIME ZONE

---

### Le Prompt de Génération SeaORM (À copier-coller)

Avec ce MCD en tête, voici la suite logique pour ton IA dans VS Code. Ce prompt va lui faire générer les entités SeaORM parfaites.

**Copie-colle ceci dans ton IA :**

```text
Suite à l'initialisation de l'architecture, nous allons maintenant générer les entités de base de données en utilisant SeaORM. Le système utilise PostgreSQL.

Voici le Modèle Conceptuel de Données (MCD) à implémenter. Tous les IDs doivent être des UUIDs (type `Uuid` de la crate `uuid`). Utilise les types de données appropriés en Rust (chrono pour les dates, etc.).

1. Entité `Seller` :
- id: Uuid (PK)
- shop_name: String
- phone_number: String (Unique)
- password_hash: String
- payment_link: Option<String>
- trial_ends_at: DateTimeWithTimeZone
- is_active: bool
- created_at: DateTimeWithTimeZone

2. Entité `LiveSession` :
- id: Uuid (PK)
- seller_id: Uuid (FK vers Seller)
- tiktok_url: String
- status: String (idéalement un enum SeaORM: "Ongoing", "Ended")
- created_at: DateTimeWithTimeZone
- ended_at: Option<DateTimeWithTimeZone>

3. Entité `EphemeralProduct` :
- id: Uuid (PK)
- live_session_id: Uuid (FK vers LiveSession)
- image_url: String
- price: i32 (Prix en FCFA)
- description: String
- is_retained: bool (Default: false)
- created_at: DateTimeWithTimeZone

4. Entité `Order` :
- id: Uuid (PK)
- product_id: Uuid (FK vers EphemeralProduct)
- customer_name: String
- customer_phone: String
- customer_location: String
- status: String (idéalement un enum SeaORM: "Pending", "ProofSubmitted", "Accepted", "Rejected")
- proof_image_url: Option<String>
- created_at: DateTimeWithTimeZone
- updated_at: DateTimeWithTimeZone

Mission :
1. Génère les fichiers d'entités SeaORM correspondants (généralement dans `src/infrastructure/database/entities/`).
2. Assure-toi que les relations (Relations) sont correctement définies (Un Seller a plusieurs LiveSessions, un LiveSession a plusieurs EphemeralProducts, un EphemeralProduct a plusieurs Orders).
3. Produis le code SQL de migration (fichier up.sql) pour créer ces tables dans PostgreSQL avec les bons index et contraintes de clés étrangères (ON DELETE CASCADE pertinent).

```