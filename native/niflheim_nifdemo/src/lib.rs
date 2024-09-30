use rustler::Encoder;
use rustler::{Env, Error, ResourceArc, Term};
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

#[rustler::nif]
fn init() -> Result<ResourceArc<CollectionResource>, Error> {
    let resource = ResourceArc::new(CollectionResource {
        custom_collection: RwLock::new(CustomCollection {
            collection: vec![1.1, 4.5, 6.7],
        }),
    });

    Ok(resource)
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

fn load(env: Env, _: Term) -> bool {
    rustler::resource!(CollectionResource, env);
    true
}

rustler::init!("Elixir.Niflheim.NifDemo", load = load);
