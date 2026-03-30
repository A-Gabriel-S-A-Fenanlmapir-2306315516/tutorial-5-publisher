pub mod notification;
pub mod product;

use rocket::fairing::AdHoc;

pub fn route_stage() -> AdHoc {
    return AdHoc::on_ignite("Initializing controller routes...", |rocket| async {
        rocket
            .mount("/product", routes![product::create, product::list, product::read, product::delete])
            .mount("    /notification", routes![notification::subscribe, notification::unsubscribe])
    });
}

pub fn route_stage() -> AdHoc {
    return AdHoc::on_ignite("Initializing controller routes...", |rocket| async {
        rocket
            .mount("/product", routes![
                product::create, 
                product::list, 
                product::read, 
                product::delete, 
                product::publish // Tambahkan ini
            ])
            .mount("/notification", routes![
                notification::subscribe, 
                notification::unsubscribe
            ])
    });
}