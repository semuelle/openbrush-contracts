"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[660],{3905:function(e,n,t){t.d(n,{Zo:function(){return d},kt:function(){return m}});var a=t(7294);function r(e,n,t){return n in e?Object.defineProperty(e,n,{value:t,enumerable:!0,configurable:!0,writable:!0}):e[n]=t,e}function i(e,n){var t=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);n&&(a=a.filter((function(n){return Object.getOwnPropertyDescriptor(e,n).enumerable}))),t.push.apply(t,a)}return t}function o(e){for(var n=1;n<arguments.length;n++){var t=null!=arguments[n]?arguments[n]:{};n%2?i(Object(t),!0).forEach((function(n){r(e,n,t[n])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(t)):i(Object(t)).forEach((function(n){Object.defineProperty(e,n,Object.getOwnPropertyDescriptor(t,n))}))}return e}function c(e,n){if(null==e)return{};var t,a,r=function(e,n){if(null==e)return{};var t,a,r={},i=Object.keys(e);for(a=0;a<i.length;a++)t=i[a],n.indexOf(t)>=0||(r[t]=e[t]);return r}(e,n);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(a=0;a<i.length;a++)t=i[a],n.indexOf(t)>=0||Object.prototype.propertyIsEnumerable.call(e,t)&&(r[t]=e[t])}return r}var s=a.createContext({}),l=function(e){var n=a.useContext(s),t=n;return e&&(t="function"==typeof e?e(n):o(o({},n),e)),t},d=function(e){var n=l(e.components);return a.createElement(s.Provider,{value:n},e.children)},p={inlineCode:"code",wrapper:function(e){var n=e.children;return a.createElement(a.Fragment,{},n)}},u=a.forwardRef((function(e,n){var t=e.components,r=e.mdxType,i=e.originalType,s=e.parentName,d=c(e,["components","mdxType","originalType","parentName"]),u=l(t),m=r,f=u["".concat(s,".").concat(m)]||u[m]||p[m]||i;return t?a.createElement(f,o(o({ref:n},d),{},{components:t})):a.createElement(f,o({ref:n},d))}));function m(e,n){var t=arguments,r=n&&n.mdxType;if("string"==typeof e||r){var i=t.length,o=new Array(i);o[0]=u;var c={};for(var s in n)hasOwnProperty.call(n,s)&&(c[s]=n[s]);c.originalType=e,c.mdxType="string"==typeof e?e:r,o[1]=c;for(var l=2;l<i;l++)o[l]=t[l];return a.createElement.apply(null,o)}return a.createElement.apply(null,t)}u.displayName="MDXCreateElement"},8797:function(e,n,t){t.r(n),t.d(n,{frontMatter:function(){return c},contentTitle:function(){return s},metadata:function(){return l},toc:function(){return d},default:function(){return u}});var a=t(7462),r=t(3366),i=(t(7294),t(3905)),o=["components"],c={sidebar_position:9,title:"Lending contract"},s=void 0,l={unversionedId:"smart-contracts/example/contract",id:"smart-contracts/example/contract",isDocsHomePage:!1,title:"Lending contract",description:"The main logic of the LendingContract is defined in the impls/lending directory.",source:"@site/docs/smart-contracts/example/contract.md",sourceDirName:"smart-contracts/example",slug:"/smart-contracts/example/contract",permalink:"/smart-contracts/example/contract",editUrl:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/docs/docs/smart-contracts/example/contract.md",tags:[],version:"current",sidebarPosition:9,frontMatter:{sidebar_position:9,title:"Lending contract"},sidebar:"tutorialSidebar",previous:{title:"Errors",permalink:"/smart-contracts/example/errors"},next:{title:"Notes about methods",permalink:"/smart-contracts/example/implementation"}},d=[{value:"Add dependencies",id:"add-dependencies",children:[]},{value:"Define the contract storage",id:"define-the-contract-storage",children:[]},{value:"Implement traits",id:"implement-traits",children:[]},{value:"Define the constructor",id:"define-the-constructor",children:[]}],p={toc:d};function u(e){var n=e.components,t=(0,r.Z)(e,o);return(0,i.kt)("wrapper",(0,a.Z)({},p,t,{components:n,mdxType:"MDXLayout"}),(0,i.kt)("p",null,"The main logic of the ",(0,i.kt)("inlineCode",{parentName:"p"},"LendingContract")," is defined in the ",(0,i.kt)("inlineCode",{parentName:"p"},"impls/lending"),' directory.\nIn this file, we only need to "inherit" it.'),(0,i.kt)("h2",{id:"add-dependencies"},"Add dependencies"),(0,i.kt)("p",null,(0,i.kt)("inlineCode",{parentName:"p"},"LendingContract")," instantiates the ",(0,i.kt)("inlineCode",{parentName:"p"},"SharesContract")," and ",(0,i.kt)("inlineCode",{parentName:"p"},"LoanContract"),", so we\nshould import them as ",(0,i.kt)("inlineCode",{parentName:"p"},"ink-as-dependency"),". Also we want to use the ",(0,i.kt)("inlineCode",{parentName:"p"},"AccessControl"),"\nand ",(0,i.kt)("inlineCode",{parentName:"p"},"Pausable"),' from OpenBrush, so we import them too. We also want to "inherit" the\nimplementation of ',(0,i.kt)("inlineCode",{parentName:"p"},"Lending")," and ",(0,i.kt)("inlineCode",{parentName:"p"},"LendingPermissioned")," traits defined in the ",(0,i.kt)("inlineCode",{parentName:"p"},"lending_project")," crate."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-toml"},'[package]\nname = "lending_contract"\nversion = "1.0.0"\nauthors = ["Supercolony <dominik.krizo@supercolony.net>"]\nedition = "2021"\n\n[dependencies]\nink_primitives = { branch = "master", git = "https://github.com/paritytech/ink", default-features = false }\nink_metadata = { branch = "master", git = "https://github.com/paritytech/ink", default-features = false, features = ["derive"], optional = true }\nink_env = { branch = "master", git = "https://github.com/paritytech/ink", default-features = false }\nink_storage = { branch = "master", git = "https://github.com/paritytech/ink", default-features = false }\nink_lang = { branch = "master", git = "https://github.com/paritytech/ink", default-features = false }\nink_prelude = { branch = "master", git = "https://github.com/paritytech/ink", default-features = false }\n\nscale = { package = "parity-scale-codec", version = "3", default-features = false, features = ["derive"] }\nscale-info = { version = "2", default-features = false, features = ["derive"], optional = true }\n\n# These dependencies\nshares_contract = { path = "../shares", default-features = false, features = ["ink-as-dependency"]  }\nloan_contract = { path = "../loan", default-features = false, features = ["ink-as-dependency"]  }\nlending_project = { path = "../..", default-features = false }\nbrush = { path = "../../..", default-features = false, features = ["psp22", "psp34", "pausable", "access_control"] }\n\n[lib]\nname = "lending_contract"\npath = "lib.rs"\ncrate-type = [\n    "cdylib",\n]\n\n[features]\ndefault = ["std"]\nstd = [\n    "ink_primitives/std",\n    "ink_metadata",\n    "ink_metadata/std",\n    "ink_env/std",\n    "ink_storage/std",\n    "ink_lang/std",\n    "scale/std",\n    "scale-info",\n    "scale-info/std",\n\n    # These dependencies\n    "loan_contract/std",\n    "shares_contract/std",\n    "brush/std",\n]\nink-as-dependency = []\n\n[profile.dev]\noverflow-checks = false\ncodegen-units = 16\n\n[profile.release]\noverflow-checks = false\n')),(0,i.kt)("h2",{id:"define-the-contract-storage"},"Define the contract storage"),(0,i.kt)("p",null,"As described earlier, we want our smart contract to be paused by the Manager account.\nTo do that, we need our contract to be ",(0,i.kt)("inlineCode",{parentName:"p"},"Pausable")," and we need a manager role.\nWe can do this with the ",(0,i.kt)("inlineCode",{parentName:"p"},"AccessControl"),". Also, we want to use the ",(0,i.kt)("inlineCode",{parentName:"p"},"LendingStorage")," we have declared.\nSo we will declare a struct and derive all the needed traits."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"#[ink(storage)]\n#[derive(Default, AccessControlStorage, PausableStorage, LendingStorage)]\npub struct LendingContract {\n    #[AccessControlStorageField]\n    access: AccessControlData,\n    #[PausableStorageField]\n    pause: PausableData,\n    #[LendingStorageField]\n    lending: LendingData,\n}\n")),(0,i.kt)("h2",{id:"implement-traits"},"Implement traits"),(0,i.kt)("p",null,'We need to "inherit" the implementation of ',(0,i.kt)("inlineCode",{parentName:"p"},"AccessControll"),", ",(0,i.kt)("inlineCode",{parentName:"p"},"Pausable"),", ",(0,i.kt)("inlineCode",{parentName:"p"},"Lending"),",\n",(0,i.kt)("inlineCode",{parentName:"p"},"LendingPermissioned")," and ",(0,i.kt)("inlineCode",{parentName:"p"},"LendingPermissionedInternal"),"."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},'impl AccessControl for LendingContract {}\n\nimpl Pausable for LendingContract {}\n\nimpl Lending for LendingContract {}\n\nimpl LendingPermissioned for LendingContract {}\n\nimpl LendingPermissionedInternal for LendingContract {\n    fn _instantiate_shares_contract(&self, contract_name: &str, contract_symbol: &str) -> AccountId {\n        let code_hash = self.lending.shares_contract_code_hash;\n        let (hash, _) =\n            ink_env::random::<ink_env::DefaultEnvironment>(contract_name.as_bytes()).expect("Failed to get salt");\n        let hash = hash.as_ref();\n        let contract = SharesContract::new(Some(String::from(contract_name)), Some(String::from(contract_symbol)))\n            .endowment(10000000000)\n            .code_hash(code_hash)\n            .salt_bytes(&hash[..4])\n            .instantiate()\n            .unwrap();\n        contract.to_account_id()\n    }\n}\n')),(0,i.kt)("p",null,"Now the ",(0,i.kt)("inlineCode",{parentName:"p"},"LendingContract")," has functionality of all that traits."),(0,i.kt)("h2",{id:"define-the-constructor"},"Define the constructor"),(0,i.kt)("p",null,"Finally, we will add a constructor, in which we will initiate the admin of\nthe contract, to whom we will also grant the manager role declared before,\nand we will also instantiate the ",(0,i.kt)("inlineCode",{parentName:"p"},"LoanContract")," here and store its AccountId\nin ",(0,i.kt)("inlineCode",{parentName:"p"},"LendingContract"),"."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},'impl LendingContract {\n    /// constructor with name and symbol\n    #[ink(constructor)]\n    pub fn new(code_hash: Hash, nft_code_hash: Hash) -> Self {\n        let mut instance = Self::default();\n        let caller = instance.env().caller();\n        instance._init_with_admin(caller);\n        instance.grant_role(MANAGER, caller).expect("Can not set manager role");\n        instance.lending.shares_contract_code_hash = code_hash;\n        // instantiate NFT contract and store its account id\n        let nft = LoanContract::new()\n            .endowment(10000000000)\n            .code_hash(nft_code_hash)\n            .salt_bytes(&[0xDE, 0xAD, 0xBE, 0xEF])\n            .instantiate()\n            .unwrap();\n        instance.lending.loan_account = nft.to_account_id();\n        instance\n    }\n}\n')))}u.isMDXComponent=!0}}]);