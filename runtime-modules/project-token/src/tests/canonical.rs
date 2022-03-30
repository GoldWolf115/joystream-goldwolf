#![cfg(test)]

use frame_support::{assert_noop, assert_ok, StorageDoubleMap, StorageMap};
use sp_arithmetic::traits::One;

use crate::tests::mock::*;
use crate::traits::{MultiCurrencyBase, ReservableMultiCurrency};
use crate::{last_event_eq, Error, RawEvent};

// base_issue test
#[test]
fn issue_base_token_ok_with_default_issuance_parameters() {
    let config = GenesisConfigBuilder::new().build();
    let issuance_params = IssuanceParams::default();

    build_test_externalities(config).execute_with(|| {
        let token_id = Token::next_token_id();
        assert_ok!(
            <Token as MultiCurrencyBase<AccountId, IssuanceParams>>::issue_token(
                issuance_params.clone(),
            )
        );
        assert_eq!(
            issuance_params.try_build::<Test>(),
            Token::ensure_token_exists(token_id)
        );
        assert_eq!(token_id + 1, Token::next_token_id());
    })
}

// #[test]
// fn issue_base_token_fails_with_existential_deposit_exceeding_issuance() {
//     let config = GenesisConfigBuilder::new().build();
//     let initial_issuance = Balance::from(10u32);
//     let issuance_params = TokenIssuanceParametersOf::<Test> {
//         initial_issuance,
//         existential_deposit: initial_issuance.saturating_add(One::one()),
//         ..Default::default()
//     };

//     build_test_externalities(config).execute_with(|| {
//         assert_noop!(
//             <Token as MultiCurrencyBase<AccountId, IssuanceParams>>::issue_token(
//                 issuance_params.clone()
//             ),
//             Error::<Test>::ExistentialDepositExceedsInitialIssuance,
//         );
//     })
// }

// base_deissue tests
#[test]
fn base_deissue_token_fails_with_non_existing_token_id() {
    let config = GenesisConfigBuilder::new().build();
    build_test_externalities(config).execute_with(|| {
        let token_id = Token::next_token_id();
        assert_noop!(
            <Token as MultiCurrencyBase<AccountId, IssuanceParams>>::deissue_token(token_id),
            Error::<Test>::TokenDoesNotExist,
        );
    })
}

#[test]
fn base_deissue_token_ok() {
    let config = GenesisConfigBuilder::new()
        .add_token_and_account_info()
        .build();
    build_test_externalities(config).execute_with(|| {
        assert_ok!(
            <Token as MultiCurrencyBase<AccountId, IssuanceParams>>::deissue_token(One::one(),)
        );
        assert_noop!(
            Token::ensure_token_exists(One::one()),
            Error::<Test>::TokenDoesNotExist
        );
    })
}

// balanceof tests
#[test]
fn balanceof_fails_with_non_existing_token_id() {
    let config = GenesisConfigBuilder::new().build();
    build_test_externalities(config).execute_with(|| {
        assert_noop!(
            <Token as MultiCurrencyBase<AccountId, IssuanceParams>>::balance(
                One::one(),
                One::one()
            ),
            Error::<Test>::AccountInformationDoesNotExist,
        );
    })
}

// deposit creating tests
#[test]
fn deposit_creating_fails_with_non_existing_token() {
    let config = GenesisConfigBuilder::new().build();
    let token_id = TokenId::one();
    let account_id = AccountId::one();
    let amount = Balance::one();
    build_test_externalities(config).execute_with(|| {
        assert_noop!(
            <Token as MultiCurrencyBase<AccountId, IssuanceParams>>::deposit_creating(
                token_id, account_id, amount
            ),
            Error::<Test>::TokenDoesNotExist,
        );
    })
}

#[test]
fn deposit_creating_ok_with_non_existing_account() {
    let config = GenesisConfigBuilder::new()
        .add_token_and_account_info()
        .build();
    let token_id = TokenId::one();
    let account_id = AccountId::from(DEFAULT_ACCOUNT_ID + 1);
    let amount = Balance::one();
    build_test_externalities(config).execute_with(|| {
        let issuance_pre = Token::token_info_by_id(token_id).current_total_issuance;
        assert_ok!(
            <Token as MultiCurrencyBase<AccountId, IssuanceParams>>::deposit_creating(
                token_id, account_id, amount
            )
        );
        assert_eq!(
            amount,
            Token::account_info_by_token_and_account(token_id, account_id).free_balance,
        );

        let issuance_post = Token::token_info_by_id(token_id).current_total_issuance;
        assert_eq!(issuance_pre.saturating_add(amount), issuance_post);

        last_event_eq!(RawEvent::TokenAmountDepositedInto(
            token_id, account_id, amount
        ));
    })
}

#[test]
fn deposit_creating_ok_with_existing_account() {
    let config = GenesisConfigBuilder::new()
        .add_token_and_account_info()
        .build();
    let token_id = TokenId::one();
    let account_id = AccountId::one();
    let amount = Balance::one();

    build_test_externalities(config).execute_with(|| {
        let issuance_pre = Token::token_info_by_id(token_id).current_total_issuance;
        let free_balance_pre =
            Token::account_info_by_token_and_account(token_id, account_id).free_balance;
        assert_ok!(
            <Token as MultiCurrencyBase<AccountId, IssuanceParams>>::deposit_creating(
                token_id, account_id, amount
            )
        );

        let issuance_post = Token::token_info_by_id(token_id).current_total_issuance;
        let free_balance_post =
            Token::account_info_by_token_and_account(token_id, account_id).free_balance;

        assert_eq!(free_balance_pre.saturating_add(amount), free_balance_post);
        assert_eq!(issuance_pre.saturating_add(amount), issuance_post);
        last_event_eq!(RawEvent::TokenAmountDepositedInto(
            token_id, account_id, amount
        ));
    })
}

#[test]
fn deposit_into_existing_ok() {
    let config = GenesisConfigBuilder::new()
        .add_token_and_account_info()
        .build();
    let token_id = TokenId::one();
    let account_id = AccountId::from(DEFAULT_ACCOUNT_ID);
    let amount = Balance::one();

    build_test_externalities(config).execute_with(|| {
        let issuance_pre = Token::token_info_by_id(token_id).current_total_issuance;
        let free_balance_pre =
            Token::account_info_by_token_and_account(token_id, account_id).free_balance;
        assert_ok!(
            <Token as MultiCurrencyBase<AccountId, IssuanceParams>>::deposit_into_existing(
                token_id, account_id, amount
            )
        );

        let issuance_post = Token::token_info_by_id(token_id).current_total_issuance;
        let free_balance_post =
            Token::account_info_by_token_and_account(token_id, account_id).free_balance;

        assert_eq!(free_balance_pre.saturating_add(amount), free_balance_post);

        assert_eq!(issuance_pre.saturating_add(amount), issuance_post);
        last_event_eq!(RawEvent::TokenAmountDepositedInto(
            token_id, account_id, amount
        ));
    })
}

#[test]
fn deposit_fails_with_nonexisting_account() {
    let config = GenesisConfigBuilder::new()
        .add_token_and_account_info()
        .build();
    let token_id = TokenId::one();
    let account_id = AccountId::from(DEFAULT_ACCOUNT_ID + 1);
    let amount = Balance::one();

    build_test_externalities(config).execute_with(|| {
        assert_noop!(
            <Token as MultiCurrencyBase<AccountId, IssuanceParams>>::deposit_into_existing(
                token_id, account_id, amount
            ),
            Error::<Test>::AccountInformationDoesNotExist,
        );
    })
}

// reserve tests
#[test]
fn reserve_fails_with_non_existing_token() {
    let config = GenesisConfigBuilder::new()
        .add_token_and_account_info()
        .build();
    let account_id = AccountId::from(DEFAULT_ACCOUNT_ID + 1);
    let amount = Balance::one();

    build_test_externalities(config).execute_with(|| {
        let token_id = Token::next_token_id();
        assert_noop!(
            <Token as ReservableMultiCurrency<AccountId>>::reserve(token_id, account_id, amount),
            Error::<Test>::TokenDoesNotExist,
        );
    })
}

#[test]
fn reserve_fails_with_non_existing_account() {
    let config = GenesisConfigBuilder::new()
        .add_token_and_account_info()
        .build();
    let token_id = TokenId::one();
    let account_id = AccountId::from(DEFAULT_ACCOUNT_ID + 1);
    let amount = Balance::one();

    build_test_externalities(config).execute_with(|| {
        assert_noop!(
            <Token as ReservableMultiCurrency<AccountId>>::reserve(token_id, account_id, amount),
            Error::<Test>::AccountInformationDoesNotExist,
        );
    })
}

#[test]
fn reserve_fails_with_insufficient_free_balance() {
    let config = GenesisConfigBuilder::new()
        .add_token_and_account_info()
        .build();
    let token_id = TokenId::one();
    let account_id = AccountId::from(DEFAULT_ACCOUNT_ID);
    let amount = Balance::from(DEFAULT_FREE_BALANCE + 1);

    build_test_externalities(config).execute_with(|| {
        assert_noop!(
            <Token as ReservableMultiCurrency<AccountId>>::reserve(token_id, account_id, amount),
            Error::<Test>::InsufficientFreeBalanceForReserving,
        );
    })
}

#[test]
fn reserve_ok_with_remaining_free_balance_above_ex_deposit() {
    let config = GenesisConfigBuilder::new()
        .add_token_and_account_info()
        .build();
    let token_id = TokenId::one();
    let account_id = AccountId::from(DEFAULT_ACCOUNT_ID);
    let amount = Balance::one();

    build_test_externalities(config).execute_with(|| {
        let issuance_pre = Token::token_info_by_id(token_id).current_total_issuance;
        let account_data_pre = Token::account_info_by_token_and_account(token_id, account_id);
        assert_ok!(<Token as ReservableMultiCurrency<AccountId>>::reserve(
            token_id, account_id, amount
        ));
        let issuance_post = Token::token_info_by_id(token_id).current_total_issuance;
        let account_data_post = Token::account_info_by_token_and_account(token_id, account_id);
        assert_eq!(issuance_pre, issuance_post);

        assert_eq!(
            account_data_pre.free_balance.saturating_sub(amount),
            account_data_post.free_balance,
        );
        assert_eq!(
            account_data_pre.reserved_balance.saturating_add(amount),
            account_data_post.reserved_balance,
        );
        last_event_eq!(RawEvent::TokenAmountReservedFrom(
            token_id, account_id, amount
        ));
    })
}

#[test]
fn reserve_ok_with_remaining_free_balance_below_ex_deposit() {
    let config = GenesisConfigBuilder::new()
        .add_token_and_account_info()
        .build();
    let token_id = TokenId::one();
    let account_id = AccountId::from(DEFAULT_ACCOUNT_ID);
    let amount = Balance::from(DEFAULT_FREE_BALANCE).saturating_sub(One::one());

    build_test_externalities(config).execute_with(|| {
        let issuance_pre = Token::token_info_by_id(token_id).current_total_issuance;
        let account_data_pre = Token::account_info_by_token_and_account(token_id, account_id);
        assert_ok!(<Token as ReservableMultiCurrency<AccountId>>::reserve(
            token_id, account_id, amount
        ));
        let issuance_post = Token::token_info_by_id(token_id).current_total_issuance;
        let account_data_post = Token::account_info_by_token_and_account(token_id, account_id);
        assert_eq!(issuance_pre, issuance_post);

        assert_eq!(
            account_data_post.free_balance.saturating_add(amount),
            account_data_pre.free_balance,
        );
        assert_eq!(
            account_data_post.reserved_balance.saturating_sub(amount),
            account_data_pre.reserved_balance,
        );
        last_event_eq!(RawEvent::TokenAmountReservedFrom(
            token_id, account_id, amount
        ));
    })
}

#[test]
fn reserve_ok_with_remaining_zero_free_balance() {
    let config = GenesisConfigBuilder::new()
        .add_token_and_account_info()
        .build();
    let token_id = TokenId::one();
    let account_id = AccountId::from(DEFAULT_ACCOUNT_ID);
    let amount = Balance::from(DEFAULT_FREE_BALANCE);

    build_test_externalities(config).execute_with(|| {
        let issuance_pre = Token::token_info_by_id(token_id).current_total_issuance;
        let account_data_pre = Token::account_info_by_token_and_account(token_id, account_id);
        assert_ok!(<Token as ReservableMultiCurrency<AccountId>>::reserve(
            token_id, account_id, amount
        ));
        let issuance_post = Token::token_info_by_id(token_id).current_total_issuance;
        let account_data_post = Token::account_info_by_token_and_account(token_id, account_id);
        assert_eq!(issuance_pre, issuance_post);

        assert_eq!(
            account_data_post.free_balance.saturating_add(amount),
            account_data_pre.free_balance,
        );
        assert_eq!(
            account_data_post.reserved_balance.saturating_sub(amount),
            account_data_pre.reserved_balance,
        );
        last_event_eq!(RawEvent::TokenAmountReservedFrom(
            token_id, account_id, amount
        ));
    })
}

// unreserve tests
#[test]
fn unreserve_fails_with_non_existing_token() {
    let config = GenesisConfigBuilder::new()
        .add_token_and_account_info()
        .build();
    let account_id = AccountId::from(DEFAULT_ACCOUNT_ID + 1);
    let amount = Balance::one();

    build_test_externalities(config).execute_with(|| {
        let token_id = Token::next_token_id();
        assert_noop!(
            <Token as ReservableMultiCurrency<AccountId>>::unreserve(token_id, account_id, amount),
            Error::<Test>::TokenDoesNotExist,
        );
    })
}

#[test]
fn unreserve_fails_with_non_existing_account() {
    let config = GenesisConfigBuilder::new()
        .add_token_and_account_info()
        .build();
    let token_id = TokenId::one();
    let account_id = AccountId::from(DEFAULT_ACCOUNT_ID + 1);
    let amount = Balance::one();

    build_test_externalities(config).execute_with(|| {
        assert_noop!(
            <Token as ReservableMultiCurrency<AccountId>>::unreserve(token_id, account_id, amount),
            Error::<Test>::AccountInformationDoesNotExist,
        );
    })
}

#[test]
fn unreserve_fails_with_insufficient_reserved_balance() {
    let config = GenesisConfigBuilder::new()
        .add_token_and_account_info()
        .build();
    let token_id = TokenId::one();
    let account_id = AccountId::from(DEFAULT_ACCOUNT_ID);
    let amount = Balance::one();

    build_test_externalities(config).execute_with(|| {
        assert_ok!(<Token as ReservableMultiCurrency<AccountId>>::reserve(
            token_id, account_id, amount
        ));
        assert_noop!(
            <Token as ReservableMultiCurrency<AccountId>>::unreserve(
                token_id,
                account_id,
                amount.saturating_add(One::one())
            ),
            Error::<Test>::InsufficientReservedBalance,
        );
    })
}

#[test]
fn unreserve_ok() {
    let config = GenesisConfigBuilder::new()
        .add_token_and_account_info()
        .build();
    let token_id = TokenId::one();
    let account_id = AccountId::from(DEFAULT_ACCOUNT_ID);
    let amount = Balance::one();

    build_test_externalities(config).execute_with(|| {
        assert_ok!(<Token as ReservableMultiCurrency<AccountId>>::reserve(
            token_id, account_id, amount
        ));
        let issuance_pre = Token::token_info_by_id(token_id).current_total_issuance;
        let account_data_pre = Token::account_info_by_token_and_account(token_id, account_id);
        assert_ok!(<Token as ReservableMultiCurrency<AccountId>>::unreserve(
            token_id, account_id, amount
        ));
        let issuance_post = Token::token_info_by_id(token_id).current_total_issuance;
        let account_data_post = Token::account_info_by_token_and_account(token_id, account_id);
        assert_eq!(issuance_pre, issuance_post);

        assert_eq!(
            account_data_post.free_balance.saturating_sub(amount),
            account_data_pre.free_balance,
        );
        assert_eq!(
            account_data_post.reserved_balance.saturating_add(amount),
            account_data_pre.reserved_balance,
        );
        last_event_eq!(RawEvent::TokenAmountUnreservedFrom(
            token_id, account_id, amount
        ));
    })
}

// slash tests
#[test]
fn slash_fails_with_non_existing_token_id() {
    let config = GenesisConfigBuilder::new().build();
    let token_id = TokenId::from(2u64);
    let account_id = AccountId::one();
    let amount = Balance::one();
    build_test_externalities(config).execute_with(|| {
        assert_noop!(
            <Token as MultiCurrencyBase<AccountId, IssuanceParams>>::slash(
                token_id, account_id, amount
            ),
            Error::<Test>::TokenDoesNotExist,
        );
    })
}

#[test]
fn slash_fails_with_non_existing_account() {
    let config = GenesisConfigBuilder::new()
        .add_token_and_account_info()
        .build();
    let token_id = TokenId::one();
    let account_id = AccountId::from(2u32);
    let amount = Balance::one();
    build_test_externalities(config).execute_with(|| {
        assert_noop!(
            <Token as MultiCurrencyBase<AccountId, IssuanceParams>>::slash(
                token_id, account_id, amount
            ),
            Error::<Test>::AccountInformationDoesNotExist,
        );
    })
}

#[test]
fn slash_fails_with_insufficient_free_balance() {
    let config = GenesisConfigBuilder::new()
        .add_token_and_account_info()
        .build();
    let token_id = TokenId::one();
    let account_id = AccountId::one();
    let amount = Balance::from(DEFAULT_FREE_BALANCE).saturating_add(Balance::one());
    build_test_externalities(config).execute_with(|| {
        assert_noop!(
            <Token as MultiCurrencyBase<AccountId, IssuanceParams>>::slash(
                token_id, account_id, amount
            ),
            Error::<Test>::InsufficientFreeBalanceForDecreasing,
        );
    })
}

#[test]
fn slash_ok_without_account_removal() {
    let config = GenesisConfigBuilder::new()
        .add_token_and_account_info()
        .build();
    let token_id = AccountId::one();
    let account_id = AccountId::one();
    let amount = Balance::one();
    build_test_externalities(config).execute_with(|| {
        let free_balance_pre =
            Token::account_info_by_token_and_account(token_id, account_id).free_balance;
        let issuance_pre = Token::token_info_by_id(token_id).current_total_issuance;
        assert_ok!(
            <Token as MultiCurrencyBase<AccountId, IssuanceParams>>::slash(
                token_id, account_id, amount
            )
        );
        let free_balance_post =
            Token::account_info_by_token_and_account(token_id, account_id).free_balance;
        let issuance_post = Token::token_info_by_id(token_id).current_total_issuance;
        assert_eq!(free_balance_pre, free_balance_post.saturating_add(amount));
        assert_eq!(issuance_pre, issuance_post.saturating_add(amount));
        last_event_eq!(RawEvent::TokenAmountSlashedFrom(
            token_id, account_id, amount
        ));
    })
}

#[test]
fn slash_ok_with_account_removal_and_zero_total_balance() {
    let config = GenesisConfigBuilder::new()
        .add_token_and_account_info()
        .build();
    let token_id = TokenId::one();
    let account_id = AccountId::from(DEFAULT_ACCOUNT_ID);
    let amount = Balance::from(DEFAULT_FREE_BALANCE);

    build_test_externalities(config).execute_with(|| {
        let issuance_pre = Token::token_info_by_id(token_id).current_total_issuance;
        let account_data = Token::account_info_by_token_and_account(token_id, account_id);

        assert_ok!(
            <Token as MultiCurrencyBase<AccountId, IssuanceParams>>::slash(
                token_id, account_id, amount
            )
        );
        let issuance_post = Token::token_info_by_id(token_id).current_total_issuance;
        assert!(!<crate::AccountInfoByTokenAndAccount<Test>>::contains_key(
            token_id, account_id
        ));
        let total_reduction = account_data
            .free_balance
            .saturating_add(account_data.reserved_balance);
        assert_eq!(issuance_pre, issuance_post.saturating_add(total_reduction));
        last_event_eq!(RawEvent::TokenAmountSlashedFrom(
            token_id, account_id, amount
        ));
    })
}

#[test]
fn slash_ok_with_account_removal_by_ex_deposit_underflow() {
    let config = GenesisConfigBuilder::new()
        .add_token_and_account_info()
        .build();
    let token_id = TokenId::one();
    let account_id = AccountId::from(DEFAULT_ACCOUNT_ID);
    let amount = Balance::from(DEFAULT_FREE_BALANCE - DEFAULT_EXISTENTIAL_DEPOSIT + 1);

    build_test_externalities(config).execute_with(|| {
        let issuance_pre = Token::token_info_by_id(token_id).current_total_issuance;
        let account_data = Token::account_info_by_token_and_account(token_id, account_id);

        assert_ok!(
            <Token as MultiCurrencyBase<AccountId, IssuanceParams>>::slash(
                token_id, account_id, amount
            )
        );
        let issuance_post = Token::token_info_by_id(token_id).current_total_issuance;
        assert!(!<crate::AccountInfoByTokenAndAccount<Test>>::contains_key(
            token_id, account_id
        ));
        let total_reduction = account_data
            .free_balance
            .saturating_add(account_data.reserved_balance);
        assert_eq!(issuance_pre, issuance_post.saturating_add(total_reduction));
        last_event_eq!(RawEvent::TokenAmountSlashedFrom(
            token_id, account_id, amount
        ));
    })
}

#[test]
fn slash_ok_with_account_and_token_removal() {
    let config = GenesisConfigBuilder::new().build();
    let account_id = AccountId::one();
    let amount = Balance::one();

    build_test_externalities(config).execute_with(|| {
        let token_id = Token::next_token_id();
        assert_ok!(
            <Token as MultiCurrencyBase<AccountId, IssuanceParams>>::issue_token(Default::default())
        );
        assert_ok!(
            <Token as MultiCurrencyBase<AccountId, IssuanceParams>>::deposit_creating(
                token_id, account_id, amount
            )
        );
        assert_ok!(
            <Token as MultiCurrencyBase<AccountId, IssuanceParams>>::slash(
                token_id, account_id, amount
            )
        );
        assert!(!<crate::AccountInfoByTokenAndAccount<Test>>::contains_key(
            token_id, account_id
        ));
        assert!(!<crate::TokenInfoById<Test>>::contains_key(token_id));
        last_event_eq!(RawEvent::TokenAmountSlashedFrom(
            token_id, account_id, amount
        ));
    })
}
