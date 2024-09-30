use rustler::Encoder;
use rustler::{Env, Error, ResourceArc, Term};
use std::sync::RwLock;

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

struct CollectionResource {
    pub collection: RwLock<Vec<f64>>,
}

#[rustler::nif]
fn init() -> Result<ResourceArc<CollectionResource>, Error> {
    let resource = ResourceArc::new(CollectionResource {
        collection: RwLock::new(vec![1.1, 4.5, 6.7]),
    });

    Ok(resource)
}

#[rustler::nif]
fn add_items(resource: ResourceArc<CollectionResource>, items: Vec<i64>) -> ResourceArc<CollectionResource> {
    resource.collection.write().unwrap().extend(items.iter().map(|&x| x as f64));
    resource
}

#[rustler::nif]
fn read(env: Env, resource: ResourceArc<CollectionResource>) -> Term {
    let resource_struct = resource.collection.read().unwrap();

    // Convert the collection to a Rustler term
    let collection: Vec<f64> = resource_struct.clone();
    collection.encode(env)
}

fn load(env: Env, _: Term) -> bool {
    rustler::resource!(CollectionResource, env);
    true
}

rustler::init!("Elixir.Niflheim.NifDemo", load = load);
