mod check_amzn_oidc_data;

#[rocket::get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().attach(check_amzn_oidc_data::User::default()).mount("/", rocket::routes![index])
}
