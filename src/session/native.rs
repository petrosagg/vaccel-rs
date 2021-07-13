use std::convert::TryInto;
use std::collections::HashMap;
use std::marker::PhantomData;

use futures::future::{self, Ready};
use tarpc::context::{self, Context};

use crate::session::Session;
use crate::resource::{Resource, ResourceRef};

pub struct NativeSession {
    resource_id: u64,
    resources: HashMap<u64, Resource>,
}

impl Session for &mut NativeSession {
    type RegisterResourceFut = Ready<u64>;

    fn register_resource(self, _: Context, resource: Resource) -> Self::RegisterResourceFut {
        self.resource_id += 1;
        self.resources.insert(self.resource_id, resource);
        future::ready(self.resource_id)
    }
}

impl NativeSession {
    pub async fn register<T: Into<Resource>>(&mut self, data: T) -> ResourceRef<T> {
        let id = self.register_resource(context::current(), data.into()).await;
        ResourceRef { id, marker: PhantomData }
    }

    pub fn get_resource<T>(&self, resource: &ResourceRef<T>) -> Option<&T>
        where for <'a> &'a Resource: TryInto<&'a T>
    {
        self.resources.get(&resource.id)?.try_into().ok()
    }
}
