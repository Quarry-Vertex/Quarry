pub use qbtc::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod qbtc {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("burn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decimals"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalSupply"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Burn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Burn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("burner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Mint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InsufficientAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20InsufficientAllowance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("allowance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("needed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InsufficientBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20InsufficientBalance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("needed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InvalidApprover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20InvalidApprover",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("approver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InvalidReceiver"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20InvalidReceiver",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InvalidSender"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ERC20InvalidSender"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20InvalidSpender"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20InvalidSpender",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidInitialization"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidInitialization",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotInitializing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotInitializing"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnableInvalidOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnableInvalidOwner",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnableUnauthorizedAccount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnableUnauthorizedAccount",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static QBTC_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x16Wa\x10\xCA\x90\x81a\0\x1C\x829\xF3[`\0\x80\xFD\xFE`\x80`@\x90\x80\x82R`\x04\x91\x826\x10\x15a\0\x17W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x92\x83c\x06\xFD\xDE\x03\x14a\x0C8WP\x82c\x07\x7F\"J\x14a\x07\xC5W\x82c\t^\xA7\xB3\x14a\x07\x1BW\x82c\x18\x16\r\xDD\x14a\x06\xDDW\x82c#\xB8r\xDD\x14a\x05\xECW\x82c1<\xE5g\x14a\x05\xD0W\x82c@\xC1\x0F\x19\x14a\x04\xCBW\x82cB\x96lh\x14a\x03_WP\x81cp\xA0\x821\x14a\x03\x1AW\x81cqP\x18\xA6\x14a\x02\xB0W\x81c\x8D\xA5\xCB[\x14a\x02zW\x81c\x95\xD8\x9BA\x14a\x01wW\x81c\xA9\x05\x9C\xBB\x14a\x01FW\x81c\xDDb\xED>\x14a\0\xFBWPc\xF2\xFD\xE3\x8B\x14a\0\xCCW`\0\x80\xFD[4a\0\xF8W` 6`\x03\x19\x01\x12a\0\xF8Wa\0\xF5a\0\xE8a\r\xD9V[a\0\xF0a\x0F\xDAV[a\x0ExV[\x80\xF3[\x80\xFD[\x90P4a\x01BW\x80`\x03\x196\x01\x12a\x01BW\x80` \x92a\x01\x19a\r\xD9V[a\x01*a\x01$a\r\xEFV[\x91a\x0E?V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x82R\x84R T\x90Q\x90\x81R\xF3[P\x80\xFD[\x90P4a\x01BW\x80`\x03\x196\x01\x12a\x01BW` \x90a\x01pa\x01fa\r\xD9V[`$5\x903a\x0E\xECV[Q`\x01\x81R\xF3[\x824a\0\xF8W\x80`\x03\x196\x01\x12a\0\xF8W\x81Q\x90\x80\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x04\x80Ta\x01\xB8\x81a\x0E\x05V[\x80\x86R\x92` \x92`\x01\x92\x80\x84\x16\x90\x81\x15a\x02KWP`\x01\x14a\x01\xF4W[a\x01\xF0\x87\x89a\x01\xE6\x82\x8A\x03\x83a\rEV[Q\x91\x82\x91\x82a\x0C\xFCV[\x03\x90\xF3[\x81R\x93P\x7FF\xA2\x80>Y\xA4\xDENzLWK\x12C\xF2Yw\xACLw\xD5\xA1\xA4\xA6\t\xB59L\xEB\xB4\xA2\xAA[\x83\x85\x10a\x028WPPPP\x81\x01` \x01a\x01\xE6\x82a\x01\xF0\x86\x80a\x01\xD5V[\x80T\x86\x86\x01\x84\x01R\x93\x82\x01\x93\x81\x01a\x02\x1AV[\x91PPa\x01\xF0\x97\x95P\x86\x93P` \x92Pa\x01\xE6\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01\x92\x94\x86\x80a\x01\xD5V[\x90P4a\x01BW\x81`\x03\x196\x01\x12a\x01BW`\0\x80Q` a\x10u\x839\x81Q\x91RT\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x824a\0\xF8W\x80`\x03\x196\x01\x12a\0\xF8Wa\x02\xC9a\x0F\xDAV[`\0\x80Q` a\x10u\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[\x90P4a\x01BW` 6`\x03\x19\x01\x12a\x01BW` \x91\x81\x90`\x01`\x01`\xA0\x1B\x03a\x03Ba\r\xD9V[\x16\x81R`\0\x80Q` a\x10U\x839\x81Q\x91R\x84R T\x90Q\x90\x81R\xF3[\x83\x824a\x01BW` \x80`\x03\x196\x01\x12a\x04\xC7W\x835\x91a\x03~a\x0F\xDAV[3\x84R`\0\x80Q` a\x10U\x839\x81Q\x91R\x80\x83R\x81\x85 T\x84\x11a\x04\x84W3\x15a\x04mW3\x85R\x80\x83R\x81\x85 T\x95\x84\x87\x10a\x04BWP\x92\x80\x94\x95\x7F\xCC\x16\xF5\xDB\xB4\x872\x80\x81\\\x1E\xE0\x9D\xBD\x06sl\xFF\xCC\x18D\x12\xCFzq\xA0\xFD\xB7]9|\xA5\x943\x88R\x84R\x03\x81\x86 U\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x02\x84\x81T\x03\x90U\x84\x81Q\x85\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x843\x92\xA3Q\x92\x83R3\x92\xA2\x80\xF3[\x82Qc9\x144\xE3`\xE2\x1B\x81R3\x91\x81\x01\x91\x82R` \x82\x01\x88\x90R`@\x82\x01\x86\x90R\x90\x81\x90``\x01\x03\x90\xFD[\x81QcKc~\x8F`\xE1\x1B\x81R\x80\x87\x01\x86\x90R`$\x90\xFD[\x81QbF\x1B\xCD`\xE5\x1B\x81R\x80\x87\x01\x84\x90R`\x1F`$\x82\x01R\x7FInsufficient balance in address\0`D\x82\x01R`d\x90\xFD[\x82\x80\xFD[\x91P4a\x04\xC7W\x80`\x03\x196\x01\x12a\x04\xC7Wa\x04\xE5a\r\xD9V[\x90`$5\x91a\x04\xF2a\x0F\xDAV[`\x01`\x01`\xA0\x1B\x03\x16\x92\x83\x15a\x05\xBBW\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x02\x80T\x91\x84\x83\x01\x80\x93\x11a\x05\xA8WP\x91\x7F\x0Fg\x98\xA5`y:T\xC3\xBC\xFE\x86\xA9<\xDE\x1Es\x08}\x94L\x0E\xA2\x05D\x13}A!9h\x85\x93\x91` \x93U\x84\x86R`\0\x80Q` a\x10U\x839\x81Q\x91R\x83R\x80\x86 \x82\x81T\x01\x90U\x84\x86\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x85\x84Q\x86\x81R\xA3Q\x90\x81R\xA2\x80\xF3[cNH{q`\xE0\x1B\x87R`\x11\x90R`$\x86\xFD[\x84`$\x92Q\x91c\xECD/\x05`\xE0\x1B\x83R\x82\x01R\xFD[\x83\x824a\x01BW\x81`\x03\x196\x01\x12a\x01BW` \x90Q`\x12\x81R\xF3[\x834a\0\xF8W``6`\x03\x19\x01\x12a\0\xF8Wa\x06\x06a\r\xD9V[a\x06\x0Ea\r\xEFV[\x91`D5\x93a\x06\x1C\x83a\x0E?V[3\x83R` R\x85\x82 T\x90`\x01\x82\x01a\x06>W[` \x87a\x01p\x88\x88\x88a\x0E\xECV[\x85\x82\x10a\x06\xB2W`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x06\x9BW3\x15a\x06\x84WP\x91a\x01p\x93\x91\x86\x86` \x98\x97\x95a\x06q\x85a\x0E?V[3\x85R\x8AR\x03\x91 U\x91\x93\x94\x81\x93a\x060V[\x86QcJ\x14\x06\xB1`\xE1\x1B\x81R\x90\x81\x01\x83\x90R`$\x90\xFD[\x86Qc\xE6\x02\xDF\x05`\xE0\x1B\x81R\x90\x81\x01\x83\x90R`$\x90\xFD[\x86Qc}\xC7\xA0\xD9`\xE1\x1B\x81R3\x91\x81\x01\x91\x82R` \x82\x01\x92\x90\x92R`@\x81\x01\x86\x90R\x81\x90``\x01\x03\x90\xFD[\x83\x824a\x01BW\x81`\x03\x196\x01\x12a\x01BW` \x90\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x02T\x90Q\x90\x81R\xF3[\x90\x91P4a\x04\xC7W\x81`\x03\x196\x01\x12a\x04\xC7Wa\x076a\r\xD9V[`$5\x903\x15a\x07\xAEW`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x15a\x07\x97WP\x80\x83` \x95a\x07_3a\x0E?V[\x85\x82R\x87R U\x82Q\x90\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x843\x92\xA3Q`\x01\x81R\xF3[\x83QcJ\x14\x06\xB1`\xE1\x1B\x81R\x90\x81\x01\x85\x90R`$\x90\xFD[\x83Qc\xE6\x02\xDF\x05`\xE0\x1B\x81R\x80\x84\x01\x86\x90R`$\x90\xFD[\x90\x91P4a\x04\xC7W``6`\x03\x19\x01\x12a\x04\xC7Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x805\x82\x81\x11a\x0C4Wa\x07\xF9\x906\x90\x83\x01a\r}V[`$5\x83\x81\x11a\x0C0Wa\x08\x10\x906\x90\x84\x01a\r}V[`D5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x0C,W\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x94\x85T\x94`\xFF\x86\x89\x1C\x16\x15\x94\x82\x87\x16\x96\x87\x15\x80a\x0C%W[`\x01\x80\x99\x14\x90\x81a\x0C\x1BW[\x15\x90\x81a\x0C\x12W[Pa\x0C\x02Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x88\x17\x89U\x86a\x0B\xE3W[Pa\x08\x95a\x10\x13V[a\x08\x9Da\x10\x13V[\x80Q\x83\x81\x11a\x0B\xD0W\x80\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x03\x92a\x08\xD3\x84Ta\x0E\x05V[`\x1F\x81\x11a\x0B^W[P` \x90\x8D`\x1F\x84\x11`\x01\x14a\n\xE1W\x92a\n\xD6W[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x88\x1B\x17\x90U[\x82Q\x91\x82\x11a\n\xC3WP\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x04\x91a\t<\x83Ta\x0E\x05V[`\x1F\x81\x11a\n^W[P` \x90`\x1F\x83\x11`\x01\x14a\t\xDBWa\t\x87\x94\x93\x92\x91\x8A\x91\x83a\t\xD0W[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x86\x1B\x17\x90U[a\t\x7Fa\x10\x13V[a\0\xF0a\x10\x13V[a\t\x8FW\x83\x80\xF3[\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x92` \x92h\xFF\0\0\0\0\0\0\0\0\x19\x81T\x16\x90UQ\x90\x81R\xA18\x80\x80\x83\x80\xF3[\x01Q\x90P8\x80a\tcV[\x83\x8AR\x93\x92\x91\x86\x91\x7FF\xA2\x80>Y\xA4\xDENzLWK\x12C\xF2Yw\xACLw\xD5\xA1\xA4\xA6\t\xB59L\xEB\xB4\xA2\xAA\x90`\x1F\x19\x83\x16\x8C[\x81\x81\x10a\nFWP\x96\x83a\t\x87\x98\x10a\n-W[PPP\x81\x1B\x01\x90Ua\twV[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80a\n V[\x82\x89\x01Q\x84U\x8A\x95\x90\x93\x01\x92` \x92\x83\x01\x92\x01a\n\x0CV[\x83\x8AR\x7FF\xA2\x80>Y\xA4\xDENzLWK\x12C\xF2Yw\xACLw\xD5\xA1\xA4\xA6\t\xB59L\xEB\xB4\xA2\xAA`\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\n\xB9W[`\x1F\x01`\x05\x1C\x01\x90\x87\x90[\x82\x81\x10a\n\xAEWPPa\tEV[\x8B\x81U\x01\x87\x90a\n\xA0V[\x90\x91P\x81\x90a\n\x95V[cNH{q`\xE0\x1B\x89R`A\x90R`$\x88\xFD[\x01Q\x90P8\x80a\x08\xF2V[\x91\x90\x8B\x94P`\x1F\x19\x84\x16\x86\x84R\x7F*\xE0\x8A\x8E)%?i\xAC]\x97\x9A\x10\x19V\xAB\x8F\x8D\x9D}\xEDc\xFAz\x83\xB1o\xC4vH\xEA\xB0\x93[\x81\x81\x10a\x0BFWP\x84\x11a\x0B-W[PPP\x81\x1B\x01\x90Ua\t\x06V[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80a\x0B V[\x82\x84\x01Q\x85U\x8D\x96\x90\x94\x01\x93` \x93\x84\x01\x93\x01a\x0B\x11V[\x90\x91P\x83\x8DR\x7F*\xE0\x8A\x8E)%?i\xAC]\x97\x9A\x10\x19V\xAB\x8F\x8D\x9D}\xEDc\xFAz\x83\xB1o\xC4vH\xEA\xB0`\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\x0B\xC6W[\x8E\x85\x94\x93\x92`\x1F\x8E\x93\x01`\x05\x1C\x01\x92\x90[\x83\x82\x10a\x0B\xB8WPPPa\x08\xDCV[\x81U\x85\x94P\x8C\x91\x01\x8Fa\x0B\xA9V[\x90\x91P\x81\x90a\x0B\x98V[cNH{q`\xE0\x1B\x8BR`A\x83R`$\x8B\xFD[h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16h\x01\0\0\0\0\0\0\0\x01\x17\x88U8a\x08\x8CV[\x89Qc\xF9.\xE8\xA9`\xE0\x1B\x81R\x83\x90\xFD[\x90P\x158a\x08qV[0;\x15\x91Pa\x08iV[P\x86a\x08]V[\x86\x80\xFD[\x85\x80\xFD[\x84\x80\xFD[\x90P\x834a\0\xF8W\x80`\x03\x196\x01\x12a\0\xF8W\x80\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x03\x80Ta\x0Cx\x81a\x0E\x05V[\x80\x86R\x92` \x92`\x01\x92\x80\x84\x16\x90\x81\x15a\x02KWP`\x01\x14a\x0C\xA5Wa\x01\xF0\x87\x89a\x01\xE6\x82\x8A\x03\x83a\rEV[\x81R\x93P\x7F*\xE0\x8A\x8E)%?i\xAC]\x97\x9A\x10\x19V\xAB\x8F\x8D\x9D}\xEDc\xFAz\x83\xB1o\xC4vH\xEA\xB0[\x83\x85\x10a\x0C\xE9WPPPP\x81\x01` \x01a\x01\xE6\x82a\x01\xF0\x86\x80a\x01\xD5V[\x80T\x86\x86\x01\x84\x01R\x93\x82\x01\x93\x81\x01a\x0C\xCBV[` \x80\x82R\x82Q\x81\x83\x01\x81\x90R\x90\x93\x92`\0[\x82\x81\x10a\r1WPP`@\x92\x93P`\0\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[\x81\x81\x01\x86\x01Q\x84\x82\x01`@\x01R\x85\x01a\r\x0FV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\rgW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x81`\x1F\x82\x01\x12\x15a\r\xD4W\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\rgW`@Q\x92a\r\xB2`\x1F\x84\x01`\x1F\x19\x16` \x01\x85a\rEV[\x82\x84R` \x83\x83\x01\x01\x11a\r\xD4W\x81`\0\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[`\0\x80\xFD[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\r\xD4WV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\r\xD4WV[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x0E5W[` \x83\x10\x14a\x0E\x1FWV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\x0E\x14V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x01` R`@\x90 \x90V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x81\x15a\x0E\xD3W`\0\x80Q` a\x10u\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x84\x17\x90\x91U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`\0\x80\xA3V[`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x90\xFD[\x91`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x92\x83\x15a\x0F\xC1W\x16\x92\x83\x15a\x0F\xA8W`\0\x90\x83\x82R`\0\x80Q` a\x10U\x839\x81Q\x91R\x80` R`@\x83 T\x91\x84\x83\x10a\x0FuWP\x82\x84\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x95\x93`@\x93\x88` \x97R\x86R\x03\x82\x82 U\x86\x81R \x81\x81T\x01\x90U`@Q\x90\x81R\xA3V[`@Qc9\x144\xE3`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x92\x90\x92RP`D\x81\x01\x83\x90R`d\x90\xFD[`@Qc\xECD/\x05`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x90\xFD[`@QcKc~\x8F`\xE1\x1B\x81R`\0`\x04\x82\x01R`$\x90\xFD[`\0\x80Q` a\x10u\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x163\x03a\x0F\xFBWV[`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x90\xFD[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15a\x10BWV[`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x90\xFD\xFER\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\0\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\xA2dipfsX\"\x12 \xEF}\xD6\x1D\x83\xD10^\xCCw\x04Z\xE4A\x9A\x87\xF6\xE6\xA0\xA2\x9D\xF2\xF1J\xFD\x97\xCB3\x03\xB0\xEF\x07dsolcC\0\x08\x17\x003";
    /// The bytecode of the contract.
    pub static QBTC_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@\x90\x80\x82R`\x04\x91\x826\x10\x15a\0\x17W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x92\x83c\x06\xFD\xDE\x03\x14a\x0C8WP\x82c\x07\x7F\"J\x14a\x07\xC5W\x82c\t^\xA7\xB3\x14a\x07\x1BW\x82c\x18\x16\r\xDD\x14a\x06\xDDW\x82c#\xB8r\xDD\x14a\x05\xECW\x82c1<\xE5g\x14a\x05\xD0W\x82c@\xC1\x0F\x19\x14a\x04\xCBW\x82cB\x96lh\x14a\x03_WP\x81cp\xA0\x821\x14a\x03\x1AW\x81cqP\x18\xA6\x14a\x02\xB0W\x81c\x8D\xA5\xCB[\x14a\x02zW\x81c\x95\xD8\x9BA\x14a\x01wW\x81c\xA9\x05\x9C\xBB\x14a\x01FW\x81c\xDDb\xED>\x14a\0\xFBWPc\xF2\xFD\xE3\x8B\x14a\0\xCCW`\0\x80\xFD[4a\0\xF8W` 6`\x03\x19\x01\x12a\0\xF8Wa\0\xF5a\0\xE8a\r\xD9V[a\0\xF0a\x0F\xDAV[a\x0ExV[\x80\xF3[\x80\xFD[\x90P4a\x01BW\x80`\x03\x196\x01\x12a\x01BW\x80` \x92a\x01\x19a\r\xD9V[a\x01*a\x01$a\r\xEFV[\x91a\x0E?V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x82R\x84R T\x90Q\x90\x81R\xF3[P\x80\xFD[\x90P4a\x01BW\x80`\x03\x196\x01\x12a\x01BW` \x90a\x01pa\x01fa\r\xD9V[`$5\x903a\x0E\xECV[Q`\x01\x81R\xF3[\x824a\0\xF8W\x80`\x03\x196\x01\x12a\0\xF8W\x81Q\x90\x80\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x04\x80Ta\x01\xB8\x81a\x0E\x05V[\x80\x86R\x92` \x92`\x01\x92\x80\x84\x16\x90\x81\x15a\x02KWP`\x01\x14a\x01\xF4W[a\x01\xF0\x87\x89a\x01\xE6\x82\x8A\x03\x83a\rEV[Q\x91\x82\x91\x82a\x0C\xFCV[\x03\x90\xF3[\x81R\x93P\x7FF\xA2\x80>Y\xA4\xDENzLWK\x12C\xF2Yw\xACLw\xD5\xA1\xA4\xA6\t\xB59L\xEB\xB4\xA2\xAA[\x83\x85\x10a\x028WPPPP\x81\x01` \x01a\x01\xE6\x82a\x01\xF0\x86\x80a\x01\xD5V[\x80T\x86\x86\x01\x84\x01R\x93\x82\x01\x93\x81\x01a\x02\x1AV[\x91PPa\x01\xF0\x97\x95P\x86\x93P` \x92Pa\x01\xE6\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01\x92\x94\x86\x80a\x01\xD5V[\x90P4a\x01BW\x81`\x03\x196\x01\x12a\x01BW`\0\x80Q` a\x10u\x839\x81Q\x91RT\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x824a\0\xF8W\x80`\x03\x196\x01\x12a\0\xF8Wa\x02\xC9a\x0F\xDAV[`\0\x80Q` a\x10u\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[\x90P4a\x01BW` 6`\x03\x19\x01\x12a\x01BW` \x91\x81\x90`\x01`\x01`\xA0\x1B\x03a\x03Ba\r\xD9V[\x16\x81R`\0\x80Q` a\x10U\x839\x81Q\x91R\x84R T\x90Q\x90\x81R\xF3[\x83\x824a\x01BW` \x80`\x03\x196\x01\x12a\x04\xC7W\x835\x91a\x03~a\x0F\xDAV[3\x84R`\0\x80Q` a\x10U\x839\x81Q\x91R\x80\x83R\x81\x85 T\x84\x11a\x04\x84W3\x15a\x04mW3\x85R\x80\x83R\x81\x85 T\x95\x84\x87\x10a\x04BWP\x92\x80\x94\x95\x7F\xCC\x16\xF5\xDB\xB4\x872\x80\x81\\\x1E\xE0\x9D\xBD\x06sl\xFF\xCC\x18D\x12\xCFzq\xA0\xFD\xB7]9|\xA5\x943\x88R\x84R\x03\x81\x86 U\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x02\x84\x81T\x03\x90U\x84\x81Q\x85\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x843\x92\xA3Q\x92\x83R3\x92\xA2\x80\xF3[\x82Qc9\x144\xE3`\xE2\x1B\x81R3\x91\x81\x01\x91\x82R` \x82\x01\x88\x90R`@\x82\x01\x86\x90R\x90\x81\x90``\x01\x03\x90\xFD[\x81QcKc~\x8F`\xE1\x1B\x81R\x80\x87\x01\x86\x90R`$\x90\xFD[\x81QbF\x1B\xCD`\xE5\x1B\x81R\x80\x87\x01\x84\x90R`\x1F`$\x82\x01R\x7FInsufficient balance in address\0`D\x82\x01R`d\x90\xFD[\x82\x80\xFD[\x91P4a\x04\xC7W\x80`\x03\x196\x01\x12a\x04\xC7Wa\x04\xE5a\r\xD9V[\x90`$5\x91a\x04\xF2a\x0F\xDAV[`\x01`\x01`\xA0\x1B\x03\x16\x92\x83\x15a\x05\xBBW\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x02\x80T\x91\x84\x83\x01\x80\x93\x11a\x05\xA8WP\x91\x7F\x0Fg\x98\xA5`y:T\xC3\xBC\xFE\x86\xA9<\xDE\x1Es\x08}\x94L\x0E\xA2\x05D\x13}A!9h\x85\x93\x91` \x93U\x84\x86R`\0\x80Q` a\x10U\x839\x81Q\x91R\x83R\x80\x86 \x82\x81T\x01\x90U\x84\x86\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x85\x84Q\x86\x81R\xA3Q\x90\x81R\xA2\x80\xF3[cNH{q`\xE0\x1B\x87R`\x11\x90R`$\x86\xFD[\x84`$\x92Q\x91c\xECD/\x05`\xE0\x1B\x83R\x82\x01R\xFD[\x83\x824a\x01BW\x81`\x03\x196\x01\x12a\x01BW` \x90Q`\x12\x81R\xF3[\x834a\0\xF8W``6`\x03\x19\x01\x12a\0\xF8Wa\x06\x06a\r\xD9V[a\x06\x0Ea\r\xEFV[\x91`D5\x93a\x06\x1C\x83a\x0E?V[3\x83R` R\x85\x82 T\x90`\x01\x82\x01a\x06>W[` \x87a\x01p\x88\x88\x88a\x0E\xECV[\x85\x82\x10a\x06\xB2W`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x06\x9BW3\x15a\x06\x84WP\x91a\x01p\x93\x91\x86\x86` \x98\x97\x95a\x06q\x85a\x0E?V[3\x85R\x8AR\x03\x91 U\x91\x93\x94\x81\x93a\x060V[\x86QcJ\x14\x06\xB1`\xE1\x1B\x81R\x90\x81\x01\x83\x90R`$\x90\xFD[\x86Qc\xE6\x02\xDF\x05`\xE0\x1B\x81R\x90\x81\x01\x83\x90R`$\x90\xFD[\x86Qc}\xC7\xA0\xD9`\xE1\x1B\x81R3\x91\x81\x01\x91\x82R` \x82\x01\x92\x90\x92R`@\x81\x01\x86\x90R\x81\x90``\x01\x03\x90\xFD[\x83\x824a\x01BW\x81`\x03\x196\x01\x12a\x01BW` \x90\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x02T\x90Q\x90\x81R\xF3[\x90\x91P4a\x04\xC7W\x81`\x03\x196\x01\x12a\x04\xC7Wa\x076a\r\xD9V[`$5\x903\x15a\x07\xAEW`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x15a\x07\x97WP\x80\x83` \x95a\x07_3a\x0E?V[\x85\x82R\x87R U\x82Q\x90\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x843\x92\xA3Q`\x01\x81R\xF3[\x83QcJ\x14\x06\xB1`\xE1\x1B\x81R\x90\x81\x01\x85\x90R`$\x90\xFD[\x83Qc\xE6\x02\xDF\x05`\xE0\x1B\x81R\x80\x84\x01\x86\x90R`$\x90\xFD[\x90\x91P4a\x04\xC7W``6`\x03\x19\x01\x12a\x04\xC7Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x805\x82\x81\x11a\x0C4Wa\x07\xF9\x906\x90\x83\x01a\r}V[`$5\x83\x81\x11a\x0C0Wa\x08\x10\x906\x90\x84\x01a\r}V[`D5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x0C,W\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x94\x85T\x94`\xFF\x86\x89\x1C\x16\x15\x94\x82\x87\x16\x96\x87\x15\x80a\x0C%W[`\x01\x80\x99\x14\x90\x81a\x0C\x1BW[\x15\x90\x81a\x0C\x12W[Pa\x0C\x02Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x88\x17\x89U\x86a\x0B\xE3W[Pa\x08\x95a\x10\x13V[a\x08\x9Da\x10\x13V[\x80Q\x83\x81\x11a\x0B\xD0W\x80\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x03\x92a\x08\xD3\x84Ta\x0E\x05V[`\x1F\x81\x11a\x0B^W[P` \x90\x8D`\x1F\x84\x11`\x01\x14a\n\xE1W\x92a\n\xD6W[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x88\x1B\x17\x90U[\x82Q\x91\x82\x11a\n\xC3WP\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x04\x91a\t<\x83Ta\x0E\x05V[`\x1F\x81\x11a\n^W[P` \x90`\x1F\x83\x11`\x01\x14a\t\xDBWa\t\x87\x94\x93\x92\x91\x8A\x91\x83a\t\xD0W[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x86\x1B\x17\x90U[a\t\x7Fa\x10\x13V[a\0\xF0a\x10\x13V[a\t\x8FW\x83\x80\xF3[\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x92` \x92h\xFF\0\0\0\0\0\0\0\0\x19\x81T\x16\x90UQ\x90\x81R\xA18\x80\x80\x83\x80\xF3[\x01Q\x90P8\x80a\tcV[\x83\x8AR\x93\x92\x91\x86\x91\x7FF\xA2\x80>Y\xA4\xDENzLWK\x12C\xF2Yw\xACLw\xD5\xA1\xA4\xA6\t\xB59L\xEB\xB4\xA2\xAA\x90`\x1F\x19\x83\x16\x8C[\x81\x81\x10a\nFWP\x96\x83a\t\x87\x98\x10a\n-W[PPP\x81\x1B\x01\x90Ua\twV[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80a\n V[\x82\x89\x01Q\x84U\x8A\x95\x90\x93\x01\x92` \x92\x83\x01\x92\x01a\n\x0CV[\x83\x8AR\x7FF\xA2\x80>Y\xA4\xDENzLWK\x12C\xF2Yw\xACLw\xD5\xA1\xA4\xA6\t\xB59L\xEB\xB4\xA2\xAA`\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\n\xB9W[`\x1F\x01`\x05\x1C\x01\x90\x87\x90[\x82\x81\x10a\n\xAEWPPa\tEV[\x8B\x81U\x01\x87\x90a\n\xA0V[\x90\x91P\x81\x90a\n\x95V[cNH{q`\xE0\x1B\x89R`A\x90R`$\x88\xFD[\x01Q\x90P8\x80a\x08\xF2V[\x91\x90\x8B\x94P`\x1F\x19\x84\x16\x86\x84R\x7F*\xE0\x8A\x8E)%?i\xAC]\x97\x9A\x10\x19V\xAB\x8F\x8D\x9D}\xEDc\xFAz\x83\xB1o\xC4vH\xEA\xB0\x93[\x81\x81\x10a\x0BFWP\x84\x11a\x0B-W[PPP\x81\x1B\x01\x90Ua\t\x06V[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80a\x0B V[\x82\x84\x01Q\x85U\x8D\x96\x90\x94\x01\x93` \x93\x84\x01\x93\x01a\x0B\x11V[\x90\x91P\x83\x8DR\x7F*\xE0\x8A\x8E)%?i\xAC]\x97\x9A\x10\x19V\xAB\x8F\x8D\x9D}\xEDc\xFAz\x83\xB1o\xC4vH\xEA\xB0`\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\x0B\xC6W[\x8E\x85\x94\x93\x92`\x1F\x8E\x93\x01`\x05\x1C\x01\x92\x90[\x83\x82\x10a\x0B\xB8WPPPa\x08\xDCV[\x81U\x85\x94P\x8C\x91\x01\x8Fa\x0B\xA9V[\x90\x91P\x81\x90a\x0B\x98V[cNH{q`\xE0\x1B\x8BR`A\x83R`$\x8B\xFD[h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16h\x01\0\0\0\0\0\0\0\x01\x17\x88U8a\x08\x8CV[\x89Qc\xF9.\xE8\xA9`\xE0\x1B\x81R\x83\x90\xFD[\x90P\x158a\x08qV[0;\x15\x91Pa\x08iV[P\x86a\x08]V[\x86\x80\xFD[\x85\x80\xFD[\x84\x80\xFD[\x90P\x834a\0\xF8W\x80`\x03\x196\x01\x12a\0\xF8W\x80\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x03\x80Ta\x0Cx\x81a\x0E\x05V[\x80\x86R\x92` \x92`\x01\x92\x80\x84\x16\x90\x81\x15a\x02KWP`\x01\x14a\x0C\xA5Wa\x01\xF0\x87\x89a\x01\xE6\x82\x8A\x03\x83a\rEV[\x81R\x93P\x7F*\xE0\x8A\x8E)%?i\xAC]\x97\x9A\x10\x19V\xAB\x8F\x8D\x9D}\xEDc\xFAz\x83\xB1o\xC4vH\xEA\xB0[\x83\x85\x10a\x0C\xE9WPPPP\x81\x01` \x01a\x01\xE6\x82a\x01\xF0\x86\x80a\x01\xD5V[\x80T\x86\x86\x01\x84\x01R\x93\x82\x01\x93\x81\x01a\x0C\xCBV[` \x80\x82R\x82Q\x81\x83\x01\x81\x90R\x90\x93\x92`\0[\x82\x81\x10a\r1WPP`@\x92\x93P`\0\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[\x81\x81\x01\x86\x01Q\x84\x82\x01`@\x01R\x85\x01a\r\x0FV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\rgW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x81`\x1F\x82\x01\x12\x15a\r\xD4W\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\rgW`@Q\x92a\r\xB2`\x1F\x84\x01`\x1F\x19\x16` \x01\x85a\rEV[\x82\x84R` \x83\x83\x01\x01\x11a\r\xD4W\x81`\0\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[`\0\x80\xFD[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\r\xD4WV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\r\xD4WV[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x0E5W[` \x83\x10\x14a\x0E\x1FWV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\x0E\x14V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x01` R`@\x90 \x90V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x81\x15a\x0E\xD3W`\0\x80Q` a\x10u\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x84\x17\x90\x91U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`\0\x80\xA3V[`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x90\xFD[\x91`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x92\x83\x15a\x0F\xC1W\x16\x92\x83\x15a\x0F\xA8W`\0\x90\x83\x82R`\0\x80Q` a\x10U\x839\x81Q\x91R\x80` R`@\x83 T\x91\x84\x83\x10a\x0FuWP\x82\x84\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x95\x93`@\x93\x88` \x97R\x86R\x03\x82\x82 U\x86\x81R \x81\x81T\x01\x90U`@Q\x90\x81R\xA3V[`@Qc9\x144\xE3`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x92\x90\x92RP`D\x81\x01\x83\x90R`d\x90\xFD[`@Qc\xECD/\x05`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x90\xFD[`@QcKc~\x8F`\xE1\x1B\x81R`\0`\x04\x82\x01R`$\x90\xFD[`\0\x80Q` a\x10u\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x163\x03a\x0F\xFBWV[`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x90\xFD[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15a\x10BWV[`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x90\xFD\xFER\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\0\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\xA2dipfsX\"\x12 \xEF}\xD6\x1D\x83\xD10^\xCCw\x04Z\xE4A\x9A\x87\xF6\xE6\xA0\xA2\x9D\xF2\xF1J\xFD\x97\xCB3\x03\xB0\xEF\x07dsolcC\0\x08\x17\x003";
    /// The deployed bytecode of the contract.
    pub static QBTC_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct QBTC<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for QBTC<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for QBTC<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for QBTC<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for QBTC<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(QBTC)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> QBTC<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    QBTC_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                QBTC_ABI.clone(),
                QBTC_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burn` (0x42966c68) function
        pub fn burn(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 150, 108, 104], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x077f224a) function
        pub fn initialize(
            &self,
            name: ::std::string::String,
            symbol: ::std::string::String,
            pool: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 127, 34, 74], (name, symbol, pool))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0x40c10f19) function
        pub fn mint(
            &self,
            account: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 193, 15, 25], (account, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Burn` event
        pub fn burn_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BurnFilter> {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Mint` event
        pub fn mint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MintFilter> {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, QBTCEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for QBTC<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ERC20InsufficientAllowance` with signature `ERC20InsufficientAllowance(address,uint256,uint256)` and selector `0xfb8f41b2`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ERC20InsufficientAllowance",
        abi = "ERC20InsufficientAllowance(address,uint256,uint256)"
    )]
    pub struct ERC20InsufficientAllowance {
        pub spender: ::ethers::core::types::Address,
        pub allowance: ::ethers::core::types::U256,
        pub needed: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC20InsufficientBalance` with signature `ERC20InsufficientBalance(address,uint256,uint256)` and selector `0xe450d38c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ERC20InsufficientBalance",
        abi = "ERC20InsufficientBalance(address,uint256,uint256)"
    )]
    pub struct ERC20InsufficientBalance {
        pub sender: ::ethers::core::types::Address,
        pub balance: ::ethers::core::types::U256,
        pub needed: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC20InvalidApprover` with signature `ERC20InvalidApprover(address)` and selector `0xe602df05`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ERC20InvalidApprover", abi = "ERC20InvalidApprover(address)")]
    pub struct ERC20InvalidApprover {
        pub approver: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC20InvalidReceiver` with signature `ERC20InvalidReceiver(address)` and selector `0xec442f05`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ERC20InvalidReceiver", abi = "ERC20InvalidReceiver(address)")]
    pub struct ERC20InvalidReceiver {
        pub receiver: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC20InvalidSender` with signature `ERC20InvalidSender(address)` and selector `0x96c6fd1e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ERC20InvalidSender", abi = "ERC20InvalidSender(address)")]
    pub struct ERC20InvalidSender {
        pub sender: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC20InvalidSpender` with signature `ERC20InvalidSpender(address)` and selector `0x94280d62`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ERC20InvalidSpender", abi = "ERC20InvalidSpender(address)")]
    pub struct ERC20InvalidSpender {
        pub spender: ::ethers::core::types::Address,
    }
    ///Custom Error type `InvalidInitialization` with signature `InvalidInitialization()` and selector `0xf92ee8a9`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidInitialization", abi = "InvalidInitialization()")]
    pub struct InvalidInitialization;
    ///Custom Error type `NotInitializing` with signature `NotInitializing()` and selector `0xd7e6bcf8`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NotInitializing", abi = "NotInitializing()")]
    pub struct NotInitializing;
    ///Custom Error type `OwnableInvalidOwner` with signature `OwnableInvalidOwner(address)` and selector `0x1e4fbdf7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OwnableInvalidOwner", abi = "OwnableInvalidOwner(address)")]
    pub struct OwnableInvalidOwner {
        pub owner: ::ethers::core::types::Address,
    }
    ///Custom Error type `OwnableUnauthorizedAccount` with signature `OwnableUnauthorizedAccount(address)` and selector `0x118cdaa7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "OwnableUnauthorizedAccount",
        abi = "OwnableUnauthorizedAccount(address)"
    )]
    pub struct OwnableUnauthorizedAccount {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum QBTCErrors {
        ERC20InsufficientAllowance(ERC20InsufficientAllowance),
        ERC20InsufficientBalance(ERC20InsufficientBalance),
        ERC20InvalidApprover(ERC20InvalidApprover),
        ERC20InvalidReceiver(ERC20InvalidReceiver),
        ERC20InvalidSender(ERC20InvalidSender),
        ERC20InvalidSpender(ERC20InvalidSpender),
        InvalidInitialization(InvalidInitialization),
        NotInitializing(NotInitializing),
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for QBTCErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <ERC20InsufficientAllowance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20InsufficientAllowance(decoded));
            }
            if let Ok(decoded) = <ERC20InsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20InsufficientBalance(decoded));
            }
            if let Ok(decoded) = <ERC20InvalidApprover as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20InvalidApprover(decoded));
            }
            if let Ok(decoded) = <ERC20InvalidReceiver as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20InvalidReceiver(decoded));
            }
            if let Ok(decoded) = <ERC20InvalidSender as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20InvalidSender(decoded));
            }
            if let Ok(decoded) = <ERC20InvalidSpender as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC20InvalidSpender(decoded));
            }
            if let Ok(decoded) = <InvalidInitialization as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidInitialization(decoded));
            }
            if let Ok(decoded) = <NotInitializing as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotInitializing(decoded));
            }
            if let Ok(decoded) = <OwnableInvalidOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnableInvalidOwner(decoded));
            }
            if let Ok(decoded) = <OwnableUnauthorizedAccount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnableUnauthorizedAccount(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for QBTCErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ERC20InsufficientAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InvalidApprover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InvalidReceiver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InvalidSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20InvalidSpender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidInitialization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotInitializing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableInvalidOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for QBTCErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ERC20InsufficientAllowance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InsufficientBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InvalidApprover as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InvalidReceiver as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InvalidSender as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC20InvalidSpender as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidInitialization as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotInitializing as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableInvalidOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableUnauthorizedAccount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for QBTCErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ERC20InsufficientAllowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20InsufficientBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20InvalidApprover(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20InvalidReceiver(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20InvalidSender(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20InvalidSpender(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidInitialization(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotInitializing(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnableInvalidOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for QBTCErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ERC20InsufficientAllowance> for QBTCErrors {
        fn from(value: ERC20InsufficientAllowance) -> Self {
            Self::ERC20InsufficientAllowance(value)
        }
    }
    impl ::core::convert::From<ERC20InsufficientBalance> for QBTCErrors {
        fn from(value: ERC20InsufficientBalance) -> Self {
            Self::ERC20InsufficientBalance(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidApprover> for QBTCErrors {
        fn from(value: ERC20InvalidApprover) -> Self {
            Self::ERC20InvalidApprover(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidReceiver> for QBTCErrors {
        fn from(value: ERC20InvalidReceiver) -> Self {
            Self::ERC20InvalidReceiver(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidSender> for QBTCErrors {
        fn from(value: ERC20InvalidSender) -> Self {
            Self::ERC20InvalidSender(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidSpender> for QBTCErrors {
        fn from(value: ERC20InvalidSpender) -> Self {
            Self::ERC20InvalidSpender(value)
        }
    }
    impl ::core::convert::From<InvalidInitialization> for QBTCErrors {
        fn from(value: InvalidInitialization) -> Self {
            Self::InvalidInitialization(value)
        }
    }
    impl ::core::convert::From<NotInitializing> for QBTCErrors {
        fn from(value: NotInitializing) -> Self {
            Self::NotInitializing(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner> for QBTCErrors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount> for QBTCErrors {
        fn from(value: OwnableUnauthorizedAccount) -> Self {
            Self::OwnableUnauthorizedAccount(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Burn", abi = "Burn(address,uint256)")]
    pub struct BurnFilter {
        #[ethevent(indexed)]
        pub burner: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Initialized", abi = "Initialized(uint64)")]
    pub struct InitializedFilter {
        pub version: u64,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Mint", abi = "Mint(address,uint256)")]
    pub struct MintFilter {
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum QBTCEvents {
        ApprovalFilter(ApprovalFilter),
        BurnFilter(BurnFilter),
        InitializedFilter(InitializedFilter),
        MintFilter(MintFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for QBTCEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(QBTCEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = BurnFilter::decode_log(log) {
                return Ok(QBTCEvents::BurnFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(QBTCEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(QBTCEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(QBTCEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(QBTCEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for QBTCEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for QBTCEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<BurnFilter> for QBTCEvents {
        fn from(value: BurnFilter) -> Self {
            Self::BurnFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for QBTCEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<MintFilter> for QBTCEvents {
        fn from(value: MintFilter) -> Self {
            Self::MintFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for QBTCEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for QBTCEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `burn` function with signature `burn(uint256)` and selector `0x42966c68`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "burn", abi = "burn(uint256)")]
    pub struct BurnCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(string,string,address)` and selector `0x077f224a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "initialize", abi = "initialize(string,string,address)")]
    pub struct InitializeCall {
        pub name: ::std::string::String,
        pub symbol: ::std::string::String,
        pub pool: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `mint` function with signature `mint(address,uint256)` and selector `0x40c10f19`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "mint", abi = "mint(address,uint256)")]
    pub struct MintCall {
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum QBTCCalls {
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        Decimals(DecimalsCall),
        Initialize(InitializeCall),
        Mint(MintCall),
        Name(NameCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for QBTCCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <BurnCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Burn(decoded));
            }
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <MintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded) = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for QBTCCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Burn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for QBTCCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllowanceCall> for QBTCCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for QBTCCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for QBTCCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BurnCall> for QBTCCalls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for QBTCCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for QBTCCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<MintCall> for QBTCCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<NameCall> for QBTCCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for QBTCCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for QBTCCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for QBTCCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for QBTCCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for QBTCCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for QBTCCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for QBTCCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ApproveReturn(pub bool);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TransferFromReturn(pub bool);
}
