# Integration API IKODDI - KREEZUS

*IKODDI* est une plateforme B2B de la société KREEZUS qui permet à ses clients d’utiliser les services suivants fournies par KREEZUS :

- Recharges Crédit Mobile
- Recharges forfait Internet
- Envoie SMS en masse (SMS Marketing)
- OTP As A Service

Les environnements

Vous avez deux environnements disponibles sur IKODDI :

1. Le staging (test) :
    1. URL de l’interface d’administration : https://staging.ikoddi.com/
    2. URL de l’API : https://api.staging.ikoddi.com/
2. La PRODUCTION :
    1. URL de l’interface d’administration : https://app.ikoddi.com/
    2. URL de l’API : https://api.ikoddi.com/

Les types d’intégrations

1. Intégration directe via l’API REST
2. Utilisation des SDK Ikoddi disponibles (Nous recommandons si c’est possible dans votre cas):
    1. NodeJS : https://www.npmjs.com/package/ikoddi-client-sdk

# Avant de commencer

Création de votre compte sur Ikoddi

<aside>
💡 Rendez-vous sur https://app.ikoddi.com pour créer votre compte.

</aside>

Création de votre clé API avec les droits nécessaires

<aside>
💡 La CLE API est la clé qui vous authentifiera et vous permettra de faire toutes les requêtes dont vous avez besoin. Vous pouvez en créer en allant a la section API KEY de la plateforme ou en suivant ce lien : https://app.ikoddi.com/team/api-keys

</aside>

***Vous devez créer une API KEY avec les droits suivants :***

### Permissions de la Clé d’API pour les opérations AIRTIME

- `Envoyer du airtime`
- `Récuperer l'historique des airtimes`
- `Récupérer le solde airtime`
- `Lire les forfaits de recharge`

### Permissions de la Clé d’API pour les opérations SMS

- `Envoyer des sms`

RECUPEREZ VOTRE ORGANIZATION_ID

<aside>
💡 L’ `organization_id` est l’id de votre organisation, ce qui vous identifie pour les requêtes sur l’Api Ikoddi, vous pouvez la récupérer dans la section ORGANISATION de la plateforme ou en suivant ce lien : https://app.ikoddi.com/team/details

</aside>

RECHARGEZ VOS COMPTES (SMS ou AIRTIME)

<aside>
💡 Pour recharger vos comptes pour la production ou pour le test veuillez nous contacter a l’adresse suivante :
*contact@kreezus.com*

</aside>

<aside>
⚠️ Pour l'envoi d'un airtime donné, **la transaction se fera uniquement sur les numéros correspondants à l'opérateur téléphonique**, donc veuillez SVP renseigner des numéros valides !

</aside>

**ETAPES D’INTEGRATION SANS LA LIBRAIRIE POUR LA  PRODUCTION**

- **API AIRTIME (ENVOYER DU AIRTIME)**

    ### **Récupérer la liste des forfaits de crédit mobile**

    - Méthode : GET
    - Nom de l’api_key  a générer: Récupérer l'historique des airtimes
    - URL :

        ```jsx
         https://api.ikoddi.com/api/v1/groups/{organization_id}/airtimes/mobile-credit-plans
        ```

    - EXEMPLE CURL

    ```jsx
    CURL -X GET -H 'x-api-key: <ikoddi-api-key>' https://api.ikoddi.com/api/v1/groups/{organization_id}/airtimes/mobile-credit-plans | jq
    ```

    - Format des données de réponses

    ```jsx
    [
     {
        "id": "cm182522i0004n9po0exd9pgl",
        "forfaitId": "11501",
        "title": "PASS-MIX - 10mn + 15SMS + 20Mo à 263 FCFA",
        "description": "Bénéficier de 10mn d'appels + 15SMS + 20Mo Vers Orange à 263 FCFA valable 1 jour",
        "amount": "263",
        "operator": "Orange",
        "bonusDays": [
          "lundi",
          "mardi",
          "mercredi",
          "jeudi",
          "vendredi",
          "samedi",
          "dimanche"
        ],
        "availableDays": [
          "lundi",
          "mardi",
          "mercredi",
          "jeudi",
          "vendredi",
          "samedi",
          "dimanche"
        ],
        "bonusFactor": null,
        "allowedAmounts": [],
        "allowedMinAmount": null,
        "bonusDescription": null,
        "isSpecial": false,
        "isAvailable": true,
        "createdAt": "2024-09-18T16:08:37.034Z",
        "updatedAt": "2024-09-18T16:08:37.034Z"
      },
      {
        "id": "cm1825251000kn9pozr3x6a1a",
        "forfaitId": "11101",
        "title": "100% de bonus sur vos recharges SAP SAP",
        "description": "Bénéficiez de 100% de bonus sur vos recharges de 125F, 525F, 1025F, 5025F, 10000F et plus.",
        "amount": "",
        "operator": "Orange",
        "bonusDays": [
          "lundi",
          "mardi",
          "mercredi",
          "jeudi",
          "vendredi",
          "samedi",
          "dimanche"
        ],
        "availableDays": [
          "lundi",
          "mardi",
          "mercredi",
          "jeudi",
          "vendredi",
          "samedi",
          "dimanche"
        ],
        "bonusFactor": 100,
        "allowedAmounts": [
          "125",
          "525",
          "1025",
          "5025"
        ],
        "allowedMinAmount": "10000",
        "bonusDescription": null,
        "isSpecial": false,
        "isAvailable": true,
        "createdAt": "2024-09-18T16:08:37.141Z",
        "updatedAt": "2025-05-10T18:40:45.369Z"
      },
      ...
    ]
    ```

    ### **Souscrire a un forfait de crédit mobile**

    - Méthode : POST
    - URL :

    ```powershell
    https://api.ikoddi.com/api/v1/groups/{organization_id}/airtimes
    ```

    - Permission de l’api_key  a générer: Envoyer du airtime
    - Corps de la requête (body)
    - Ref (A utiliser si vous ne récupérez pas la liste des forfaits de crédit mobile ) : La ref est l’identifiant du type de transaction en fonction de l’opérateur téléphonique, dans la liste de forfaits mobile récupéré est correspond a forfaitId
        - Pour du crédit mobile Orange la ref est : 11101
        - Pour du crédit mobile Moov la ref est : 12131
        - Pour du crédit mobile Telecel la ref est : 13160

    ```json
    {
        "sentTo": ["22670707070","22670707071"],
        "amount": "1000",
        "ref":"12131"
        "countryStringCode": "BF",
        "countryNumberCode": "226",
            "campaignName":"Recharge de mois de juillet"
     }
    ```

    - CURL

    ```powershell
    curl -X POST -H 'Content-Type: application/json' -H 'x-api-key: <ikoddi-api-key>' -d '{"sentTo":["22670707070"],"amount": "100","ref":"12131","countryStringCode": "BF","countryNumberCode": "226"}' https://api.ikoddi.com/api/v1/groups/{organization_id}/airtimes
    ```

    - Eléments du corps de la requête

    <aside>
    ✅        **sentTo: Les destinataires du crédit mobile**

    </aside>

    <aside>
    ✅        **amount: Le montant de crédit mobile**

    </aside>

    <aside>
    ✅        **countryStringCode: Le code pays des numéros**

    </aside>

    <aside>
    ✅        **countryNumberCode: Le code téléphonique international des numéros**

    </aside>

    <aside>
    ✅       **campaignName: Le nom que vous donner a cette transaction**

    </aside>

    ### **Récupérer  la liste des forfaits internet**

    - Méthode : GET
    - URL

    ```powershell
    https://api.ikoddi.com/api/v1/groups/{organization_id}/airtimes/internet-plans
    ```

    - Nom de l’api_key  a générer: Récupérer l'historique des airtimes
    - EXEMPLE CURL

    ```powershell
    curl -X GET -H 'Content-Type: application/json' -H 'x-api-key: <ikoddi-api-key>' https://api.ikoddi.com/api/v1/groups/{organization_id}/airtimes/internet-plans
    ```

    - Format des données de réponse

    ```json
    [

      {

            "id" : "12558",
            "title" : "4 Go ce jour",
            "description" : "Gagner 4 Go aujourd’hui valable 30 jours",
            "amount" : "2000",
            "operator" : "Moov",
            "isAvailable" :true

      },
      {

        "id" : "12559",
        "title" : "2 Go ce jour",
        "description" : "Gagner 2 Go aujourd’hui valable 30 jours",
        "amount" : "1000",
        "operator" : "Orange",
        "isAvailable" :true

       }

     ]
    ```

    ### **Souscrire a un forfait internet**

    - Vous devez d’abord récupérer la liste des forfaits internet afin d’avoir l’identifiant (id) et le montant (amount) du forfait nécessaire pour la requête
    - Méthode : POST
    - URL

    ```powershell
     http://api.ikoddi.com/api/v1/groups/{organization_id}/airtimes
    ```

    - Permission de l’api_key  a générer: Envoyer du airtime
    - Corps de la requête (body) :

    ```json
    {
       "sentTo": ["22670707070","22670707071"],
       "ref": "14258",
       "amount": "1000",
       "countryStringCode": "BF",
       "countryNumberCode": "226",
       "campaignName":"Recharge de mois de juillet"
    }
    ```

    - EXEMPLE CURL

    ```powershell
    curl -X POST -H 'Content-Type: application/json' -H 'x-api-key: <ikoddi-api-key>' -d '{"sentTo":["22670707070","22670707071"],"ref": "14258","amount": "100","countryStringCode": "BF","countryNumberCode": "226","campaignName":"Recharge de mois de juillet"}' https://api.ikoddi.com/api/v1/groups/{organization_id}/airtimes
    ```

    - Eléments du corps de la requête

    <aside>
    ✅   **sentTo: Les destinataires du forfait internet**

    </aside>

    <aside>
    ✅   **ref: C’est l’id du forfait internet que vous obtenez en récupérant la liste des forfaits internet**

    </aside>

    <aside>
    ✅   **amount: Le montant du forfait internet contenu dans le forfait internet que vous récupérer**

    </aside>

    <aside>
    ✅   **countryStringCode: Le code pays des numéros**

    </aside>

    <aside>
    ✅   **countryNumberCode: Le code téléphonique international des numéros**

    </aside>

    <aside>
    ✅   **campaignName: Le nom que vous donner a cette transaction**

    </aside>

    ### **Récupérer le solde du compte AIRTIME**

    - Méthode : GET
    - URL

    ```powershell
    https://api.ikoddi.com/api/v1/groups/{organization_id}/airtime/accounts/current/balance
    ```

    - Nom de l’api_key  a générer: Récupérer le solde airtime
    - EXEMPLE CURL

    ```powershell
    curl -X GET -H 'Content-Type: application/json' -H 'x-api-key: <ikoddi-api-key>' https://api.ikoddi.com/api/v1/groups/{organization_id}/airtime/accounts/current/balance
    ```

    - Format des données de réponse

    ```json
     {
       "balance":"10000",
       "groupId": "5516556",
       "currency":"XOF"
     }
    ```

    ### **Récupération de l’historique des AIRTIMES envoyés**

    - Méthode : GET
    - URL

    ```powershell
    https://api.ikoddi.com/api/v1/groups/{organization_id}/airtimes
    ```

    - Permission de l’api_key  a générer: Récupérer l’historique des airtimes
    - EXEMPLE CURL

    ```powershell
    curl -X GET -H 'Content-Type: application/json' -H 'x-api-key: <ikoddi-api-key>' https://api.ikoddi.com/api/v1/groups/{organization_id}/airtimes
    ```

    - Format des données de réponse

    ```json
    [
      {
            "id":"clk85k5k3000pvxazzxb96nxo",
            "ref":"12249",
        "to":"22670707070",
            "amount":200,
            "airtimeAccountId":"clk84gq2x0009vxazi1gkg9si",
            "status":"SendingOKNoReport",
            "createdAt":"2023-07-18T10:29:20.548Z",
            "updatedAt":"2023-07-18T10:29:20.548Z",
            "campaignName":"Recharge de mois de juillet"
      },
      {
            "id":"clk84tyve000hvxaz2a716ggu",
            "ref":null,
        "to":"22670707071",
            "amount":100,
        "airtimeAccountId":"clk84gq2x0009vxazi1gkg9si",
            "status":"SendingOKNoReport",
            "createdAt":"2023-07-18T10:08:58.827Z",
            "updatedAt":"2023-07-18T10:08:58.827Z",
            "campaignName":null
      },

    ]
    ```

- **API SMS (ENVOYER DES SMS)**

    **NB**: Pour tout envois de SMS, le Sender id (Nom de l’expéditeur) est par défaut Ikoddi, avec un autre Sender Id non vérifié l’envoi échouera.

    Si vous souhaitez utiliser un Sender id personnalisé faite un mail décrivant la plateforme sur laquelle l’intégration sera faite et le Sender id désiré  a contact@kreezus.com .

    Votre Sender id peut contenir jusqu'à 11 caractères alphanumériques ou tirets (-). Il doit contenir au moins une lettre, et ne peut pas se composer uniquement de chiffres. Il doit commencer et se terminer par un caractère alphanumérique. Certains pays et régions peuvent avoir des restrictions supplémentaires.

    ### **Envoyer des sms**

    - Méthode : POST
    - URL

    ```powershell
     https://api.ikoddi.com/api/v1/groups/{organization_id}/sms
    ```

    - Nom de l’api_key  a générer: Envoyer des sms
    - EXEMPLE CURL

    ```powershell
    curl -X POST -H 'Content-Type: application/json' -H 'x-api-key: <ikoddi-api-key>' -d '{"sentTo": ["22670180874","22655995626"],"message": "Hello world","from": "Kreezus","smsBroadCast": "com 1","countryStringCode": "BF","countryNumberCode": "226","messageType": "sms"}' https://api.ikoddi.com/api/v1/groups/{organization_id}/sms
    ```

    - Corps de la requête (body)

    ```json
    {

        "sentTo": ["22670707070","22670707071"],
        "message": "Hello world",
        "from": "Kreezus",
        "smsBroadCast": "com 1",
            "countryStringCode": "BF",
            "countryNumberCode": "226",
            "messageType": "sms"

    }
    ```

    - Eléments du corps de la requête

    <aside>
    ✅   **sentTo: Les destinataires du sms**

    </aside>

    <aside>
    ✅   **message: Votre message**

    </aside>

    <aside>
    ✅   **from: Nom de la personne ou de l’entité qui envoie le message**

    </aside>

    <aside>
    ✅   **countryStringCode: Le code pays des numéros**

    </aside>

    <aside>
    ✅   **countryNumberCode: Le code téléphonique international des numéros**

    </aside>

    <aside>
    ✅   **campaignName: Le nom que vous donner a cette transaction**

    </aside>

    ### **Récupérer le solde du compte SMS**

    - Méthode : GET
    - URL

    ```powershell
    https://api.ikoddi.com/api/v1/groups/{organization_id}/sms/accounts/current/balance
    ```

    - Nom de l’api_key  a générer: Récupérer le solde sms
    - EXEMPLE CURL

    ```powershell
    curl -X GET -H 'Content-Type: application/json' -H 'x-api-key: <ikoddi-api-key>' https://api.ikoddi.com/api/v1/groups/{organization_id}/sms/accounts/current/balance
    ```

    - Format des données de réponse

    ```json
    {
       "balance":"10000",
       "groupId": "5516556",
       "currency":"XOF"
    }
    ```

    ### **Récupérer l’historique des SMS envoyés**

    - Méthode : GET
    - URL

    ```powershell
    https://api.ikoddi.com/api/v1/groups/{organization_id}/sms
    ```

    - Nom de l’api_key  a générer: Récupérer l’historique des sms
    - EXEMPLE CURL

    ```powershell
    curl -X GET -H 'Content-Type: application/json' -H 'x-api-key: <ikoddi-api-key>' https://api.ikoddi.com/api/v1/groups/{organization_id}/sms
    ```

    - Format des données de réponse

    ```json
    [

      {
           "sentTo": ["22670707070","22670707071"],
           "message": "Hello world",
           "from": "Kreezus",
           "cost": "10",
           "smsBroadCast": "com 1",
           "status": "Ok",
           "countryStringCode": "BF",
           "countryNumberCode": "226",
           "messageType": "sms"
      },
      {
             "sentTo": ["22670180872","22655955629"],
             "message": "Hello world",
             "from": "Kreezus",
             "cost": "10",
             "smsBroadCast": "com 1",
             "status": "Ok",
             "countryStringCode": "BF",
             "countryNumberCode": "226",
             "messageType": "sms"
       }

    ]
    ```

    ### OTP

    Envoyer le code OTP

    - Méthode : POST
    - URL

    ```powershell
    https://api.ikoddi.com/api/v1/groups/{organization_id}/otp/{otp_app_id}/{type}/{identity}
    ```

    **type** = **sms**, **email**, **whatsapp**

    - Nom de l’api_key  a générer: Demander et vérifier un OTP
    - EXEMPLE CURL

    ```powershell
    curl -X POST -H 'Content-Type: application/json' -H 'x-api-key: <ikoddi-api-key>' https://api.ikoddi.com/api/v1/groups/{organization_id}/otp/{otp_app_id}/{type}/{identity}
    ```

    - Format des données de réponse

    ```json
    {
        "status":0,
            "otpToken":"<otp-token-returned-by-ikoddi>"
    }
    ```

    Vérifier le code OTP

    - Méthode : POST
    - URL

    ```powershell
    https://api.ikoddi.com/api/v1/groups/{organization_id}/otp/{otp_app_id}/verify
    ```

    - Nom de l’api_key  a générer: Demander et vérifier un OTP
    - EXEMPLE CURL

    ```powershell
    curl -X POST -H 'Content-Type: application/json' -H 'x-api-key: <ikoddi-api-key>' https://api.ikoddi.com/api/v1/groups/{organization_id}/otp/{otp_app_id}/verify
    ```

    - Corps de la requête (body)

    ```json
     {
        "verificationKey": {otpToken}, //UCQS2bNyfVOKENFcCnQTV17OTL/Ja2rt0ku5C0aZMopzE0kQOX10OQ4RF8aT2zQTN0LsTiozcY9e1YMxK7xAd8Tbz1xNnFMlIfz43D0ZQofy3TVAed1zmg52a1+29GGYGuN0NSzvE5fVPFxWvk0jC0f8q8R/84BxhmZD2OaMGVkh1DufftnSXvnV8LXtCMI3
        "otp": {otp}, //12345
        "identity": {identity} //22670707070
     }
    ```

    - Format des données de réponse

    ```json
    {
        "status":0,
        "message":"OTP Matched for 22670707070"
    }

    ```

- API WHATSAPP (Envoyer des messages whatsapp)
    - CURL

        ```jsx
        curl --request POST \
          --url "https://api.ikoddi.com/api/v1/groups/12345/whatsapp/messages" \
          --header "Authorization: Bearer VOTRE_CLE_API" \
          --header "Content-Type: application/json" \
          --data '{
            "to": "22670000000",
            "whatsAppPhoneNumber": "+226 70 00 00 00",
            "templateName": "template_variables_nommes",
            "category": "MARKETING",
            "parameters": [
              {
                "parameter_name": "first_name",
                "text": "Aziz",
                "type": "text"
              },
              {
                "parameter_name": "last_name",
                "text": "Ky",
                "type": "text"
              }
            ]
          }'
        ```

        - `12345` → votre `groupId`
        - `VOTRE_CLE_API` → la clé générée avec la permission **Écrire les messages WhatsApp**
        - `22670000000` → numéro du destinataire au format international sans `+`
        - `+226 70 00 00 00` → numéro WhatsApp expéditeur configuré
        - `template_variables_nommes` → nom réel du template Meta approuvé

        **Description des champs**

        - `to` : numéro du destinataire au format international sans le signe `+`.Exemple : `22670000000`
        - `whatsAppPhoneNumber` : numéro WhatsApp Business expéditeur configuré sur votre compte.Exemple : `+226 70 00 00 00`
        - `templateName` : nom exact du template WhatsApp approuvé dans Meta.Exemple : `template_variables_nommes`
        - `category` : catégorie du template WhatsApp.Valeurs possibles :
            - `MARKETING`
            - `UTILITY`
            - `AUTHENTICATION`
        - `parameters` : (optionnel) liste des paramètres dynamiques du template.Ce champ est obligatoire uniquement si le template contient des variables.

        ### Description des champs `parameters`

        - `parameter_name` : nom du paramètre défini dans le template Meta.Ce champ est requis uniquement pour les templates utilisant des paramètres nommés.Exemple : `first_name`
        - `text` : valeur à injecter dans la variable du template.Exemple : `Aziz`
        - `type` : type du paramètre.Valeur généralement utilisée : `text`

    **NB**

    status :  0 ⇒ succès
    status :  -1 ⇒ échec


**ETAPES D’INTEGRATION VIA LA LIBRAIRIE**

- **INSTALLATION DE LA LIBRAIRIE IKODDI**

    ```powershell
    npm install ikoddi-client-sdk
    ```

    ```powershell
     yarn install ikoddi-client-sdk
    ```

- **RECUPERER LA LISTE DES FORFAITS INTERNET**

    ```jsx
    import {Ikoddi} from 'ikoddi-client-sdk';

    /*Initialisation de de la clé api d'envoi de AIRTIME, de l'url,
      et de l'organisation_id*/

    const airtimeAccount = new Ikoddi()
      .withApiKey("OyDOoUbrAaWMm5U67j4JIzwhzUeZvGWT")
      .withGroupId("10268496");

    /*Récuperation de la liste de forfaits internet*/
    const internetPlans = ikoddiClient.internetPlans();
    ```

- **ENVOYER DU AIRTIME**

    ```jsx
    import {Ikoddi} from 'ikoddi-client-sdk';

    /*Initialisation de de la clé api d'envoi de AIRTIME, de l'url,
    et de l'organisation_id*/

    /*Pour les tests ajouter
    .withApiBaseURL("https://api.staging.ikoddi.com/api/v1/groups/")
    dans l'initialisation*/

    const ikoddiClient = new Ikoddi()
      .withApiKey("OyDOoUbrAaWMm5U67j4JIzwhzUeZvGWT")
      .withGroupId("10268496");

    /*Envoi d'Airtime*/
    ikoddiClient.sendAirtime(["22670707070"],"12131","1000","Recharge du mois");
    ```

- ENVOYER DES SMS

    ```jsx
    import {Ikoddi} from 'ikoddi-client-sdk';

    /*Initialisation de de la clé api d'envoi de SMS, de l'url,
     et de l'organisation_id*/

    /*Pour les tests ajouter
    .withApiBaseURL("https://api.staging.ikoddi.com/api/v1/groups/")
    dans l'initialisation*/

    const ikoddiClient = new Ikoddi()
       .withApiKey("9V1yvLG5m6zOp6zz4X6HI6N1YbZgdIQj")
       .withGroupId("10268496");

    /*Envoi SMS*/
    ikoddiClient.sendSMS(["22670707070"], "ikoddi", "Hello les devs", "Message jounalier");
    ```

- OTP

    ```jsx
    // 1- Initialize Ikoddi Client
    const ikoddiClient = new Ikoddi()
      .withApiKey("OyDOoUbrAaWMm5U67j4JIzwhzUeZvGWT")
      .withGroupId("10268496")
      // Create an OTP App and set its value
      .withOtpAppId("clmecvbby0000j39j4ozy2z3c");

    // 2- Send OTP Code

    const otpResponse = await ikoddiClient.sendOTP("22670707070");
    console.log(otpResponse);
    // {"status":0,"otpToken":"UCQS2bNyfVOKENFcCnQTV17OTL/Ja2rt0ku5C0aZMopzE0kQOX10OQ4RF8aT2zQTN0LsTiozcY9e1YMxK7xAd8Tbz1xNnFMlIfz43D0ZQofy3TVAed1zmg52a1+29GGYGuN0NSzvE5fVPFxWvk0jC0f8q8R/84BxhmZD2OaMGVkh1DufftnSXvnV8LXtCMI3"}

    // The User should receive an OTP code

    // 3- Verify the OTP
    const otpVerifyResponse = await ikoddiClient
      .verifyOTP({ identity: "22670707070", otp: "629185", verificationKey: otpResponse.otpToken })
    console.log(otpVerifyResponse);
    // {"status":0,"message":"OTP Matched for 22670707070"}
    ```


## **REPONSES DE L’API**

<aside>
🙂

Succès :

- 200 : **La ressource à été récupérée avec succès**
- 201  : La r**essource à été créée avec succès**
</aside>

<aside>
😑 Erreur :

- 400 :  **Mauvaise** **requête,** **Solde insuffisant**
- 401  :  **Problème d’authentification**
- 403 :  **Pas de permission pour cette requête**
- 404 :  **Numéros incorrectes ou absent**
</aside>

**Vous pouvez consulter la documentation API SWAGGER**

👉🏽  https://api.ikoddi.com/api/


# ##########################
