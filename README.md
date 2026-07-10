# VendeursEnLive

Plateforme de live commerce hybride avec backend Rust/Actix-Web et frontend Vue/PrimeVue.

## Build et Push Docker Hub

Registre Docker Hub cible:

```sh
ciacems/vel
```

Tags utilisés:

```text
ciacems/vel:backend-latest
ciacems/vel:frontend-latest
```

### Option A : build local + push Docker Hub

À utiliser quand le quota du builder cloud est atteint.

Créer une seule fois un builder local multi-plateforme:

```sh
docker buildx create --name vel-local --driver docker-container --use \
  --driver-opt network=host 2>/dev/null || docker buildx use vel-local
```

S'authentifier sur Docker Hub:

```sh
docker login
```

Si la machine n'est pas `amd64`, activer QEMU pour cross-compiler vers `linux/amd64`:

```sh
docker run --privileged --rm tonistiigi/binfmt --install amd64
```

Build des deux images en local sans cache, puis push:

```sh
docker buildx bake -f docker-compose.build.yml \
  --builder vel-local --no-cache --push
```

Build des deux images en local avec cache, puis push:

```sh
docker buildx bake -f docker-compose.build.yml \
  --builder vel-local --push
```

### Option B : build cloud

À utiliser quand le quota du builder cloud est disponible.

```sh
docker buildx bake -f docker-compose.build.yml \
  --builder cloud-ciacems-ciacems-builder --push
```

Build uniquement le frontend:

```sh
docker buildx bake -f docker-compose.build.yml frontend \
  --builder cloud-ciacems-ciacems-builder --push
```

Build uniquement le backend:

```sh
docker buildx bake -f docker-compose.build.yml backend \
  --builder cloud-ciacems-ciacems-builder --push
```

### Variables de build utiles

Par défaut, le frontend utilise l'API de production:

```sh
VITE_API_BASE_URL=https://api.vendeursenlive.shop
```

Pour changer les tags au moment du build:

```sh
BACKEND_IMAGE=ciacems/vel:backend-2026-07-10 \
FRONTEND_IMAGE=ciacems/vel:frontend-2026-07-10 \
docker buildx bake -f docker-compose.build.yml \
  --builder cloud-ciacems-ciacems-builder --push
```

## Déploiement

Pour Dokploy, créer les services `backend` et `frontend` directement depuis les images
Docker Hub. Dokploy orchestre les conteneurs via son Swarm interne; il n'y a pas besoin
d'un compose dédié à Dokploy.

La stack production autonome, qui lance aussi Postgres et Redis, reste disponible comme
référence ou pour un déploiement hors Dokploy:

```sh
docker-compose.prod.yml
```

Variables Dokploy:

```sh
deploy/production.env.example
```

Documentation de déploiement:

```sh
docs/deployment-production.md
```


# ##################################### ACCÈS EN BASE DE DONNÉES POSGRES SUR DOKPLOY LE VPS #################"""""
psql -U postgres_vel -d vel_db -P pager=off -c "
SELECT
  u.id,
  u.full_name,
  u.status,
  u.is_admin,
  i.provider,
  i.email,
  i.phone_number,
  i.provider_subject,
  u.created_at
FROM users u
LEFT JOIN user_auth_identities i ON i.user_id = u.id
ORDER BY u.created_at DESC;
"

# Si tu veux supprimer ce user et repartir propre pour les tests TikTok :
```psql -U postgres_vel -d vel_db -c "DELETE FROM users;"```