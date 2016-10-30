use std::ops::Deref;

use iron::prelude::*;
use iron::status;

use rustc_serialize::json;
use persistent::State;
use bodyparser;

use todo::{TodoItem, TodoPartial};
use todos::{TodoItems};

pub fn list_handler(request: &mut Request) -> IronResult<Response> {
    let mutex = request.get::<State<TodoItems>>().unwrap();
    let todo_items = mutex.read().unwrap();

    let encoded_json = json::encode(todo_items.deref()).unwrap();
    Ok(Response::with((status::Ok, encoded_json)))
}

pub fn create_handler(request: &mut Request) -> IronResult<Response> {
    // get the body
    let body_option = itry!(request.get::<bodyparser::Raw>(), status::BadRequest);

    // I'm tired of this shit
    let raw_body = iexpect!(body_option, status::BadRequest);

    // Parse json
    let todo_partial = itry!(json::decode::<TodoPartial>(&raw_body), status::BadRequest);
    
    let mutex = request.get::<State<TodoItems>>().unwrap();
    let mut todo_items = mutex.write().unwrap();

    let new_todo_item = TodoItem::new(todo_partial.text);
    todo_items.push(new_todo_item.clone());
    
    // ugly json creation
    Ok(Response::with((status::Ok, format!("{{\"id\": \"{}\"}}", new_todo_item.get_id()))))
}

pub fn update_handler(request: &mut Request) -> IronResult<Response> {
    unimplemented!();
}

pub fn delete_handler(request: &mut Request) -> IronResult<Response> {
    unimplemented!();
}
