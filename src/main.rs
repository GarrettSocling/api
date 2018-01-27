/* Pi-hole: A black hole for Internet advertisements
*  (c) 2018 Pi-hole, LLC (https://pi-hole.net)
*  Network-wide ad blocking via your own hardware.
*
*  API
*  Program Main
*
*  This file is copyright under the latest version of the EUPL.
*  Please see LICENSE file for your rights under this license. */

#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate rmp;
extern crate regex;

mod util;
mod ftl;
mod stats;
mod dns;
mod web;

fn main() {
    // Start up the server
    rocket::ignite()
        .manage(ftl::FtlConnectionType::Socket)
        // Mount the web interface
        .mount("/", routes![
            web::web_interface_index,
            web::web_interface
        ])
        // Mount the API
        .mount("/admin/api", routes![
            stats::summary,
            stats::top_domains,
            stats::top_domains_params,
            stats::top_blocked,
            stats::top_blocked_params,
            stats::top_clients,
            stats::top_clients_params,
            stats::forward_destinations,
            stats::query_types,
            stats::history,
            stats::history_timespan,
            stats::recent_blocked,
            stats::recent_blocked_multi,
            stats::clients,
            stats::unknown_queries,
            stats::over_time_history,
            stats::over_time_forward_destinations,
            stats::over_time_query_types,
            stats::over_time_clients,
            dns::get_whitelist,
            dns::get_blacklist,
            dns::get_wildlist,
            dns::status,
            dns::add_whitelist,
            dns::add_blacklist,
            dns::add_wildlist,
            dns::delete_whitelist,
            dns::delete_blacklist,
            dns::delete_wildlist
        ])
        // Add custom error handlers
        .catch(errors![not_found])
        .launch();
}

/// This is run when no route could be found and returns a custom 404 message.
#[error(404)]
fn not_found() -> util::Reply {
    util::reply_error(util::Error::NotFound)
}
