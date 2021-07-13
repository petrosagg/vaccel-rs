use std::convert::TryInto;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use futures::future::{self, Ready};
use tarpc::context::{self, Context};
use crate::resource::{Resource, ResourceRef};

#[tarpc::service]
trait Vaccel {
    async fn register_resource(resource: Resource) -> u64;

    // Image ops
    async fn segmentation(image: ResourceRef<Vec<u8>>) -> u64;

    // Tensorflow ops
    async fn inference(model: ResourceRef<Vec<u8>>) -> String;
}

#[derive(Default, Clone)]
struct VaccelHandler(Arc<Mutex<VaccelState>>);

#[derive(Default)]
struct VaccelState {
    resource_id: u64,
    resources: HashMap<u64, Arc<Resource>>,
}

impl VaccelHandler {
    pub async fn register<T: Into<Resource>>(&self, data: T) -> ResourceRef<T> {
        let id = self.clone().register_resource(context::current(), data.into()).await;
        ResourceRef { id, marker: PhantomData }
    }

    pub fn get_resource<T>(&self, resource: &ResourceRef<T>) -> Option<Arc<Resource>> {
        let this = self.0.lock().unwrap();
        this.resources.get(&resource.id).cloned()
    }
}

impl Vaccel for VaccelHandler {
    type RegisterResourceFut = Ready<u64>;
    fn register_resource(self, _: Context, resource: Resource) -> Self::RegisterResourceFut {
        let mut this = self.0.lock().unwrap();
        this.resource_id += 1;
        let id = this.resource_id;
        this.resources.insert(id, Arc::new(resource));
        future::ready(id)
    }

    type SegmentationFut = Ready<u64>;
    fn segmentation(self, _: Context, _image: ResourceRef<Vec<u8>>) -> Self::SegmentationFut {
        future::ready(42)
    }

    type InferenceFut = Ready<String>;
    fn inference(self, _: Context, _model: ResourceRef<Vec<u8>>) -> Self::InferenceFut {
        future::ready("cat".to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tarpc::server::{BaseChannel, Channel};
    use tarpc::transport::channel;
    use tarpc::client;

    #[tokio::test]
    async fn it_works() {
        let (client_transport, server_transport) = channel::unbounded();
        let server = BaseChannel::with_defaults(server_transport);
        tokio::spawn(server.execute(VaccelHandler::default().serve()));
        let mut client = VaccelClient::new(client::Config::default(), client_transport).spawn();

        let fake_resource = ResourceRef { id: 0, marker: std::marker::PhantomData };
        let resp = client.inference(context::current(), fake_resource).await.unwrap();
        assert_eq!(&resp, "cat");
    }
}
