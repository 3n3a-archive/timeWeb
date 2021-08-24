#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Welcome to time web. An API to calculate time values."
}

#[get("/from/<hours>/<minutes>/to/<hours2>/<minutes2>")]
fn from_to(hours: f64, minutes: f64, hours2: f64, minutes2: f64) -> String {
    // convert from hours:minutes to all minutes
    let from_minutes: f64 = (hours*60_f64)+minutes;
    let to_minutes: f64 = (hours2*60_f64)+minutes2;

    let time_diff: f64 = (to_minutes - from_minutes)/60_f64; // time diff in hours -> float

    // convert back to hours:minutes
    let time_diff_hrs: f64 = time_diff.floor();
    let time_diff_mins: f64 = (time_diff - time_diff_hrs)*60_f64;

    return format!("{}:{} hrs", &time_diff_hrs, &time_diff_mins.round());
}

#[get("/minutes/<hours>/<minutes>")]
fn in_minutes(hours: f64, minutes: f64) -> String {
    let time_minutes: f64 = (hours*60_f64)+minutes;

    return format!("{} mins", time_minutes);
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/time", routes![from_to, in_minutes])
}
