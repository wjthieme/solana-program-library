use {
    crate::extension::{Extension, ExtensionType},
    bytemuck::{Pod, Zeroable},
    spl_pod::optional_keys::OptionalNonZeroPubkey,
};

/// Instructions for the MetadataPointer extension
pub mod instruction;
/// Instruction processor for the MetadataPointer extension
pub mod processor;

/// Metadata pointer extension data for mints.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
pub struct MetadataPointer {
    /// Authority that can set the metadata address
    pub authority: OptionalNonZeroPubkey,
    /// Account address that holds the metadata
    pub metadata_address: OptionalNonZeroPubkey,
}

impl Extension for MetadataPointer {
    const TYPE: ExtensionType = ExtensionType::MetadataPointer;
}
