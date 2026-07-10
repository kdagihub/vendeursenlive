# VendeursEnLive Frontend

Frontend Vue 3 de VendeursEnLive, construit avec PrimeVue, Pinia, Vue Router et Axios.

## Configuration

Créer un fichier `.env` à partir de `.env.example`:

```sh
cp .env.example .env
```

Variable principale:

```sh
VITE_API_BASE_URL=http://localhost:8080
```

En local, il faut garder le même host entre le frontend et le backend pour que le
cookie CSRF soit lisible par le navigateur. Si tu ouvres `http://localhost:5173`,
utilise `http://localhost:8080`. Si tu ouvres `http://127.0.0.1:5173`, utilise
`http://127.0.0.1:8080`.

En production, elle devra pointer vers:

```sh
VITE_API_BASE_URL=https://api.vendeursenlive.shop
```

## Parcours disponibles

- `/login`: inscription, connexion email/téléphone et départ OAuth TikTok.
- `/reset-password`: demande et confirmation de réinitialisation.
- `/app`: espace authentifié minimal avec session, refresh, logout et changement de mot de passe.
- `/terms`: conditions générales d’utilisation.
- `/privacy`: politique de confidentialité.

Axios envoie les cookies avec `withCredentials` et injecte automatiquement le header
`X-CSRF-Token` sur les routes protégées.

## Commandes

```sh
npm install
npm run dev
npm run build
npm run test:unit -- --run
npm run lint
```

## Docker

L'image Docker construit l'application Vue puis sert les fichiers statiques avec Nginx.

```sh
docker compose build frontend
docker compose up -d frontend
```

En local, le service compose expose le frontend sur `http://localhost:5173`.
