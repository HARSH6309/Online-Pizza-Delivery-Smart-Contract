use ink_lang as ink;

#[ink::contract]
mod pizza_delivery {
    use ink_storage::collections::HashMap as StorageHashMap;

    #[ink(storage)]
    pub struct PizzaDelivery {
        orders: StorageHashMap<AccountId, Order>,
        owner: AccountId,
    }

    #[derive(scale::Encode, scale::Decode, Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Order {
        pizza_type: String,
        address: String,
        amount: Balance,
        status: OrderStatus,
    }

    #[derive(scale::Encode, scale::Decode, Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum OrderStatus {
        Placed,
        Paid,
        Confirmed,
        Delivered,
    }

    impl PizzaDelivery {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            Self {
                orders: StorageHashMap::new(),
                owner: caller,
            }
        }

        #[ink(message)]
        pub fn place_order(&mut self, pizza_type: String, address: String, amount: Balance) {
            let caller = self.env().caller();
            let order = Order {
                pizza_type,
                address,
                amount,
                status: OrderStatus::Placed,
            };
            self.orders.insert(caller, order);
        }

        #[ink(message, payable)]
        pub fn pay_order(&mut self) {
            let caller = self.env().caller();
            let value = self.env().transferred_balance();
            let mut order = self.orders.get_mut(&caller).expect("Order does not exist");
            assert_eq!(order.amount, value, "Incorrect payment amount");
            order.status = OrderStatus::Paid;
        }

        #[ink(message)]
        pub fn confirm_order(&mut self, user: AccountId) {
            let caller = self.env().caller();
            assert_eq!(caller, self.owner, "Only owner can confirm orders");
            let mut order = self.orders.get_mut(&user).expect("Order does not exist");
            order.status = OrderStatus::Confirmed;
        }

        #[ink(message)]
        pub fn confirm_delivery(&mut self, user: AccountId) {
            let caller = self.env().caller();
            assert_eq!(caller, self.owner, "Only owner can confirm delivery");
            let mut order = self.orders.get_mut(&user).expect("Order does not exist");
            order.status = OrderStatus::Delivered;
        }

        #[ink(message)]
        pub fn get_order(&self, user: AccountId) -> Order {
            self.orders.get(&user).expect("Order does not exist").clone()
        }
    }
}
