extern crate iron;
#[macro_use] 
extern crate router;
extern crate uuid;

mod todo;
mod controller;

use controller::{
    list_handler, 
    create_handler, 
    detail_handler, 
    delete_handler, 
    update_handler
};

use iron::prelude::Iron;

fn main() {
    let router = router!(
        list: get "/api/tasks" => list_handler,
        create: post "/api/tasks" => create_handler,
        detail: get "/api/tasks/:id" => detail_handler,
        update: put "/api/tasks/:id" => update_handler,
        delete: delete "/api/tasks/:id" => delete_handler
    );
    
    Iron::new(router).http("localhost:3000").unwrap();
}
