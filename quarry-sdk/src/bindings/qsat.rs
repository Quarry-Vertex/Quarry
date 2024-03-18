pub use qsat::*;
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
pub mod qsat {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("TOTAL_SUPPLY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("TOTAL_SUPPLY"),
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
                                    name: ::std::borrow::ToOwned::to_owned("bridgeAddress"),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static QSAT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x16Wa\r\x99\x90\x81a\0\x1C\x829\xF3[`\0\x80\xFD\xFE`\x80`@\x81\x81R`\x04\x91\x826\x10\x15a\0\x16W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x91\x82c\x06\xFD\xDE\x03\x14a\t\xB3WP\x81c\x07\x7F\"J\x14a\x04\x8EW\x81c\t^\xA7\xB3\x14a\x03\xE1W\x81c\x18\x16\r\xDD\x14a\x03\xA3W\x81c#\xB8r\xDD\x14a\x02\xB0WP\x80c1<\xE5g\x14a\x02\x95W\x80cp\xA0\x821\x14a\x02?W\x80c\x90-U\xA5\x14a\x02\x1EW\x80c\x95\xD8\x9BA\x14a\x01\x17W\x80c\xA9\x05\x9C\xBB\x14a\0\xE7Wc\xDDb\xED>\x14a\0\x9EW`\0\x80\xFD[4a\0\xE3W\x80`\x03\x196\x01\x12a\0\xE3W\x80` \x92a\0\xBAa\x0B\x83V[a\0\xCBa\0\xC5a\x0B\x99V[\x91a\x0B\xE9V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x82R\x84R T\x90Q\x90\x81R\xF3[P\x80\xFD[P4a\0\xE3W\x80`\x03\x196\x01\x12a\0\xE3W` \x90a\x01\x10a\x01\x06a\x0B\x83V[`$5\x903a\x0C\"V[Q`\x01\x81R\xF3[P\x904a\x02\x1BW\x80`\x03\x196\x01\x12a\x02\x1BW\x81Q\x90\x80\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x04\x80Ta\x01Y\x81a\x0B\xAFV[\x80\x86R\x92` \x92`\x01\x92\x80\x84\x16\x90\x81\x15a\x01\xECWP`\x01\x14a\x01\x95W[a\x01\x91\x87\x89a\x01\x87\x82\x8A\x03\x83a\n\xEFV[Q\x91\x82\x91\x82a\n\xA6V[\x03\x90\xF3[\x81R\x93P\x7FF\xA2\x80>Y\xA4\xDENzLWK\x12C\xF2Yw\xACLw\xD5\xA1\xA4\xA6\t\xB59L\xEB\xB4\xA2\xAA[\x83\x85\x10a\x01\xD9WPPPP\x81\x01` \x01a\x01\x87\x82a\x01\x918\x80a\x01vV[\x80T\x86\x86\x01\x84\x01R\x93\x82\x01\x93\x81\x01a\x01\xBBV[\x91PPa\x01\x91\x97\x95P\x86\x93P` \x92Pa\x01\x87\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01\x92\x948\x80a\x01vV[\x80\xFD[P4a\0\xE3W\x81`\x03\x196\x01\x12a\0\xE3W` \x90Qf\x07u\xF0Z\x07@\0\x81R\xF3[P4a\0\xE3W` 6`\x03\x19\x01\x12a\0\xE3W` \x91\x81\x90`\x01`\x01`\xA0\x1B\x03a\x02fa\x0B\x83V[\x16\x81R\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\0\x84R T\x90Q\x90\x81R\xF3[P4a\0\xE3W\x81`\x03\x196\x01\x12a\0\xE3W` \x90Q`\x12\x81R\xF3[\x90P\x824a\x02\x1BW``6`\x03\x19\x01\x12a\x02\x1BWa\x02\xCCa\x0B\x83V[a\x02\xD4a\x0B\x99V[\x91`D5\x93a\x02\xE2\x83a\x0B\xE9V[3\x83R` R\x85\x82 T\x90`\x01\x82\x01a\x03\x04W[` \x87a\x01\x10\x88\x88\x88a\x0C\"V[\x85\x82\x10a\x03xW`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x03aW3\x15a\x03JWP\x91a\x01\x10\x93\x91\x86\x86` \x98\x97\x95a\x037\x85a\x0B\xE9V[3\x85R\x8AR\x03\x91 U\x91\x93\x94\x81\x93a\x02\xF6V[\x86QcJ\x14\x06\xB1`\xE1\x1B\x81R\x90\x81\x01\x83\x90R`$\x90\xFD[\x86Qc\xE6\x02\xDF\x05`\xE0\x1B\x81R\x90\x81\x01\x83\x90R`$\x90\xFD[\x86Qc}\xC7\xA0\xD9`\xE1\x1B\x81R3\x91\x81\x01\x91\x82R` \x82\x01\x92\x90\x92R`@\x81\x01\x86\x90R\x81\x90``\x01\x03\x90\xFD[PP4a\0\xE3W\x81`\x03\x196\x01\x12a\0\xE3W` \x90\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x02T\x90Q\x90\x81R\xF3[\x90P4a\x04\x8AW\x81`\x03\x196\x01\x12a\x04\x8AWa\x03\xFBa\x0B\x83V[`$5\x903\x15a\x04sW`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x15a\x04\\WP\x80\x83` \x95a\x04$3a\x0B\xE9V[\x85\x82R\x87R U\x82Q\x90\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x843\x92\xA3Q`\x01\x81R\xF3[\x83QcJ\x14\x06\xB1`\xE1\x1B\x81R\x90\x81\x01\x85\x90R`$\x90\xFD[\x83Qc\xE6\x02\xDF\x05`\xE0\x1B\x81R\x80\x84\x01\x86\x90R`$\x90\xFD[\x82\x80\xFD[\x90P4a\x04\x8AW``6`\x03\x19\x01\x12a\x04\x8AWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x815\x81\x81\x11a\t\xAFWa\x04\xC0\x906\x90\x84\x01a\x0B'V[`$5\x82\x81\x11a\t\xABWa\x04\xD7\x906\x90\x85\x01a\x0B'V[\x92`D5`\x01`\x01`\xA0\x1B\x03\x81\x16\x91\x90\x82\x90\x03a\t\xA7W\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x94\x85T\x94`\xFF\x86\x89\x1C\x16\x15\x94\x81\x87\x16\x96\x87\x15\x80a\t\xA0W[`\x01\x80\x99\x14\x90\x81a\t\x96W[\x15\x90\x81a\t\x8DW[Pa\t}Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x88\x17\x89U\x86a\t^W[Pa\x05_a\r\"V[a\x05ga\r\"V[\x80Q\x82\x81\x11a\tKW\x80\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x03\x92a\x05\x9D\x84Ta\x0B\xAFV[`\x1F\x81\x11a\x08\xD9W[P` \x90\x8D`\x1F\x84\x11`\x01\x14a\x08\\W\x92a\x08QW[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x88\x1B\x17\x90U[\x81Q\x90\x81\x11a\x08>W\x80\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x04\x92a\x06\x06\x84Ta\x0B\xAFV[`\x1F\x81\x11a\x07\xCFW[P` \x90`\x1F\x83\x11`\x01\x14a\x07RW\x8B\x92a\x07GW[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x86\x1B\x17\x90U[\x81\x15a\x070W\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x02\x80T\x90f\x07u\xF0Z\x07@\0\x92\x83\x83\x01\x80\x93\x11a\x07\x1DWP\x88\x92\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x92` \x92U\x84\x84R\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\0\x82R\x88\x84 \x81\x81T\x01\x90U\x88Q\x90\x81R\xA3a\x06\xDCW\x83\x80\xF3[\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x92` \x92h\xFF\0\0\0\0\0\0\0\0\x19\x81T\x16\x90UQ\x90\x81R\xA18\x80\x80\x83\x80\xF3[cNH{q`\xE0\x1B\x8AR`\x11\x90R`$\x89\xFD[\x85Qc\xECD/\x05`\xE0\x1B\x81R\x90\x81\x01\x87\x90R`$\x90\xFD[\x01Q\x90P8\x80a\x06%V[\x84\x8CR\x88\x93P\x7FF\xA2\x80>Y\xA4\xDENzLWK\x12C\xF2Yw\xACLw\xD5\xA1\xA4\xA6\t\xB59L\xEB\xB4\xA2\xAA\x91\x90`\x1F\x19\x84\x16\x8D[\x81\x81\x10a\x07\xB7WP\x84\x11a\x07\x9EW[PPP\x81\x1B\x01\x90Ua\x069V[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80a\x07\x91V[\x82\x84\x01Q\x85U\x8B\x96\x90\x94\x01\x93` \x93\x84\x01\x93\x01a\x07\x82V[\x90\x91P\x83\x8BR\x7FF\xA2\x80>Y\xA4\xDENzLWK\x12C\xF2Yw\xACLw\xD5\xA1\xA4\xA6\t\xB59L\xEB\xB4\xA2\xAA`\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\x084W[\x84\x93\x92\x91`\x1F\x8B\x92\x01`\x05\x1C\x01\x91[\x82\x81\x10a\x08&WPPa\x06\x0FV[\x8D\x81U\x85\x94P\x8A\x91\x01a\x08\x18V[\x90\x91P\x81\x90a\x08\tV[cNH{q`\xE0\x1B\x89R`A\x83R`$\x89\xFD[\x01Q\x90P8\x80a\x05\xBCV[\x91\x90\x8B\x94P`\x1F\x19\x84\x16\x86\x84R\x7F*\xE0\x8A\x8E)%?i\xAC]\x97\x9A\x10\x19V\xAB\x8F\x8D\x9D}\xEDc\xFAz\x83\xB1o\xC4vH\xEA\xB0\x93[\x81\x81\x10a\x08\xC1WP\x84\x11a\x08\xA8W[PPP\x81\x1B\x01\x90Ua\x05\xD0V[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80a\x08\x9BV[\x82\x84\x01Q\x85U\x8D\x96\x90\x94\x01\x93` \x93\x84\x01\x93\x01a\x08\x8CV[\x90\x91P\x83\x8DR\x7F*\xE0\x8A\x8E)%?i\xAC]\x97\x9A\x10\x19V\xAB\x8F\x8D\x9D}\xEDc\xFAz\x83\xB1o\xC4vH\xEA\xB0`\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\tAW[\x8E\x85\x94\x93\x92`\x1F\x8E\x93\x01`\x05\x1C\x01\x92\x90[\x83\x82\x10a\t3WPPPa\x05\xA6V[\x81U\x85\x94P\x8C\x91\x01\x8Fa\t$V[\x90\x91P\x81\x90a\t\x13V[cNH{q`\xE0\x1B\x8BR`A\x85R`$\x8B\xFD[h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16h\x01\0\0\0\0\0\0\0\x01\x17\x88U8a\x05VV[\x89Qc\xF9.\xE8\xA9`\xE0\x1B\x81R\x85\x90\xFD[\x90P\x158a\x05;V[0;\x15\x91Pa\x053V[P\x86a\x05'V[\x86\x80\xFD[\x85\x80\xFD[\x84\x80\xFD[\x83\x90\x854a\x02\x1BW\x80`\x03\x196\x01\x12a\x02\x1BW\x80\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x03\x80Ta\t\xF3\x81a\x0B\xAFV[\x80\x86R\x92` \x92`\x01\x92\x80\x84\x16\x90\x81\x15a\nwWP`\x01\x14a\n Wa\x01\x91\x87\x89a\x01\x87\x82\x8A\x03\x83a\n\xEFV[\x81R\x93P\x7F*\xE0\x8A\x8E)%?i\xAC]\x97\x9A\x10\x19V\xAB\x8F\x8D\x9D}\xEDc\xFAz\x83\xB1o\xC4vH\xEA\xB0[\x83\x85\x10a\ndWPPPP\x81\x01` \x01a\x01\x87\x82a\x01\x91\x86\x80a\x01vV[\x80T\x86\x86\x01\x84\x01R\x93\x82\x01\x93\x81\x01a\nFV[\x91PPa\x01\x91\x97\x95P\x86\x93P` \x92Pa\x01\x87\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01\x92\x94\x86\x80a\x01vV[` \x80\x82R\x82Q\x81\x83\x01\x81\x90R\x90\x93\x92`\0[\x82\x81\x10a\n\xDBWPP`@\x92\x93P`\0\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[\x81\x81\x01\x86\x01Q\x84\x82\x01`@\x01R\x85\x01a\n\xB9V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0B\x11W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x81`\x1F\x82\x01\x12\x15a\x0B~W\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x0B\x11W`@Q\x92a\x0B\\`\x1F\x84\x01`\x1F\x19\x16` \x01\x85a\n\xEFV[\x82\x84R` \x83\x83\x01\x01\x11a\x0B~W\x81`\0\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[`\0\x80\xFD[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x0B~WV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x0B~WV[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x0B\xDFW[` \x83\x10\x14a\x0B\xC9WV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\x0B\xBEV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x01` R`@\x90 \x90V[\x91`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x92\x83\x15a\r\tW\x16\x92\x83\x15a\x0C\xF0W`\0\x90\x83\x82R\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\0\x80` R`@\x83 T\x91\x84\x83\x10a\x0C\xBDWP\x82\x84\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x95\x93`@\x93\x88` \x97R\x86R\x03\x82\x82 U\x86\x81R \x81\x81T\x01\x90U`@Q\x90\x81R\xA3V[`@Qc9\x144\xE3`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x92\x90\x92RP`D\x81\x01\x83\x90R`d\x90\xFD[`@Qc\xECD/\x05`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x90\xFD[`@QcKc~\x8F`\xE1\x1B\x81R`\0`\x04\x82\x01R`$\x90\xFD[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15a\rQWV[`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x90\xFD\xFE\xA2dipfsX\"\x12 \xC6\x08u\xFFB\xBE\xC1t\xAE\x13\xD0\x92\x8B?\xF5\xCF\xCD\xE0\xDE,D\xAAn\xAA\xB3\x11g\x07\xF2\x94\x05\xC3dsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static QSAT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@\x81\x81R`\x04\x91\x826\x10\x15a\0\x16W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x91\x82c\x06\xFD\xDE\x03\x14a\t\xB3WP\x81c\x07\x7F\"J\x14a\x04\x8EW\x81c\t^\xA7\xB3\x14a\x03\xE1W\x81c\x18\x16\r\xDD\x14a\x03\xA3W\x81c#\xB8r\xDD\x14a\x02\xB0WP\x80c1<\xE5g\x14a\x02\x95W\x80cp\xA0\x821\x14a\x02?W\x80c\x90-U\xA5\x14a\x02\x1EW\x80c\x95\xD8\x9BA\x14a\x01\x17W\x80c\xA9\x05\x9C\xBB\x14a\0\xE7Wc\xDDb\xED>\x14a\0\x9EW`\0\x80\xFD[4a\0\xE3W\x80`\x03\x196\x01\x12a\0\xE3W\x80` \x92a\0\xBAa\x0B\x83V[a\0\xCBa\0\xC5a\x0B\x99V[\x91a\x0B\xE9V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x82R\x84R T\x90Q\x90\x81R\xF3[P\x80\xFD[P4a\0\xE3W\x80`\x03\x196\x01\x12a\0\xE3W` \x90a\x01\x10a\x01\x06a\x0B\x83V[`$5\x903a\x0C\"V[Q`\x01\x81R\xF3[P\x904a\x02\x1BW\x80`\x03\x196\x01\x12a\x02\x1BW\x81Q\x90\x80\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x04\x80Ta\x01Y\x81a\x0B\xAFV[\x80\x86R\x92` \x92`\x01\x92\x80\x84\x16\x90\x81\x15a\x01\xECWP`\x01\x14a\x01\x95W[a\x01\x91\x87\x89a\x01\x87\x82\x8A\x03\x83a\n\xEFV[Q\x91\x82\x91\x82a\n\xA6V[\x03\x90\xF3[\x81R\x93P\x7FF\xA2\x80>Y\xA4\xDENzLWK\x12C\xF2Yw\xACLw\xD5\xA1\xA4\xA6\t\xB59L\xEB\xB4\xA2\xAA[\x83\x85\x10a\x01\xD9WPPPP\x81\x01` \x01a\x01\x87\x82a\x01\x918\x80a\x01vV[\x80T\x86\x86\x01\x84\x01R\x93\x82\x01\x93\x81\x01a\x01\xBBV[\x91PPa\x01\x91\x97\x95P\x86\x93P` \x92Pa\x01\x87\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01\x92\x948\x80a\x01vV[\x80\xFD[P4a\0\xE3W\x81`\x03\x196\x01\x12a\0\xE3W` \x90Qf\x07u\xF0Z\x07@\0\x81R\xF3[P4a\0\xE3W` 6`\x03\x19\x01\x12a\0\xE3W` \x91\x81\x90`\x01`\x01`\xA0\x1B\x03a\x02fa\x0B\x83V[\x16\x81R\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\0\x84R T\x90Q\x90\x81R\xF3[P4a\0\xE3W\x81`\x03\x196\x01\x12a\0\xE3W` \x90Q`\x12\x81R\xF3[\x90P\x824a\x02\x1BW``6`\x03\x19\x01\x12a\x02\x1BWa\x02\xCCa\x0B\x83V[a\x02\xD4a\x0B\x99V[\x91`D5\x93a\x02\xE2\x83a\x0B\xE9V[3\x83R` R\x85\x82 T\x90`\x01\x82\x01a\x03\x04W[` \x87a\x01\x10\x88\x88\x88a\x0C\"V[\x85\x82\x10a\x03xW`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x03aW3\x15a\x03JWP\x91a\x01\x10\x93\x91\x86\x86` \x98\x97\x95a\x037\x85a\x0B\xE9V[3\x85R\x8AR\x03\x91 U\x91\x93\x94\x81\x93a\x02\xF6V[\x86QcJ\x14\x06\xB1`\xE1\x1B\x81R\x90\x81\x01\x83\x90R`$\x90\xFD[\x86Qc\xE6\x02\xDF\x05`\xE0\x1B\x81R\x90\x81\x01\x83\x90R`$\x90\xFD[\x86Qc}\xC7\xA0\xD9`\xE1\x1B\x81R3\x91\x81\x01\x91\x82R` \x82\x01\x92\x90\x92R`@\x81\x01\x86\x90R\x81\x90``\x01\x03\x90\xFD[PP4a\0\xE3W\x81`\x03\x196\x01\x12a\0\xE3W` \x90\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x02T\x90Q\x90\x81R\xF3[\x90P4a\x04\x8AW\x81`\x03\x196\x01\x12a\x04\x8AWa\x03\xFBa\x0B\x83V[`$5\x903\x15a\x04sW`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x15a\x04\\WP\x80\x83` \x95a\x04$3a\x0B\xE9V[\x85\x82R\x87R U\x82Q\x90\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x843\x92\xA3Q`\x01\x81R\xF3[\x83QcJ\x14\x06\xB1`\xE1\x1B\x81R\x90\x81\x01\x85\x90R`$\x90\xFD[\x83Qc\xE6\x02\xDF\x05`\xE0\x1B\x81R\x80\x84\x01\x86\x90R`$\x90\xFD[\x82\x80\xFD[\x90P4a\x04\x8AW``6`\x03\x19\x01\x12a\x04\x8AWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x815\x81\x81\x11a\t\xAFWa\x04\xC0\x906\x90\x84\x01a\x0B'V[`$5\x82\x81\x11a\t\xABWa\x04\xD7\x906\x90\x85\x01a\x0B'V[\x92`D5`\x01`\x01`\xA0\x1B\x03\x81\x16\x91\x90\x82\x90\x03a\t\xA7W\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x94\x85T\x94`\xFF\x86\x89\x1C\x16\x15\x94\x81\x87\x16\x96\x87\x15\x80a\t\xA0W[`\x01\x80\x99\x14\x90\x81a\t\x96W[\x15\x90\x81a\t\x8DW[Pa\t}Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x88\x17\x89U\x86a\t^W[Pa\x05_a\r\"V[a\x05ga\r\"V[\x80Q\x82\x81\x11a\tKW\x80\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x03\x92a\x05\x9D\x84Ta\x0B\xAFV[`\x1F\x81\x11a\x08\xD9W[P` \x90\x8D`\x1F\x84\x11`\x01\x14a\x08\\W\x92a\x08QW[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x88\x1B\x17\x90U[\x81Q\x90\x81\x11a\x08>W\x80\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x04\x92a\x06\x06\x84Ta\x0B\xAFV[`\x1F\x81\x11a\x07\xCFW[P` \x90`\x1F\x83\x11`\x01\x14a\x07RW\x8B\x92a\x07GW[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x86\x1B\x17\x90U[\x81\x15a\x070W\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x02\x80T\x90f\x07u\xF0Z\x07@\0\x92\x83\x83\x01\x80\x93\x11a\x07\x1DWP\x88\x92\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x92` \x92U\x84\x84R\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\0\x82R\x88\x84 \x81\x81T\x01\x90U\x88Q\x90\x81R\xA3a\x06\xDCW\x83\x80\xF3[\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x92` \x92h\xFF\0\0\0\0\0\0\0\0\x19\x81T\x16\x90UQ\x90\x81R\xA18\x80\x80\x83\x80\xF3[cNH{q`\xE0\x1B\x8AR`\x11\x90R`$\x89\xFD[\x85Qc\xECD/\x05`\xE0\x1B\x81R\x90\x81\x01\x87\x90R`$\x90\xFD[\x01Q\x90P8\x80a\x06%V[\x84\x8CR\x88\x93P\x7FF\xA2\x80>Y\xA4\xDENzLWK\x12C\xF2Yw\xACLw\xD5\xA1\xA4\xA6\t\xB59L\xEB\xB4\xA2\xAA\x91\x90`\x1F\x19\x84\x16\x8D[\x81\x81\x10a\x07\xB7WP\x84\x11a\x07\x9EW[PPP\x81\x1B\x01\x90Ua\x069V[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80a\x07\x91V[\x82\x84\x01Q\x85U\x8B\x96\x90\x94\x01\x93` \x93\x84\x01\x93\x01a\x07\x82V[\x90\x91P\x83\x8BR\x7FF\xA2\x80>Y\xA4\xDENzLWK\x12C\xF2Yw\xACLw\xD5\xA1\xA4\xA6\t\xB59L\xEB\xB4\xA2\xAA`\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\x084W[\x84\x93\x92\x91`\x1F\x8B\x92\x01`\x05\x1C\x01\x91[\x82\x81\x10a\x08&WPPa\x06\x0FV[\x8D\x81U\x85\x94P\x8A\x91\x01a\x08\x18V[\x90\x91P\x81\x90a\x08\tV[cNH{q`\xE0\x1B\x89R`A\x83R`$\x89\xFD[\x01Q\x90P8\x80a\x05\xBCV[\x91\x90\x8B\x94P`\x1F\x19\x84\x16\x86\x84R\x7F*\xE0\x8A\x8E)%?i\xAC]\x97\x9A\x10\x19V\xAB\x8F\x8D\x9D}\xEDc\xFAz\x83\xB1o\xC4vH\xEA\xB0\x93[\x81\x81\x10a\x08\xC1WP\x84\x11a\x08\xA8W[PPP\x81\x1B\x01\x90Ua\x05\xD0V[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80a\x08\x9BV[\x82\x84\x01Q\x85U\x8D\x96\x90\x94\x01\x93` \x93\x84\x01\x93\x01a\x08\x8CV[\x90\x91P\x83\x8DR\x7F*\xE0\x8A\x8E)%?i\xAC]\x97\x9A\x10\x19V\xAB\x8F\x8D\x9D}\xEDc\xFAz\x83\xB1o\xC4vH\xEA\xB0`\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\tAW[\x8E\x85\x94\x93\x92`\x1F\x8E\x93\x01`\x05\x1C\x01\x92\x90[\x83\x82\x10a\t3WPPPa\x05\xA6V[\x81U\x85\x94P\x8C\x91\x01\x8Fa\t$V[\x90\x91P\x81\x90a\t\x13V[cNH{q`\xE0\x1B\x8BR`A\x85R`$\x8B\xFD[h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16h\x01\0\0\0\0\0\0\0\x01\x17\x88U8a\x05VV[\x89Qc\xF9.\xE8\xA9`\xE0\x1B\x81R\x85\x90\xFD[\x90P\x158a\x05;V[0;\x15\x91Pa\x053V[P\x86a\x05'V[\x86\x80\xFD[\x85\x80\xFD[\x84\x80\xFD[\x83\x90\x854a\x02\x1BW\x80`\x03\x196\x01\x12a\x02\x1BW\x80\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x03\x80Ta\t\xF3\x81a\x0B\xAFV[\x80\x86R\x92` \x92`\x01\x92\x80\x84\x16\x90\x81\x15a\nwWP`\x01\x14a\n Wa\x01\x91\x87\x89a\x01\x87\x82\x8A\x03\x83a\n\xEFV[\x81R\x93P\x7F*\xE0\x8A\x8E)%?i\xAC]\x97\x9A\x10\x19V\xAB\x8F\x8D\x9D}\xEDc\xFAz\x83\xB1o\xC4vH\xEA\xB0[\x83\x85\x10a\ndWPPPP\x81\x01` \x01a\x01\x87\x82a\x01\x91\x86\x80a\x01vV[\x80T\x86\x86\x01\x84\x01R\x93\x82\x01\x93\x81\x01a\nFV[\x91PPa\x01\x91\x97\x95P\x86\x93P` \x92Pa\x01\x87\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01\x92\x94\x86\x80a\x01vV[` \x80\x82R\x82Q\x81\x83\x01\x81\x90R\x90\x93\x92`\0[\x82\x81\x10a\n\xDBWPP`@\x92\x93P`\0\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[\x81\x81\x01\x86\x01Q\x84\x82\x01`@\x01R\x85\x01a\n\xB9V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0B\x11W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x81`\x1F\x82\x01\x12\x15a\x0B~W\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x0B\x11W`@Q\x92a\x0B\\`\x1F\x84\x01`\x1F\x19\x16` \x01\x85a\n\xEFV[\x82\x84R` \x83\x83\x01\x01\x11a\x0B~W\x81`\0\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[`\0\x80\xFD[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x0B~WV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x0B~WV[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x0B\xDFW[` \x83\x10\x14a\x0B\xC9WV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\x0B\xBEV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\x01` R`@\x90 \x90V[\x91`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x92\x83\x15a\r\tW\x16\x92\x83\x15a\x0C\xF0W`\0\x90\x83\x82R\x7FR\xC62G\xE1\xF4}\xB1\x9D\\\xE0F\x000\xC4\x97\xF0g\xCAL\xEB\xF7\x1B\xA9\x8E\xEA\xDA\xBE \xBA\xCE\0\x80` R`@\x83 T\x91\x84\x83\x10a\x0C\xBDWP\x82\x84\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x95\x93`@\x93\x88` \x97R\x86R\x03\x82\x82 U\x86\x81R \x81\x81T\x01\x90U`@Q\x90\x81R\xA3V[`@Qc9\x144\xE3`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x92\x90\x92RP`D\x81\x01\x83\x90R`d\x90\xFD[`@Qc\xECD/\x05`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x90\xFD[`@QcKc~\x8F`\xE1\x1B\x81R`\0`\x04\x82\x01R`$\x90\xFD[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15a\rQWV[`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x90\xFD\xFE\xA2dipfsX\"\x12 \xC6\x08u\xFFB\xBE\xC1t\xAE\x13\xD0\x92\x8B?\xF5\xCF\xCD\xE0\xDE,D\xAAn\xAA\xB3\x11g\x07\xF2\x94\x05\xC3dsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static QSAT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct QSAT<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for QSAT<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for QSAT<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for QSAT<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for QSAT<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(QSAT)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> QSAT<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    QSAT_ABI.clone(),
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
                QSAT_ABI.clone(),
                QSAT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `TOTAL_SUPPLY` (0x902d55a5) function
        pub fn TOTAL_SUPPLY(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([144, 45, 85, 165], ())
                .expect("method not found (this should never happen)")
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
            bridge_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 127, 34, 74], (name, symbol, bridge_address))
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
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn totalSupply(
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, QSATEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for QSAT<M> {
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
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum QSATErrors {
        ERC20InsufficientAllowance(ERC20InsufficientAllowance),
        ERC20InsufficientBalance(ERC20InsufficientBalance),
        ERC20InvalidApprover(ERC20InvalidApprover),
        ERC20InvalidReceiver(ERC20InvalidReceiver),
        ERC20InvalidSender(ERC20InvalidSender),
        ERC20InvalidSpender(ERC20InvalidSpender),
        InvalidInitialization(InvalidInitialization),
        NotInitializing(NotInitializing),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for QSATErrors {
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for QSATErrors {
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
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for QSATErrors {
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
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for QSATErrors {
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
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for QSATErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ERC20InsufficientAllowance> for QSATErrors {
        fn from(value: ERC20InsufficientAllowance) -> Self {
            Self::ERC20InsufficientAllowance(value)
        }
    }
    impl ::core::convert::From<ERC20InsufficientBalance> for QSATErrors {
        fn from(value: ERC20InsufficientBalance) -> Self {
            Self::ERC20InsufficientBalance(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidApprover> for QSATErrors {
        fn from(value: ERC20InvalidApprover) -> Self {
            Self::ERC20InvalidApprover(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidReceiver> for QSATErrors {
        fn from(value: ERC20InvalidReceiver) -> Self {
            Self::ERC20InvalidReceiver(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidSender> for QSATErrors {
        fn from(value: ERC20InvalidSender) -> Self {
            Self::ERC20InvalidSender(value)
        }
    }
    impl ::core::convert::From<ERC20InvalidSpender> for QSATErrors {
        fn from(value: ERC20InvalidSpender) -> Self {
            Self::ERC20InvalidSpender(value)
        }
    }
    impl ::core::convert::From<InvalidInitialization> for QSATErrors {
        fn from(value: InvalidInitialization) -> Self {
            Self::InvalidInitialization(value)
        }
    }
    impl ::core::convert::From<NotInitializing> for QSATErrors {
        fn from(value: NotInitializing) -> Self {
            Self::NotInitializing(value)
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
    pub enum QSATEvents {
        ApprovalFilter(ApprovalFilter),
        InitializedFilter(InitializedFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for QSATEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(QSATEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(QSATEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(QSATEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for QSATEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for QSATEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for QSATEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for QSATEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    ///Container type for all input parameters for the `TOTAL_SUPPLY` function with signature `TOTAL_SUPPLY()` and selector `0x902d55a5`
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
    #[ethcall(name = "TOTAL_SUPPLY", abi = "TOTAL_SUPPLY()")]
    pub struct TOTAL_SUPPLYCall;
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
        pub bridge_address: ::ethers::core::types::Address,
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
    pub struct totalSupplyCall;
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
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum QSATCalls {
        TOTAL_SUPPLY(TOTAL_SUPPLYCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Decimals(DecimalsCall),
        Initialize(InitializeCall),
        Name(NameCall),
        Symbol(SymbolCall),
        totalSupply(totalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for QSATCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <TOTAL_SUPPLYCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TOTAL_SUPPLY(decoded));
            }
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
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) = <totalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::totalSupply(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for QSATCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::TOTAL_SUPPLY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::totalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for QSATCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::TOTAL_SUPPLY(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::totalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<TOTAL_SUPPLYCall> for QSATCalls {
        fn from(value: TOTAL_SUPPLYCall) -> Self {
            Self::TOTAL_SUPPLY(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for QSATCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for QSATCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for QSATCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for QSATCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for QSATCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<NameCall> for QSATCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for QSATCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<totalSupplyCall> for QSATCalls {
        fn from(value: totalSupplyCall) -> Self {
            Self::totalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for QSATCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for QSATCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    ///Container type for all return fields from the `TOTAL_SUPPLY` function with signature `TOTAL_SUPPLY()` and selector `0x902d55a5`
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
    pub struct TOTAL_SUPPLYReturn(pub ::ethers::core::types::U256);
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
    pub struct totalSupplyReturn(pub ::ethers::core::types::U256);
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
