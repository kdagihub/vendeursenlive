**Mon avis honnête**
Le refus TikTok détruit le cœur de la V1 : regarder le LIVE et commander au même endroit. Gemini a raison sur le diagnostic, mais exagère lorsqu’il affirme que le deep-linking conserve une « valeur ajoutée maximale ». Basculer entre TikTok et VendeursEnLive reproduirait en partie la friction de WhatsApp.

En revanche, ton nouveau pivot est cohérent à condition de l’assumer comme **un autre produit** :

> VendeursEnLive devient d’abord l’annuaire fiable des vendeurs ivoiriens actuellement en direct.

Ce positionnement peut constituer une porte d’entrée, mais il faut le valider avant de développer les commandes et les catalogues.

**La vraie valeur initiale**
Le premier problème résolu serait :

- trouver des vendeurs par catégorie et commune ;
- savoir lesquels sont réellement en direct ;
- découvrir des vendeurs que l’algorithme TikTok ne montre pas ;
- recevoir une alerte lorsqu’un vendeur suivi démarre ;
- ouvrir immédiatement son LIVE TikTok ;
- consulter une fiche permanente avec ses coordonnées et horaires habituels.

Ce n’est pas encore du live commerce intégré, mais c’est plus utile qu’un simple catalogue statique.

**Le point critique**
TikTok ne fournit pas actuellement à ton application un moyen général et fiable de connaître tous les vendeurs en direct.

Le scope `video.list` concerne les **vidéos publiées par un utilisateur ayant autorisé ton application**. Il ne donne pas :

- la liste arbitraire des vendeurs TikTok ;
- leur statut LIVE en temps réel ;
- l’autorisation de lire leur LIVE ;
- l’autorisation de republier leurs vidéos sans consentement.

Il faut donc construire ton propre mécanisme déclaratif :

1. Le vendeur crée et fait vérifier son profil.
2. Il renseigne son identifiant TikTok, sa catégorie et sa commune.
3. Lorsqu’il démarre, il clique sur **Je suis en LIVE** et colle son lien.
4. L’annonce devient active pendant une durée limitée, par exemple deux heures.
5. Sans prolongation ou confirmation du vendeur, elle expire automatiquement.
6. La carte indique honnêtement : **« Déclaré en direct à 18 h 42 »**.
7. Un clic ouvre le LIVE dans TikTok.

Cela évite d’afficher comme actif un LIVE déjà terminé.

**MVP conseillé**
Je ne construirais pas encore la boutique permanente, les paiements ou les commandes. Je testerais pendant deux à quatre semaines :

- 20 à 30 vendeurs sélectionnés manuellement ;
- profils vérifiés ;
- catégories et communes ;
- activation et expiration des annonces LIVE ;
- favoris et alertes ;
- redirection vers TikTok ;
- statistiques de clics pour chaque vendeur.

Les métriques déterminantes seront :

- vendeurs actifs chaque semaine ;
- nombre de LIVE déclarés ;
- clics vers TikTok ;
- visiteurs qui reviennent ;
- abonnements aux alertes ;
- vendeurs qui demandent spontanément plus de visibilité.

Si personne ne revient régulièrement, il faudra arrêter avant d’investir davantage. Si l’usage apparaît, les boutiques et catalogues auront alors une demande réelle.

**Modèle économique progressif**
Le modèle pourrait ensuite devenir :

- profil vendeur gratuit ;
- mise en avant sponsorisée ;
- badge vendeur vérifié ;
- statistiques de consultation et de clics ;
- alertes prioritaires aux abonnés ;
- boutique permanente payante ;
- collecte structurée des commandes plus tard.

Le futur **Live Photo** peut devenir une fonctionnalité propriétaire intéressante, mais il faut la différer jusqu’à ce que l’annuaire possède déjà une audience.

**Impact sur le code actuel**
Une grande partie reste utile :

- authentification téléphone/email/Google ;
- profils clients et vendeurs ;
- recherche et vue Explorer ;
- catégories et communes ;
- sessions LIVE et validation des liens ;
- navigation responsive ;
- infrastructure PostgreSQL/Redis.

Il faudra transformer :

- le lecteur TikTok en bouton **Voir le LIVE sur TikTok** ;
- la salle `/live/:id` en fiche de session/vendeur ;
- les cartes vidéo en cartes vendeur avec avatar ou image autorisée ;
- la session LIVE en annonce temporaire avec expiration ;
- les textes légaux qui annoncent encore un lecteur intégré.

Je conserverais le nom **VendeursEnLive** : il correspond toujours parfaitement à l’annuaire. Le prochain travail solide est de réécrire le CDC autour de ce nouveau périmètre avant de modifier davantage le code.