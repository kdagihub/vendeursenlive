# TikTok LIVE Embed - Access Request

## Subject

TikTok LIVE Embed access request for VendeursEnLive

## Company and product

- Company: CIACEMS
- Product: VendeursEnLive
- Website: https://vendeursenlive.shop
- Terms: https://vendeursenlive.shop/terms
- Privacy policy: https://vendeursenlive.shop/privacy

## Integration scenario

VendeursEnLive is a live-commerce web application for sellers and customers in
Cote d'Ivoire. Sellers create a VendeursEnLive session linked to their own
TikTok LIVE. Customers watch the seller's LIVE inside VendeursEnLive while the
application displays the product currently presented and a separate order
workflow.

The TikTok player remains the video source. VendeursEnLive does not download,
restream, alter or record TikTok LIVE content. The surrounding commerce flow is
managed by VendeursEnLive and orders are manually accepted or rejected by the
seller.

## Requested host domains

- vendeursenlive.shop
- www.vendeursenlive.shop (only if this hostname will serve the frontend)

The API hostname `api.vendeursenlive.shop` does not host the iframe and does not
need LIVE Embed access.

## Planned player behavior

- HTTPS only
- `embed_domain` set to the exact frontend hostname
- autoplay enabled in muted mode
- lazy loading outside the first viewport
- native TikTok controls retained
- fullscreen allowed
- `postMessage` events accepted only from `https://www.tiktok.com`
- fallback UI shown when the player is unavailable or the LIVE has ended

## Estimated traffic

- Estimated monthly page views (PV): TO COMPLETE
- Estimated monthly unique visitors (UV): TO COMPLETE
- Expected launch date: TO COMPLETE

## Contacts

- Compliance contact: TO COMPLETE
- Technical contact: TO COMPLETE
- Contact email: contact@vendeursenlive.shop

## Current status

- VendeursEnLive authentication: phone/email and Google OAuth 2.0
- TikTok Login Kit: paused and not required by the LIVE Embed flow
- LIVE Embed frontend integration: implemented behind a disabled build flag
- LIVE Embed production flag: `VITE_TIKTOK_LIVE_EMBED_ENABLED=false`
- Seller LIVE link normalization and session management: implemented
- Authenticated user space: complete profile, contact, verification and seller shop information
- TikTok developer application: approved
- LIVE Embed access request: submitted, domain allowlist confirmation pending
