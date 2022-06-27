use scrypto::prelude::*;

blueprint! {
    struct EventService {
        payment_vault: Vault,
        ticket_vault: Vault,
        ticket_fee: Decimal,
        event_end: u64,
        event_admin: ResourceAddress,
        service_auth: Vault,
    }

    impl EventService {
        pub fn create(
            event_name: String,
            event_symbol: String,
            ticket_fee: Decimal,
            num_of_tickets: Decimal,
            event_duration: u64,
        ) -> (ComponentAddress, Bucket) {
            // Create a service badge that would have rights with ticket tokens
            // Tickets are only withdrawn when authorized by service_auth badge
            let service_auth = ResourceBuilder::new_fungible().initial_supply(1);

            // Used for the owner of this component or event
            // Able to withdraw earnings by presenting this badge
            let event_admin = ResourceBuilder::new_fungible().initial_supply(1);

            // The actual ticket given to users where they are not allowed to withdraw
            // to anyone
            let ticket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", event_name)
                .metadata("symbol", event_symbol)
                .burnable(
                    rule!(require(service_auth.resource_address())),
                    Mutability::LOCKED,
                )
                .initial_supply(num_of_tickets);

            // AccessRules are used to implicitly give access to methods
            // In this case, we are saying only event admin token holders are allowed to call
            // "claim_event_earnings"
            let access_check = AccessRules::new()
                .method(
                    "claim_event_earnings",
                    rule!(require(event_admin.resource_address())),
                )
                .default(AccessRule::AllowAll);

            let component = Self {
                payment_vault: Vault::new(RADIX_TOKEN),
                ticket_vault: Vault::with_bucket(ticket),
                ticket_fee,
                event_admin: event_admin.resource_address(),
                event_end: Runtime::current_epoch() + event_duration,
                service_auth: Vault::with_bucket(service_auth),
            }
            .instantiate()
            .add_access_check(access_check)
            .globalize();

            (component, event_admin)
        }

        // Claims all the XRD in vault only to the event admin user
        pub fn claim_event_earnings(&mut self) -> Bucket {
            self.payment_vault.take_all()
        }

        // Pays for a ticket, puts the payment in a vault. Ticket is then taken from the vault by
        // using the service auth badge since its the only allowed to withdraw from the vault
        // return remaining XRD (if any) and the ticket
        pub fn pay_for_ticket(&mut self, mut payment: Bucket) -> (Bucket, Bucket) {
            assert!(
                payment.resource_address() == RADIX_TOKEN.into(),
                "XRD Payment Required"
            );
            assert!(payment.amount() >= self.ticket_fee, "Insufficient Balance");

            let fee = payment.take(self.ticket_fee);

            self.payment_vault.put(fee);

            (payment, self.ticket_vault.take(1))
        }

        // Verifies the ticket to check if the event has expired.
        // burns ticket if the event has expired
        // returns the ticket if event has not expired
        pub fn verify_ticket(&self, ticket: Bucket) -> Option<Bucket> {
            if Runtime::current_epoch() > self.event_end {
                self.service_auth.authorize(|| {
                    ticket.burn();
                });
                None
            } else {
                Some(ticket)
            }
        }
    }
}
