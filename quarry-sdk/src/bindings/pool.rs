pub use pool::*;
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
pub mod pool {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("_calculateDifficulty"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "_calculateDifficulty",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_bits"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
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
                    ::std::borrow::ToOwned::to_owned("buffer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("buffer"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("bufferSize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bufferSize"),
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
                    ::std::borrow::ToOwned::to_owned("bytes32ToString"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bytes32ToString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_bytes32"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("chainTip"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("chainTip"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("header"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Pool.BlockHeader"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("outputAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockReward"),
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
                    ::std::borrow::ToOwned::to_owned("commits"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("commits"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("confirmations"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("confirmations"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("currSize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("currSize"),
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
                    ::std::borrow::ToOwned::to_owned("distributeRewards"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("distributeRewards"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_blockHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
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
                    ::std::borrow::ToOwned::to_owned("doubleSha256"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("doubleSha256"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("end"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("end"),
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
                    ::std::borrow::ToOwned::to_owned("getAddressForSubmittedHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getAddressForSubmittedHash",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_blockHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("getChainTip"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getChainTip"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tip"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Pool.BitcoinBlock"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_oracleAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_pegInAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ringBufferSize"),
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
                    ::std::borrow::ToOwned::to_owned("numSharesInRingBuffer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "numSharesInRingBuffer",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("popFromRingBuffer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("popFromRingBuffer"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pushToRingBuffer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pushToRingBuffer"),
                            inputs: ::std::vec![
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("reverseBytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reverseBytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ringBufferIsEmpty"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ringBufferIsEmpty"),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ringBufferIsFull"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ringBufferIsFull"),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setChainTip"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setChainTip"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_chainTip"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Pool.BitcoinBlock"),
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
                    ::std::borrow::ToOwned::to_owned("setQSATBridgeContract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setQSATBridgeContract",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_qsatBridgeAddress",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("setShareContract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setShareContract"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_shares"),
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
                    ::std::borrow::ToOwned::to_owned("spvProof"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("spvProof"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("merklePath"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("merkleRootHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("spvProof1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("spvProof1"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("merkleRootHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                                    name: ::std::borrow::ToOwned::to_owned("success"),
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
                    ::std::borrow::ToOwned::to_owned("spvValidator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("spvValidator"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract ValidateSPVScript",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("start"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("start"),
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
                    ::std::borrow::ToOwned::to_owned("submitBlock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("submitBlock"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_block"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Pool.BitcoinBlock"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_txHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_proof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
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
                    ::std::borrow::ToOwned::to_owned("submitHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("submitHash"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_blockHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_account"),
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
                (
                    ::std::borrow::ToOwned::to_owned("usedBlockHashes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("usedBlockHashes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validCoinbaseTx"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("validCoinbaseTx"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tx_hash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("merkle"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("valid"),
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
                    ::std::borrow::ToOwned::to_owned("BlockRevealed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("BlockRevealed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("blockHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ChainTipSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ChainTipSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("blockHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("HashCommitted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("HashCommitted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("blockHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
                    ::std::borrow::ToOwned::to_owned("RewardsDistributed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RewardsDistributed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("blockHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RingBufferPop"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RingBufferPop"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
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
                    ::std::borrow::ToOwned::to_owned("RingBufferPush"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RingBufferPush"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
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
    pub static POOL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4`\x82Wa\n\xF6\x81\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x83\x82\x10\x17`lW\x82\x91a\x1FF\x839\x03\x90`\0\xF0\x80\x15``W`\x11\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@Qa\x1E\xBE\x90\x81a\0\x88\x829\xF3[`@Q=`\0\x82>=\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80\xFD\xFE`@`\x80\x81R`\x04\x90\x816\x10\x15a\0\x15W`\0\x80\xFD[`\0\x91\x825`\xE0\x1C\x90\x81b\xEAw\x08\x14a\x171W\x81c\x19\n\xE7_\x14a\x12\xCFW\x81c\"1'\xF2\x14a\x12\x98W\x81c\"3\xAA\xAF\x14a\x07\x9FW\x81c+\xBB\x06C\x14a\x12\x0FW\x81c=Q\xA3j\x14a\x11\xCFW\x81cG\x88W\x81\x14a\x06\nW\x81cV\xE7\xEFc\x14a\x11\xA6W\x81c[}\x7F\xA6\x14a\x11FW\x81cg\xCA\xDAY\x14a\x10\x8AW\x81chE`\xA2\x14a\r\xADW\x81cqP\x18\xA6\x14a\rBW\x81c\x87\xCD\x91\xD8\x14a\x0B3W\x81c\x8D\xA5\xCB[\x14a\n\xFDW\x81c\x92\x01\xDEU\x14a\n\x83W\x81c\x92W\xF4\xDF\x14a\x08\xAEW\x81c\x96\xB1N9\x14a\x07\xDBW\x81c\x9C,w\x0B\x14a\x07\xBEW\x81c\x9EK\x0F\x8E\x14a\x07\x9FW\x81c\x9FD\x1DT\x14a\x07kW\x81c\xA2 \xE2r\x14a\x07+W\x81c\xA5g\xC9\xEC\x14a\x07\x07W\x81c\xAC\x82\x1Bm\x14a\x06uW\x81c\xB8\x14n\x87\x14a\x06=W\x81c\xBB\xB4\x82\xB6\x14a\x06\nW\x81c\xBE\x9AeU\x14a\x05\xEBW\x81c\xC3\x93\xFD2\x14a\x03\x1EW\x81c\xC8\t\xB4[\x14a\x02\xFEW\x81c\xD5R6`\x14a\x02\xD1W\x81c\xD9\xC9\x86\x1D\x14a\x02\xAFW\x81c\xEC\x95\xBF\xE7\x14a\x02\x83WP\x80c\xEF\xBE\x1C\x1C\x14a\x02eW\x80c\xF2\xFD\xE3\x8B\x14a\x025Wc\xF4Fh}\x14a\x01\x98W`\0\x80\xFD[4a\x021W\x81`\x03\x196\x01\x12a\x021W`\xC0\x90a\x01\xB3a\x1B\xDBV[Pa\x01\xC9`\x01\x80`\xA0\x1B\x03`\x07T\x163\x14a\x1C\x01V[\x80Qa\x01\xD4\x81a\x17wV[a\x01\xDCa\x18\xC1V[\x91\x82\x82Ra\x02#`\x16T\x91` \x84\x01\x92\x83R\x80`\x17T\x94\x01\x93\x84RQ\x80\x94c\xFF\xFF\xFF\xFF``\x80\x92\x80Q\x85R` \x81\x01Q` \x86\x01R`@\x81\x01Q`@\x86\x01R\x01Q\x16\x91\x01RV[Q`\x80\x83\x01RQ`\xA0\x82\x01R\xF3[P\x80\xFD[\x824a\x02bW` 6`\x03\x19\x01\x12a\x02bWa\x02_a\x02Ra\x18\xABV[a\x02Za\x1D\xEEV[a\x1DzV[\x80\xF3[\x80\xFD[P4a\x021W\x81`\x03\x196\x01\x12a\x021W` \x90`\x03T\x90Q\x90\x81R\xF3[\x90P4a\x02\xABW` 6`\x03\x19\x01\x12a\x02\xABW` \x92\x82\x915\x81R`\x0C\x84R T\x90Q\x90\x81R\xF3[\x82\x80\xFD[PP4a\x021W\x81`\x03\x196\x01\x12a\x021W` \x91`\x01T\x90T\x14\x90Q\x90\x81R\xF3[\x90P4a\x02\xABW` 6`\x03\x19\x01\x12a\x02\xABW\x81` \x93`\xFF\x925\x81R`\x10\x85R T\x16\x90Q\x90\x15\x15\x81R\xF3[PP4a\x021W\x81`\x03\x196\x01\x12a\x021W` \x90`\x01T\x15\x90Q\x90\x81R\xF3[\x90P4a\x02\xABW` \x92\x83`\x03\x196\x01\x12a\x02bW\x815\x91\x82\x82R`\r\x85R\x80\x84\x83 \x01T`\x08T\x03a\x05\x82W\x82\x82R`\x0C\x85R`\x06\x84\x83 T\x10a\x053W\x90\x84\x91`\x01\x80a\x03\x85`\x01T\x87\x86R`\r\x87R`\x01`\x01`@\x1B\x03`\x05\x8A\x88 \x01T\x16a\x1CaV[\x91[a\x03\xBFW[PPPP\x7FV\xAD\xDA\xAAi\xE1\xD6P\xAF\xE8J\x8E'\x8D|\xE6h5z\x05\xF6?iFKr\xEE\xA8r\xFD\x80\x10\x91\x83Q\x90\x81R\xA1Q`\x01\x81R\xF3[\x90\x91\x92\x93\x81T\x15a\x05-WPa\x03\xD3a\x1C\xD2V[\x87`\x01\x80`\xA0\x1B\x03`\t\x81\x81T\x16\x8AQ\x93\x84\x80\x92c1\xA9\x10\x8F`\xE1\x1B\x82R\x87\x8B\x83\x01R`$\x96\x87\x91Z\xFA\x90\x81\x15a\x04\xE5W\x89\x91a\x04\xF3W[P\x82`\x0BT\x16\x90\x81;\x15a\x04\xEFW\x8BQc\xBAp\x90\xC9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81\x8A\x01\x90\x81R` \x81\x01\x89\x90R\x90\x91\x8A\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x04\xE5W\x90\x89\x91a\x04\xCDW[PPT\x16\x91\x82;\x15a\x04\xC9W\x88Qc\x08R\xCD\x8D`\xE3\x1B\x81R\x86\x81\x01\x91\x90\x91R\x91\x86\x91\x83\x91\x82\x90\x84\x90Z\xF1\x80\x15a\x04\xBFW\x82\x91\x86\x91a\x04\xA4W[PP\x87\x94\x93\x92\x91a\x03\x87V[a\x04\xAF\x91\x92Pa\x17NV[a\x04\xBBW\x80\x848a\x04\x98V[\x83\x80\xFD[\x87Q=\x87\x82>=\x90\xFD[\x86\x80\xFD[a\x04\xD6\x90a\x17NV[a\x04\xE1W\x878a\x04_V[\x87\x80\xFD[\x8BQ=\x8B\x82>=\x90\xFD[\x89\x80\xFD[\x90P\x8B\x81\x81=\x83\x11a\x05&W[a\x05\n\x81\x83a\x17\xC8V[\x81\x01\x03\x12a\x05\"WQ\x82\x81\x16\x81\x03a\x05\"W8a\x04\x0BV[\x88\x80\xFD[P=a\x05\0V[\x93a\x03\x8CV[\x83QbF\x1B\xCD`\xE5\x1B\x81R\x90\x81\x01\x85\x90R`$\x80\x82\x01R\x7FBlock does not have 6+ confirmat`D\x82\x01Rcions`\xE0\x1B`d\x82\x01R`\x84\x90\xFD[\x83QbF\x1B\xCD`\xE5\x1B\x81R\x90\x81\x01\x85\x90R`;`$\x82\x01R\x7FBlock's output address does not `D\x82\x01R\x7Fmatch quarry peg in address\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[PP4a\x021W\x81`\x03\x196\x01\x12a\x021W` \x90`\x02T\x90Q\x90\x81R\xF3[\x90P4a\x02\xABW` 6`\x03\x19\x01\x12a\x02\xABW5\x82R`\x0F` \x90\x81R\x91\x81\x90 T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\xF3[\x82\x844a\x02bW` 6`\x03\x19\x01\x12a\x02bW\x825\x92T\x83\x10\x15a\x02bWPa\x06g` \x92a\x19\xFFV[\x91\x90T\x90Q\x91`\x03\x1B\x1C\x81R\xF3[\x83\x834a\x021W` \x80`\x03\x196\x01\x12a\x02\xABW\x82\x90\x845\x82[\x82\x84\x10a\x06\xA0W` \x86\x86Q\x90\x81R\xF3[\x90\x91\x92\x94`\xFF`\xF8\x1B`\x1F\x87\x81\x03\x90\x80\x82\x11a\x06\xF4W\x88\x82`\x03\x1B\x92\x83\x04`\x08\x14\x91\x14\x17\x15a\x06\xE1W\x84\x88\x1A`\xF8\x1B\x82\x16\x90\x1B\x16\x17\x94`\x01\x01\x92\x91\x90a\x06\x8FV[cNH{q`\xE0\x1B\x84R`\x11\x89R`$\x84\xFD[cNH{q`\xE0\x1B\x85R`\x11\x8AR`$\x85\xFD[PP4a\x021W\x81`\x03\x196\x01\x12a\x021W` \x90a\x07$a\x1C\xD2V[\x90Q\x90\x81R\xF3[\x834a\x02bW` 6`\x03\x19\x01\x12a\x02bWa\x07Ea\x18\xABV[a\x07Ma\x1D\xEEV[`\x01\x80`\xA0\x1B\x03\x16`\x01`\x01``\x1B\x03`\xA0\x1B`\tT\x16\x17`\tU\x80\xF3[\x90P\x824a\x02bW` 6`\x03\x19\x01\x12a\x02bWP5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x07\x9AWa\x07$` \x92a\x1CkV[`\0\x80\xFD[PP4a\x021W\x81`\x03\x196\x01\x12a\x021W` \x90`\x01T\x90Q\x90\x81R\xF3[PP4a\x021W\x81`\x03\x196\x01\x12a\x021W` \x91T\x90Q\x90\x81R\xF3[\x91\x90P4a\x02\xABW``6`\x03\x19\x01\x12a\x02\xABW`D5`\x01`\x01`@\x1B\x03\x81\x11a\x04\xBBWa\x08R\x92a\x08\x13` \x926\x90\x83\x01a\x18UV[\x90`\x01\x80`\xA0\x1B\x03`\x11T\x16\x90\x86\x85Q\x80\x97\x81\x95\x82\x94c1\xC0=\x0F`\xE0\x1B\x84R\x805\x90\x84\x01R`$5`$\x84\x01R`\x80`D\x84\x01R`\x84\x83\x01\x90a\x19\x8DV[\x82`d\x83\x01R\x03\x92Z\xF1\x91\x82\x15a\x08\xA4W` \x93\x92a\x08uW[PQ\x90\x15\x15\x81R\xF3[a\x08\x96\x91\x92P\x83=\x85\x11a\x08\x9DW[a\x08\x8E\x81\x83a\x17\xC8V[\x81\x01\x90a\x1B\x16V[\x908a\x08lV[P=a\x08\x84V[\x81Q=\x85\x82>=\x90\xFD[\x90P4a\x02\xABW\x81`\x03\x196\x01\x12a\x02\xABW\x805`\x01`\x01`@\x1B\x03\x81\x11a\x04\xBBWa\x08\xDD\x906\x90\x83\x01a\x19\nV[\x80Q\x15a\nnW` \x93\x84\x82\x01Q\x91`\x01\x91\x82\x91`\x01\x93[a\tBW[PPPP`$5\x03a\t\x0EWPQ`\x01\x81R\xF3[\x82`d\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\x10`$\x82\x01Ro\x14\xD4\x15\x88\x1C\x1C\x9B\xDB\xD9\x88\x19\x98Z[\x19Y`\x82\x1B`D\x82\x01R\xFD[\x90\x91\x92\x93\x81Q\x85\x10\x15a\nhW\x87\x83a\t[\x87\x85a\x1B\xA2V[Q\x80\x84\x10\x15a\t\xFAW\x89Q\x90\x83\x82\x01\x94\x85R\x8A\x82\x01R\x89\x81Ra\t}\x81a\x17wV[a\t\x8E\x8AQ\x94\x85\x92Q\x95\x86\x91a\x19jV[\x80`\x02\x94\x81\x01\x03\x90\x84Z\xFA\x15a\t\xF0W\x82\x88\x91a\t\xCA\x82Q\x8AQ\x85\x81\x01\x91\x82R\x85\x81Ra\t\xBA\x81a\x17\xADV[\x8BQ\x92\x83\x92\x83\x92Q\x92\x83\x91a\x19jV[\x81\x01\x03\x91Z\xFA\x15a\t\xE4W\x90\x82\x91\x82\x82Q\x95[\x01\x93a\x08\xF5V[P\x84Q\x90=\x90\x82>=\x90\xFD[\x86Q=\x84\x82>=\x90\xFD[\x92\x89Q\x90\x83\x82\x01\x94\x85R\x8A\x82\x01R\x89\x81Ra\n\x14\x81a\x17wV[a\n%\x8AQ\x94\x85\x92Q\x95\x86\x91a\x19jV[\x80`\x02\x94\x81\x01\x03\x90\x84Z\xFA\x15a\t\xF0W\x82\x88\x91a\nQ\x82Q\x8AQ\x85\x81\x01\x91\x82R\x85\x81Ra\t\xBA\x81a\x17\xADV[\x81\x01\x03\x91Z\xFA\x15a\t\xE4W\x90\x82\x91\x82\x82Q\x95a\t\xDDV[\x93a\x08\xFAV[`2\x82cNH{q`\xE0\x1B`\0RR`$`\0\xFD[\x91\x90P4a\x02\xABW` \x91\x82`\x03\x196\x01\x12a\x04\xBBW\x81Q\x93\x815a\n\xA7\x86a\x17\xADV[\x84\x86R\x846\x81\x88\x017\x81[\x85\x81\x10a\n\xCFW\x84Q\x86\x81R\x80a\n\xCB\x81\x89\x01\x8Aa\x19\x8DV[\x03\x90\xF3[\x86Q\x81\x10\x15a\n\xEAW\x80\x82`\x01\x92\x1A\x87\x82\x8A\x01\x01S\x01a\n\xB2V[cNH{q`\xE0\x1B\x83R`2\x84R`$\x83\xFD[PP4a\x021W\x81`\x03\x196\x01\x12a\x021W`\0\x80Q` a\x1Ei\x839\x81Q\x91RT\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x83\x834a\x021W`\xC06`\x03\x19\x01\x12a\x021Wa\x0BO6a\x17\xE9V[a\x0Bd`\x01\x80`\xA0\x1B\x03`\x07T\x163\x14a\x1C\x01V[`\x12T\x80a\x0C\xB6W[P``\x92\x93\x81Q\x91\x82Q\x80`\x12U` \x93\x84\x81\x01Q`\x13U\x85\x81\x01Q`\x14Uc\xFF\xFF\xFF\xFF\x96\x87\x91\x01Q\x16c\xFF\xFF\xFF\xFF\x19\x90\x81`\x15T\x16\x17`\x15U\x84\x83\x01\x90\x81Q`\x16U\x7FG\x81\x1A\xEB\xD2\x8C\xBA\x7F\x02U\xE7\xF5U\x8C]\x8F\x15o\x87\x98H\x11\xE4B\x8B\x95IH\x0B\xE4\xBE$\x86\x88\x86\x01\x94\x85Q`\x17U\x89Q\x90\x81R\xA1\x88[`\x0ET\x81\x10\x15a\x0C\x1BW\x80a\x0B\xF9`\x01\x92a\x19\xB2V[\x90T\x90`\x03\x1B\x1C\x8BR`\x0C\x88R\x88\x8B a\x0C\x13\x81Ta\x1AVV[\x90U\x01a\x0B\xE3V[P\x90\x88\x96`\x05\x93\x92\x89\x86QQ\x8AR`\r\x89R\x82\x8A \x96Q\x92\x83Q\x99\x8A\x89U\x84\x01Q`\x01\x89\x01U\x83\x01Q`\x02\x88\x01U```\x03\x88\x01\x93\x01Q\x16\x90\x82T\x16\x17\x90UQ\x84\x84\x01UQ\x91\x01U`\x0ET\x90`\x01`@\x1B\x82\x10\x15a\x0C\xA3WPa\x0C\x87\x81`\x01a\x0C\x9E\x93\x01`\x0EUa\x19\xB2V[\x81\x93\x91T\x90`\x03\x1B\x91\x82\x1B\x91`\0\x19\x90\x1B\x19\x16\x17\x90V[\x90U\x80\xF3[cNH{q`\xE0\x1B\x84R`A\x90R`$\x83\xFD[` \x82Q\x01Q\x03a\x0C\xC7W\x84a\x0BmV[\x81QbF\x1B\xCD`\xE5\x1B\x81R` \x81\x86\x01R`I`$\x82\x01R\x7FNew chain tip prev block hash do`D\x82\x01R\x7Fes not match current chain tip b`d\x82\x01Rh\r\x8D\xECmd\r\x0C.m`\xBB\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x834a\x02bW\x80`\x03\x196\x01\x12a\x02bWa\r[a\x1D\xEEV[`\0\x80Q` a\x1Ei\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U`\0\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[\x83\x91P4a\x021W``6`\x03\x19\x01\x12a\x021Wa\r\xC9a\x18\xABV[`D5\x90\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x93\x84T\x93`\xFF\x85\x88\x1C\x16\x15\x93`\x01`\x01`@\x1B\x03\x92\x83\x87\x16\x96\x87\x15\x80a\x10\x83W[`\x01\x80\x99\x14\x90\x81a\x10yW[\x15\x90\x81a\x10pW[Pa\x10`Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x88\x17\x89U\x86a\x10AW[Pa\x0EGa\x1E'V[a\x0EOa\x1E'V[a\x0EX3a\x1DzV[\x80d\x02T\x0B\xE4\0`\x06Ua\x10.WPi\x01z\xA5\xBF\xB9.\xE0\x1D4\0`\x05U\x80`\0Ua\x0E\x9Aa\x0E\x85\x82a\x18\xF3V[\x91a\x0E\x92\x8AQ\x93\x84a\x17\xC8V[\x80\x83Ra\x18\xF3V[` \x91\x80\x83\x01\x91`\x1F\x19\x016\x837Q\x93\x84\x11a\x10\x19W`\x01`@\x1B\x84\x11a\x10\x19W\x82T\x84\x84U\x80\x85\x10a\x0F\xD6W[P\x91`\0R\x85`\0[\x84\x81\x10a\x0F\xA4WP\x91PP`\0\x91PU`\0`\x02U`\0`\x03U`\x01\x80`\xA0\x1B\x03\x16`\x01`\x01``\x1B\x03`\xA0\x1B`\x07T\x16\x17`\x07U`$5`\x08U\x83a\x0F\x15a\x1B\xDBV[c\xFF\xFF\xFF\xFF``a\x0F$a\x1B\xB6V[`\0\x81R\x80\x84R\x80Q`\x12U` \x81\x01Q`\x13U\x84\x81\x01Q`\x14U\x01Q\x16c\xFF\xFF\xFF\xFF\x19`\x15T\x16\x17`\x15U` \x81\x01Q`\x16U\x01Q`\x17U`\0`\nUa\x0FhW\0[\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x92` \x92h\xFF\0\0\0\0\0\0\0\0\x19\x81T\x16\x90UQ\x90\x81R\xA1\0[\x82\x84Q\x94\x01\x93\x81\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x01U\x01\x86\x90a\x0E\xD1V[\x87\x85\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x92\x83\x01\x92\x01[\x82\x81\x10a\x10\rWPPa\x0E\xC8V[`\0\x81U\x01\x88\x90a\x0F\xFFV[`A\x83cNH{q`\xE0\x1B`\0RR`$`\0\xFD[cNH{q`\xE0\x1B\x81R`\x11\x83R`$\x90\xFD[h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16h\x01\0\0\0\0\0\0\0\x01\x17\x88U\x89a\x0E>V[\x89Qc\xF9.\xE8\xA9`\xE0\x1B\x81R\x84\x90\xFD[\x90P\x15\x8Ba\x0E#V[0;\x15\x91Pa\x0E\x1BV[P\x86a\x0E\x0FV[\x83\x91P4a\x021W`\x806`\x03\x19\x01\x12a\x021W\x805`\x01`\x01`@\x1B\x03\x81\x11a\x02\xABWa\x10\xBA\x916\x91\x01a\x19\nV[\x91`D5\x91\x90`d5[\x84Q\x83\x10\x15a\x118W`\x01\x90a\x10\xDA\x84\x87a\x1B\xA2V[Q\x94\x82\x95\x86\x83\x16\x15`\0\x14a\x11\x16W\x90a\x11\x0B\x91\x85Q\x91` \x83\x01R\x85\x82\x01R\x84\x81Ra\x11\x06\x81a\x17wV[a\x1B.V[\x94[\x1C\x92\x01\x91a\x10\xC4V[a\x112\x91\x85Q\x91` \x83\x01R\x85\x82\x01R\x84\x81Ra\x11\x06\x81a\x17wV[\x94a\x11\rV[` \x84\x83Q\x90`$5\x14\x81R\xF3[PP4a\x021W\x81`\x03\x196\x01\x12a\x021W`\xC0\x90a\x11ca\x18\xC1V[\x90`\x16Ta\x11\x9A`\x17T\x92Q\x80\x94c\xFF\xFF\xFF\xFF``\x80\x92\x80Q\x85R` \x81\x01Q` \x86\x01R`@\x81\x01Q`@\x86\x01R\x01Q\x16\x91\x01RV[`\x80\x83\x01R`\xA0\x82\x01R\xF3[PP4a\x021W\x81`\x03\x196\x01\x12a\x021W`\x11T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x834a\x02bW` 6`\x03\x19\x01\x12a\x02bWa\x11\xE9a\x18\xABV[a\x11\xF1a\x1D\xEEV[`\x01\x80`\xA0\x1B\x03\x16`\x01`\x01``\x1B\x03`\xA0\x1B`\x0BT\x16\x17`\x0BU\x80\xF3[\x90P4a\x02\xABW\x81`\x03\x196\x01\x12a\x02\xABW`$5\x905`\x01`\x01`\xA0\x1B\x03\x82\x16\x80\x83\x03a\x07\x9AW\x81\x85R`\x0F` \x90\x81R\x84\x86 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x90\x92\x17\x90\x91U\x92Q\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x91\x81\x01\x91\x90\x91R\x7FP\xD2\x7F\xCE\xED\xD1M\xADe\x8A\xD3\xE3\x94\x160Y\xB4\xB8\xCE\x97\x1E\xD4\x86\t\xF5F\x9Es\xB6\xD3\xFA\x84\x90`@\x90\xA1\x80\xF3[\x82\x844a\x02bW` 6`\x03\x19\x01\x12a\x02bW\x825\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02bWPa\x11\x06` \x93a\x07$\x926\x91\x01a\x18UV[\x83\x834a\x021Wa\x01 6`\x03\x19\x01\x12a\x021Wa\x12\xEC6a\x17\xE9V[\x91`\xE45`\x01`\x01`@\x1B\x03\x81\x11a\x021Wa\x13\x0B\x906\x90\x86\x01a\x18UV[a\x01\x045`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x95\x91\x93\x91\x92\x90\x86\x85\x03a\x02\xABW\x83QQ\x91\x82\x84R` \x97`\x0F\x89R\x82\x88\x86 T\x16\x14a\x16\xEEW\x82\x84R`\x10\x88R`\xFF\x87\x85 T\x16a\x16\x9DWa\x13fc\xFF\xFF\xFF\xFF``\x87Q\x01Q\x16a\x1CkV[a\x13u`\x05T`\x06T\x90a\x1B\x03V[\x11\x15a\x16ZW\x87\x85Q\x01Q`\x12T\x03a\x16\x17W\x87\x85\x01Q`\x08T\x03a\x15\xAEW\x87a\x13\xD5\x91\x83`\x11T\x16\x89\x88Q\x01Q\x87\x8D\x8CQ\x96\x87\x95\x86\x94\x85\x93c1\xC0=\x0F`\xE0\x1B\x85R`\xC45\x90\x85\x01R`$\x84\x01R`\x80`D\x84\x01R`\x84\x83\x01\x90a\x19\x8DV[\x82`d\x83\x01R\x03\x92Z\xF1\x80\x15a\x15\x87W\x90a\x14Q\x92\x91a\x15\x91W[P\x82\x84R`\x10\x88R\x86\x84 \x80T`\xFF\x19\x16`\x01\x17\x90U`\tT`\nT\x89\x91\x83\x16a\x14\x19\x82a\x1AVV[`\nU\x89Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x81\x8E\x01\x90\x81R` \x81\x01\x93\x90\x93R\x94\x85\x92\x83\x91\x89\x91\x83\x91`@\x90\x91\x01\x90V[\x03\x92Z\xF1\x91\x82\x15a\x15\x87W\x84\x92a\x15XW[P`\x01T\x84T\x14a\x14\xCBW[P\x7F=j\xC0\xE1\x17\xAAG\xA8.\x86\x9D.\xC9\xAF$\xBA\xDFS\xD2\xBF\x1Dm\t\xA0\xEBH\x9E\xEC\xC82\x0Cx\x95\x96\x97Pa\x14\x9E\x90a\x1A{V[\x81R`\x0C\x86R\x84\x81 UQQ\x83Q\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16` \x82\x01R`@\x90\xA1Q`\x01\x81R\xF3[a\x14\xD3a\x1C\xD2V[\x90`\tT\x16\x98\x89;\x15a\x15TW\x90`$\x85\x92\x83\x8AQ\x9C\x8D\x94\x85\x93c\x08R\xCD\x8D`\xE3\x1B\x85R\x84\x01RZ\xF1\x97\x88\x15a\x15JW\x7F=j\xC0\xE1\x17\xAAG\xA8.\x86\x9D.\xC9\xAF$\xBA\xDFS\xD2\xBF\x1Dm\t\xA0\xEBH\x9E\xEC\xC82\x0Cx\x96\x97\x98a\x155W[\x88\x97\x96Pa\x14oV[\x92a\x15Ca\x14\x9E\x92\x94a\x17NV[\x92\x90a\x15,V[\x86Q=\x85\x82>=\x90\xFD[\x84\x80\xFD[\x90\x91P\x87\x81\x81=\x83\x11a\x15\x80W[a\x15p\x81\x83a\x17\xC8V[\x81\x01\x03\x12a\x04\xBBWQ\x90\x89a\x14cV[P=a\x15fV[\x87Q=\x86\x82>=\x90\xFD[a\x15\xA7\x90\x89=\x8B\x11a\x08\x9DWa\x08\x8E\x81\x83a\x17\xC8V[P\x89a\x13\xF0V[\x86QbF\x1B\xCD`\xE5\x1B\x81R\x80\x8A\x01\x89\x90R`<`$\x82\x01R\x7FCoinbase transaction does not po`D\x82\x01R\x7Fint to quarry peg in address\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x86QbF\x1B\xCD`\xE5\x1B\x81R\x80\x8A\x01\x89\x90R`\x18`$\x82\x01R\x7FSubmitted block is stale\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x86QbF\x1B\xCD`\xE5\x1B\x81R\x80\x8A\x01\x89\x90R`\x17`$\x82\x01R\x7FPool difficulty not met\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x86QbF\x1B\xCD`\xE5\x1B\x81R\x80\x8A\x01\x89\x90R`%`$\x82\x01R\x7FBlock hash has already been subm`D\x82\x01Rd\x1A]\x1D\x19Y`\xDA\x1B`d\x82\x01R`\x84\x90\xFD[\x86QbF\x1B\xCD`\xE5\x1B\x81R\x80\x8A\x01\x89\x90R`\x1D`$\x82\x01R\x7FAddress doesn't match account\0\0\0`D\x82\x01R`d\x90\xFD[\x83\x904a\x021W` 6`\x03\x19\x01\x12a\x021Wa\x02_\x905a\x1A{V[`\x01`\x01`@\x1B\x03\x81\x11a\x17aW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x17aW`@RV[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x17aW`@RV[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x17aW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x17aW`@RV[`\x03\x19\x01\x90`\xC0\x82\x12a\x07\x9AW`@Qa\x18\x02\x81a\x17wV[`\x80\x81\x93\x12a\x07\x9AW`@Qa\x18\x17\x81a\x17\x92V[`\x045\x81R`$5` \x82\x01R`D5`@\x82\x01R`d5c\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x07\x9AW``\x82\x01R\x81R`\x845` \x82\x01R`@`\xA45\x91\x01RV[\x81`\x1F\x82\x01\x12\x15a\x07\x9AW\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x17aW`@Q\x92a\x18\x89`\x1F\x84\x01`\x1F\x19\x16` \x01\x85a\x17\xC8V[\x82\x84R` \x83\x83\x01\x01\x11a\x07\x9AW\x81`\0\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x07\x9AWV[`@Q\x90a\x18\xCE\x82a\x17\x92V[`\x12T\x82R`\x13T` \x83\x01R`\x14T`@\x83\x01R`\x15Tc\xFF\xFF\xFF\xFF\x16``\x83\x01RV[`\x01`\x01`@\x1B\x03\x81\x11a\x17aW`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x07\x9AW` \x90\x825a\x19$\x81a\x18\xF3V[\x93a\x192`@Q\x95\x86a\x17\xC8V[\x81\x85R` \x80\x86\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x07\x9AW` \x01\x90[\x82\x82\x10a\x19[WPPPP\x90V[\x815\x81R\x90\x83\x01\x90\x83\x01a\x19MV[`\0[\x83\x81\x10a\x19}WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x19mV[\x90` \x91a\x19\xA6\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x19jV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`\x0ET\x81\x10\x15a\x19\xE9W`\x0E`\0R\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD\x01\x90`\0\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\x04T\x81\x10\x15a\x19\xE9W`\x04`\0R\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x01\x90`\0\x90V[\x81\x15a\x1A@W\x06\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x19\x81\x14a\x1AeW`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01T`\0T\x14a\x1A\xF5W[\x80a\x1A\x96a\x0C\x87`\x03Ta\x19\xFFV[\x90U\x7Fs\xCB\0<j\xB2L\xDE\xC6\x1C\xA1\xF33S7\xAB\xD9'\xCFg\tv\x99,\xEC\xB2\x8D\xED\x13q\xB7\x94`@`\x03T\x92\x81Q\x90\x81R\x83` \x82\x01R\xA1`\x01\x81\x01\x80\x91\x11a\x1AeW`\0Ta\x1A\xE2\x91a\x1A6V[`\x03Ua\x1A\xF0`\x01Ta\x1AVV[`\x01UV[a\x1A\xFDa\x1C\xD2V[Pa\x1A\x87V[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x1AeWV[\x90\x81` \x91\x03\x12a\x07\x9AWQ\x80\x15\x15\x81\x03a\x07\x9AW\x90V[` \x80a\x1BD`@Q\x93\x84\x81Q\x93\x84\x92\x01a\x19jV[\x82`\0\x93\x84\x92\x81\x01\x03\x90`\x02Z\xFA\x15a\x1B\x96W` \x81a\x1B\x85\x81Q`@Q\x84\x81\x01\x91\x82R\x84\x81Ra\x1Bt\x81a\x17\xADV[`@Q\x92\x83\x92\x83\x92Q\x92\x83\x91a\x19jV[\x81\x01\x03\x90`\x02Z\xFA\x15a\x1B\x96WQ\x90V[`@Q\x90=\x90\x82>=\x90\xFD[\x80Q\x82\x10\x15a\x19\xE9W` \x91`\x05\x1B\x01\x01\x90V[`@Q\x90a\x1B\xC3\x82a\x17\x92V[`\0``\x83\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x01RV[`@Q\x90a\x1B\xE8\x82a\x17wV[`\0`@\x83a\x1B\xF5a\x1B\xB6V[\x81R\x82` \x82\x01R\x01RV[\x15a\x1C\x08WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FOnly the oracleAddress can call `D\x82\x01Rj\x1D\x1A\x1A\\\xC8\x1BY]\x1A\x1B\xD9`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[\x81\x15a\x1A@W\x04\x90V[`\xFF\x81`\x18\x1C\x16`\x1D\x03c\xFF\xFF\xFF\xFF\x81\x11a\x1AeW`\x03\x1B\x90d\x07\xFF\xFF\xFF\xF8c\xFF\xFF\xFF\xF8\x83\x16\x92\x16\x82\x03a\x1AeW`\x06Ta\xFF\xFF\x90\x80\x82\x02\x91\x82\x04\x03a\x1AeWb\xFF\xFF\xFFa\x1C\xBA\x92\x16\x90a\x1CaV[`\xFF\x82\x11a\x1AeW`\x01a\x1C\xCF\x92\x1B\x90a\x1B\x03V[\x90V[`\x01T\x80\x15a\x1DCW`\x02T\x90a\x1C\xE8\x82a\x19\xFFV[\x90T\x90`\x03\x1B\x1C\x91\x7F\xC0\x81\x7F\xF9\xCE!\xEC\xED?kg\x0E5\x13\xACj\x0C\xDF\x83\xD1\x96\xCAk\x1C\xFF\x9F\n\x0E\xC1\xD2\xDF\xD5`@\x80Q\x85\x81R\x83` \x82\x01R\xA1`\x01\x81\x01\x80\x91\x11a\x1AeW`\0Ta\x1D6\x91a\x1A6V[`\x02U`\0\x19\x01`\x01U\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01RnBuffer is empty`\x88\x1B`D\x82\x01R`d\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x81\x15a\x1D\xD5W`\0\x80Q` a\x1Ei\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x84\x17\x90\x91U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`\0\x80\xA3V[`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x90\xFD[`\0\x80Q` a\x1Ei\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1E\x0FWV[`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x90\xFD[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15a\x1EVWV[`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x90\xFD\xFE\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\xA2dipfsX\"\x12 \xF2\xA0\xC0]%le\xA9\xBF5o\xC3\xC0\x1C\xEC\xBD\x8FM\x0CD[\xEF\x89C\x80\xC1\n\xED{fy\\dsolcC\0\x08\x19\x003`\x80\x80`@R4`\x15Wa\n\xDB\x90\x81a\0\x1B\x829\xF3[`\0\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\0\x805`\xE0\x1C\x90\x81c\"C\x029\x14a\0\xA5WP\x80c1\xC0=\x0F\x14a\0\xA0W\x80c>6\x13\x89\x14a\0\x8CW\x80cq2H\xCA\x14a\0\x9BW\x80c\xB1K0P\x14a\0\x96W\x80c\xC3/\\\x9A\x14a\0\x91W\x80c\xD3\x96ya\x14a\0\x8CW\x80c\xE0A2Z\x14a\0\x87Wc\xE6\x8C\x18O\x14a\0\x82W`\0\x80\xFD[a\x03\x9CV[a\x03]V[a\x01\xD4V[a\x02\x83V[a\x028V[a\x02\x17V[a\x01\x85V[4a\0\xC1W\x80`\x03\x196\x01\x12a\0\xC1W`\x01\x19`\x80R` `\x80\xF3[\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\0\xF6W`@RV[a\0\xC4V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\0\xF6W`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xF6W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x81`\x1F\x82\x01\x12\x15a\x01\x80W\x805\x90a\x01P\x82a\x01\x1DV[\x92a\x01^`@Q\x94\x85a\0\xFBV[\x82\x84R` \x83\x83\x01\x01\x11a\x01\x80W\x81`\0\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[`\0\x80\xFD[4a\x01\x80W`\x806`\x03\x19\x01\x12a\x01\x80W`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\x80Wa\x01\xCAa\x01\xBB` \x926\x90`\x04\x01a\x019V[`d5\x90`$5`\x045a\x03\xB9V[`@Q\x90\x15\x15\x81R\xF3[4a\x01\x80W` 6`\x03\x19\x01\x12a\x01\x80W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\x80Wa\x02\x0Fa\x02\n` \x926\x90`\x04\x01a\x019V[a\x04*V[`@Q\x90\x81R\xF3[4a\x01\x80W`@6`\x03\x19\x01\x12a\x01\x80W` a\x01\xCA`$5`\x045a\x04\xDDV[4a\x01\x80W`\x006`\x03\x19\x01\x12a\x01\x80W` `@Q`\0\x19\x81R\xF3[`\x045\x90`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x82\x03a\x01\x80WV[`d5\x90`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x82\x03a\x01\x80WV[4a\x01\x80W`\x806`\x03\x19\x01\x12a\x01\x80Wa\x02\x9Ca\x02UV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$5\x81\x81\x11a\x01\x80Wa\x02\xBD\x906\x90`\x04\x01a\x019V[\x90`D5\x90\x81\x11a\x01\x80Wa\x02\xD8`\0\x916\x90`\x04\x01a\x019V[\x91a\x02\xE1a\x02lV[a\x03?`(`@Q\x80\x93` \x98\x89\x98\x89\x84\x01\x97c\xFF\xFF\xFF\xFF`\xE0\x1B\x80\x93\x16\x89Ra\x03\x14\x81Q\x80\x92\x8D`$\x89\x01\x91\x01a\x04\xF7V[\x84\x01a\x03)\x82Q\x80\x93\x8D`$\x85\x01\x91\x01a\x04\xF7V[\x01\x91\x16`$\x82\x01R\x03`\x08\x81\x01\x84R\x01\x82a\0\xFBV[Q\x90`\x02Z\xFAP`\0\x81\x81`\x02Z\xFAP`\0Q`@Q\x90\x81R` \x90\xF3[4a\x01\x80W`@6`\x03\x19\x01\x12a\x01\x80W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\x80Wa\x01\xCAa\x03\x93` \x926\x90`\x04\x01a\x019V[`$5\x90a\x05\x1AV[4a\x01\x80W`\x006`\x03\x19\x01\x12a\x01\x80W` `@Q`\x02\x19\x81R\xF3[\x92\x91\x81\x84\x14\x80a\x03\xECW[\x80a\x03\xE3W[a\x03\xDAWa\x03\xD7\x93a\x06&V[\x90V[PPPP`\x01\x90V[P\x80Q\x15a\x03\xCAV[P\x82\x15a\x03\xC4V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90` \x82\x01\x80\x92\x11a\x04\x18WV[a\x03\xF4V[\x91\x90\x82\x01\x80\x92\x11a\x04\x18WV[\x90`P\x90`P\x83Q\x06a\x04\xD4W`\0\x92\x83\x91\x90\x82[\x81Q\x84\x10\x15a\x04\xCDW\x83a\x04\xA6W[Pa\x04Y\x83\x82a\x07>V[\x94a\x04d\x84\x83a\x07\xADV[\x95\x80a\x04o\x88a\x07\xCEV[\x11a\x04\x99W\x90a\x04\x81a\x04\x87\x92a\n]V[\x90a\x04\x1DV[\x92\x84\x81\x01\x80\x91\x11a\x04\x18W\x92\x94a\x04?V[PPP\x92PPP`\x02\x19\x90V[a\x04\xB4a\x04\xB8\x91\x85\x84a\x05-V[\x15\x90V[a\x04\xC2W8a\x04NV[P`\x01\x19\x93P\x91PPV[PP\x91PPV[`\0\x19\x92P\x90PV[\x80\x15a\x04\xF0Wa\x04\xEC\x90a\x07\xCEV[\x10\x90V[PP`\0\x90V[`\0[\x83\x81\x10a\x05\nWPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x04\xFAV[`$\x01Q\x03a\x05(W`\x01\x90V[`\0\x90V[\x90\x80`\x04\x01`\x04\x11a\x04\x18W\x01`$\x01Q\x03a\x05(W`\x01\x90V[`@Q\x90``\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\0\xF6W`@R`!\x82R`\x19`\xF9\x1B`@\x83\x7Fnot an even number of hashes: 86` \x82\x01R\x01RV[`@Q\x90a\x05\xA9\x82a\0\xDAV[`\x15\x82Rt\x0E\x8EL\xAC\xA4\r\x8C\xAD\xCC\xEE\x8D\x04\r.d\x06\x07D\x07\x06\xC7`[\x1B` \x83\x01RV[`@Q\x90a\x05\xDA\x82a\0\xDAV[`\x04\x82Rc\x0E\x8E\r\xCD`\xE2\x1B` \x83\x01RV[`@Q\x90a\x05\xFA\x82a\0\xDAV[`\x19\x82R\x7Fcurrent not equal to root\0\0\0\0\0\0\0` \x83\x01RV[\x93\x91\x92a\x064\x82Q`\x1F\x16\x90V[a\x06\xEFW\x81Q\x15a\x06\xDFW\x91a\x06\\a\x06R\x86a\x06Wa\x06Ra\x05\xCDV[a\t\x19V[a\t\xC0V[`\0\x92[\x82Q\x84\x10\x15a\x06\xB1Wa\x06\x95\x90`\x01\x96\x87\x80\x83\x16\x14`\0\x14a\x06\x9BWa\x06\x8C\x90` \x87\x87\x01\x01Qa\n\x83V[\x96[\x1C\x93a\x04\nV[\x92a\x06`V[\x84\x86\x01` \x01Qa\x06\xAB\x91a\n\x83V[\x96a\x06\x8EV[P\x93\x92\x91PP\x80\x82\x14\x91\x82\x15a\x06\xC6WPP\x90V[a\x03\xD7\x91a\x06Wa\x06Ra\x06R\x93a\x06Wa\x06Ra\x05\xEDV[PP\x91PPa\x05(a\x06Ra\x05\x9CV[PP\x91PPa\x05(a\x06Ra\x05HV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x90\x81Q\x81\x10\x15a\x07&W\x01` \x01\x90V[a\x06\xFFV[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x04\x18WV[\x81`H\x01`H\x11a\x04\x18W`h\x82\x82\x01\x01Q\x91`K\x01\x80`K\x11a\x04\x18Wa\x07i\x90`\x02\x19\x92a\x07\x15V[Q`\xF8\x1C\x01`\xFF\x81\x11a\x04\x18W`\xFF\x16\x90`\x1F\x82\x11a\x04\x18Wb\xFF\xFF\xFFa\x03\xD7\x92a\x01\0\n\x91\x80`\xF8\x1C\x90b\xFF\0\0a\xFF\0\x82`\xE8\x1C\x16\x91`\xD8\x1C\x16\x17\x17\x16a\x07+V[`P` \x80\x93`\0\x93\x01\x01`\x02Z\xFAP` `\0\x81\x81`\x02Z\xFAP`\0Q\x90V[\x80`\x08\x1C\x90`\x08\x1B\x90|\xFF\0\0\0\xFF\0\0\0\xFF\0\0\0\xFF\0\0\0\xFF\0\0\0\xFF\0\0\0\xFF\0\0\0\xFF}\xFF\0\0\0\xFF\0\0\0\xFF\0\0\0\xFF\0\0\0\xFF\0\0\0\xFF\0\0\0\xFF\0\0\0\xFF\0\x7F\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\x84\x16~\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\x84\x16\x17`\x10\x1C\x93\x16\x91\x16\x17`\x10\x1B\x80}\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\x83\x16\x17` \x1C\x91y\xFF\xFF\0\0\0\0\0\0\xFF\xFF\0\0\0\0\0\0\xFF\xFF\0\0\0\0\0\0\xFF\xFF{\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\x80\x93\x16\x91\x16\x17` \x1Bw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x82\x16s\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\x85\x16\x17`@\x1B\x93\x16\x17`@\x1C\x16\x17\x80`\x80\x1B\x90`\x80\x1C\x17\x90V[`\0\x80\x91`@Qa\tl`d\x82` \x81\x01\x94c\x10L\x13\xEB`\xE2\x1B\x86R` `$\x83\x01Ra\tU\x81Q\x80\x92\x81`D\x86\x01R` \x86\x86\x01\x91\x01a\x04\xF7V[`\x1F\x80\x19\x91\x01\x16\x81\x01\x03`D\x81\x01\x84R\x01\x82a\0\xFBV[Q\x90jconsole.logZ\xFAPV[`\0\x19\x81\x14a\x04\x18W`\x01\x01\x90V[\x90a\t\x98\x82a\x01\x1DV[a\t\xA5`@Q\x91\x82a\0\xFBV[\x82\x81R\x80\x92a\t\xB6`\x1F\x19\x91a\x01\x1DV[\x01\x90` 6\x91\x017V[\x90`\0[` \x81\x10\x80a\nAW[\x15a\t\xE1Wa\t\xDC\x90a\t\x7FV[a\t\xC4V[a\t\xEA\x90a\t\x8EV[`\0[` \x81\x10\x80\x80a\n%W[\x15a\n\x1EW\x15a\x07&W\x80\x84a\n\x19\x92\x1Aa\n\x13\x82\x85a\x07\x15V[Sa\t\x7FV[a\t\xEDV[P\x90\x92PPV[\x15a\x07&W\x84\x82\x1A`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x15\x15a\t\xF8V[\x15a\x07&W\x82\x81\x1A`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x15\x15a\t\xCEV[\x80\x15a\nmWa\xFF\xFF`\xD0\x1B\x04\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x90`\0\x91\x82R` R` \x81`@\x81`\x02Z\xFAP` \x81\x81\x81`\x02Z\xFAPQ\x90V\xFE\xA2dipfsX\"\x12 \x83\xD5\x13\x03\x96\xE9\x83\xFB\xC1\x83%\xE8T\xE5\xE4MC\xBCn\xD76\x8C\xF4\x7Flv\x07\x82\xCA\x81\xB0\xE8dsolcC\0\x08\x19\x003";
    /// The bytecode of the contract.
    pub static POOL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`@`\x80\x81R`\x04\x90\x816\x10\x15a\0\x15W`\0\x80\xFD[`\0\x91\x825`\xE0\x1C\x90\x81b\xEAw\x08\x14a\x171W\x81c\x19\n\xE7_\x14a\x12\xCFW\x81c\"1'\xF2\x14a\x12\x98W\x81c\"3\xAA\xAF\x14a\x07\x9FW\x81c+\xBB\x06C\x14a\x12\x0FW\x81c=Q\xA3j\x14a\x11\xCFW\x81cG\x88W\x81\x14a\x06\nW\x81cV\xE7\xEFc\x14a\x11\xA6W\x81c[}\x7F\xA6\x14a\x11FW\x81cg\xCA\xDAY\x14a\x10\x8AW\x81chE`\xA2\x14a\r\xADW\x81cqP\x18\xA6\x14a\rBW\x81c\x87\xCD\x91\xD8\x14a\x0B3W\x81c\x8D\xA5\xCB[\x14a\n\xFDW\x81c\x92\x01\xDEU\x14a\n\x83W\x81c\x92W\xF4\xDF\x14a\x08\xAEW\x81c\x96\xB1N9\x14a\x07\xDBW\x81c\x9C,w\x0B\x14a\x07\xBEW\x81c\x9EK\x0F\x8E\x14a\x07\x9FW\x81c\x9FD\x1DT\x14a\x07kW\x81c\xA2 \xE2r\x14a\x07+W\x81c\xA5g\xC9\xEC\x14a\x07\x07W\x81c\xAC\x82\x1Bm\x14a\x06uW\x81c\xB8\x14n\x87\x14a\x06=W\x81c\xBB\xB4\x82\xB6\x14a\x06\nW\x81c\xBE\x9AeU\x14a\x05\xEBW\x81c\xC3\x93\xFD2\x14a\x03\x1EW\x81c\xC8\t\xB4[\x14a\x02\xFEW\x81c\xD5R6`\x14a\x02\xD1W\x81c\xD9\xC9\x86\x1D\x14a\x02\xAFW\x81c\xEC\x95\xBF\xE7\x14a\x02\x83WP\x80c\xEF\xBE\x1C\x1C\x14a\x02eW\x80c\xF2\xFD\xE3\x8B\x14a\x025Wc\xF4Fh}\x14a\x01\x98W`\0\x80\xFD[4a\x021W\x81`\x03\x196\x01\x12a\x021W`\xC0\x90a\x01\xB3a\x1B\xDBV[Pa\x01\xC9`\x01\x80`\xA0\x1B\x03`\x07T\x163\x14a\x1C\x01V[\x80Qa\x01\xD4\x81a\x17wV[a\x01\xDCa\x18\xC1V[\x91\x82\x82Ra\x02#`\x16T\x91` \x84\x01\x92\x83R\x80`\x17T\x94\x01\x93\x84RQ\x80\x94c\xFF\xFF\xFF\xFF``\x80\x92\x80Q\x85R` \x81\x01Q` \x86\x01R`@\x81\x01Q`@\x86\x01R\x01Q\x16\x91\x01RV[Q`\x80\x83\x01RQ`\xA0\x82\x01R\xF3[P\x80\xFD[\x824a\x02bW` 6`\x03\x19\x01\x12a\x02bWa\x02_a\x02Ra\x18\xABV[a\x02Za\x1D\xEEV[a\x1DzV[\x80\xF3[\x80\xFD[P4a\x021W\x81`\x03\x196\x01\x12a\x021W` \x90`\x03T\x90Q\x90\x81R\xF3[\x90P4a\x02\xABW` 6`\x03\x19\x01\x12a\x02\xABW` \x92\x82\x915\x81R`\x0C\x84R T\x90Q\x90\x81R\xF3[\x82\x80\xFD[PP4a\x021W\x81`\x03\x196\x01\x12a\x021W` \x91`\x01T\x90T\x14\x90Q\x90\x81R\xF3[\x90P4a\x02\xABW` 6`\x03\x19\x01\x12a\x02\xABW\x81` \x93`\xFF\x925\x81R`\x10\x85R T\x16\x90Q\x90\x15\x15\x81R\xF3[PP4a\x021W\x81`\x03\x196\x01\x12a\x021W` \x90`\x01T\x15\x90Q\x90\x81R\xF3[\x90P4a\x02\xABW` \x92\x83`\x03\x196\x01\x12a\x02bW\x815\x91\x82\x82R`\r\x85R\x80\x84\x83 \x01T`\x08T\x03a\x05\x82W\x82\x82R`\x0C\x85R`\x06\x84\x83 T\x10a\x053W\x90\x84\x91`\x01\x80a\x03\x85`\x01T\x87\x86R`\r\x87R`\x01`\x01`@\x1B\x03`\x05\x8A\x88 \x01T\x16a\x1CaV[\x91[a\x03\xBFW[PPPP\x7FV\xAD\xDA\xAAi\xE1\xD6P\xAF\xE8J\x8E'\x8D|\xE6h5z\x05\xF6?iFKr\xEE\xA8r\xFD\x80\x10\x91\x83Q\x90\x81R\xA1Q`\x01\x81R\xF3[\x90\x91\x92\x93\x81T\x15a\x05-WPa\x03\xD3a\x1C\xD2V[\x87`\x01\x80`\xA0\x1B\x03`\t\x81\x81T\x16\x8AQ\x93\x84\x80\x92c1\xA9\x10\x8F`\xE1\x1B\x82R\x87\x8B\x83\x01R`$\x96\x87\x91Z\xFA\x90\x81\x15a\x04\xE5W\x89\x91a\x04\xF3W[P\x82`\x0BT\x16\x90\x81;\x15a\x04\xEFW\x8BQc\xBAp\x90\xC9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81\x8A\x01\x90\x81R` \x81\x01\x89\x90R\x90\x91\x8A\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x04\xE5W\x90\x89\x91a\x04\xCDW[PPT\x16\x91\x82;\x15a\x04\xC9W\x88Qc\x08R\xCD\x8D`\xE3\x1B\x81R\x86\x81\x01\x91\x90\x91R\x91\x86\x91\x83\x91\x82\x90\x84\x90Z\xF1\x80\x15a\x04\xBFW\x82\x91\x86\x91a\x04\xA4W[PP\x87\x94\x93\x92\x91a\x03\x87V[a\x04\xAF\x91\x92Pa\x17NV[a\x04\xBBW\x80\x848a\x04\x98V[\x83\x80\xFD[\x87Q=\x87\x82>=\x90\xFD[\x86\x80\xFD[a\x04\xD6\x90a\x17NV[a\x04\xE1W\x878a\x04_V[\x87\x80\xFD[\x8BQ=\x8B\x82>=\x90\xFD[\x89\x80\xFD[\x90P\x8B\x81\x81=\x83\x11a\x05&W[a\x05\n\x81\x83a\x17\xC8V[\x81\x01\x03\x12a\x05\"WQ\x82\x81\x16\x81\x03a\x05\"W8a\x04\x0BV[\x88\x80\xFD[P=a\x05\0V[\x93a\x03\x8CV[\x83QbF\x1B\xCD`\xE5\x1B\x81R\x90\x81\x01\x85\x90R`$\x80\x82\x01R\x7FBlock does not have 6+ confirmat`D\x82\x01Rcions`\xE0\x1B`d\x82\x01R`\x84\x90\xFD[\x83QbF\x1B\xCD`\xE5\x1B\x81R\x90\x81\x01\x85\x90R`;`$\x82\x01R\x7FBlock's output address does not `D\x82\x01R\x7Fmatch quarry peg in address\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[PP4a\x021W\x81`\x03\x196\x01\x12a\x021W` \x90`\x02T\x90Q\x90\x81R\xF3[\x90P4a\x02\xABW` 6`\x03\x19\x01\x12a\x02\xABW5\x82R`\x0F` \x90\x81R\x91\x81\x90 T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\xF3[\x82\x844a\x02bW` 6`\x03\x19\x01\x12a\x02bW\x825\x92T\x83\x10\x15a\x02bWPa\x06g` \x92a\x19\xFFV[\x91\x90T\x90Q\x91`\x03\x1B\x1C\x81R\xF3[\x83\x834a\x021W` \x80`\x03\x196\x01\x12a\x02\xABW\x82\x90\x845\x82[\x82\x84\x10a\x06\xA0W` \x86\x86Q\x90\x81R\xF3[\x90\x91\x92\x94`\xFF`\xF8\x1B`\x1F\x87\x81\x03\x90\x80\x82\x11a\x06\xF4W\x88\x82`\x03\x1B\x92\x83\x04`\x08\x14\x91\x14\x17\x15a\x06\xE1W\x84\x88\x1A`\xF8\x1B\x82\x16\x90\x1B\x16\x17\x94`\x01\x01\x92\x91\x90a\x06\x8FV[cNH{q`\xE0\x1B\x84R`\x11\x89R`$\x84\xFD[cNH{q`\xE0\x1B\x85R`\x11\x8AR`$\x85\xFD[PP4a\x021W\x81`\x03\x196\x01\x12a\x021W` \x90a\x07$a\x1C\xD2V[\x90Q\x90\x81R\xF3[\x834a\x02bW` 6`\x03\x19\x01\x12a\x02bWa\x07Ea\x18\xABV[a\x07Ma\x1D\xEEV[`\x01\x80`\xA0\x1B\x03\x16`\x01`\x01``\x1B\x03`\xA0\x1B`\tT\x16\x17`\tU\x80\xF3[\x90P\x824a\x02bW` 6`\x03\x19\x01\x12a\x02bWP5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x07\x9AWa\x07$` \x92a\x1CkV[`\0\x80\xFD[PP4a\x021W\x81`\x03\x196\x01\x12a\x021W` \x90`\x01T\x90Q\x90\x81R\xF3[PP4a\x021W\x81`\x03\x196\x01\x12a\x021W` \x91T\x90Q\x90\x81R\xF3[\x91\x90P4a\x02\xABW``6`\x03\x19\x01\x12a\x02\xABW`D5`\x01`\x01`@\x1B\x03\x81\x11a\x04\xBBWa\x08R\x92a\x08\x13` \x926\x90\x83\x01a\x18UV[\x90`\x01\x80`\xA0\x1B\x03`\x11T\x16\x90\x86\x85Q\x80\x97\x81\x95\x82\x94c1\xC0=\x0F`\xE0\x1B\x84R\x805\x90\x84\x01R`$5`$\x84\x01R`\x80`D\x84\x01R`\x84\x83\x01\x90a\x19\x8DV[\x82`d\x83\x01R\x03\x92Z\xF1\x91\x82\x15a\x08\xA4W` \x93\x92a\x08uW[PQ\x90\x15\x15\x81R\xF3[a\x08\x96\x91\x92P\x83=\x85\x11a\x08\x9DW[a\x08\x8E\x81\x83a\x17\xC8V[\x81\x01\x90a\x1B\x16V[\x908a\x08lV[P=a\x08\x84V[\x81Q=\x85\x82>=\x90\xFD[\x90P4a\x02\xABW\x81`\x03\x196\x01\x12a\x02\xABW\x805`\x01`\x01`@\x1B\x03\x81\x11a\x04\xBBWa\x08\xDD\x906\x90\x83\x01a\x19\nV[\x80Q\x15a\nnW` \x93\x84\x82\x01Q\x91`\x01\x91\x82\x91`\x01\x93[a\tBW[PPPP`$5\x03a\t\x0EWPQ`\x01\x81R\xF3[\x82`d\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\x10`$\x82\x01Ro\x14\xD4\x15\x88\x1C\x1C\x9B\xDB\xD9\x88\x19\x98Z[\x19Y`\x82\x1B`D\x82\x01R\xFD[\x90\x91\x92\x93\x81Q\x85\x10\x15a\nhW\x87\x83a\t[\x87\x85a\x1B\xA2V[Q\x80\x84\x10\x15a\t\xFAW\x89Q\x90\x83\x82\x01\x94\x85R\x8A\x82\x01R\x89\x81Ra\t}\x81a\x17wV[a\t\x8E\x8AQ\x94\x85\x92Q\x95\x86\x91a\x19jV[\x80`\x02\x94\x81\x01\x03\x90\x84Z\xFA\x15a\t\xF0W\x82\x88\x91a\t\xCA\x82Q\x8AQ\x85\x81\x01\x91\x82R\x85\x81Ra\t\xBA\x81a\x17\xADV[\x8BQ\x92\x83\x92\x83\x92Q\x92\x83\x91a\x19jV[\x81\x01\x03\x91Z\xFA\x15a\t\xE4W\x90\x82\x91\x82\x82Q\x95[\x01\x93a\x08\xF5V[P\x84Q\x90=\x90\x82>=\x90\xFD[\x86Q=\x84\x82>=\x90\xFD[\x92\x89Q\x90\x83\x82\x01\x94\x85R\x8A\x82\x01R\x89\x81Ra\n\x14\x81a\x17wV[a\n%\x8AQ\x94\x85\x92Q\x95\x86\x91a\x19jV[\x80`\x02\x94\x81\x01\x03\x90\x84Z\xFA\x15a\t\xF0W\x82\x88\x91a\nQ\x82Q\x8AQ\x85\x81\x01\x91\x82R\x85\x81Ra\t\xBA\x81a\x17\xADV[\x81\x01\x03\x91Z\xFA\x15a\t\xE4W\x90\x82\x91\x82\x82Q\x95a\t\xDDV[\x93a\x08\xFAV[`2\x82cNH{q`\xE0\x1B`\0RR`$`\0\xFD[\x91\x90P4a\x02\xABW` \x91\x82`\x03\x196\x01\x12a\x04\xBBW\x81Q\x93\x815a\n\xA7\x86a\x17\xADV[\x84\x86R\x846\x81\x88\x017\x81[\x85\x81\x10a\n\xCFW\x84Q\x86\x81R\x80a\n\xCB\x81\x89\x01\x8Aa\x19\x8DV[\x03\x90\xF3[\x86Q\x81\x10\x15a\n\xEAW\x80\x82`\x01\x92\x1A\x87\x82\x8A\x01\x01S\x01a\n\xB2V[cNH{q`\xE0\x1B\x83R`2\x84R`$\x83\xFD[PP4a\x021W\x81`\x03\x196\x01\x12a\x021W`\0\x80Q` a\x1Ei\x839\x81Q\x91RT\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x83\x834a\x021W`\xC06`\x03\x19\x01\x12a\x021Wa\x0BO6a\x17\xE9V[a\x0Bd`\x01\x80`\xA0\x1B\x03`\x07T\x163\x14a\x1C\x01V[`\x12T\x80a\x0C\xB6W[P``\x92\x93\x81Q\x91\x82Q\x80`\x12U` \x93\x84\x81\x01Q`\x13U\x85\x81\x01Q`\x14Uc\xFF\xFF\xFF\xFF\x96\x87\x91\x01Q\x16c\xFF\xFF\xFF\xFF\x19\x90\x81`\x15T\x16\x17`\x15U\x84\x83\x01\x90\x81Q`\x16U\x7FG\x81\x1A\xEB\xD2\x8C\xBA\x7F\x02U\xE7\xF5U\x8C]\x8F\x15o\x87\x98H\x11\xE4B\x8B\x95IH\x0B\xE4\xBE$\x86\x88\x86\x01\x94\x85Q`\x17U\x89Q\x90\x81R\xA1\x88[`\x0ET\x81\x10\x15a\x0C\x1BW\x80a\x0B\xF9`\x01\x92a\x19\xB2V[\x90T\x90`\x03\x1B\x1C\x8BR`\x0C\x88R\x88\x8B a\x0C\x13\x81Ta\x1AVV[\x90U\x01a\x0B\xE3V[P\x90\x88\x96`\x05\x93\x92\x89\x86QQ\x8AR`\r\x89R\x82\x8A \x96Q\x92\x83Q\x99\x8A\x89U\x84\x01Q`\x01\x89\x01U\x83\x01Q`\x02\x88\x01U```\x03\x88\x01\x93\x01Q\x16\x90\x82T\x16\x17\x90UQ\x84\x84\x01UQ\x91\x01U`\x0ET\x90`\x01`@\x1B\x82\x10\x15a\x0C\xA3WPa\x0C\x87\x81`\x01a\x0C\x9E\x93\x01`\x0EUa\x19\xB2V[\x81\x93\x91T\x90`\x03\x1B\x91\x82\x1B\x91`\0\x19\x90\x1B\x19\x16\x17\x90V[\x90U\x80\xF3[cNH{q`\xE0\x1B\x84R`A\x90R`$\x83\xFD[` \x82Q\x01Q\x03a\x0C\xC7W\x84a\x0BmV[\x81QbF\x1B\xCD`\xE5\x1B\x81R` \x81\x86\x01R`I`$\x82\x01R\x7FNew chain tip prev block hash do`D\x82\x01R\x7Fes not match current chain tip b`d\x82\x01Rh\r\x8D\xECmd\r\x0C.m`\xBB\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x834a\x02bW\x80`\x03\x196\x01\x12a\x02bWa\r[a\x1D\xEEV[`\0\x80Q` a\x1Ei\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U`\0\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[\x83\x91P4a\x021W``6`\x03\x19\x01\x12a\x021Wa\r\xC9a\x18\xABV[`D5\x90\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x93\x84T\x93`\xFF\x85\x88\x1C\x16\x15\x93`\x01`\x01`@\x1B\x03\x92\x83\x87\x16\x96\x87\x15\x80a\x10\x83W[`\x01\x80\x99\x14\x90\x81a\x10yW[\x15\x90\x81a\x10pW[Pa\x10`Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x88\x17\x89U\x86a\x10AW[Pa\x0EGa\x1E'V[a\x0EOa\x1E'V[a\x0EX3a\x1DzV[\x80d\x02T\x0B\xE4\0`\x06Ua\x10.WPi\x01z\xA5\xBF\xB9.\xE0\x1D4\0`\x05U\x80`\0Ua\x0E\x9Aa\x0E\x85\x82a\x18\xF3V[\x91a\x0E\x92\x8AQ\x93\x84a\x17\xC8V[\x80\x83Ra\x18\xF3V[` \x91\x80\x83\x01\x91`\x1F\x19\x016\x837Q\x93\x84\x11a\x10\x19W`\x01`@\x1B\x84\x11a\x10\x19W\x82T\x84\x84U\x80\x85\x10a\x0F\xD6W[P\x91`\0R\x85`\0[\x84\x81\x10a\x0F\xA4WP\x91PP`\0\x91PU`\0`\x02U`\0`\x03U`\x01\x80`\xA0\x1B\x03\x16`\x01`\x01``\x1B\x03`\xA0\x1B`\x07T\x16\x17`\x07U`$5`\x08U\x83a\x0F\x15a\x1B\xDBV[c\xFF\xFF\xFF\xFF``a\x0F$a\x1B\xB6V[`\0\x81R\x80\x84R\x80Q`\x12U` \x81\x01Q`\x13U\x84\x81\x01Q`\x14U\x01Q\x16c\xFF\xFF\xFF\xFF\x19`\x15T\x16\x17`\x15U` \x81\x01Q`\x16U\x01Q`\x17U`\0`\nUa\x0FhW\0[\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x92` \x92h\xFF\0\0\0\0\0\0\0\0\x19\x81T\x16\x90UQ\x90\x81R\xA1\0[\x82\x84Q\x94\x01\x93\x81\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x01U\x01\x86\x90a\x0E\xD1V[\x87\x85\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x92\x83\x01\x92\x01[\x82\x81\x10a\x10\rWPPa\x0E\xC8V[`\0\x81U\x01\x88\x90a\x0F\xFFV[`A\x83cNH{q`\xE0\x1B`\0RR`$`\0\xFD[cNH{q`\xE0\x1B\x81R`\x11\x83R`$\x90\xFD[h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16h\x01\0\0\0\0\0\0\0\x01\x17\x88U\x89a\x0E>V[\x89Qc\xF9.\xE8\xA9`\xE0\x1B\x81R\x84\x90\xFD[\x90P\x15\x8Ba\x0E#V[0;\x15\x91Pa\x0E\x1BV[P\x86a\x0E\x0FV[\x83\x91P4a\x021W`\x806`\x03\x19\x01\x12a\x021W\x805`\x01`\x01`@\x1B\x03\x81\x11a\x02\xABWa\x10\xBA\x916\x91\x01a\x19\nV[\x91`D5\x91\x90`d5[\x84Q\x83\x10\x15a\x118W`\x01\x90a\x10\xDA\x84\x87a\x1B\xA2V[Q\x94\x82\x95\x86\x83\x16\x15`\0\x14a\x11\x16W\x90a\x11\x0B\x91\x85Q\x91` \x83\x01R\x85\x82\x01R\x84\x81Ra\x11\x06\x81a\x17wV[a\x1B.V[\x94[\x1C\x92\x01\x91a\x10\xC4V[a\x112\x91\x85Q\x91` \x83\x01R\x85\x82\x01R\x84\x81Ra\x11\x06\x81a\x17wV[\x94a\x11\rV[` \x84\x83Q\x90`$5\x14\x81R\xF3[PP4a\x021W\x81`\x03\x196\x01\x12a\x021W`\xC0\x90a\x11ca\x18\xC1V[\x90`\x16Ta\x11\x9A`\x17T\x92Q\x80\x94c\xFF\xFF\xFF\xFF``\x80\x92\x80Q\x85R` \x81\x01Q` \x86\x01R`@\x81\x01Q`@\x86\x01R\x01Q\x16\x91\x01RV[`\x80\x83\x01R`\xA0\x82\x01R\xF3[PP4a\x021W\x81`\x03\x196\x01\x12a\x021W`\x11T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x834a\x02bW` 6`\x03\x19\x01\x12a\x02bWa\x11\xE9a\x18\xABV[a\x11\xF1a\x1D\xEEV[`\x01\x80`\xA0\x1B\x03\x16`\x01`\x01``\x1B\x03`\xA0\x1B`\x0BT\x16\x17`\x0BU\x80\xF3[\x90P4a\x02\xABW\x81`\x03\x196\x01\x12a\x02\xABW`$5\x905`\x01`\x01`\xA0\x1B\x03\x82\x16\x80\x83\x03a\x07\x9AW\x81\x85R`\x0F` \x90\x81R\x84\x86 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x90\x92\x17\x90\x91U\x92Q\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x91\x81\x01\x91\x90\x91R\x7FP\xD2\x7F\xCE\xED\xD1M\xADe\x8A\xD3\xE3\x94\x160Y\xB4\xB8\xCE\x97\x1E\xD4\x86\t\xF5F\x9Es\xB6\xD3\xFA\x84\x90`@\x90\xA1\x80\xF3[\x82\x844a\x02bW` 6`\x03\x19\x01\x12a\x02bW\x825\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02bWPa\x11\x06` \x93a\x07$\x926\x91\x01a\x18UV[\x83\x834a\x021Wa\x01 6`\x03\x19\x01\x12a\x021Wa\x12\xEC6a\x17\xE9V[\x91`\xE45`\x01`\x01`@\x1B\x03\x81\x11a\x021Wa\x13\x0B\x906\x90\x86\x01a\x18UV[a\x01\x045`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x95\x91\x93\x91\x92\x90\x86\x85\x03a\x02\xABW\x83QQ\x91\x82\x84R` \x97`\x0F\x89R\x82\x88\x86 T\x16\x14a\x16\xEEW\x82\x84R`\x10\x88R`\xFF\x87\x85 T\x16a\x16\x9DWa\x13fc\xFF\xFF\xFF\xFF``\x87Q\x01Q\x16a\x1CkV[a\x13u`\x05T`\x06T\x90a\x1B\x03V[\x11\x15a\x16ZW\x87\x85Q\x01Q`\x12T\x03a\x16\x17W\x87\x85\x01Q`\x08T\x03a\x15\xAEW\x87a\x13\xD5\x91\x83`\x11T\x16\x89\x88Q\x01Q\x87\x8D\x8CQ\x96\x87\x95\x86\x94\x85\x93c1\xC0=\x0F`\xE0\x1B\x85R`\xC45\x90\x85\x01R`$\x84\x01R`\x80`D\x84\x01R`\x84\x83\x01\x90a\x19\x8DV[\x82`d\x83\x01R\x03\x92Z\xF1\x80\x15a\x15\x87W\x90a\x14Q\x92\x91a\x15\x91W[P\x82\x84R`\x10\x88R\x86\x84 \x80T`\xFF\x19\x16`\x01\x17\x90U`\tT`\nT\x89\x91\x83\x16a\x14\x19\x82a\x1AVV[`\nU\x89Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x81\x8E\x01\x90\x81R` \x81\x01\x93\x90\x93R\x94\x85\x92\x83\x91\x89\x91\x83\x91`@\x90\x91\x01\x90V[\x03\x92Z\xF1\x91\x82\x15a\x15\x87W\x84\x92a\x15XW[P`\x01T\x84T\x14a\x14\xCBW[P\x7F=j\xC0\xE1\x17\xAAG\xA8.\x86\x9D.\xC9\xAF$\xBA\xDFS\xD2\xBF\x1Dm\t\xA0\xEBH\x9E\xEC\xC82\x0Cx\x95\x96\x97Pa\x14\x9E\x90a\x1A{V[\x81R`\x0C\x86R\x84\x81 UQQ\x83Q\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16` \x82\x01R`@\x90\xA1Q`\x01\x81R\xF3[a\x14\xD3a\x1C\xD2V[\x90`\tT\x16\x98\x89;\x15a\x15TW\x90`$\x85\x92\x83\x8AQ\x9C\x8D\x94\x85\x93c\x08R\xCD\x8D`\xE3\x1B\x85R\x84\x01RZ\xF1\x97\x88\x15a\x15JW\x7F=j\xC0\xE1\x17\xAAG\xA8.\x86\x9D.\xC9\xAF$\xBA\xDFS\xD2\xBF\x1Dm\t\xA0\xEBH\x9E\xEC\xC82\x0Cx\x96\x97\x98a\x155W[\x88\x97\x96Pa\x14oV[\x92a\x15Ca\x14\x9E\x92\x94a\x17NV[\x92\x90a\x15,V[\x86Q=\x85\x82>=\x90\xFD[\x84\x80\xFD[\x90\x91P\x87\x81\x81=\x83\x11a\x15\x80W[a\x15p\x81\x83a\x17\xC8V[\x81\x01\x03\x12a\x04\xBBWQ\x90\x89a\x14cV[P=a\x15fV[\x87Q=\x86\x82>=\x90\xFD[a\x15\xA7\x90\x89=\x8B\x11a\x08\x9DWa\x08\x8E\x81\x83a\x17\xC8V[P\x89a\x13\xF0V[\x86QbF\x1B\xCD`\xE5\x1B\x81R\x80\x8A\x01\x89\x90R`<`$\x82\x01R\x7FCoinbase transaction does not po`D\x82\x01R\x7Fint to quarry peg in address\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x86QbF\x1B\xCD`\xE5\x1B\x81R\x80\x8A\x01\x89\x90R`\x18`$\x82\x01R\x7FSubmitted block is stale\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x86QbF\x1B\xCD`\xE5\x1B\x81R\x80\x8A\x01\x89\x90R`\x17`$\x82\x01R\x7FPool difficulty not met\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x86QbF\x1B\xCD`\xE5\x1B\x81R\x80\x8A\x01\x89\x90R`%`$\x82\x01R\x7FBlock hash has already been subm`D\x82\x01Rd\x1A]\x1D\x19Y`\xDA\x1B`d\x82\x01R`\x84\x90\xFD[\x86QbF\x1B\xCD`\xE5\x1B\x81R\x80\x8A\x01\x89\x90R`\x1D`$\x82\x01R\x7FAddress doesn't match account\0\0\0`D\x82\x01R`d\x90\xFD[\x83\x904a\x021W` 6`\x03\x19\x01\x12a\x021Wa\x02_\x905a\x1A{V[`\x01`\x01`@\x1B\x03\x81\x11a\x17aW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x17aW`@RV[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x17aW`@RV[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x17aW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x17aW`@RV[`\x03\x19\x01\x90`\xC0\x82\x12a\x07\x9AW`@Qa\x18\x02\x81a\x17wV[`\x80\x81\x93\x12a\x07\x9AW`@Qa\x18\x17\x81a\x17\x92V[`\x045\x81R`$5` \x82\x01R`D5`@\x82\x01R`d5c\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x07\x9AW``\x82\x01R\x81R`\x845` \x82\x01R`@`\xA45\x91\x01RV[\x81`\x1F\x82\x01\x12\x15a\x07\x9AW\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x17aW`@Q\x92a\x18\x89`\x1F\x84\x01`\x1F\x19\x16` \x01\x85a\x17\xC8V[\x82\x84R` \x83\x83\x01\x01\x11a\x07\x9AW\x81`\0\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x07\x9AWV[`@Q\x90a\x18\xCE\x82a\x17\x92V[`\x12T\x82R`\x13T` \x83\x01R`\x14T`@\x83\x01R`\x15Tc\xFF\xFF\xFF\xFF\x16``\x83\x01RV[`\x01`\x01`@\x1B\x03\x81\x11a\x17aW`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x07\x9AW` \x90\x825a\x19$\x81a\x18\xF3V[\x93a\x192`@Q\x95\x86a\x17\xC8V[\x81\x85R` \x80\x86\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x07\x9AW` \x01\x90[\x82\x82\x10a\x19[WPPPP\x90V[\x815\x81R\x90\x83\x01\x90\x83\x01a\x19MV[`\0[\x83\x81\x10a\x19}WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x19mV[\x90` \x91a\x19\xA6\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x19jV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`\x0ET\x81\x10\x15a\x19\xE9W`\x0E`\0R\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD\x01\x90`\0\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\x04T\x81\x10\x15a\x19\xE9W`\x04`\0R\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x01\x90`\0\x90V[\x81\x15a\x1A@W\x06\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x19\x81\x14a\x1AeW`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01T`\0T\x14a\x1A\xF5W[\x80a\x1A\x96a\x0C\x87`\x03Ta\x19\xFFV[\x90U\x7Fs\xCB\0<j\xB2L\xDE\xC6\x1C\xA1\xF33S7\xAB\xD9'\xCFg\tv\x99,\xEC\xB2\x8D\xED\x13q\xB7\x94`@`\x03T\x92\x81Q\x90\x81R\x83` \x82\x01R\xA1`\x01\x81\x01\x80\x91\x11a\x1AeW`\0Ta\x1A\xE2\x91a\x1A6V[`\x03Ua\x1A\xF0`\x01Ta\x1AVV[`\x01UV[a\x1A\xFDa\x1C\xD2V[Pa\x1A\x87V[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x1AeWV[\x90\x81` \x91\x03\x12a\x07\x9AWQ\x80\x15\x15\x81\x03a\x07\x9AW\x90V[` \x80a\x1BD`@Q\x93\x84\x81Q\x93\x84\x92\x01a\x19jV[\x82`\0\x93\x84\x92\x81\x01\x03\x90`\x02Z\xFA\x15a\x1B\x96W` \x81a\x1B\x85\x81Q`@Q\x84\x81\x01\x91\x82R\x84\x81Ra\x1Bt\x81a\x17\xADV[`@Q\x92\x83\x92\x83\x92Q\x92\x83\x91a\x19jV[\x81\x01\x03\x90`\x02Z\xFA\x15a\x1B\x96WQ\x90V[`@Q\x90=\x90\x82>=\x90\xFD[\x80Q\x82\x10\x15a\x19\xE9W` \x91`\x05\x1B\x01\x01\x90V[`@Q\x90a\x1B\xC3\x82a\x17\x92V[`\0``\x83\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x01RV[`@Q\x90a\x1B\xE8\x82a\x17wV[`\0`@\x83a\x1B\xF5a\x1B\xB6V[\x81R\x82` \x82\x01R\x01RV[\x15a\x1C\x08WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FOnly the oracleAddress can call `D\x82\x01Rj\x1D\x1A\x1A\\\xC8\x1BY]\x1A\x1B\xD9`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[\x81\x15a\x1A@W\x04\x90V[`\xFF\x81`\x18\x1C\x16`\x1D\x03c\xFF\xFF\xFF\xFF\x81\x11a\x1AeW`\x03\x1B\x90d\x07\xFF\xFF\xFF\xF8c\xFF\xFF\xFF\xF8\x83\x16\x92\x16\x82\x03a\x1AeW`\x06Ta\xFF\xFF\x90\x80\x82\x02\x91\x82\x04\x03a\x1AeWb\xFF\xFF\xFFa\x1C\xBA\x92\x16\x90a\x1CaV[`\xFF\x82\x11a\x1AeW`\x01a\x1C\xCF\x92\x1B\x90a\x1B\x03V[\x90V[`\x01T\x80\x15a\x1DCW`\x02T\x90a\x1C\xE8\x82a\x19\xFFV[\x90T\x90`\x03\x1B\x1C\x91\x7F\xC0\x81\x7F\xF9\xCE!\xEC\xED?kg\x0E5\x13\xACj\x0C\xDF\x83\xD1\x96\xCAk\x1C\xFF\x9F\n\x0E\xC1\xD2\xDF\xD5`@\x80Q\x85\x81R\x83` \x82\x01R\xA1`\x01\x81\x01\x80\x91\x11a\x1AeW`\0Ta\x1D6\x91a\x1A6V[`\x02U`\0\x19\x01`\x01U\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01RnBuffer is empty`\x88\x1B`D\x82\x01R`d\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x81\x15a\x1D\xD5W`\0\x80Q` a\x1Ei\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x84\x17\x90\x91U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`\0\x80\xA3V[`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x90\xFD[`\0\x80Q` a\x1Ei\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1E\x0FWV[`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x90\xFD[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15a\x1EVWV[`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x90\xFD\xFE\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\xA2dipfsX\"\x12 \xF2\xA0\xC0]%le\xA9\xBF5o\xC3\xC0\x1C\xEC\xBD\x8FM\x0CD[\xEF\x89C\x80\xC1\n\xED{fy\\dsolcC\0\x08\x19\x003";
    /// The deployed bytecode of the contract.
    pub static POOL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Pool<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Pool<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Pool<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Pool<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Pool<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Pool)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Pool<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    POOL_ABI.clone(),
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
                POOL_ABI.clone(),
                POOL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `_calculateDifficulty` (0x9f441d54) function
        pub fn calculate_difficulty(
            &self,
            bits: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([159, 68, 29, 84], bits)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `buffer` (0xb8146e87) function
        pub fn buffer(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([184, 20, 110, 135], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bufferSize` (0x9c2c770b) function
        pub fn buffer_size(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([156, 44, 119, 11], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bytes32ToString` (0x9201de55) function
        pub fn bytes_32_to_string(
            &self,
            bytes_32: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([146, 1, 222, 85], bytes_32)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `chainTip` (0x5b7d7fa6) function
        pub fn chain_tip(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (BlockHeader, [u8; 32], ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([91, 125, 127, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `commits` (0x47885781) function
        pub fn commits(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([71, 136, 87, 129], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `confirmations` (0xec95bfe7) function
        pub fn confirmations(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([236, 149, 191, 231], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `currSize` (0x2233aaaf) function
        pub fn curr_size(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([34, 51, 170, 175], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `distributeRewards` (0xc393fd32) function
        pub fn distribute_rewards(
            &self,
            block_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([195, 147, 253, 50], block_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `doubleSha256` (0x223127f2) function
        pub fn double_sha_256(
            &self,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([34, 49, 39, 242], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `end` (0xefbe1c1c) function
        pub fn end(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([239, 190, 28, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAddressForSubmittedHash` (0xbbb482b6) function
        pub fn get_address_for_submitted_hash(
            &self,
            block_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([187, 180, 130, 182], block_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getChainTip` (0xf446687d) function
        pub fn get_chain_tip(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, BitcoinBlock> {
            self.0
                .method_hash([244, 70, 104, 125], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x684560a2) function
        pub fn initialize(
            &self,
            oracle_address: ::ethers::core::types::Address,
            peg_in_address: [u8; 32],
            ring_buffer_size: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [104, 69, 96, 162],
                    (oracle_address, peg_in_address, ring_buffer_size),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `numSharesInRingBuffer` (0x9e4b0f8e) function
        pub fn num_shares_in_ring_buffer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([158, 75, 15, 142], ())
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
        ///Calls the contract's `popFromRingBuffer` (0xa567c9ec) function
        pub fn pop_from_ring_buffer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([165, 103, 201, 236], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pushToRingBuffer` (0x00ea7708) function
        pub fn push_to_ring_buffer(
            &self,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([0, 234, 119, 8], value)
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
        ///Calls the contract's `reverseBytes32` (0xac821b6d) function
        pub fn reverse_bytes_32(
            &self,
            input: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([172, 130, 27, 109], input)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ringBufferIsEmpty` (0xc809b45b) function
        pub fn ring_buffer_is_empty(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([200, 9, 180, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ringBufferIsFull` (0xd9c9861d) function
        pub fn ring_buffer_is_full(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([217, 201, 134, 29], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setChainTip` (0x87cd91d8) function
        pub fn set_chain_tip(
            &self,
            chain_tip: BitcoinBlock,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([135, 205, 145, 216], (chain_tip,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setQSATBridgeContract` (0x3d51a36a) function
        pub fn set_qsat_bridge_contract(
            &self,
            qsat_bridge_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([61, 81, 163, 106], qsat_bridge_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setShareContract` (0xa220e272) function
        pub fn set_share_contract(
            &self,
            shares: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 32, 226, 114], shares)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `spvProof` (0x9257f4df) function
        pub fn spv_proof(
            &self,
            merkle_path: ::std::vec::Vec<[u8; 32]>,
            merkle_root_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([146, 87, 244, 223], (merkle_path, merkle_root_hash))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `spvProof1` (0x67cada59) function
        pub fn spv_proof_1(
            &self,
            proof: ::std::vec::Vec<[u8; 32]>,
            merkle_root_hash: [u8; 32],
            tx_hash: [u8; 32],
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [103, 202, 218, 89],
                    (proof, merkle_root_hash, tx_hash, index),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `spvValidator` (0x56e7ef63) function
        pub fn spv_validator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([86, 231, 239, 99], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `start` (0xbe9a6555) function
        pub fn start(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([190, 154, 101, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitBlock` (0x190ae75f) function
        pub fn submit_block(
            &self,
            block: BitcoinBlock,
            tx_hash: [u8; 32],
            proof: ::ethers::core::types::Bytes,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([25, 10, 231, 95], (block, tx_hash, proof, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitHash` (0x2bbb0643) function
        pub fn submit_hash(
            &self,
            block_hash: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([43, 187, 6, 67], (block_hash, account))
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
        ///Calls the contract's `usedBlockHashes` (0xd5523660) function
        pub fn used_block_hashes(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([213, 82, 54, 96], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validCoinbaseTx` (0x96b14e39) function
        pub fn valid_coinbase_tx(
            &self,
            tx_hash: [u8; 32],
            merkle: [u8; 32],
            proof: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([150, 177, 78, 57], (tx_hash, merkle, proof))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `BlockRevealed` event
        pub fn block_revealed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BlockRevealedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ChainTipSet` event
        pub fn chain_tip_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChainTipSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `HashCommitted` event
        pub fn hash_committed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            HashCommittedFilter,
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
        ///Gets the contract's `RewardsDistributed` event
        pub fn rewards_distributed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RewardsDistributedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RingBufferPop` event
        pub fn ring_buffer_pop_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RingBufferPopFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RingBufferPush` event
        pub fn ring_buffer_push_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RingBufferPushFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PoolEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Pool<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
    pub enum PoolErrors {
        InvalidInitialization(InvalidInitialization),
        NotInitializing(NotInitializing),
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for PoolErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
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
    impl ::ethers::core::abi::AbiEncode for PoolErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
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
    impl ::ethers::contract::ContractRevert for PoolErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
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
    impl ::core::fmt::Display for PoolErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
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
    impl ::core::convert::From<::std::string::String> for PoolErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidInitialization> for PoolErrors {
        fn from(value: InvalidInitialization) -> Self {
            Self::InvalidInitialization(value)
        }
    }
    impl ::core::convert::From<NotInitializing> for PoolErrors {
        fn from(value: NotInitializing) -> Self {
            Self::NotInitializing(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner> for PoolErrors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount> for PoolErrors {
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
    #[ethevent(name = "BlockRevealed", abi = "BlockRevealed(bytes32,address)")]
    pub struct BlockRevealedFilter {
        pub block_hash: [u8; 32],
        pub account: ::ethers::core::types::Address,
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
    #[ethevent(name = "ChainTipSet", abi = "ChainTipSet(bytes32)")]
    pub struct ChainTipSetFilter {
        pub block_hash: [u8; 32],
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
    #[ethevent(name = "HashCommitted", abi = "HashCommitted(bytes32,address)")]
    pub struct HashCommittedFilter {
        pub block_hash: [u8; 32],
        pub account: ::ethers::core::types::Address,
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
    #[ethevent(name = "RewardsDistributed", abi = "RewardsDistributed(bytes32)")]
    pub struct RewardsDistributedFilter {
        pub block_hash: [u8; 32],
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
    #[ethevent(name = "RingBufferPop", abi = "RingBufferPop(uint256,uint256)")]
    pub struct RingBufferPopFilter {
        pub value: ::ethers::core::types::U256,
        pub position: ::ethers::core::types::U256,
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
    #[ethevent(name = "RingBufferPush", abi = "RingBufferPush(uint256,uint256)")]
    pub struct RingBufferPushFilter {
        pub value: ::ethers::core::types::U256,
        pub position: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PoolEvents {
        BlockRevealedFilter(BlockRevealedFilter),
        ChainTipSetFilter(ChainTipSetFilter),
        HashCommittedFilter(HashCommittedFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        RewardsDistributedFilter(RewardsDistributedFilter),
        RingBufferPopFilter(RingBufferPopFilter),
        RingBufferPushFilter(RingBufferPushFilter),
    }
    impl ::ethers::contract::EthLogDecode for PoolEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = BlockRevealedFilter::decode_log(log) {
                return Ok(PoolEvents::BlockRevealedFilter(decoded));
            }
            if let Ok(decoded) = ChainTipSetFilter::decode_log(log) {
                return Ok(PoolEvents::ChainTipSetFilter(decoded));
            }
            if let Ok(decoded) = HashCommittedFilter::decode_log(log) {
                return Ok(PoolEvents::HashCommittedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(PoolEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(PoolEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = RewardsDistributedFilter::decode_log(log) {
                return Ok(PoolEvents::RewardsDistributedFilter(decoded));
            }
            if let Ok(decoded) = RingBufferPopFilter::decode_log(log) {
                return Ok(PoolEvents::RingBufferPopFilter(decoded));
            }
            if let Ok(decoded) = RingBufferPushFilter::decode_log(log) {
                return Ok(PoolEvents::RingBufferPushFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PoolEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BlockRevealedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChainTipSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::HashCommittedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RewardsDistributedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RingBufferPopFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RingBufferPushFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<BlockRevealedFilter> for PoolEvents {
        fn from(value: BlockRevealedFilter) -> Self {
            Self::BlockRevealedFilter(value)
        }
    }
    impl ::core::convert::From<ChainTipSetFilter> for PoolEvents {
        fn from(value: ChainTipSetFilter) -> Self {
            Self::ChainTipSetFilter(value)
        }
    }
    impl ::core::convert::From<HashCommittedFilter> for PoolEvents {
        fn from(value: HashCommittedFilter) -> Self {
            Self::HashCommittedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for PoolEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for PoolEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<RewardsDistributedFilter> for PoolEvents {
        fn from(value: RewardsDistributedFilter) -> Self {
            Self::RewardsDistributedFilter(value)
        }
    }
    impl ::core::convert::From<RingBufferPopFilter> for PoolEvents {
        fn from(value: RingBufferPopFilter) -> Self {
            Self::RingBufferPopFilter(value)
        }
    }
    impl ::core::convert::From<RingBufferPushFilter> for PoolEvents {
        fn from(value: RingBufferPushFilter) -> Self {
            Self::RingBufferPushFilter(value)
        }
    }
    ///Container type for all input parameters for the `_calculateDifficulty` function with signature `_calculateDifficulty(uint32)` and selector `0x9f441d54`
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
    #[ethcall(name = "_calculateDifficulty", abi = "_calculateDifficulty(uint32)")]
    pub struct CalculateDifficultyCall {
        pub bits: u32,
    }
    ///Container type for all input parameters for the `buffer` function with signature `buffer(uint256)` and selector `0xb8146e87`
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
    #[ethcall(name = "buffer", abi = "buffer(uint256)")]
    pub struct BufferCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `bufferSize` function with signature `bufferSize()` and selector `0x9c2c770b`
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
    #[ethcall(name = "bufferSize", abi = "bufferSize()")]
    pub struct BufferSizeCall;
    ///Container type for all input parameters for the `bytes32ToString` function with signature `bytes32ToString(bytes32)` and selector `0x9201de55`
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
    #[ethcall(name = "bytes32ToString", abi = "bytes32ToString(bytes32)")]
    pub struct Bytes32ToStringCall {
        pub bytes_32: [u8; 32],
    }
    ///Container type for all input parameters for the `chainTip` function with signature `chainTip()` and selector `0x5b7d7fa6`
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
    #[ethcall(name = "chainTip", abi = "chainTip()")]
    pub struct ChainTipCall;
    ///Container type for all input parameters for the `commits` function with signature `commits(bytes32)` and selector `0x47885781`
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
    #[ethcall(name = "commits", abi = "commits(bytes32)")]
    pub struct CommitsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `confirmations` function with signature `confirmations(bytes32)` and selector `0xec95bfe7`
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
    #[ethcall(name = "confirmations", abi = "confirmations(bytes32)")]
    pub struct ConfirmationsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `currSize` function with signature `currSize()` and selector `0x2233aaaf`
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
    #[ethcall(name = "currSize", abi = "currSize()")]
    pub struct CurrSizeCall;
    ///Container type for all input parameters for the `distributeRewards` function with signature `distributeRewards(bytes32)` and selector `0xc393fd32`
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
    #[ethcall(name = "distributeRewards", abi = "distributeRewards(bytes32)")]
    pub struct DistributeRewardsCall {
        pub block_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `doubleSha256` function with signature `doubleSha256(bytes)` and selector `0x223127f2`
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
    #[ethcall(name = "doubleSha256", abi = "doubleSha256(bytes)")]
    pub struct DoubleSha256Call {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `end` function with signature `end()` and selector `0xefbe1c1c`
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
    #[ethcall(name = "end", abi = "end()")]
    pub struct EndCall;
    ///Container type for all input parameters for the `getAddressForSubmittedHash` function with signature `getAddressForSubmittedHash(bytes32)` and selector `0xbbb482b6`
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
    #[ethcall(
        name = "getAddressForSubmittedHash",
        abi = "getAddressForSubmittedHash(bytes32)"
    )]
    pub struct GetAddressForSubmittedHashCall {
        pub block_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `getChainTip` function with signature `getChainTip()` and selector `0xf446687d`
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
    #[ethcall(name = "getChainTip", abi = "getChainTip()")]
    pub struct GetChainTipCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,bytes32,uint256)` and selector `0x684560a2`
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
    #[ethcall(name = "initialize", abi = "initialize(address,bytes32,uint256)")]
    pub struct InitializeCall {
        pub oracle_address: ::ethers::core::types::Address,
        pub peg_in_address: [u8; 32],
        pub ring_buffer_size: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `numSharesInRingBuffer` function with signature `numSharesInRingBuffer()` and selector `0x9e4b0f8e`
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
    #[ethcall(name = "numSharesInRingBuffer", abi = "numSharesInRingBuffer()")]
    pub struct NumSharesInRingBufferCall;
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
    ///Container type for all input parameters for the `popFromRingBuffer` function with signature `popFromRingBuffer()` and selector `0xa567c9ec`
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
    #[ethcall(name = "popFromRingBuffer", abi = "popFromRingBuffer()")]
    pub struct PopFromRingBufferCall;
    ///Container type for all input parameters for the `pushToRingBuffer` function with signature `pushToRingBuffer(uint256)` and selector `0x00ea7708`
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
    #[ethcall(name = "pushToRingBuffer", abi = "pushToRingBuffer(uint256)")]
    pub struct PushToRingBufferCall {
        pub value: ::ethers::core::types::U256,
    }
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
    ///Container type for all input parameters for the `reverseBytes32` function with signature `reverseBytes32(bytes32)` and selector `0xac821b6d`
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
    #[ethcall(name = "reverseBytes32", abi = "reverseBytes32(bytes32)")]
    pub struct ReverseBytes32Call {
        pub input: [u8; 32],
    }
    ///Container type for all input parameters for the `ringBufferIsEmpty` function with signature `ringBufferIsEmpty()` and selector `0xc809b45b`
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
    #[ethcall(name = "ringBufferIsEmpty", abi = "ringBufferIsEmpty()")]
    pub struct RingBufferIsEmptyCall;
    ///Container type for all input parameters for the `ringBufferIsFull` function with signature `ringBufferIsFull()` and selector `0xd9c9861d`
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
    #[ethcall(name = "ringBufferIsFull", abi = "ringBufferIsFull()")]
    pub struct RingBufferIsFullCall;
    ///Container type for all input parameters for the `setChainTip` function with signature `setChainTip(((bytes32,bytes32,bytes32,uint32),bytes32,uint256))` and selector `0x87cd91d8`
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
    #[ethcall(
        name = "setChainTip",
        abi = "setChainTip(((bytes32,bytes32,bytes32,uint32),bytes32,uint256))"
    )]
    pub struct SetChainTipCall {
        pub chain_tip: BitcoinBlock,
    }
    ///Container type for all input parameters for the `setQSATBridgeContract` function with signature `setQSATBridgeContract(address)` and selector `0x3d51a36a`
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
    #[ethcall(name = "setQSATBridgeContract", abi = "setQSATBridgeContract(address)")]
    pub struct SetQSATBridgeContractCall {
        pub qsat_bridge_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setShareContract` function with signature `setShareContract(address)` and selector `0xa220e272`
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
    #[ethcall(name = "setShareContract", abi = "setShareContract(address)")]
    pub struct SetShareContractCall {
        pub shares: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `spvProof` function with signature `spvProof(bytes32[],bytes32)` and selector `0x9257f4df`
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
    #[ethcall(name = "spvProof", abi = "spvProof(bytes32[],bytes32)")]
    pub struct SpvProofCall {
        pub merkle_path: ::std::vec::Vec<[u8; 32]>,
        pub merkle_root_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `spvProof1` function with signature `spvProof1(bytes32[],bytes32,bytes32,uint256)` and selector `0x67cada59`
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
    #[ethcall(name = "spvProof1", abi = "spvProof1(bytes32[],bytes32,bytes32,uint256)")]
    pub struct SpvProof1Call {
        pub proof: ::std::vec::Vec<[u8; 32]>,
        pub merkle_root_hash: [u8; 32],
        pub tx_hash: [u8; 32],
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `spvValidator` function with signature `spvValidator()` and selector `0x56e7ef63`
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
    #[ethcall(name = "spvValidator", abi = "spvValidator()")]
    pub struct SpvValidatorCall;
    ///Container type for all input parameters for the `start` function with signature `start()` and selector `0xbe9a6555`
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
    #[ethcall(name = "start", abi = "start()")]
    pub struct StartCall;
    ///Container type for all input parameters for the `submitBlock` function with signature `submitBlock(((bytes32,bytes32,bytes32,uint32),bytes32,uint256),bytes32,bytes,address)` and selector `0x190ae75f`
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
    #[ethcall(
        name = "submitBlock",
        abi = "submitBlock(((bytes32,bytes32,bytes32,uint32),bytes32,uint256),bytes32,bytes,address)"
    )]
    pub struct SubmitBlockCall {
        pub block: BitcoinBlock,
        pub tx_hash: [u8; 32],
        pub proof: ::ethers::core::types::Bytes,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `submitHash` function with signature `submitHash(bytes32,address)` and selector `0x2bbb0643`
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
    #[ethcall(name = "submitHash", abi = "submitHash(bytes32,address)")]
    pub struct SubmitHashCall {
        pub block_hash: [u8; 32],
        pub account: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `usedBlockHashes` function with signature `usedBlockHashes(bytes32)` and selector `0xd5523660`
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
    #[ethcall(name = "usedBlockHashes", abi = "usedBlockHashes(bytes32)")]
    pub struct UsedBlockHashesCall(pub [u8; 32]);
    ///Container type for all input parameters for the `validCoinbaseTx` function with signature `validCoinbaseTx(bytes32,bytes32,bytes)` and selector `0x96b14e39`
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
    #[ethcall(name = "validCoinbaseTx", abi = "validCoinbaseTx(bytes32,bytes32,bytes)")]
    pub struct ValidCoinbaseTxCall {
        pub tx_hash: [u8; 32],
        pub merkle: [u8; 32],
        pub proof: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PoolCalls {
        CalculateDifficulty(CalculateDifficultyCall),
        Buffer(BufferCall),
        BufferSize(BufferSizeCall),
        Bytes32ToString(Bytes32ToStringCall),
        ChainTip(ChainTipCall),
        Commits(CommitsCall),
        Confirmations(ConfirmationsCall),
        CurrSize(CurrSizeCall),
        DistributeRewards(DistributeRewardsCall),
        DoubleSha256(DoubleSha256Call),
        End(EndCall),
        GetAddressForSubmittedHash(GetAddressForSubmittedHashCall),
        GetChainTip(GetChainTipCall),
        Initialize(InitializeCall),
        NumSharesInRingBuffer(NumSharesInRingBufferCall),
        Owner(OwnerCall),
        PopFromRingBuffer(PopFromRingBufferCall),
        PushToRingBuffer(PushToRingBufferCall),
        RenounceOwnership(RenounceOwnershipCall),
        ReverseBytes32(ReverseBytes32Call),
        RingBufferIsEmpty(RingBufferIsEmptyCall),
        RingBufferIsFull(RingBufferIsFullCall),
        SetChainTip(SetChainTipCall),
        SetQSATBridgeContract(SetQSATBridgeContractCall),
        SetShareContract(SetShareContractCall),
        SpvProof(SpvProofCall),
        SpvProof1(SpvProof1Call),
        SpvValidator(SpvValidatorCall),
        Start(StartCall),
        SubmitBlock(SubmitBlockCall),
        SubmitHash(SubmitHashCall),
        TransferOwnership(TransferOwnershipCall),
        UsedBlockHashes(UsedBlockHashesCall),
        ValidCoinbaseTx(ValidCoinbaseTxCall),
    }
    impl ::ethers::core::abi::AbiDecode for PoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CalculateDifficultyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateDifficulty(decoded));
            }
            if let Ok(decoded) = <BufferCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Buffer(decoded));
            }
            if let Ok(decoded) = <BufferSizeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BufferSize(decoded));
            }
            if let Ok(decoded) = <Bytes32ToStringCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Bytes32ToString(decoded));
            }
            if let Ok(decoded) = <ChainTipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChainTip(decoded));
            }
            if let Ok(decoded) = <CommitsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Commits(decoded));
            }
            if let Ok(decoded) = <ConfirmationsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Confirmations(decoded));
            }
            if let Ok(decoded) = <CurrSizeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CurrSize(decoded));
            }
            if let Ok(decoded) = <DistributeRewardsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DistributeRewards(decoded));
            }
            if let Ok(decoded) = <DoubleSha256Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DoubleSha256(decoded));
            }
            if let Ok(decoded) = <EndCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::End(decoded));
            }
            if let Ok(decoded) = <GetAddressForSubmittedHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAddressForSubmittedHash(decoded));
            }
            if let Ok(decoded) = <GetChainTipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetChainTip(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <NumSharesInRingBufferCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NumSharesInRingBuffer(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PopFromRingBufferCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PopFromRingBuffer(decoded));
            }
            if let Ok(decoded) = <PushToRingBufferCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PushToRingBuffer(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <ReverseBytes32Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReverseBytes32(decoded));
            }
            if let Ok(decoded) = <RingBufferIsEmptyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RingBufferIsEmpty(decoded));
            }
            if let Ok(decoded) = <RingBufferIsFullCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RingBufferIsFull(decoded));
            }
            if let Ok(decoded) = <SetChainTipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetChainTip(decoded));
            }
            if let Ok(decoded) = <SetQSATBridgeContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetQSATBridgeContract(decoded));
            }
            if let Ok(decoded) = <SetShareContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetShareContract(decoded));
            }
            if let Ok(decoded) = <SpvProofCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SpvProof(decoded));
            }
            if let Ok(decoded) = <SpvProof1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SpvProof1(decoded));
            }
            if let Ok(decoded) = <SpvValidatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SpvValidator(decoded));
            }
            if let Ok(decoded) = <StartCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Start(decoded));
            }
            if let Ok(decoded) = <SubmitBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SubmitBlock(decoded));
            }
            if let Ok(decoded) = <SubmitHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SubmitHash(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UsedBlockHashesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UsedBlockHashes(decoded));
            }
            if let Ok(decoded) = <ValidCoinbaseTxCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ValidCoinbaseTx(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CalculateDifficulty(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Buffer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BufferSize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Bytes32ToString(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChainTip(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Commits(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Confirmations(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurrSize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DistributeRewards(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DoubleSha256(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::End(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAddressForSubmittedHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChainTip(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NumSharesInRingBuffer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PopFromRingBuffer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PushToRingBuffer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReverseBytes32(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RingBufferIsEmpty(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RingBufferIsFull(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetChainTip(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetQSATBridgeContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetShareContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SpvProof(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SpvProof1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SpvValidator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Start(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SubmitBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmitHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UsedBlockHashes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidCoinbaseTx(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PoolCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CalculateDifficulty(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Buffer(element) => ::core::fmt::Display::fmt(element, f),
                Self::BufferSize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Bytes32ToString(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChainTip(element) => ::core::fmt::Display::fmt(element, f),
                Self::Commits(element) => ::core::fmt::Display::fmt(element, f),
                Self::Confirmations(element) => ::core::fmt::Display::fmt(element, f),
                Self::CurrSize(element) => ::core::fmt::Display::fmt(element, f),
                Self::DistributeRewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::DoubleSha256(element) => ::core::fmt::Display::fmt(element, f),
                Self::End(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAddressForSubmittedHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetChainTip(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::NumSharesInRingBuffer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PopFromRingBuffer(element) => ::core::fmt::Display::fmt(element, f),
                Self::PushToRingBuffer(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReverseBytes32(element) => ::core::fmt::Display::fmt(element, f),
                Self::RingBufferIsEmpty(element) => ::core::fmt::Display::fmt(element, f),
                Self::RingBufferIsFull(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetChainTip(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetQSATBridgeContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetShareContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::SpvProof(element) => ::core::fmt::Display::fmt(element, f),
                Self::SpvProof1(element) => ::core::fmt::Display::fmt(element, f),
                Self::SpvValidator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Start(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitBlock(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UsedBlockHashes(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidCoinbaseTx(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CalculateDifficultyCall> for PoolCalls {
        fn from(value: CalculateDifficultyCall) -> Self {
            Self::CalculateDifficulty(value)
        }
    }
    impl ::core::convert::From<BufferCall> for PoolCalls {
        fn from(value: BufferCall) -> Self {
            Self::Buffer(value)
        }
    }
    impl ::core::convert::From<BufferSizeCall> for PoolCalls {
        fn from(value: BufferSizeCall) -> Self {
            Self::BufferSize(value)
        }
    }
    impl ::core::convert::From<Bytes32ToStringCall> for PoolCalls {
        fn from(value: Bytes32ToStringCall) -> Self {
            Self::Bytes32ToString(value)
        }
    }
    impl ::core::convert::From<ChainTipCall> for PoolCalls {
        fn from(value: ChainTipCall) -> Self {
            Self::ChainTip(value)
        }
    }
    impl ::core::convert::From<CommitsCall> for PoolCalls {
        fn from(value: CommitsCall) -> Self {
            Self::Commits(value)
        }
    }
    impl ::core::convert::From<ConfirmationsCall> for PoolCalls {
        fn from(value: ConfirmationsCall) -> Self {
            Self::Confirmations(value)
        }
    }
    impl ::core::convert::From<CurrSizeCall> for PoolCalls {
        fn from(value: CurrSizeCall) -> Self {
            Self::CurrSize(value)
        }
    }
    impl ::core::convert::From<DistributeRewardsCall> for PoolCalls {
        fn from(value: DistributeRewardsCall) -> Self {
            Self::DistributeRewards(value)
        }
    }
    impl ::core::convert::From<DoubleSha256Call> for PoolCalls {
        fn from(value: DoubleSha256Call) -> Self {
            Self::DoubleSha256(value)
        }
    }
    impl ::core::convert::From<EndCall> for PoolCalls {
        fn from(value: EndCall) -> Self {
            Self::End(value)
        }
    }
    impl ::core::convert::From<GetAddressForSubmittedHashCall> for PoolCalls {
        fn from(value: GetAddressForSubmittedHashCall) -> Self {
            Self::GetAddressForSubmittedHash(value)
        }
    }
    impl ::core::convert::From<GetChainTipCall> for PoolCalls {
        fn from(value: GetChainTipCall) -> Self {
            Self::GetChainTip(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for PoolCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<NumSharesInRingBufferCall> for PoolCalls {
        fn from(value: NumSharesInRingBufferCall) -> Self {
            Self::NumSharesInRingBuffer(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for PoolCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PopFromRingBufferCall> for PoolCalls {
        fn from(value: PopFromRingBufferCall) -> Self {
            Self::PopFromRingBuffer(value)
        }
    }
    impl ::core::convert::From<PushToRingBufferCall> for PoolCalls {
        fn from(value: PushToRingBufferCall) -> Self {
            Self::PushToRingBuffer(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for PoolCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<ReverseBytes32Call> for PoolCalls {
        fn from(value: ReverseBytes32Call) -> Self {
            Self::ReverseBytes32(value)
        }
    }
    impl ::core::convert::From<RingBufferIsEmptyCall> for PoolCalls {
        fn from(value: RingBufferIsEmptyCall) -> Self {
            Self::RingBufferIsEmpty(value)
        }
    }
    impl ::core::convert::From<RingBufferIsFullCall> for PoolCalls {
        fn from(value: RingBufferIsFullCall) -> Self {
            Self::RingBufferIsFull(value)
        }
    }
    impl ::core::convert::From<SetChainTipCall> for PoolCalls {
        fn from(value: SetChainTipCall) -> Self {
            Self::SetChainTip(value)
        }
    }
    impl ::core::convert::From<SetQSATBridgeContractCall> for PoolCalls {
        fn from(value: SetQSATBridgeContractCall) -> Self {
            Self::SetQSATBridgeContract(value)
        }
    }
    impl ::core::convert::From<SetShareContractCall> for PoolCalls {
        fn from(value: SetShareContractCall) -> Self {
            Self::SetShareContract(value)
        }
    }
    impl ::core::convert::From<SpvProofCall> for PoolCalls {
        fn from(value: SpvProofCall) -> Self {
            Self::SpvProof(value)
        }
    }
    impl ::core::convert::From<SpvProof1Call> for PoolCalls {
        fn from(value: SpvProof1Call) -> Self {
            Self::SpvProof1(value)
        }
    }
    impl ::core::convert::From<SpvValidatorCall> for PoolCalls {
        fn from(value: SpvValidatorCall) -> Self {
            Self::SpvValidator(value)
        }
    }
    impl ::core::convert::From<StartCall> for PoolCalls {
        fn from(value: StartCall) -> Self {
            Self::Start(value)
        }
    }
    impl ::core::convert::From<SubmitBlockCall> for PoolCalls {
        fn from(value: SubmitBlockCall) -> Self {
            Self::SubmitBlock(value)
        }
    }
    impl ::core::convert::From<SubmitHashCall> for PoolCalls {
        fn from(value: SubmitHashCall) -> Self {
            Self::SubmitHash(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for PoolCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UsedBlockHashesCall> for PoolCalls {
        fn from(value: UsedBlockHashesCall) -> Self {
            Self::UsedBlockHashes(value)
        }
    }
    impl ::core::convert::From<ValidCoinbaseTxCall> for PoolCalls {
        fn from(value: ValidCoinbaseTxCall) -> Self {
            Self::ValidCoinbaseTx(value)
        }
    }
    ///Container type for all return fields from the `_calculateDifficulty` function with signature `_calculateDifficulty(uint32)` and selector `0x9f441d54`
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
    pub struct CalculateDifficultyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `buffer` function with signature `buffer(uint256)` and selector `0xb8146e87`
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
    pub struct BufferReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `bufferSize` function with signature `bufferSize()` and selector `0x9c2c770b`
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
    pub struct BufferSizeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `bytes32ToString` function with signature `bytes32ToString(bytes32)` and selector `0x9201de55`
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
    pub struct Bytes32ToStringReturn(pub ::std::string::String);
    ///Container type for all return fields from the `chainTip` function with signature `chainTip()` and selector `0x5b7d7fa6`
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
    pub struct ChainTipReturn {
        pub header: BlockHeader,
        pub output_address: [u8; 32],
        pub block_reward: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `commits` function with signature `commits(bytes32)` and selector `0x47885781`
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
    pub struct CommitsReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `confirmations` function with signature `confirmations(bytes32)` and selector `0xec95bfe7`
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
    pub struct ConfirmationsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `currSize` function with signature `currSize()` and selector `0x2233aaaf`
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
    pub struct CurrSizeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `distributeRewards` function with signature `distributeRewards(bytes32)` and selector `0xc393fd32`
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
    pub struct DistributeRewardsReturn {
        pub success: bool,
    }
    ///Container type for all return fields from the `doubleSha256` function with signature `doubleSha256(bytes)` and selector `0x223127f2`
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
    pub struct DoubleSha256Return(pub [u8; 32]);
    ///Container type for all return fields from the `end` function with signature `end()` and selector `0xefbe1c1c`
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
    pub struct EndReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getAddressForSubmittedHash` function with signature `getAddressForSubmittedHash(bytes32)` and selector `0xbbb482b6`
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
    pub struct GetAddressForSubmittedHashReturn {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getChainTip` function with signature `getChainTip()` and selector `0xf446687d`
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
    pub struct GetChainTipReturn {
        pub tip: BitcoinBlock,
    }
    ///Container type for all return fields from the `numSharesInRingBuffer` function with signature `numSharesInRingBuffer()` and selector `0x9e4b0f8e`
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
    pub struct NumSharesInRingBufferReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `popFromRingBuffer` function with signature `popFromRingBuffer()` and selector `0xa567c9ec`
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
    pub struct PopFromRingBufferReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `reverseBytes32` function with signature `reverseBytes32(bytes32)` and selector `0xac821b6d`
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
    pub struct ReverseBytes32Return(pub [u8; 32]);
    ///Container type for all return fields from the `ringBufferIsEmpty` function with signature `ringBufferIsEmpty()` and selector `0xc809b45b`
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
    pub struct RingBufferIsEmptyReturn(pub bool);
    ///Container type for all return fields from the `ringBufferIsFull` function with signature `ringBufferIsFull()` and selector `0xd9c9861d`
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
    pub struct RingBufferIsFullReturn(pub bool);
    ///Container type for all return fields from the `spvProof` function with signature `spvProof(bytes32[],bytes32)` and selector `0x9257f4df`
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
    pub struct SpvProofReturn {
        pub success: bool,
    }
    ///Container type for all return fields from the `spvProof1` function with signature `spvProof1(bytes32[],bytes32,bytes32,uint256)` and selector `0x67cada59`
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
    pub struct SpvProof1Return {
        pub success: bool,
    }
    ///Container type for all return fields from the `spvValidator` function with signature `spvValidator()` and selector `0x56e7ef63`
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
    pub struct SpvValidatorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `start` function with signature `start()` and selector `0xbe9a6555`
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
    pub struct StartReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `submitBlock` function with signature `submitBlock(((bytes32,bytes32,bytes32,uint32),bytes32,uint256),bytes32,bytes,address)` and selector `0x190ae75f`
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
    pub struct SubmitBlockReturn {
        pub success: bool,
    }
    ///Container type for all return fields from the `usedBlockHashes` function with signature `usedBlockHashes(bytes32)` and selector `0xd5523660`
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
    pub struct UsedBlockHashesReturn(pub bool);
    ///Container type for all return fields from the `validCoinbaseTx` function with signature `validCoinbaseTx(bytes32,bytes32,bytes)` and selector `0x96b14e39`
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
    pub struct ValidCoinbaseTxReturn {
        pub valid: bool,
    }
    ///`BitcoinBlock((bytes32,bytes32,bytes32,uint32),bytes32,uint256)`
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
    pub struct BitcoinBlock {
        pub header: BlockHeader,
        pub output_address: [u8; 32],
        pub block_reward: ::ethers::core::types::U256,
    }
    ///`BlockHeader(bytes32,bytes32,bytes32,uint32)`
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
    pub struct BlockHeader {
        pub block_hash: [u8; 32],
        pub previous_block_hash: [u8; 32],
        pub merkle_root_hash: [u8; 32],
        pub bits: u32,
    }
}
