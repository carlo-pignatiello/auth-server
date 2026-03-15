use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation
};
use std::fs;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: i64,
    pub iat: i64,
    pub scope: String,
    pub email: Option<String>,
    pub name: Option<String>
}

#[derive(Clone)]
pub struct JwtKeys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
    pub kid: String
}


impl JwtKeys {
    pub fn load(private_path: &str, public_key: &str, kid: &str) -> anyhow::Result<Self> {
        let private_pem = fs::read(private_path)?;
        let public_pem = fs::read(public_key)?;

        Ok(Self {
            encoding: EncodingKey::from_rsa_pem(&private_pem)?,
            decoding: DecodingKey::from_rsa_pem(&public_pem)?,
            kid: kid.to_string(),
        })
    }

    pub fn sign(&self, claims: &Claims) -> anyhow::Result<String> {
        let mut header = Header::new(Algorithm::RS256);
        header.kid = Some(self.kid.clone());
        encode(&header, claims, &self.encoding)
            .map_err(|e| anyhow::anyhow!("Sign error: {e}"))
    } 

    pub fn verify(&self, token: &str, audience: &str) -> anyhow::Result<TokenData<Claims>> {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[audience]);
        decode::<Claims>(token, &self.decoding, &validation)
            .map_err(|e| anyhow::anyhow!("Verify error: {e}"))
    }
}

