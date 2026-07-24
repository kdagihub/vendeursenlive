<script setup lang="ts">
import LegalLayout from '@/layouts/LegalLayout.vue'

const sections = [
  { id: 'responsable', label: 'Responsable du traitement' },
  { id: 'donnees', label: 'Données collectées' },
  { id: 'finalites', label: 'Finalités' },
  { id: 'tiktok', label: 'Connexion TikTok' },
  { id: 'cookies', label: 'Cookies et sécurité' },
  { id: 'destinataires', label: 'Destinataires' },
  { id: 'transferts', label: 'Transferts internationaux' },
  { id: 'conservation', label: 'Conservation' },
  { id: 'securite', label: 'Sécurité' },
  { id: 'droits', label: 'Vos droits' },
  { id: 'mineurs', label: 'Mineurs' },
  { id: 'contact', label: 'Contact et mise à jour' },
]
</script>

<template>
  <LegalLayout>
    <div class="mx-auto w-full max-w-[1180px] px-4 py-8 sm:px-6 sm:py-12 lg:px-8 lg:py-16">
      <header class="max-w-4xl">
        <p class="text-xs font-black text-[#d92035] uppercase">Protection des données</p>
        <h1 class="mt-3 text-3xl leading-tight font-black text-[#211536] sm:text-4xl lg:text-5xl">
          Politique de confidentialité
        </h1>
        <p class="mt-5 max-w-3xl text-base leading-7 text-slate-600 sm:text-lg">
          Cette politique explique quelles données VendeursEnLive utilise, pourquoi elles sont
          nécessaires et comment exercer les droits reconnus par la législation ivoirienne.
        </p>
        <div class="mt-5 flex flex-wrap gap-x-6 gap-y-2 text-sm text-slate-500">
          <span><strong class="text-slate-800">Version :</strong> 24 juillet 2026</span>
          <span>
            <strong class="text-slate-800">Référence :</strong> loi ivoirienne n° 2013-450
          </span>
          <span><strong class="text-slate-800">Contact :</strong> contact@vendeursenlive.shop</span>
        </div>
      </header>

      <div class="mt-10 grid items-start gap-10 lg:grid-cols-[230px_minmax(0,1fr)] lg:gap-16">
        <aside class="lg:sticky lg:top-24">
          <details class="group border-y border-slate-200 py-3 lg:border-0 lg:py-0" open>
            <summary class="cursor-pointer text-sm font-black text-[#211536] lg:cursor-default">
              Sur cette page
            </summary>
            <nav class="mt-3 grid gap-2 border-l border-slate-300 pl-4 text-sm" aria-label="Sommaire de la politique">
              <a
                v-for="section in sections"
                :key="section.id"
                class="text-slate-600 hover:text-[#d92035]"
                :href="`#${section.id}`"
              >
                {{ section.label }}
              </a>
            </nav>
          </details>
        </aside>

        <article class="legal-document min-w-0">
          <section id="responsable">
            <h2>1. Responsable du traitement</h2>
            <p>
              Le
              <strong>
                Cabinet d’Ingénierie Avancée et de Conception d’Environnements Multiservices
                Sécurisés (CIACEMS)
              </strong>,
              SARL pluripersonnelle au capital de 1 000 000 FCFA, détermine les finalités et les
              moyens des traitements décrits dans cette politique.
            </p>
            <dl class="mt-5 grid gap-x-8 gap-y-4 sm:grid-cols-2">
              <div>
                <dt class="text-xs font-black text-slate-500 uppercase">RCCM</dt>
                <dd class="mt-1 font-semibold text-slate-800">CI-ABJ-03-2024-M-44362</dd>
              </div>
              <div>
                <dt class="text-xs font-black text-slate-500 uppercase">
                  Numéro de compte contribuable
                </dt>
                <dd class="mt-1 font-semibold text-slate-800">2304005 U</dd>
              </div>
              <div class="sm:col-span-2">
                <dt class="text-xs font-black text-slate-500 uppercase">Siège social</dt>
                <dd class="mt-1 font-semibold text-slate-800">
                  Abidjan, Cocody, Angré 8e Tranche, Cité EVE, lot 664, îlot 43 — 09 BP 3815
                  Abidjan 09, Côte d’Ivoire
                </dd>
              </div>
              <div>
                <dt class="text-xs font-black text-slate-500 uppercase">
                  Direction technique de VendeursEnLive
                </dt>
                <dd class="mt-1 font-semibold text-slate-800">M. KOFFI DJÈCLAY Alexandre</dd>
              </div>
              <div>
                <dt class="text-xs font-black text-slate-500 uppercase">Contact</dt>
                <dd class="mt-1 font-semibold text-slate-800">
                  <a href="mailto:contact@vendeursenlive.shop">contact@vendeursenlive.shop</a><br />
                  <a href="tel:+2250797969394">+225 07 97 96 93 94</a>
                </dd>
              </div>
            </dl>
            <p>
              Pour toute question ou demande relative aux données personnelles :
              <a href="mailto:contact@vendeursenlive.shop">contact@vendeursenlive.shop</a>.
            </p>
            <p>
              La présente politique s’applique au site vendeursenlive.shop, à son application web
              installable et aux services d’authentification et de commande associés.
            </p>
          </section>

          <section id="donnees">
            <h2>2. Catégories de données traitées</h2>
            <h3>Compte et identité</h3>
            <ul>
              <li>nom complet, email ou numéro de téléphone ;</li>
              <li>identifiant interne, statut du compte et rôle client, vendeur ou administrateur ;</li>
              <li>
                mot de passe conservé uniquement sous forme de condensat cryptographique
                irréversible ;
              </li>
              <li>état de vérification de l’email ou du téléphone.</li>
            </ul>

            <h3>Profil et activité commerciale</h3>
            <ul>
              <li>nom de boutique, lien de paiement et préférences du vendeur ;</li>
              <li>localisation ou moyen de paiement préféré du client, lorsqu’ils sont renseignés ;</li>
              <li>URL du live, images, descriptions et prix des produits présentés ;</li>
              <li>
                nom, téléphone, lieu de livraison, statut de commande et éventuelle preuve de
                paiement.
              </li>
            </ul>

            <h3>Données techniques et de sécurité</h3>
            <ul>
              <li>adresse IP, type de navigateur ou appareil et horodatages de connexion ;</li>
              <li>identifiants de session, traces de sécurité et statut de révocation ;</li>
              <li>jetons temporaires de vérification d’email ou de réinitialisation du mot de passe.</li>
            </ul>
          </section>

          <section id="finalites">
            <h2>3. Pourquoi ces données sont utilisées</h2>
            <div class="overflow-x-auto">
              <table>
                <thead>
                  <tr>
                    <th>Finalité</th>
                    <th>Justification</th>
                  </tr>
                </thead>
                <tbody>
                  <tr>
                    <td>Créer le compte, authentifier l’utilisateur et maintenir sa session</td>
                    <td>Fourniture du service demandé et sécurité</td>
                  </tr>
                  <tr>
                    <td>Afficher les lives, gérer les produits et transmettre les commandes</td>
                    <td>Exécution du service et des mesures précontractuelles</td>
                  </tr>
                  <tr>
                    <td>Vérifier l’email et permettre la récupération du compte</td>
                    <td>Sécurité du compte et demande de l’utilisateur</td>
                  </tr>
                  <tr>
                    <td>Prévenir la fraude, les abus et les accès non autorisés</td>
                    <td>Protection du service, des utilisateurs et obligations légales</td>
                  </tr>
                  <tr>
                    <td>Répondre au support, aux réclamations et aux demandes de droits</td>
                    <td>Demande de l’utilisateur et respect de la loi</td>
                  </tr>
                </tbody>
              </table>
            </div>
            <p>
              VendeursEnLive ne vend pas les données personnelles. Aucune prospection commerciale
              par email ou SMS n’est réalisée sans information préalable et, lorsqu’il est requis,
              sans consentement.
            </p>
          </section>

          <section id="tiktok">
            <h2>4. Données utilisées avec TikTok Login Kit</h2>
            <p>
              Lorsque l’utilisateur choisit « Continuer avec TikTok », il est redirigé vers TikTok
              afin de s’authentifier et d’autoriser l’accès demandé. VendeursEnLive utilise
              uniquement la portée <code>user.info.basic</code>.
            </p>
            <p>Après autorisation, VendeursEnLive reçoit et utilise :</p>
            <ul>
              <li>l’<strong>open_id</strong>, identifiant TikTok propre à l’application ;</li>
              <li>le <strong>nom d’affichage</strong> ;</li>
              <li>l’<strong>URL de l’avatar</strong>, lorsqu’elle est disponible.</li>
            </ul>
            <p>
              Ces données servent exclusivement à créer ou retrouver le compte VendeursEnLive et à
              afficher son profil. Le jeton d’accès TikTok est utilisé de manière transitoire pour
              récupérer ces informations et n’est pas conservé dans la base de données de
              VendeursEnLive après la connexion.
            </p>
            <p>
              Révoquer VendeursEnLive depuis les paramètres TikTok empêche les futurs accès, mais ne
              supprime pas automatiquement le compte VendeursEnLive déjà créé. Sa suppression peut
              être demandée à
              <a href="mailto:contact@vendeursenlive.shop">contact@vendeursenlive.shop</a>.
              L’utilisation de TikTok reste également soumise à la politique de confidentialité de
              TikTok.
            </p>
          </section>

          <section id="cookies">
            <h2>5. Cookies et stockage de session</h2>
            <p>
              Le service utilise uniquement les cookies nécessaires à l’authentification et à la
              sécurité. Ils ne servent pas à la publicité comportementale.
            </p>
            <div class="overflow-x-auto">
              <table>
                <thead>
                  <tr>
                    <th>Cookie</th>
                    <th>Utilité</th>
                    <th>Protection</th>
                  </tr>
                </thead>
                <tbody>
                  <tr>
                    <td><code>vel_access_token</code></td>
                    <td>Authentifier les requêtes pendant une courte durée</td>
                    <td>Secure, HttpOnly, SameSite</td>
                  </tr>
                  <tr>
                    <td><code>vel_refresh_token</code></td>
                    <td>Renouveler la session sans nouvelle connexion</td>
                    <td>Secure, HttpOnly, SameSite</td>
                  </tr>
                  <tr>
                    <td><code>vel_refresh_session</code></td>
                    <td>Identifier la session renouvelable</td>
                    <td>Secure, HttpOnly, SameSite</td>
                  </tr>
                  <tr>
                    <td><code>vel_csrf_token</code></td>
                    <td>Protéger les actions sensibles contre les requêtes forgées</td>
                    <td>Secure, SameSite</td>
                  </tr>
                  <tr>
                    <td><code>vel_tiktok_oauth_state</code></td>
                    <td>Sécuriser temporairement le retour de connexion TikTok</td>
                    <td>Secure, HttpOnly, durée limitée</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </section>

          <section id="destinataires">
            <h2>6. Destinataires et sous-traitants</h2>
            <p>Les données sont accessibles uniquement selon le besoin :</p>
            <ul>
              <li>aux personnes habilitées de CIACEMS ;</li>
              <li>
                au vendeur concerné pour traiter une commande, notamment le nom, le téléphone, le
                lieu de livraison et la preuve transmise par le client ;
              </li>
              <li>
                aux prestataires d’hébergement, de messagerie et d’infrastructure agissant sur
                instruction de CIACEMS ;
              </li>
              <li>à TikTok lorsque l’utilisateur demande la connexion sociale ;</li>
              <li>aux autorités légalement habilitées, sur demande valable.</li>
            </ul>
            <p>
              Les sous-traitants sont sélectionnés en tenant compte de leurs garanties de sécurité
              et ne doivent utiliser les données que pour fournir les services convenus.
            </p>
          </section>

          <section id="transferts">
            <h2>7. Hébergement et transferts internationaux</h2>
            <p>
              L’infrastructure principale est administrée pour le compte de CIACEMS. Certains
              prestataires, notamment TikTok ou les services techniques et de messagerie, peuvent
              traiter des données hors de Côte d’Ivoire.
            </p>
            <p>
              Lorsqu’un traitement implique un transfert international, CIACEMS applique les
              formalités et garanties prévues par la loi ivoirienne n° 2013-450, notamment les
              exigences de l’Autorité de protection pour les transferts vers un pays tiers.
            </p>
          </section>

          <section id="conservation">
            <h2>8. Durées de conservation</h2>
            <ul>
              <li>
                <strong>Compte et profil :</strong> pendant l’utilisation du service, puis le temps
                nécessaire au traitement de la fermeture et aux obligations applicables.
              </li>
              <li>
                <strong>Sessions :</strong> jusqu’à leur expiration, leur révocation ou la
                déconnexion.
              </li>
              <li>
                <strong>Liens de vérification et de réinitialisation :</strong> jusqu’à leur
                utilisation ou expiration, puis suppression lors des opérations de maintenance.
              </li>
              <li>
                <strong>Commandes et justificatifs :</strong> pendant le suivi de la transaction,
                puis pendant la durée nécessaire à la preuve, au règlement des litiges et aux
                obligations légales.
              </li>
              <li>
                <strong>Contenus de live et produits éphémères :</strong> pendant la session et,
                lorsqu’ils sont conservés, pendant la durée utile à l’historique du vendeur.
              </li>
            </ul>
            <p>
              Une suppression de compte peut laisser subsister certaines informations de commande
              lorsqu’une obligation légale ou la défense d’un droit l’exige. Leur accès est alors
              limité à cette finalité.
            </p>
          </section>

          <section id="securite">
            <h2>9. Mesures de sécurité</h2>
            <p>
              Des mesures techniques et organisationnelles sont mises en œuvre pour limiter la
              perte, l’altération et l’accès non autorisé : chiffrement TLS, mots de passe hachés,
              cookies sécurisés, contrôle des habilitations, protection CSRF, limitation des
              requêtes, journalisation utile à la sécurité et sauvegardes protégées.
            </p>
            <p>
              Aucun système n’étant infaillible, tout incident suspect peut être signalé à
              <a href="mailto:contact@vendeursenlive.shop">contact@vendeursenlive.shop</a>.
            </p>
          </section>

          <section id="droits">
            <h2>10. Vos droits sur les données</h2>
            <p>
              Conformément à la loi ivoirienne n° 2013-450 du 19 juin 2013, la personne concernée
              peut notamment demander :
            </p>
            <ul>
              <li>l’information et l’accès aux données la concernant ;</li>
              <li>la rectification ou la mise à jour de données inexactes ;</li>
              <li>l’opposition à un traitement pour un motif légitime et à toute prospection ;</li>
              <li>la suppression, l’effacement ou le verrouillage lorsque les conditions sont réunies ;</li>
              <li>le retrait de son consentement lorsqu’un traitement repose sur celui-ci ;</li>
              <li>la portabilité des données dans un format structuré et couramment utilisé.</li>
            </ul>
            <p>
              La demande doit être envoyée à
              <a href="mailto:contact@vendeursenlive.shop">contact@vendeursenlive.shop</a> en
              précisant le compte et le droit exercé. Une preuve d’identité peut être demandée
              lorsqu’elle est nécessaire pour éviter de communiquer les données à un tiers.
            </p>
            <p>
              En cas de difficulté non résolue, une réclamation peut être adressée à l’ARTCI,
              Autorité de protection des données personnelles en Côte d’Ivoire. Les informations
              officielles sont disponibles sur
              <a
                href="https://www.autoritedeprotection.ci/"
                rel="noopener noreferrer"
                target="_blank"
              >
                autoritedeprotection.ci
              </a>.
            </p>
          </section>

          <section id="mineurs">
            <h2>11. Protection des mineurs</h2>
            <p>
              Le service commercial n’est pas destiné à être utilisé de manière autonome par une
              personne ne disposant pas de la capacité juridique requise. Un mineur doit agir sous
              la responsabilité et avec l’autorisation de son représentant légal.
            </p>
          </section>

          <section id="contact">
            <h2>12. Évolution de la politique et contact</h2>
            <p>
              Cette politique peut évoluer pour refléter une modification du service, des
              prestataires ou de la réglementation. La date de version est indiquée en haut de page.
              Un changement important peut être signalé dans l’application ou par un moyen de
              contact disponible.
            </p>
            <p>
              Responsable du traitement : <strong>CIACEMS</strong><br />
              Service : VendeursEnLive<br />
              Email :
              <a href="mailto:contact@vendeursenlive.shop">contact@vendeursenlive.shop</a><br />
              Téléphone : <a href="tel:+2250797969394">+225 07 97 96 93 94</a><br />
              Siège social : Abidjan, Cocody, Angré 8e Tranche, Cité EVE, lot 664, îlot 43 —
              09 BP 3815 Abidjan 09, Côte d’Ivoire
            </p>
          </section>
        </article>
      </div>
    </div>
  </LegalLayout>
</template>
