use rustler::Encoder;
use rustler::{Env, Error, ResourceArc, Term};
use std::sync::RwLock;

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

mod atoms {
    rustler::atoms! {
        ok,
        error,
        unknown, // Other error
    }
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
fn read(
    env: Env,
    resource: ResourceArc<CollectionResource>,
) -> Result<Term, Error> {
    let resource_struct = resource.collection.read().unwrap();

    // Convert the collection to a Rustler term
    let collection: Vec<f64> = resource_struct.clone();
    let term = collection.encode(env);

    Ok(term)
}

fn load(env: Env, _: Term) -> bool {
    rustler::resource!(CollectionResource, env);
    true
}

rustler::init!("Elixir.Niflheim.NifDemo", load = load);
