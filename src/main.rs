fn main() {
    const NEAR_NOMINATION: u128 = 1000000000000000000000000;
    const MIN_FRACTION: u128 = 1000000000000;
    const MIN_BALANCE: u128 = NEAR_NOMINATION * 5;
    const COMMISSION_PERCENT: u128 = 5;
    println!("{}", u128::max_value());
    // println!("{}", f64::MAX);

    #[derive(Default)]
    pub struct BananaSwap {
        pub berries_db: u128,
        pub contract_account_balance: u128,
    }

    impl BananaSwap {
        pub fn with_commission(&self, price: u128) -> u128 {
            let hundred: f64 = 100.0;
            let data: f64 = price as f64 / hundred * (hundred + COMMISSION_PERCENT as f64);
            data as u128
        }

        pub fn get_buy_price(&self, berries: u128) -> u128 {
            return self.get_buy_price_internal(berries, self.contract_account_balance);
        }

        fn get_buy_price_internal(&self, mut berries: u128, near_balance: u128) -> u128 {
            assert!(self.berries_db <= 340282366920938400000000000000000000000, "berries must be less than this" );
            println!("my berries{}",self.berries_db as f64);
            let internal_berries_float: f64 = self.berries_db as f64 / MIN_FRACTION as f64;
            let internal_berries = internal_berries_float as u128;
            println!("Internal berries{:?}",internal_berries );
            let berries_float = berries as f64/ MIN_FRACTION as f64;
            berries = berries_float as u128;
            println!("Berries{:?}", berries);
            assert!(berries > 0, "cannot exchange less than {:?} berries", MIN_FRACTION);
            assert!(berries < internal_berries, "not enough berries in pool");
            assert!(near_balance >= MIN_BALANCE + MIN_FRACTION  , "near balance should be greater than {:?} balance", MIN_BALANCE + MIN_FRACTION);
            let resulting_berries = internal_berries - berries;
            println!("resulting berries{}",resulting_berries);
            let current_near_amount = (near_balance - MIN_BALANCE) as f64 / MIN_FRACTION as f64;
            println!("current near amount{:?}", current_near_amount);
            let new_near_ammount = internal_berries as f64 * current_near_amount as f64 / resulting_berries as f64;
            println!("internal berries {} * current near ammount {} = {}",internal_berries as f64, current_near_amount as f64 , internal_berries as f64 * current_near_amount as f64 );
            println!("new near amount {:?}", new_near_ammount);
            let near_price = new_near_ammount - current_near_amount;
            assert!(near_price > 1.0, "near price must be greater than zero");
            println!("near price {:?}", near_price);
            let min_fraction_float = MIN_FRACTION as f64;
            return self.with_commission((near_price * min_fraction_float) as u128) ;
        }

        pub fn buy(&mut self, berries: u128, attached_deposit_r: u128) {
            let nearprice = self.get_buy_price_internal(berries, self.contract_account_balance - attached_deposit_r);
            self.berries_db = self.berries_db - berries;

            // transfer the left amount [attached_deposit_r - nearprice] to predecessor


            // tokens_bought are berries
            // ext_fungible_token::ft_transfer(
            //     env::predecessor_account_id().try_into().unwrap(),
            //     U128(tokens_bought),
            //     None,
            //     &self.token_account_id,
            //     NO_DEPOSIT,
            //     env::prepaid_gas() - GAS_FOR_SWAP,
            // );
        }

        pub fn get_sell_price(&self, near_amount: u128) -> u128  {
            let current_near_amount = (self.contract_account_balance - MIN_BALANCE) as f64 / MIN_FRACTION as f64;
            let current_berries = self.berries_db as f64 / MIN_FRACTION as f64;
            let near_amount_float = near_amount as f64 / MIN_FRACTION as f64;
            let near_amount_int = near_amount_float as u128;
            let current_near_amount_int = current_near_amount as u128;
            assert!(near_amount_int > 0, "cannot exchange less than {} yoctoNear", MIN_FRACTION);
            assert!(near_amount_int < current_near_amount_int, "not enough near in pool");
            let new_near = current_near_amount - near_amount_float;
            let new_berries = current_berries *  current_near_amount / new_near;
            let berries_price = new_berries - current_berries;
            let min_fraction_float = MIN_FRACTION as f64;
            return self.with_commission((berries_price * min_fraction_float) as u128) ;
        }
    }

    let mut contract = BananaSwap::default();
    contract.berries_db = 7000000000001000000000000;
    contract.contract_account_balance  = 5000000000001000000000000;
    let commission = contract.with_commission(50);
    let price = contract.get_buy_price(5000000000001000000000000);
    println!("{:?}", price);
    //    let commission = with_commission(50);
    //     println!("{:?}",commission);
    //     let price = get_buy_price(100, 200);
    //     println!("{:?}", price);
}
