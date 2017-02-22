#[macro_use] 
extern crate router;
extern crate uuid;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate iron;
extern crate persistent;
extern crate bodyparser;

mod todo;
mod todos;
mod controller;

use iron::prelude::*;
use persistent::{State, Read};

use todos::{TodoItems};
use controller::{
    list_handler,
    create_handler,
    delete_handler,
    update_handler
};

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

fn main() {
    // initialize router
    let router = router!(
        list: get "/api/tasks" => list_handler,
        create: post "/api/tasks" => create_handler,
        update: put "/api/tasks/:id" => update_handler,
        delete: delete "/api/tasks/:id" => delete_handler
    );

    // initialize tasks storage (in memory)
    let tasks_store = State::<TodoItems>::one(TodoItems::new());
    let body_parser = Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH);

    let mut chain = Chain::new(router);
    chain.link_before(tasks_store); // Register in memory store
    chain.link_before(body_parser); // Register iron body parser
    
    Iron::new(chain).http("localhost:3000").unwrap();
}
