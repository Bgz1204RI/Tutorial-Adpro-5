pub mod product;
pub mod notification;

use rocket::fairing::AdHoc;

pub fn route_stage() -> AdHoc {
    return AdHoc::on_ignite("Initializing controller routes...", |rocket| async {
        rocket
            .mount("/product", routes![product::create, product::list, product::read])
            .mount("/notification", routes![notification::subscribe, notification::unsubscribe])
    });
}