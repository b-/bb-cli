use blue_build_utils::platform::Platform;
use bon::Builder;
use oci_distribution::Reference;

#[derive(Debug, Clone, Builder, Hash)]
#[builder(derive(Clone))]
pub struct GetMetadataOpts<'scope> {
    #[builder(into)]
    pub image: &'scope Reference,

    #[builder(default)]
    pub platform: Platform,
}
