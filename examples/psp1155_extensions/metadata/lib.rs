#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_psp1155 {
    use brush::contracts::psp1155::extensions::metadata::*;
    use ink_prelude::string::String;

    #[derive(Default, PSP1155Storage, PSP1155MetadataStorage)]
    #[ink(storage)]
    pub struct MyPSP1155 {
        #[PSP1155StorageField]
        psp1155: PSP1155Data,
        #[PSP1155MetadataStorageField]
        metadata: PSP1155MetadataData,
    }

    impl PSP1155 for MyPSP1155 {}

    impl PSP1155Metadata for MyPSP1155 {}

    impl MyPSP1155 {
        /// contract constructor
        #[ink(constructor)]
        pub fn new(uri: Option<String>) -> Self {
            let mut instance = Self::default();
            instance.metadata.uri = uri;
            instance
        }
    }
}
