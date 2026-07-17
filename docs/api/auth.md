# API Auth Frontend

Base locale: `http://127.0.0.1:8080`

Production:
- Frontend: `https://vendeursenlive.shop`
- Backend: `https://api.vendeursenlive.shop`

Toutes les requetes frontend doivent envoyer les cookies:

```js
fetch(url, {
  credentials: 'include'
})
```

Avec Axios:

```js
axios.defaults.withCredentials = true
```

## Cookies

| Cookie | HttpOnly | Usage |
| --- | --- | --- |
| `vel_access_token` | Oui | JWT court utilise par `/auth/me` et les routes protegees |
| `vel_refresh_token` | Oui | Refresh token opaque |
| `vel_refresh_session` | Oui | ID de session refresh |
| `vel_csrf_token` | Non | Token a recopier dans le header `X-CSRF-Token` |

## CSRF

Avant les requetes mutantes protegees, recuperer un token CSRF:

```http
GET /auth/csrf
```

Reponse: `204 No Content`

Ensuite envoyer:

```http
X-CSRF-Token: <valeur du cookie vel_csrf_token>
```

Routes mutantes qui exigent le header CSRF:
- `POST /auth/refresh`
- `POST /auth/logout`
- `POST /auth/change-password`
- `POST /auth/email-verification/request`

Les routes d'inscription, login et reset password public n'exigent pas le header CSRF.

## Endpoints

### Health

```http
GET /health
```

Reponse `200`:

```json
{
  "status": "ok",
  "service": "vendeursenlive-backend"
}
```

### Register

```http
POST /auth/register
```

Client:

```json
{
  "full_name": "Awa Client",
  "email": "awa@example.com",
  "password": "Password123!",
  "account_type": "customer",
  "default_location": "Cocody"
}
```

Vendeur:

```json
{
  "full_name": "Awa Seller",
  "phone_number": "+2250700000000",
  "password": "Password123!",
  "account_type": "seller",
  "shop_name": "Boutique Awa",
  "payment_link": "https://pay.wave.com/..."
}
```

Reponse `201`: les cookies auth sont poses.

```json
{
  "user_id": "uuid",
  "session_id": "uuid",
  "token_type": "Bearer",
  "expires_in_seconds": 900,
  "is_seller": false,
  "is_admin": false,
  "account_verified": false,
  "verification_channel": "email"
}
```

### Login

```http
POST /auth/login
```

```json
{
  "identifier": "awa@example.com",
  "password": "Password123!"
}
```

`identifier` peut etre un email ou un numero de telephone.

Reponse `200`: les cookies auth sont poses.

### Current User

```http
GET /auth/me
```

Reponse `200`:

```json
{
  "user_id": "uuid",
  "session_id": "uuid",
  "is_seller": false,
  "is_admin": false,
  "account_verified": false,
  "verification_channel": "email"
}
```

Une identite TikTok renvoie toujours `account_verified: true`. Une inscription email
reste connectee mais les actions metier protegees sont refusees jusqu'a la verification.

### Refresh

```http
POST /auth/refresh
X-CSRF-Token: <vel_csrf_token>
```

Reponse `200`: rotation de session, nouveaux cookies poses.

### Logout

```http
POST /auth/logout
X-CSRF-Token: <vel_csrf_token>
```

Reponse `204`: session revoquee et cookies supprimes.

### Change Password

```http
POST /auth/change-password
X-CSRF-Token: <vel_csrf_token>
```

```json
{
  "current_password": "Password123!",
  "new_password": "Password456!"
}
```

Reponse `204`.

### Request Password Reset

```http
POST /auth/password-reset/request
```

```json
{
  "email": "awa@example.com"
}
```

Reponse `202`:

```json
{
  "message": "if the account exists, password reset instructions will be sent"
}
```

Le backend ne renvoie jamais le token de reset. Si le compte existe, un email est
envoye avec un lien vers `/reset-password?token=...`.

### Confirm Password Reset

```http
POST /auth/password-reset/confirm
```

```json
{
  "reset_token": "token-recu-par-email",
  "new_password": "Password789!"
}
```

Reponse `204`: mot de passe change, toutes les sessions de l'utilisateur sont revoquees.

### Request Email Verification

```http
POST /auth/email-verification/request
X-CSRF-Token: <vel_csrf_token>
```

Le compte doit etre authentifie avec une identite email non verifiee. L'adresse est
deduite de la session et ne doit pas etre envoyee par le frontend.

Reponse `202`:

```json
{
  "message": "email verification instructions were sent"
}
```

### Confirm Email Verification

```http
POST /auth/email-verification/confirm
```

```json
{
  "verification_token": "token-recu-par-email"
}
```

Reponse `204`. Le frontend appelle ensuite `/auth/refresh` pour recevoir un JWT avec
`account_verified: true`.

## TikTok Login

### Start

```http
GET /auth/tiktok/start
```

Redirige vers TikTok avec:
- `client_key`
- `scope`
- `redirect_uri`
- `state`
- `response_type=code`

### Callback

```http
GET /auth/tiktok/callback?code=...&state=...
```

Le backend:
- valide le `state` anti-CSRF OAuth
- echange le `code` contre un access token TikTok
- appelle `https://open.tiktokapis.com/v2/user/info/` avec le scope `user.info.basic`
- retrouve ou cree une identite `TikTok`
- cree une session VendeursEnLive
- pose les cookies auth VendeursEnLive
- redirige vers `TIKTOK_SUCCESS_REDIRECT_URL`, par defaut `/app`

Reponse attendue en cas de succes:

```http
302 Location: https://vendeursenlive.shop/app
Set-Cookie: vel_access_token=...
Set-Cookie: vel_refresh_token=...
Set-Cookie: vel_refresh_session=...
Set-Cookie: vel_csrf_token=...
```

## Erreurs attendues

| Code | Cas courant |
| --- | --- |
| `400` | Payload invalide, mot de passe trop court, champ requis absent |
| `401` | Identifiants invalides, token absent/invalide |
| `403` | CSRF absent ou invalide |
| `403` | Action metier interdite tant que le compte n'est pas verifie |
| `409` | Email ou telephone deja utilise |
| `429` | Rate limit depasse |
| `500` | Erreur infrastructure |
