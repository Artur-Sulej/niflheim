use rustler::{Encoder, NifStruct};
use rustler::{Env, ResourceArc, Term};
use std::sync::RwLock;

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

struct CustomCollection {
    pub collection: Vec<f64>,
}

struct CollectionResource {
    pub custom_collection: RwLock<CustomCollection>,
}

#[derive(NifStruct)]
#[module = "Niflheim.NifDemo.CustomData"]
struct CustomData {
    pub a: i64,
    pub b: f64,
}

#[rustler::nif]
fn init() -> ResourceArc<CollectionResource> {
    ResourceArc::new(CollectionResource {
        custom_collection: RwLock::new(CustomCollection {
            collection: vec![1.1, 4.5, 6.7],
        }),
    })
}

#[rustler::nif]
fn add_items(resource: ResourceArc<CollectionResource>, items: Vec<i64>) -> ResourceArc<CollectionResource> {
    resource.custom_collection.write().unwrap().collection.extend(items.iter().map(|&x| x as f64));
    resource
}

#[rustler::nif]
fn read(env: Env, resource: ResourceArc<CollectionResource>) -> Term {
    let custom_collection = resource.custom_collection.read().unwrap();

    // Convert the collection to a Rustler term
    custom_collection.collection.encode(env)
}

#[rustler::nif]
fn get_data(env: Env, resource: ResourceArc<CollectionResource>) -> Term {
    let custom_collection = resource.custom_collection.read().unwrap();

    let data = *custom_collection.collection.last().unwrap();

    CustomData { a: (data * 2.0) as i64, b: data }.encode(env)
}

fn load(env: Env, _: Term) -> bool {
    rustler::resource!(CollectionResource, env);
    true
}

rustler::init!("Elixir.Niflheim.NifDemo", load = load);
