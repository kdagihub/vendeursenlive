<script setup lang="ts">
import LegalLayout from '@/layouts/LegalLayout.vue'

const sections = [
  { id: 'responsable', label: 'Responsable du traitement' },
  { id: 'donnees', label: 'Données collectées' },
  { id: 'finalites', label: 'Finalités' },
  { id: 'auth-phone', label: 'Authentification téléphone' },
  { id: 'auth-google', label: 'Connexion Google' },
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
        <h1
          class="mt-3 text-3xl leading-tight font-black text-[#211536] sm:text-4xl lg:text-5xl dark:text-white"
        >
          Politique de confidentialité
        </h1>
        <p class="mt-5 max-w-3xl text-base leading-7 text-slate-600 sm:text-lg dark:text-slate-300">
          Cette politique explique quelles données VendeursEnLive utilise, pourquoi elles sont
          nécessaires et comment exercer les droits reconnus par la législation ivoirienne.
        </p>
        <div class="mt-5 flex flex-wrap gap-x-6 gap-y-2 text-sm text-slate-500 dark:text-slate-400">
          <span>
            <strong class="text-slate-800 dark:text-slate-200">Version :</strong> 24 juillet 2026
          </span>
          <span>
            <strong class="text-slate-800 dark:text-slate-200">Référence :</strong> loi ivoirienne
            n° 2013-450
          </span>
          <span>
            <strong class="text-slate-800 dark:text-slate-200">Contact :</strong>
            contact@vendeursenlive.shop
          </span>
        </div>
      </header>

      <div class="mt-10 grid items-start gap-10 lg:grid-cols-[230px_minmax(0,1fr)] lg:gap-16">
        <aside class="lg:sticky lg:top-24">
          <details
            class="group border-y border-slate-200 py-3 lg:border-0 lg:py-0 dark:border-white/12"
            open
          >
            <summary
              class="cursor-pointer text-sm font-black text-[#211536] lg:cursor-default dark:text-white"
            >
              Sur cette page
            </summary>
            <nav
              class="mt-3 grid gap-2 border-l border-slate-300 pl-4 text-sm dark:border-white/20"
              aria-label="Sommaire de la politique"
            >
              <a
                v-for="section in sections"
                :key="section.id"
                class="text-slate-600 hover:text-[#d92035] dark:text-slate-300 dark:hover:text-[#ff7381]"
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
                Sécurisés (CIACEMS) </strong
              >, SARL pluripersonnelle au capital de 1 000 000 FCFA, détermine les finalités et les
              moyens des traitements décrits dans cette politique.
            </p>
            <dl class="mt-5 grid gap-x-8 gap-y-4 sm:grid-cols-2">
              <div>
                <dt class="text-xs font-black text-slate-500 uppercase dark:text-slate-400">
                  RCCM
                </dt>
                <dd class="mt-1 font-semibold text-slate-800 dark:text-slate-200">
                  CI-ABJ-03-2024-M-44362
                </dd>
              </div>
              <div>
                <dt class="text-xs font-black text-slate-500 uppercase dark:text-slate-400">
                  Numéro de compte contribuable
                </dt>
                <dd class="mt-1 font-semibold text-slate-800 dark:text-slate-200">2304005 U</dd>
              </div>
              <div class="sm:col-span-2">
                <dt class="text-xs font-black text-slate-500 uppercase dark:text-slate-400">
                  Siège social
                </dt>
                <dd class="mt-1 font-semibold text-slate-800 dark:text-slate-200">
                  Abidjan, Cocody, Angré 8e Tranche, Cité EVE, lot 664, îlot 43 — 09 BP 3815 Abidjan
                  09, Côte d’Ivoire
                </dd>
              </div>
              <div>
                <dt class="text-xs font-black text-slate-500 uppercase dark:text-slate-400">
                  Direction technique de VendeursEnLive
                </dt>
                <dd class="mt-1 font-semibold text-slate-800 dark:text-slate-200">
                  M. KOFFI DJÈCLAY Alexandre
                </dd>
              </div>
              <div>
                <dt class="text-xs font-black text-slate-500 uppercase dark:text-slate-400">
                  Contact
                </dt>
                <dd class="mt-1 font-semibold text-slate-800 dark:text-slate-200">
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
              <li>
                identifiant interne, statut du compte et rôle client, vendeur ou administrateur ;
              </li>
              <li>
                mot de passe conservé uniquement sous forme de condensat cryptographique
                irréversible ;
              </li>
              <li>état de vérification de l’email ou du téléphone.</li>
            </ul>
            <p>
              Lors d’une connexion Google, le service reçoit l’identifiant stable du compte Google,
              son email vérifié et, lorsque disponibles, le nom d’affichage et l’avatar.
            </p>

            <h3>Profil et activité commerciale</h3>
            <ul>
              <li>nom de boutique, lien de paiement et préférences du vendeur ;</li>
              <li>
                localisation ou moyen de paiement préféré du client, lorsqu’ils sont renseignés ;
              </li>
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
              <li>
                jetons temporaires de vérification d’email, de réinitialisation du mot de passe ou
                de challenge OTP.
              </li>
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
                    <td>Vérifier l’email ou le téléphone et permettre la récupération du compte</td>
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

          <section id="auth-phone">
            <h2>4. Authentification par téléphone et code OTP</h2>
            <p>
              Lorsque l’utilisateur choisit « Continuer avec le téléphone », VendeursEnLive transmet
              son numéro au prestataire de messagerie IKODDI afin d’envoyer et de vérifier un code à
              usage unique par SMS.
            </p>
            <ul>
              <li>le numéro normalisé au format international ;</li>
              <li>un jeton technique de vérification fourni par IKODDI ;</li>
              <li>le résultat de la vérification du code saisi.</li>
            </ul>
            <p>
              Le jeton technique reste côté serveur dans Redis, n’est jamais communiqué au
              navigateur et expire normalement après cinq minutes. Le challenge est également
              supprimé après une vérification réussie ou un nombre maximal de tentatives invalides.
            </p>
            <p>
              Le numéro vérifié sert à créer ou retrouver le compte VendeursEnLive. Il peut aussi
              être utilisé pour les notifications strictement nécessaires au service, dans le
              respect des choix de l’utilisateur et des règles applicables.
            </p>
          </section>

          <section id="auth-google">
            <h2>5. Connexion avec Google</h2>
            <p>
              Lorsque l’utilisateur choisit « Continuer avec Google », il est redirigé vers Google
              pour s’authentifier et autoriser l’accès aux seules informations de profil de base.
              VendeursEnLive demande les permissions <code>openid</code>, <code>email</code> et
              <code>profile</code>.
            </p>
            <ul>
              <li>l’identifiant Google stable du compte (<code>sub</code>) ;</li>
              <li>l’adresse email et son état de vérification ;</li>
              <li>le nom d’affichage et l’avatar, lorsqu’ils sont disponibles.</li>
            </ul>
            <p>
              Le jeton d’accès Google est utilisé uniquement côté serveur pour obtenir ces
              informations pendant la connexion. Il n’est ni envoyé au navigateur, ni conservé en
              base de données. Les données Google ne sont pas vendues et ne servent pas à la
              publicité.
            </p>
          </section>

          <section id="cookies">
            <h2>6. Cookies et stockage de session</h2>
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
                    <td><code>vel_google_oauth_state</code></td>
                    <td>Vérifier l’origine du retour OAuth Google</td>
                    <td>Temporaire, Secure, HttpOnly, SameSite</td>
                  </tr>
                  <tr>
                    <td><code>vel_google_account_type</code></td>
                    <td>Conserver le choix client ou vendeur pendant la redirection</td>
                    <td>Temporaire, Secure, HttpOnly, SameSite</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </section>

          <section id="destinataires">
            <h2>7. Destinataires et sous-traitants</h2>
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
              <li>à IKODDI pour envoyer et vérifier le code OTP demandé par l’utilisateur ;</li>
              <li>à Google lorsque l’utilisateur choisit volontairement ce mode de connexion ;</li>
              <li>aux autorités légalement habilitées, sur demande valable.</li>
            </ul>
            <p>
              Les sous-traitants sont sélectionnés en tenant compte de leurs garanties de sécurité
              et ne doivent utiliser les données que pour fournir les services convenus.
            </p>
          </section>

          <section id="transferts">
            <h2>8. Hébergement et transferts internationaux</h2>
            <p>
              L’infrastructure principale est administrée pour le compte de CIACEMS. Certains
              prestataires, notamment Google et les services techniques ou de messagerie, peuvent
              traiter des données hors de Côte d’Ivoire.
            </p>
            <p>
              Lorsqu’un traitement implique un transfert international, CIACEMS applique les
              formalités et garanties prévues par la loi ivoirienne n° 2013-450, notamment les
              exigences de l’Autorité de protection pour les transferts vers un pays tiers.
            </p>
          </section>

          <section id="conservation">
            <h2>9. Durées de conservation</h2>
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
            <h2>10. Mesures de sécurité</h2>
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
            <h2>11. Vos droits sur les données</h2>
            <p>
              Conformément à la loi ivoirienne n° 2013-450 du 19 juin 2013, la personne concernée
              peut notamment demander :
            </p>
            <ul>
              <li>l’information et l’accès aux données la concernant ;</li>
              <li>la rectification ou la mise à jour de données inexactes ;</li>
              <li>l’opposition à un traitement pour un motif légitime et à toute prospection ;</li>
              <li>
                la suppression, l’effacement ou le verrouillage lorsque les conditions sont réunies
                ;
              </li>
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
                autoritedeprotection.ci </a
              >.
            </p>
          </section>

          <section id="mineurs">
            <h2>12. Protection des mineurs</h2>
            <p>
              Le service commercial n’est pas destiné à être utilisé de manière autonome par une
              personne ne disposant pas de la capacité juridique requise. Un mineur doit agir sous
              la responsabilité et avec l’autorisation de son représentant légal.
            </p>
          </section>

          <section id="contact">
            <h2>13. Évolution de la politique et contact</h2>
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
              Siège social : Abidjan, Cocody, Angré 8e Tranche, Cité EVE, lot 664, îlot 43 — 09 BP
              3815 Abidjan 09, Côte d’Ivoire
            </p>
          </section>
        </article>
      </div>
    </div>
  </LegalLayout>
</template>
