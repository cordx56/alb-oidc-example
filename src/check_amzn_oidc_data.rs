use rocket::{Request, Data};
use rocket::fairing::{Fairing, Info, Kind};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AmznOidcJwtHeader {
    pub typ: String,
    pub kid: String,
    pub alg: String,
    pub iss: String,
    pub client: String,
    pub signer: String,
    pub exp: i64,
}
#[derive(Serialize, Deserialize)]
pub struct AmznOidcJwtPayload {
    pub sub: String,
    pub name: String,
    pub email: String,
    pub exp: i64,
    pub iss: String,
}


#[derive(Default, Clone)]
pub struct User {
    email: String,
}

#[rocket::async_trait]
impl Fairing for User {
    fn info(&self) -> Info {
        Info {
            name: "Amazon OIDC data",
            kind: Kind::Request,
        }
    }
    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        let oidc_data_some = req.headers().get_one("x-amzn-oidc-data");
        if let Some(oidc_data) = oidc_data_some {
            let splitted: Vec<&str> = oidc_data.split(".").collect();
            if splitted.len() == 3 {
                let jwt_headers = splitted[0];
                if let Ok(decoded_jwt_headers_bytes) = base64::decode(jwt_headers) {
                    if let Ok(decoded_jwt_headers) = String::from_utf8(decoded_jwt_headers_bytes.to_vec()) {
                        if let Ok(jwt_header_data) = serde_json::from_str::<AmznOidcJwtHeader>(&decoded_jwt_headers) {
                            let kid = jwt_header_data.kid;
                            let region = "ap-northeast-1";
                            let url = format!("https://public-keys.auth.elb.{}.amazonaws.com/{}", region, kid);
                            if let Ok(get_request) = reqwest::get(url).await {
                                if let Ok(pub_key) = get_request.text().await {
                                    if let Ok(decoding_key) = jsonwebtoken::DecodingKey::from_rsa_pem(pub_key.as_bytes()) {
                                        if let Ok(payload) = jsonwebtoken::decode::<AmznOidcJwtPayload>(oidc_data, &decoding_key, &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::ES256)) {
                                            println!("{}", payload.claims.email);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
