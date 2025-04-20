use actix_session::Session;
use oauth2::{
    basic::{self, BasicClient, BasicErrorResponseType, BasicTokenType}, AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields, PkceCodeVerifier, RedirectUrl, Scope, StandardErrorResponse, StandardTokenIntrospectionResponse, StandardTokenResponse, TokenResponse, TokenUrl
};
use sea_orm::DatabaseConnection;
use std::env;

#[derive(Clone)]
pub struct AuthService {
    db: DatabaseConnection,
    oauth_client: Client<StandardErrorResponse<BasicErrorResponseType>, StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>, oauth2::StandardRevocableToken, StandardErrorResponse<oauth2::RevocationErrorResponseType>, oauth2::EndpointSet, oauth2::EndpointNotSet, oauth2::EndpointNotSet, oauth2::EndpointNotSet, oauth2::EndpointSet>,
}

impl AuthService {
    pub fn new(db: DatabaseConnection) -> Self {
        let client_id = ClientId::new(
            env::var("OAUTH_CLIENT_ID").expect("Missing OAUTH_CLIENT_ID")
        );
        let client_secret = ClientSecret::new(
            env::var("OAUTH_CLIENT_SECRET").expect("Missing OAUTH_CLIENT_SECRET")
        );
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .expect("Invalid auth URL");

        let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
            .expect("Invalid token URL");
        
        let redirect_url = RedirectUrl::new(
            env::var("OAUTH_REDIRECT_URL").expect("Missing OAUTH_REDIRECT_URL")
        ).expect("Invalid redirect URL");

        let oauth_client = BasicClient::new(client_id)
            .set_client_secret(client_secret)
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(redirect_url);

        AuthService {
            db,
            oauth_client,
        }
    }

    pub fn login(&self, session: Session) -> (String, CsrfToken, PkceCodeVerifier) {
        let (pkce_challenge, pkce_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token) = self.oauth_client.clone()
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("profile".to_string()))
            .add_scope(Scope::new("email".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();
        
        session.insert("pkce_verifier", pkce_verifier.secret()).unwrap();
        session.insert("csrf_token", csrf_token.secret()).unwrap();

        (auth_url.to_string(), csrf_token, pkce_verifier)
    }

    pub async fn auth(&self, code: AuthorizationCode, csrf_token: CsrfToken, session: Session) -> Result<String, String> {
        // Verify the CSRF token
        // In a real application, you'd want to check this against a session or database
        println!("CSRF Token: {}", csrf_token.secret());

        let p = session.get::<String>("pkce_verifier").unwrap();
        let csrf_session = session.get::<String>("csrf_token").unwrap().unwrap();
        let csrf_req = csrf_token.secret().to_string();

        println!("Session PKCE Verifier: {:?}", p);
        println!("Session CSRF Token: {:?}", csrf_session);
        println!("CSRF Token: {}", csrf_token.secret());
        println!("Code: {}", code.secret());

        if csrf_req != csrf_session {
            return Err("Invalid CSRF token".to_string());
        }

        let http_client = reqwest::ClientBuilder::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

        // Exchange the authorization code for an access token
        let token = self.oauth_client
            .exchange_code(code)
            .set_pkce_verifier(PkceCodeVerifier::new(p.unwrap()))
            .request_async(&http_client)
            .await.unwrap();

        println!("Token: {:?}", token);
        // Here you would typically:
        // 1. Get user info from Google using the access token
        // 2. Create or update user in your database
        // 3. Create a session or JWT token
        // For now, we'll just return the access token
        Ok(token.access_token().secret().to_string())
    }

}
