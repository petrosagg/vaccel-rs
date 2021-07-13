use crate::resource::Resource;

mod native;
mod vsock;

#[tarpc::service]
trait Session {
    async fn register_resource(resource: Resource) -> u64;
}

pub use native::NativeSession;
