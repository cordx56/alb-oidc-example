use rocket::{Request, Data};
use rocket::fairing::{Fairing, Info, Kind};
use std::io::{stdout, Write};

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
            println!("{}", oidc_data);
        }
    }
}
