# Déploiement Production

Domaines cibles:

- Frontend: `https://vendeursenlive.shop`
- Backend API: `https://api.vendeursenlive.shop`

URLs à fournir à TikTok Developer Console:

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
- `TIKTOK_REDIRECT_URI=https://api.vendeursenlive.shop/auth/tiktok/callback`
- `TIKTOK_SUCCESS_REDIRECT_URL=https://vendeursenlive.shop/app`

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

## TikTok

Le secret TikTok déjà partagé pendant le développement doit être régénéré avant la
mise en production. Dans TikTok Developer Console, enregistrer exactement:

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
TIKTOK_SUCCESS_REDIRECT_URL=https://vendeursenlive.shop/app
```

## Vidéo demo TikTok

TikTok demande une vidéo montrant le flux complet sur le domaine web déclaré. Pour
la review de VendeursEnLive, enregistrer une vidéo courte, claire, en MP4 ou MOV,
avec ce scénario:

1. Ouvrir `https://vendeursenlive.shop/login`.
2. Montrer le bouton `Continuer avec TikTok`.
3. Cliquer sur le bouton.
4. Montrer la redirection vers TikTok.
5. Autoriser le scope `user.info.basic`.
6. Montrer le retour automatique vers `https://vendeursenlive.shop/app`.
7. Montrer que l'utilisateur est connecté dans VendeursEnLive.
8. Cliquer sur `Déconnexion`.

La vidéo doit montrer l'interface réelle du site `vendeursenlive.shop`, car ce
domaine est celui fourni dans `Web/Desktop URL`.
