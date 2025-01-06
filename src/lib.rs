#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait MyNeversea2025 {
    #[init]
    fn init(&self, registration_fee: BigUint, vip_registration_fee: BigUint) {
        self.registration_fee().set(registration_fee);
        self.registration_fee_vip().set(vip_registration_fee);
    }

    // Registration Fees
    #[view(getRegistrationFee)]
    #[storage_mapper("registration_fee")]
    fn registration_fee(&self) -> SingleValueMapper<BigUint>;

    #[view(getRegistrationFeeVip)]
    #[storage_mapper("registration_fee_vip")]
    fn registration_fee_vip(&self) -> SingleValueMapper<BigUint>;

    // Participants
    #[view(getParticipants)]
    #[storage_mapper("participants")]
    fn participants(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getVipParticipants)]
    #[storage_mapper("vip_participants")]
    fn vip_participants(&self) -> UnorderedSetMapper<ManagedAddress>;

    // Owner Endpoints
    #[only_owner]
    #[endpoint]
    fn update_normal_registration_fee(&self, new_registration_fee: BigUint) {
        self.registration_fee().set(new_registration_fee.clone());
    }

    #[only_owner]
    #[endpoint]
    fn update_vip_registration_fee(&self, new_registration_fee: BigUint) {
        self.registration_fee_vip().set(new_registration_fee.clone());
    }

    // User Endpoints
    #[endpoint]
    #[payable("EGLD")]
    fn register(&self) {
        let caller = self.blockchain().get_caller();
        let payment_amount = self.call_value().egld_value().clone_value();

        // Define VIP and normal registration fees
        let normal_fee = self.registration_fee().get();
        let vip_fee = self.registration_fee_vip().get();

        // Check if the payment amount is either the normal fee or VIP fee
        require!(
            payment_amount == normal_fee || payment_amount == vip_fee,
            "Registration fee is incorrect; please pay either the normal or VIP fee."
        );

        if payment_amount.eq(&normal_fee) {
            self.participants().insert(caller);
            let owner = self.blockchain().get_owner_address();
            self.send().direct_egld(&owner, &payment_amount);
        } else if payment_amount.eq(&vip_fee) {
            self.vip_participants().insert(caller);
            let owner = self.blockchain().get_owner_address();
            self.send().direct_egld(&owner, &payment_amount);
        }
    }
}
