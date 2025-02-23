use starknet::ContractAddress;

#[derive(Copy, PartialEq, Drop, Serde, Debug)]
struct AgeCrdential {
    age: u32
}


#[starknet::interface]
pub trait IAgeCredDriverWrite<TContractState> {
    fn intend(ref self: TContractState, input: ByteArray);
    fn done(ref self: TContractState, input: ByteArray);
    fn transfer(ref self: TContractState, from: ByteArray, to: ByteArray, value: ByteArray);
    fn bind(ref self: TContractState, input: ByteArray);
}

#[starknet::interface]
pub trait IAgeCredDriverRead<T> {
    fn view(self: @T, input: ByteArray) -> AgeCrdential;

}

#[starknet::contract]
pub mod AgeCredDriverModule {
    use starknet::storage::{Map};
    use super::{IAgeCredDriverRead, IAgeCredDriverWrite, AgeCrdential};
    use starknet::ContractAddress;

    #[storage]
    struct Storage {
        pub credentials: Map<ContractAddress, AgeCrdential>,
    }

    #[abi(embed_v0)]
    impl WriteMethods of IAgeCredDriverWrite<ContractState> {
        fn intend(ref self: ContractState, input: ByteArray) {
            // do nothing
        }
    
        fn done(ref self: ContractState, input: ByteArray) {
            // do nothing
        }
    
        fn transfer(ref self: ContractState, from: ByteArray, to: ByteArray, value: ByteArray) {
            assert(false, 'Not Allowed');
        }

        fn bind(ref self: ContractState, input: ByteArray) {
            // TODO: store the credential
        }
    }

    impl ReadMethods of IAgeCredDriverRead<ContractState> {
         fn view(self: @ContractState, input: ByteArray) -> AgeCrdential {
            // TODO: fix read issue
            // self.credentials.read(input)
            AgeCrdential {
                age: 2
            }
        }
    }

}

