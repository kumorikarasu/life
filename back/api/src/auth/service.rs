use actix_session::Session;
use oauth2::{
    basic::{self, BasicClient, BasicErrorResponseType, BasicTokenType}, AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields, PkceCodeVerifier, RedirectUrl, Scope, StandardErrorResponse, StandardTokenIntrospectionResponse, StandardTokenResponse, TokenResponse, TokenUrl
};
use sea_orm::{sqlx::types::chrono, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleUserInfo {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub picture: Option<String>,
    pub locale: Option<String>,
}

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
        
        // Get user info from Google using the access token
        let user_info = self.fetch_google_user_info(token.access_token().secret()).await?;
        println!("User info: {:?}", user_info);
        
        // Create or update user in the database
        let user_id = self.create_or_update_user(&user_info).await?;
        
        // Store user info in session
        session.insert("user_id", &user_info.id).unwrap();
        session.insert("user_email", &user_info.email).unwrap();
        session.insert("user_name", &user_info.name).unwrap();
        session.insert("user_picture", &user_info.picture).unwrap();
        
        Ok(user_id)
    }
    
    /// Fetch user information from Google using the access token
    async fn fetch_google_user_info(&self, access_token: &str) -> Result<GoogleUserInfo, String> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| format!("Failed to fetch user info: {}", e))?;
            
        if !response.status().is_success() {
            return Err(format!("Google API error: {}", response.status()));
        }
        
        let user_info = response
            .json::<GoogleUserInfo>()
            .await
            .map_err(|e| format!("Failed to parse user info: {}", e))?;
            
        Ok(user_info)
    }
    
    /// Create or update a user in the database using Google user information
    async fn create_or_update_user(&self, user_info: &GoogleUserInfo) -> Result<String, String> {
        use crate::entities::users::{Entity as User, ActiveModel};
        use crate::entities::users::Column;
        use sea_orm::IntoActiveModel;
        
        // Check if user already exists
        let existing_user = User::find()
            .filter(Column::GoogleId.eq(&user_info.id))
            .one(&self.db)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
            
        match existing_user {
            Some(user) => {
                // Update existing user
                let mut user_model: ActiveModel = user.into_active_model();
                user_model.email = Set(user_info.email.clone());
                user_model.name = Set(user_info.name.clone());
                user_model.given_name = Set(user_info.given_name.clone());
                user_model.family_name = Set(user_info.family_name.clone());
                user_model.picture = Set(user_info.picture.clone());
                user_model.locale = Set(user_info.locale.clone());
                
                let updated_user = user_model.update(&self.db)
                    .await
                    .map_err(|e| format!("Failed to update user: {}", e))?;
                    
                Ok(updated_user.google_id)
            },
            None => {
                // Create new user
                let new_user = ActiveModel {
                    id: Default::default(), // Auto-increment
                    google_id: Set(user_info.id.clone()),
                    email: Set(user_info.email.clone()),
                    name: Set(user_info.name.clone()),
                    given_name: Set(user_info.given_name.clone()),
                    family_name: Set(user_info.family_name.clone()),
                    picture: Set(user_info.picture.clone()),
                    locale: Set(user_info.locale.clone()),
                    created_at: Set(chrono::Utc::now().naive_utc()),
                    updated_at: Set(None),
                };
                
                let inserted_user = new_user.insert(&self.db)
                    .await
                    .map_err(|e| format!("Failed to create user: {}", e))?;
                    
                Ok(inserted_user.google_id)
            }
        }
    }
}
