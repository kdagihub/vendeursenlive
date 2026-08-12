<script setup lang="ts">
import LegalLayout from '@/layouts/LegalLayout.vue'

const sections = [
  { id: 'editeur', label: 'Éditeur et objet' },
  { id: 'acces', label: 'Accès et comptes' },
  { id: 'roles', label: 'Rôles des utilisateurs' },
  { id: 'commandes', label: 'Commandes et arbitrage' },
  { id: 'paiements', label: 'Paiements et livraison' },
  { id: 'tiktok', label: 'Lives et services tiers' },
  { id: 'contenus', label: 'Contenus et propriété' },
  { id: 'interdictions', label: 'Usages interdits' },
  { id: 'responsabilite', label: 'Disponibilité et responsabilité' },
  { id: 'resiliation', label: 'Suspension et résiliation' },
  { id: 'droit', label: 'Droit applicable' },
  { id: 'contact', label: 'Contact' },
]
</script>

<template>
  <LegalLayout>
    <div class="mx-auto w-full max-w-[1180px] px-4 py-8 sm:px-6 sm:py-12 lg:px-8 lg:py-16">
      <header class="max-w-4xl">
        <p class="text-xs font-black text-[#d92035] uppercase">Informations légales</p>
        <h1
          class="mt-3 text-3xl leading-tight font-black text-[#211536] sm:text-4xl lg:text-5xl dark:text-white"
        >
          Conditions générales d’utilisation
        </h1>
        <p class="mt-5 max-w-3xl text-base leading-7 text-slate-600 sm:text-lg dark:text-slate-300">
          Ces conditions encadrent l’accès à VendeursEnLive et l’utilisation de ses outils de
          découverte, de live commerce et de prise de commandes.
        </p>
        <div class="mt-5 flex flex-wrap gap-x-6 gap-y-2 text-sm text-slate-500 dark:text-slate-400">
          <span>
            <strong class="text-slate-800 dark:text-slate-200">Version :</strong> 24 juillet 2026
          </span>
          <span>
            <strong class="text-slate-800 dark:text-slate-200">Territoire :</strong> Côte d’Ivoire
          </span>
          <span>
            <strong class="text-slate-800 dark:text-slate-200">Service :</strong>
            vendeursenlive.shop
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
              aria-label="Sommaire des conditions"
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
          <section id="editeur">
            <h2>1. Éditeur et objet du service</h2>
            <p>
              La plateforme VendeursEnLive, accessible à l’adresse vendeursenlive.shop, est éditée
              et exploitée par le
              <strong>
                Cabinet d’Ingénierie Avancée et de Conception d’Environnements Multiservices
                Sécurisés (CIACEMS) </strong
              >.
            </p>
            <dl class="mt-5 grid gap-x-8 gap-y-4 sm:grid-cols-2">
              <div>
                <dt class="text-xs font-black text-slate-500 uppercase dark:text-slate-400">
                  Forme juridique
                </dt>
                <dd class="mt-1 font-semibold text-slate-800 dark:text-slate-200">
                  SARL pluripersonnelle
                </dd>
              </div>
              <div>
                <dt class="text-xs font-black text-slate-500 uppercase dark:text-slate-400">
                  Capital social
                </dt>
                <dd class="mt-1 font-semibold text-slate-800 dark:text-slate-200">
                  1 000 000 FCFA
                </dd>
              </div>
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
                  Contact VendeursEnLive
                </dt>
                <dd class="mt-1 font-semibold text-slate-800 dark:text-slate-200">
                  <a href="mailto:contact@vendeursenlive.shop">contact@vendeursenlive.shop</a><br />
                  <a href="tel:+2250797969394">+225 07 97 96 93 94</a>
                </dd>
              </div>
            </dl>
            <p>
              La plateforme met à disposition des vendeurs des outils pour référencer leurs directs,
              présenter des produits et recevoir des demandes de commande. Elle permet aux clients
              de découvrir les lives, consulter les articles présentés et transmettre une demande
              d’achat.
            </p>
            <p>
              VendeursEnLive agit comme intermédiaire technique. Sauf mention explicite contraire,
              la vente est conclue directement entre le vendeur et le client. VendeursEnLive n’est
              ni propriétaire des marchandises, ni vendeur des produits proposés par ses
              utilisateurs.
            </p>
          </section>

          <section id="acces">
            <h2>2. Acceptation, accès et comptes</h2>
            <p>
              L’utilisation du service implique l’acceptation des présentes conditions. Toute
              personne qui crée un compte déclare avoir la capacité juridique nécessaire ou agir
              avec l’autorisation de son représentant légal.
            </p>
            <ul>
              <li>
                L’inscription peut être réalisée par téléphone, email ou par un fournisseur de
                connexion proposé par la plateforme.
              </li>
              <li>
                Les informations communiquées doivent être exactes, actuelles et appartenir à
                l’utilisateur.
              </li>
              <li>
                L’utilisateur protège ses identifiants et signale rapidement toute utilisation non
                autorisée de son compte.
              </li>
              <li>
                Certaines actions sensibles peuvent rester bloquées tant que l’email ou le numéro de
                téléphone n’a pas été vérifié.
              </li>
            </ul>
          </section>

          <section id="roles">
            <h2>3. Obligations des clients et des vendeurs</h2>
            <h3>Clients</h3>
            <p>
              Le client vérifie les caractéristiques, le prix, les conditions de paiement et de
              livraison communiqués par le vendeur avant de confirmer une transaction. Il fournit
              des coordonnées exactes et n’envoie une preuve de paiement que pour une opération
              réellement effectuée.
            </p>
            <h3>Vendeurs</h3>
            <p>
              Le vendeur est seul responsable de son activité commerciale, de la licéité des
              produits proposés, de ses prix, de ses déclarations, de la disponibilité des articles
              et de l’exécution des commandes acceptées. Il doit notamment :
            </p>
            <ul>
              <li>décrire honnêtement les produits, leur état et leurs conditions de vente ;</li>
              <li>
                respecter les règles fiscales, commerciales et de protection du consommateur ;
              </li>
              <li>
                disposer des droits nécessaires sur les images, marques et contenus diffusés ;
              </li>
              <li>
                traiter les coordonnées des clients uniquement pour exécuter la commande concernée ;
              </li>
              <li>ne pas proposer de produits interdits, contrefaits, dangereux ou frauduleux.</li>
            </ul>
          </section>

          <section id="commandes">
            <h2>4. Demandes de commande et arbitrage</h2>
            <p>
              Le bouton de commande transmet une <strong>demande au vendeur</strong>. Il ne garantit
              ni la disponibilité d’un article, ni l’acceptation automatique de la vente. Le modèle
              de VendeursEnLive repose sur un arbitrage manuel, sans blocage préalable du stock.
            </p>
            <p>
              La demande devient confirmée lorsque le vendeur l’accepte et communique, si
              nécessaire, les modalités finales de paiement, de remise ou de livraison. Une demande
              peut rester en attente ou être refusée, notamment lorsque plusieurs clients réclament
              le même article.
            </p>
            <p>
              Les statuts affichés dans l’application constituent des informations de suivi. Le
              client et le vendeur doivent conserver les justificatifs utiles à leur transaction.
            </p>
          </section>

          <section id="paiements">
            <h2>5. Paiements, livraison, retours et litiges commerciaux</h2>
            <p>
              À ce stade, les paiements sont réalisés directement au vendeur, par exemple via un
              lien Mobile Money ou selon les moyens qu’il propose. VendeursEnLive ne reçoit pas les
              fonds et ne détient pas de compte de paiement pour le compte des utilisateurs.
            </p>
            <p>
              Le vendeur demeure responsable de la confirmation du paiement, de la livraison, des
              retours, des remboursements et du service après-vente. En cas de désaccord, le client
              doit d’abord contacter le vendeur. VendeursEnLive peut faciliter les échanges ou
              examiner les éléments disponibles, sans se substituer aux parties ni à une autorité
              compétente.
            </p>
          </section>

          <section id="tiktok">
            <h2>6. Lives TikTok et services tiers</h2>
            <p>
              Des directs, profils ou liens TikTok peuvent être référencés ou intégrés dans
              VendeursEnLive. TikTok reste un service tiers soumis à ses propres conditions.
              VendeursEnLive n’est ni sponsorisé ni administré par TikTok.
            </p>
            <p>
              La disponibilité d’un direct, d’une intégration ou d’une connexion sociale dépend
              également du service tiers, de ses autorisations et de ses règles. Une interruption ou
              modification provenant d’un tiers peut limiter temporairement certaines
              fonctionnalités.
            </p>
          </section>

          <section id="contenus">
            <h2>7. Contenus et propriété intellectuelle</h2>
            <p>
              La marque, l’interface, le code et les éléments propres à VendeursEnLive sont
              protégés. Aucun droit de reproduction ou d’exploitation n’est accordé en dehors de
              l’usage normal du service.
            </p>
            <p>
              L’utilisateur conserve ses droits sur les contenus qu’il publie. Il accorde à
              VendeursEnLive, pendant la durée nécessaire au service, une autorisation non exclusive
              d’héberger, reproduire techniquement et afficher ces contenus afin d’assurer leur
              diffusion sur la plateforme.
            </p>
          </section>

          <section id="interdictions">
            <h2>8. Utilisations interdites</h2>
            <p>Il est notamment interdit de :</p>
            <ul>
              <li>frauder, usurper une identité ou manipuler une preuve de paiement ;</li>
              <li>
                publier un contenu illicite, haineux, trompeur ou portant atteinte à un tiers ;
              </li>
              <li>collecter ou détourner les données d’autres utilisateurs ;</li>
              <li>
                contourner la sécurité, perturber le service ou automatiser des accès abusifs ;
              </li>
              <li>utiliser la plateforme à des fins de blanchiment ou de financement illicite.</li>
            </ul>
          </section>

          <section id="responsabilite">
            <h2>9. Disponibilité et responsabilité</h2>
            <p>
              CIACEMS met en œuvre des moyens raisonnables pour assurer la sécurité et la continuité
              du service. Une disponibilité permanente ne peut toutefois être garantie, notamment en
              cas de maintenance, panne, incident réseau ou défaillance d’un fournisseur tiers.
            </p>
            <p>
              Dans les limites autorisées par la loi, VendeursEnLive ne répond pas des déclarations
              des vendeurs, de la qualité des produits, d’un paiement envoyé au mauvais
              destinataire, d’un défaut de livraison ou d’un préjudice résultant d’un usage
              contraire aux présentes conditions. Les garanties légales impératives restent
              applicables.
            </p>
          </section>

          <section id="resiliation">
            <h2>10. Suspension, suppression et résiliation</h2>
            <p>
              Un compte ou un contenu peut être limité, suspendu ou supprimé en cas de fraude, de
              risque pour la sécurité, de violation des présentes conditions, d’obligation légale ou
              de demande d’une autorité compétente.
            </p>
            <p>
              L’utilisateur peut demander la fermeture de son compte. Certaines données liées aux
              commandes ou aux obligations légales peuvent être conservées pendant la durée
              nécessaire, conformément à la
              <RouterLink to="/privacy">politique de confidentialité</RouterLink>.
            </p>
          </section>

          <section id="droit">
            <h2>11. Modification, droit applicable et différends</h2>
            <p>
              Les conditions peuvent évoluer avec le service ou la réglementation. La date de
              version est mise à jour et les changements importants peuvent être signalés dans
              l’application.
            </p>
            <p>
              Les présentes conditions sont régies par le droit ivoirien. Les parties recherchent
              d’abord une solution amiable. À défaut, le différend relève des juridictions
              ivoiriennes compétentes, sous réserve des règles impératives protégeant le
              consommateur.
            </p>
          </section>

          <section id="contact">
            <h2>12. Contact et signalement</h2>
            <p>
              Pour une question, une réclamation ou le signalement d’un contenu ou vendeur :
              <a href="mailto:contact@vendeursenlive.shop">contact@vendeursenlive.shop</a>. Indiquez
              les informations permettant d’identifier la session, la commande ou le compte
              concerné, sans transmettre de mot de passe.
            </p>
          </section>
        </article>
      </div>
    </div>
  </LegalLayout>
</template>
