use std::ops::Deref;

use iron::prelude::*;
use iron::status;

use serde_json;
use persistent::State;
use bodyparser;
use router::Router;

use todo::{TodoItem, TodoPartial};
use todos::{TodoItems};

pub fn list_handler(request: &mut Request) -> IronResult<Response> {
    let mutex = request.get::<State<TodoItems>>().unwrap();
    let todo_items = mutex.read().unwrap();

    let encoded_json = serde_json::to_string(todo_items.deref()).unwrap();
    Ok(Response::with((status::Ok, encoded_json)))
}

pub fn create_handler(request: &mut Request) -> IronResult<Response> {
    // TODO: all the itry iexpect calls can be probably rewitten using the functor methods
    let body_option = itry!(request.get::<bodyparser::Raw>(), status::BadRequest);
    let raw_body = iexpect!(body_option, status::BadRequest);
    let todo_partial: TodoPartial = itry!(serde_json::from_str(&raw_body), status::BadRequest);
    
    let mutex = request.get::<State<TodoItems>>().unwrap();
    let mut todo_items = mutex.write().unwrap();

    let new_todo_item = TodoItem::new(todo_partial.text);
    todo_items.push(new_todo_item.clone());
    
    // ugly json creation
    Ok(Response::with((status::Ok, format!("{{\"id\": \"{}\"}}", new_todo_item.get_id()))))
}

// TODO: add support for updating state as well
pub fn update_handler(request: &mut Request) -> IronResult<Response> {
    // get param id
    let id = request.extensions.get::<Router>().unwrap().find("id").unwrap().to_owned();
    
    // get the body
    let body_option = itry!(request.get::<bodyparser::Raw>(), status::BadRequest);
    let raw_body = iexpect!(body_option, status::BadRequest);
    let todo_partial = itry!(serde_json::from_str(&raw_body), status::BadRequest);

    let mutex = request.get::<State<TodoItems>>().unwrap();
    let mut todo_items = mutex.write().unwrap();

    // find item (as mutable)
    let item_option = todo_items.iter_mut().find(|item| item.get_id() == id);
    let mut item = iexpect!(item_option, status::NotFound);

    item.update(todo_partial);

    Ok(Response::with((status::Ok, format!("{{\"id\": \"{}\"}}", item.get_id()))))
}

pub fn delete_handler(request: &mut Request) -> IronResult<Response> {
    // get param id
    let id = request.extensions.get::<Router>().unwrap().find("id").unwrap().to_owned();

    let mutex = request.get::<State<TodoItems>>().unwrap();
    let mut todo_items = mutex.write().unwrap();

    let item_option = todo_items.iter().position(|item| item.get_id() == id);
    let item_position = iexpect!(item_option, status::NotFound);

    todo_items.remove(item_position);

    Ok(Response::with((status::Ok, "")))
}
