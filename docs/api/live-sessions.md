# API vendeur - Sessions LIVE TikTok

Base URL de production: `https://api.vendeursenlive.shop`

Les endpoints nécessitent une session VendeursEnLive valide, un compte vendeur
vérifié et le cookie CSRF accompagné du header `X-CSRF-Token` pour les requêtes
POST.

## Démarrer un LIVE

`POST /seller/lives`

```json
{
  "tiktok_live_url": "https://www.tiktok.com/@vendeursenlive/live"
}
```

Les liens TikTok courts partagés sont acceptés lorsqu'ils peuvent être résolus
vers une page LIVE canonique. Les redirections sont limitées aux domaines TikTok
explicitement autorisés par le backend.

Réponse `201 Created`:

```json
{
  "id": "019f0000-0000-7000-8000-000000000001",
  "tiktok_live_url": "https://www.tiktok.com/@vendeursenlive/live",
  "tiktok_username": "vendeursenlive",
  "status": "ongoing",
  "created_at": "2026-08-03T10:00:00Z",
  "ended_at": null
}
```

Erreurs principales:

- `400`: lien non TikTok, lien ne désignant pas un LIVE ou nom de boutique absent;
- `401`: session absente ou expirée;
- `403`: compte non vérifié ou profil non vendeur;
- `409`: un LIVE est déjà actif pour cette boutique;
- `503`: lien court temporairement impossible à résoudre auprès de TikTok.

## Consulter le LIVE courant

`GET /seller/lives/current`

- `200`: retourne la session en cours;
- `204`: aucune session en cours.

## Terminer un LIVE

`POST /seller/lives/{live_session_id}/end`

Réponse `200`: retourne la session avec `status: "ended"` et `ended_at`
renseigné.

Le lecteur TikTok reste contrôlé séparément par
`VITE_TIKTOK_LIVE_EMBED_ENABLED`. Cette variable doit rester à `false` tant que
le domaine frontend n'a pas été autorisé par TikTok LIVE Embed.
