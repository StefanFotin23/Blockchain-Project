#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait Project {
    #[init]
    fn init(&self) {
        // Initial values
        let token_id = ManagedBuffer::from("EVENT-TICKET");
        self.nft_token_id().set(token_id);
        let registration_fee: BigUint = BigUint::from(100u32);
        let vip_registration_fee: BigUint = BigUint::from(300u32);
        let normal_ticket_number: BigUint = BigUint::from(20u32);
        let vip_ticket_number: BigUint = BigUint::from(10u32);
        let resale_ticket_fee: BigUint = BigUint::from(10u32);

        self.registration_fee().set(registration_fee);
        self.registration_fee_vip().set(vip_registration_fee);
        self.normal_ticket_number().set(normal_ticket_number);
        self.vip_ticket_number().set(vip_ticket_number);
        self.resale_ticket_fee().set(resale_ticket_fee);
        self.resale_normal_ticket_number().set(BigUint::from(0u32));
        self.resale_vip_ticket_number().set(BigUint::from(0u32));
    }

    // NFT Collection Token Id
    #[storage_mapper("nft_token_id")]
    fn nft_token_id(&self) -> SingleValueMapper<ManagedBuffer>;

    // Registration Fees
    #[view(getRegistrationFee)]
    #[storage_mapper("registration_fee")]
    fn registration_fee(&self) -> SingleValueMapper<BigUint>;

    #[view(getRegistrationFeeVip)]
    #[storage_mapper("registration_fee_vip")]
    fn registration_fee_vip(&self) -> SingleValueMapper<BigUint>;

    // Ticket numbers
    #[view(getNormalTicketNumber)]
    #[storage_mapper("normal_ticket_number")]
    fn normal_ticket_number(&self) -> SingleValueMapper<BigUint>;

    #[view(getVipTicketNumber)]
    #[storage_mapper("vip_ticket_number")]
    fn vip_ticket_number(&self) -> SingleValueMapper<BigUint>;

    // Resale Ticket Fee
    #[view(resaleTicketFee)]
    #[storage_mapper("resale_ticket_fee")]
    fn resale_ticket_fee(&self) -> SingleValueMapper<BigUint>;

    // Resale Ticket Numbers
    #[view(getResaleNormalTicketNumber)]
    #[storage_mapper("resale_normal_ticket_number")]
    fn resale_normal_ticket_number(&self) -> SingleValueMapper<BigUint>;

    #[view(getResaleVipTicketNumber)]
    #[storage_mapper("resale_vip_ticket_number")]
    fn resale_vip_ticket_number(&self) -> SingleValueMapper<BigUint>;

    // Resale Ticket Price List

    // Participants
    #[view(getParticipants)]
    #[storage_mapper("participants")]
    fn participants(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getVipParticipants)]
    #[storage_mapper("vip_participants")]
    fn vip_participants(&self) -> UnorderedSetMapper<ManagedAddress>;

    // User Endpoints
    #[endpoint]
    fn see_normal_ticket_price(&self) -> BigUint {
        self.registration_fee().get()
    }

    #[endpoint]
    fn see_vip_ticket_price(&self) -> BigUint {
        self.registration_fee_vip().get()
    }

    #[endpoint]
    fn see_vip_ticket_number(&self) -> BigUint {
        self.registration_fee_vip().get()
    }

    // User Endpoints
    #[endpoint]
    #[payable("EGLD")]
    fn register(&self) -> u64{
        let caller = self.blockchain().get_caller();
        let payment_amount = self.call_value().egld().clone_value();

        // Define VIP and normal registration fees
        let normal_fee = self.registration_fee().get();
        let vip_fee = self.registration_fee_vip().get();

        // Check if the payment amount is either the normal fee or VIP fee
        require!(
            payment_amount == normal_fee || payment_amount == vip_fee,
            "Registration fee is incorrect; please pay either the normal or VIP fee."
        );

        // Determine the participant type and process the payment
        if payment_amount.eq(&normal_fee) {
            // Send the payment to the contract owner
            let owner = self.blockchain().get_owner_address();
            self.send().direct_egld(&owner, &payment_amount);
            self.participants().insert(caller.clone());
        } else if payment_amount.eq(&vip_fee) {
            // Send the payment to the contract owner
            let owner = self.blockchain().get_owner_address();
            self.send().direct_egld(&owner, &payment_amount);
            self.vip_participants().insert(caller.clone());
        }

        // Mint NFT for the participant
        let token_name = ManagedBuffer::from("Event Ticket");
        let description = if payment_amount == normal_fee {
            ManagedBuffer::from("Normal Ticket")
        } else {
            ManagedBuffer::from("VIP Ticket")
        };
        let uri = ManagedBuffer::from("https://th.bing.com/th/id/OIP.xL8YC9fXHMksjUmBIAGrIwHaGz?rs=1&pid=ImgDetMain");

        // // Ensure the NFT token ID is set before minting
        require!(
            !self.nft_token_id().is_empty(),
            "NFT token ID is not set. Please set it first."
        );

        let token_id = TokenIdentifier::from_esdt_bytes(self.nft_token_id().get());
        let amount = BigUint::from(1u32);
        let attributes = ManagedBuffer::new();
        let nonce = self.send().esdt_nft_create_compact( &token_id, &amount, &(&attributes, ),);

        nonce
    }

    #[upgrade]
    fn upgrade(&self) {}
}
