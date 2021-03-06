use std::convert::TryInto;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use dashmap::DashMap;
use tarpc::context::Context;

use crate::resource::{Resource, ResourceRef};

#[tarpc::service]
pub trait Vaccel {
    /// Core method that registers a resource for future use
    async fn register_resource(resource: Resource) -> u64;

    /// Example operation that returns the length of the passed resource
    async fn length(data: ResourceRef<String>) -> usize;
}

/// An RPC request Server that is created for every RPC request that comes in.
/// It holds a reference counted ServerState in order to interact with the registered resources
#[derive(Default, Clone)]
pub struct Server(Arc<ServerState>);

/// A Vaccel server
#[derive(Default)]
pub struct ServerState {
    resource_id: AtomicU64,
    resources: DashMap<u64, Arc<Resource>>,
}

impl Server {
    /// Used by operation implementations to resolve resource references into actual resources that
    /// they can use. Since resources can be used concurrently by many requests only shared
    /// references are even given
    pub fn get_resource<T>(&self, resource: &ResourceRef<T>) -> Option<Arc<Resource>> {
        self.0
            .resources
            .get(&resource.id)
            .map(|r| Arc::clone(r.value()))
    }
}

#[tarpc::server]
impl Vaccel for Server {
    async fn register_resource(self, _: Context, resource: Resource) -> u64 {
        let id = self.0.resource_id.fetch_add(1, Ordering::SeqCst);
        self.0.resources.insert(id, Arc::new(resource));
        id
    }

    /// Example implementation of an operation. This is where the code would call into library code
    /// like tensorflow etc to do the actual work.
    async fn length(self, _: Context, data: ResourceRef<String>) -> usize {
        // TODO: this is ugly, lift this logic in get_resource and make that return an Option<&T>
        let resource = self.get_resource(&data).unwrap();
        let data: &String = (&*resource).try_into().unwrap();
        data.len()
    }
}
