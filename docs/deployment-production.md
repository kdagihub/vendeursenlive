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
- `AUTH_EXPOSE_PASSWORD_RESET_TOKEN=false`
- `TIKTOK_REDIRECT_URI=https://api.vendeursenlive.shop/auth/tiktok/callback`

## Images DockerHub

Exemple de tags à utiliser:

```sh
docker build -t your-dockerhub-user/vendeursenlive-backend:latest ./vendeursenlive-backend
docker build -t your-dockerhub-user/vendeursenlive-frontend:latest ./vel-frontend

docker push your-dockerhub-user/vendeursenlive-backend:latest
docker push your-dockerhub-user/vendeursenlive-frontend:latest
```

Puis renseigner dans Dokploy:

```text
BACKEND_IMAGE=docker.io/your-dockerhub-user/vendeursenlive-backend:latest
FRONTEND_IMAGE=docker.io/your-dockerhub-user/vendeursenlive-frontend:latest
```

Le frontend sait automatiquement utiliser `https://api.vendeursenlive.shop` quand il
est ouvert depuis `https://vendeursenlive.shop`. Si tu veux figer l'URL au build:

```sh
docker build \
  --build-arg VITE_API_BASE_URL=https://api.vendeursenlive.shop \
  -t your-dockerhub-user/vendeursenlive-frontend:latest \
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
