use futures::future::Ready;
use tarpc::context::Context;

use crate::resource::{ResourceRef};
use crate::session::NativeSession;

#[tarpc::service]
trait Image {
    async fn segmentation(image: ResourceRef<Vec<u8>>) -> u64;
}

impl Image for &mut NativeSession {
    type SegmentationFut = Ready<u64>;

    fn segmentation(self, _: Context, image: ResourceRef<Vec<u8>>) -> Self::SegmentationFut {
        // Get the resource from the session
        let _image = self.get_resource(&image).unwrap();

        unimplemented!();
    }
}
