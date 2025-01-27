#![feature(min_specialization)]
#[cfg(feature = "psp1155")]
#[brush::contract]
mod psp1155_metadata {
    use contracts::psp1155::extensions::metadata::*;
    use ink_lang as ink;

    #[derive(Default, PSP1155Storage, PSP1155MetadataStorage)]
    #[ink(storage)]
    pub struct PSP1155Struct {
        #[PSP1155StorageField]
        psp1155: PSP1155Data,
        #[PSP1155MetadataStorageField]
        metadata: PSP1155MetadataData,
    }

    impl PSP1155 for PSP1155Struct {}

    impl PSP1155Metadata for PSP1155Struct {}

    impl PSP1155Struct {
        #[ink(constructor)]
        pub fn new(uri: Option<String>) -> Self {
            let mut instance = Self::default();
            instance.metadata.uri = uri;
            instance
        }
    }

    #[ink::test]
    fn metadata_works() {
        let nft = PSP1155Struct::new(Some(String::from("https://www.supercolony.net/")));

        assert_eq!(nft.uri([0; 32]), Some(String::from("https://www.supercolony.net/")));
    }
}
