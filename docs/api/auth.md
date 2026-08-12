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
- `PATCH /auth/profile`
- `POST /auth/email-verification/request`

Les routes d'inscription, login, OTP et reset password public n'exigent pas le header CSRF.

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
  "email": "awa@example.com",
  "password": "Password123!",
  "account_type": "customer",
  "default_location": "Cocody"
}
```

Vendeur:

```json
{
  "phone_number": "+2250700000000",
  "password": "Password123!",
  "account_type": "seller"
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
  "verification_channel": "email",
  "can_change_password": true
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

### Demander un OTP téléphone

```http
POST /auth/phone/otp/request
```

Connexion d'un compte existant:

```json
{
  "phone_number": "+2250700000000",
  "purpose": "login"
}
```

Inscription client:

```json
{
  "phone_number": "+2250700000000",
  "purpose": "register",
  "account_type": "customer"
}
```

Pour un vendeur, `account_type` vaut `seller`. Le nom complet et le nom de boutique ne sont pas
demandés pendant l’inscription afin de réduire la friction. Ils peuvent être renseignés ensuite
avec `/auth/profile`. Le numéro est normalisé au format international. Le backend transmet à IKODDI
uniquement l'identité nécessaire à l'envoi du SMS.

Réponse `202`:

```json
{
  "challenge_id": "uuid",
  "expires_in_seconds": 300,
  "resend_after_seconds": 60
}
```

Le `otpToken` retourné par IKODDI reste dans Redis et n'est jamais exposé au navigateur.

### Vérifier un OTP téléphone

```http
POST /auth/phone/otp/verify
```

```json
{
  "challenge_id": "uuid",
  "otp": "629185"
}
```

Réponse `200`: le téléphone est considéré comme vérifié et les quatre cookies de session sont
posés. Un challenge expire après cinq minutes et est supprimé après validation ou après cinq codes
invalides.

### Current User

```http
GET /auth/me
```

Reponse `200`:

```json
{
  "user_id": "uuid",
  "session_id": "uuid",
  "full_name": "Awa Kouamé",
  "avatar_url": "https://lh3.googleusercontent.com/...",
  "email": "awa@example.com",
  "phone_number": null,
  "is_seller": true,
  "is_admin": false,
  "account_status": "active",
  "account_verified": true,
  "verification_channel": null,
  "can_change_password": false,
  "auth_methods": ["google"],
  "shop_name": "Boutique Awa",
  "default_location": null,
  "member_since": "2026-08-04T10:00:00Z"
}
```

Une identité téléphone validée par OTP renvoie `account_verified: true`. Une inscription email
reste connectée mais les actions métier protégées sont refusées jusqu'à la vérification.
`can_change_password` vaut `false` pour un compte Google ou téléphone OTP qui ne possède pas de mot
de passe local. `avatar_url` provient du fournisseur OAuth lorsqu'il en fournit un. Les UUID restent
présents dans le contrat technique mais ne sont jamais affichés dans l'espace utilisateur.

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

### Mettre à jour le profil

```http
PATCH /auth/profile
X-CSRF-Token: <vel_csrf_token>
```

Client:

```json
{
  "full_name": "Awa Kouamé"
}
```

Vendeur:

```json
{
  "full_name": "Awa Kouamé",
  "shop_name": "Boutique Awa"
}
```

Au moins un champ non vide est requis. `full_name` reste facultatif et `shop_name` est accepté
uniquement pour un compte vendeur. Réponse `204`.

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

## Google OAuth 2.0 / OpenID Connect

### Démarrer la connexion

```http
GET /auth/google/start?account_type=customer
```

`account_type` est optionnel et accepte `customer` ou `seller`. Il est utilisé uniquement lors de
la création d’un nouveau compte. Le backend génère un `state` aléatoire, le conserve dans un cookie
`HttpOnly` de dix minutes et redirige vers Google avec les scopes minimaux `openid email profile`.

### Callback

```http
GET /auth/google/callback?code=...&state=...
```

Le backend vérifie le `state`, échange le code sans exposer le secret au frontend, appelle le
endpoint OpenID Connect `userinfo` et exige un email Google vérifié. Le champ stable `sub` devient
l’identifiant fournisseur. Si cet email appartient déjà à un compte VendeursEnLive, l’identité
Google est liée à ce compte; sinon un nouveau profil client ou vendeur est créé.

En cas de succès, le backend pose les quatre cookies de session VendeursEnLive puis redirige vers
`GOOGLE_SUCCESS_REDIRECT_URL`, configurée sur la page d’accueil. Le frontend restaure ensuite la
session et reprend une éventuelle action interne, comme l’ouverture du formulaire LIVE. Le jeton
d’accès Google n’est ni renvoyé au navigateur ni conservé.

## TikTok Login (en veille)

Ces routes sont conservées pour une activation ultérieure, mais elles ne sont plus exposées dans
l'interface utilisateur. Elles ne doivent pas être configurées en production tant que le Login Kit
n'est pas réactivé.

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
- redirige vers `TIKTOK_SUCCESS_REDIRECT_URL`, configurée sur `/`

Reponse attendue en cas de succes:

```http
302 Location: https://vendeursenlive.shop/
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
| `429` | Renvoi OTP trop rapide ou trop de codes invalides |
| `503` | IKODDI désactivé, mal configuré ou temporairement indisponible |
| `503` | Google OAuth incomplet ou temporairement indisponible |
| `500` | Erreur infrastructure |
