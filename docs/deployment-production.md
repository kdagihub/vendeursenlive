# Déploiement Production

Domaines cibles:

- Frontend: `https://vendeursenlive.shop`
- Backend API: `https://api.vendeursenlive.shop`

URLs TikTok conservées pour une réactivation ultérieure du Login Kit:

- Website URL: `https://vendeursenlive.shop`
- Terms of Service URL: `https://vendeursenlive.shop/terms`
- Privacy Policy URL: `https://vendeursenlive.shop/privacy`
- Redirect URI: `https://api.vendeursenlive.shop/auth/tiktok/callback`

## Variables Dokploy

Copier `deploy/production.env.example` dans les variables d'environnement Dokploy et
remplacer toutes les valeurs `change-me` / `replace-with`.

Ne pas committer de fichier `.env` réel. Le `.gitignore` racine ignore les fichiers
d'environnement privés.

Points obligatoires:

- `APP_ENV=production`
- `AUTH_COOKIE_SECURE=true`
- `AUTH_COOKIE_DOMAIN=.vendeursenlive.shop`
- `CORS_ALLOWED_ORIGINS=https://vendeursenlive.shop`
- `EMAIL_DELIVERY_ENABLED=true`
- `SMTP_HOST=smtp.hostinger.com`
- `SMTP_PORT=465`
- `SMTP_USERNAME=contact@vendeursenlive.shop`
- `SMTP_PASSWORD=<hostinger-smtp-password>`
- `SMTP_FROM_EMAIL=contact@vendeursenlive.shop`
- `SMTP_FROM_NAME=VendeursEnLive`
- `PASSWORD_RESET_URL=https://vendeursenlive.shop/reset-password`
- `EMAIL_VERIFICATION_URL=https://vendeursenlive.shop/verify-email`
- `EMAIL_VERIFICATION_TOKEN_TTL_SECONDS=86400`
- `EMAIL_VERIFICATION_RESEND_COOLDOWN_SECONDS=60`
- `IKODDI_ENABLED=true`
- `IKODDI_BASE_URL=https://api.ikoddi.com`
- `IKODDI_API_KEY=<secret-ikoddi>`
- `IKODDI_ORGANIZATION_ID=<organisation-ikoddi>`
- `IKODDI_OTP_APP_ID=<application-otp-ikoddi>`
- `OTP_CHALLENGE_TTL_SECONDS=300`
- `OTP_RESEND_COOLDOWN_SECONDS=60`
- `OTP_MAX_ATTEMPTS=5`
- `GOOGLE_CLIENT_ID=<google-web-client-id>`
- `GOOGLE_CLIENT_SECRET=<google-web-client-secret>`
- `GOOGLE_REDIRECT_URI=https://api.vendeursenlive.shop/auth/google/callback`
- `GOOGLE_SCOPES=openid,email,profile`
- `GOOGLE_SUCCESS_REDIRECT_URL=https://vendeursenlive.shop/`

## Services Dokploy VEL

Le projet Dokploy `VEL` utilise des services managés séparés:

```text
Postgres service: vel-postgres-7dverw
Postgres database: vel_db
Postgres user: postgres_vel
Postgres port: 5432

Redis service: vel-redis-xju9f1
Redis user: default
Redis port: 6379
```

Le backend doit donc recevoir:

```env
DATABASE_URL=postgres://postgres_vel:<postgres-password>@vel-postgres-7dverw:5432/vel_db
REDIS_URL=redis://default:<redis-password>@vel-redis-xju9f1:6379
```

Ces variables sont à renseigner dans le service `backend` Dokploy. Le service
`frontend` n'a pas besoin de `DATABASE_URL` ni de `REDIS_URL`.

Le mot de passe SMTP doit rester uniquement dans les variables d'environnement
Dokploy. S'il a été partagé ou copié dans un document, le régénérer avant usage.

Si un mot de passe contient des caractères spéciaux (`@`, `:`, `/`, `#`, `%`, etc.),
il faut l'encoder pour une URL avant de le mettre dans `DATABASE_URL` ou `REDIS_URL`.
Exemple: `@` devient `%40`.

Il faut utiliser les credentials internes Dokploy. Les ports externes Postgres/Redis
ne sont pas nécessaires pour que le backend communique avec ces services depuis le
projet `VEL`.

Dans Dokploy, il suffit de créer les services applicatifs `backend` et `frontend`
depuis les images Docker Hub. Dokploy orchestre ensuite les conteneurs via son
Swarm interne. Aucun compose spécifique Dokploy n'est nécessaire.

`docker-compose.prod.yml` reste disponible uniquement comme référence de stack
autonome, ou pour un déploiement hors Dokploy qui lancerait aussi Postgres et Redis.

## Images DockerHub

Exemple de tags à utiliser:

```sh
docker build -t ciacems/vel:backend-latest ./vendeursenlive-backend
docker build -t ciacems/vel:frontend-latest ./vel-frontend

docker push ciacems/vel:backend-latest
docker push ciacems/vel:frontend-latest
```

Puis renseigner dans Dokploy:

```text
BACKEND_IMAGE=docker.io/ciacems/vel:backend-latest
FRONTEND_IMAGE=docker.io/ciacems/vel:frontend-latest
```

Le frontend sait automatiquement utiliser `https://api.vendeursenlive.shop` quand il
est ouvert depuis `https://vendeursenlive.shop`. Si tu veux figer l'URL au build:

```sh
docker build \
  --build-arg VITE_API_BASE_URL=https://api.vendeursenlive.shop \
  -t ciacems/vel:frontend-latest \
  ./vel-frontend
```

## Cookies et CSRF

Le frontend et l'API sont sur deux sous-domaines différents. Le cookie CSRF doit donc
être posé sur `.vendeursenlive.shop` pour que le frontend puisse lire `vel_csrf_token`
et le recopier dans le header `X-CSRF-Token`.

## IKODDI OTP

Créer une clé API limitée à la permission « Demander et vérifier un OTP ». La clé,
l’identifiant d’organisation et l’identifiant de l’application OTP doivent être injectés
uniquement dans le service backend Dokploy. Ne jamais les exposer dans les variables Vite.

Valider d’abord le parcours avec `https://api.staging.ikoddi.com`, puis utiliser
`https://api.ikoddi.com` en production. Redis doit rester disponible : il conserve le challenge
opaque pendant cinq minutes et empêche les renvois trop rapprochés.

## Google OAuth

Dans Google Cloud Console, utiliser un client OAuth de type **Application Web** et enregistrer
exactement l’URI de redirection autorisée suivante:

```text
https://api.vendeursenlive.shop/auth/google/callback
```

Configurer l’écran de consentement avec le nom `VendeursEnLive`, le domaine autorisé
`vendeursenlive.shop`, la page d’accueil, les CGU et la politique de confidentialité déjà publiées.
Les seuls scopes demandés sont `openid`, `email` et `profile`.

Le client ID et le secret sont injectés uniquement dans le service backend Dokploy. Aucun secret
Google ne doit être placé dans une variable `VITE_*`. Après modification des variables, redéployer
le backend afin que sa configuration soit relue.

L’authentification actuelle utilise le flux OAuth 2.0 **Application de serveur Web** : Vue redirige
vers `/auth/google/start`, puis Rust échange le code et récupère le profil OpenID Connect. Le SDK
JavaScript Google Identity Services et Google One Tap ne sont pas chargés pour ce parcours. Ils
restent une évolution optionnelle qui nécessiterait un endpoint distinct chargé de valider le JWT
d’identité Google reçu par le navigateur.

Le bouton français utilise le logo officiel non modifié du pack Google Android + Web. Google
autorise la localisation du libellé ; conserver « Continuer avec Google » et les variantes officielles
claire/sombre du logo.

## TikTok (en veille)

Le Login Kit n’est plus exposé dans l’interface. Les routes sont conservées pour une activation
ultérieure et les variables `TIKTOK_*` peuvent rester absentes. Lors de la réactivation, régénérer
le secret et enregistrer exactement:

```text
https://api.vendeursenlive.shop/auth/tiktok/callback
```

L'URL doit être HTTPS, statique, sans query string et sans fragment.

Configurer uniquement le scope nécessaire:

```text
user.info.basic
```

Le backend utilise:

```env
TIKTOK_AUTH_URL=https://www.tiktok.com/v2/auth/authorize/
TIKTOK_TOKEN_URL=https://open.tiktokapis.com/v2/oauth/token/
TIKTOK_USER_INFO_URL=https://open.tiktokapis.com/v2/user/info/
TIKTOK_SUCCESS_REDIRECT_URL=https://vendeursenlive.shop/
```

## Vidéo demo TikTok (à conserver pour une reprise de la review)

TikTok demande une vidéo montrant le flux complet sur le domaine web déclaré. Pour
la review de VendeursEnLive, enregistrer une vidéo courte, claire, en MP4 ou MOV,
avec ce scénario:

1. Ouvrir `https://vendeursenlive.shop/login`.
2. Montrer le bouton `Continuer avec TikTok`.
3. Cliquer sur le bouton.
4. Montrer la redirection vers TikTok.
5. Autoriser le scope `user.info.basic`.
6. Montrer le retour automatique vers `https://vendeursenlive.shop/`.
7. Montrer que l'utilisateur est connecté dans VendeursEnLive.
8. Cliquer sur `Déconnexion`.

La vidéo doit montrer l'interface réelle du site `vendeursenlive.shop`, car ce
domaine est celui fourni dans `Web/Desktop URL`.
