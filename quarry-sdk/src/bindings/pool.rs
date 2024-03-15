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
    pub use super::super::shared_types::*;
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
                    ::std::borrow::ToOwned::to_owned("chainTip"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("chainTip"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("previousBlockHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_block"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(8usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Pool.BitcoinBlock"),
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
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Pool.ChainTip"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOneHundred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOneHundred"),
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_stratumPoolAddress",
                                    ),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_bufferSize"),
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
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Pool.ChainTip"),
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
                    ::std::borrow::ToOwned::to_owned("setQBTCContract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setQBTCContract"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_qbtcAddress"),
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_poolSharesAddress",
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
                                    name: ::std::borrow::ToOwned::to_owned("blockHash"),
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
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(8usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Pool.BitcoinBlock"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_merklePath"),
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
                                    name: ::std::borrow::ToOwned::to_owned("merkleRootHash"),
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
        ///Calls the contract's `chainTip` (0x5b7d7fa6) function
        pub fn chain_tip(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ([u8; 32], [u8; 32])> {
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
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
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
        ///Calls the contract's `distributeRewards` (0xed723cdf) function
        pub fn distribute_rewards(
            &self,
            block: BitcoinBlock,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([237, 114, 60, 223], (block,))
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
        ) -> ::ethers::contract::builders::ContractCall<M, ChainTip> {
            self.0
                .method_hash([244, 70, 104, 125], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOneHundred` (0xffbe5d5a) function
        pub fn get_one_hundred(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([255, 190, 93, 90], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x277a2389) function
        pub fn initialize_with_oracle_address_and_stratum_pool_address_and_peg_in_address_and_ring_buffer_size(
            &self,
            oracle_address: ::ethers::core::types::Address,
            stratum_pool_address: ::ethers::core::types::Address,
            peg_in_address: [u8; 32],
            ring_buffer_size: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [39, 122, 35, 137],
                    (
                        oracle_address,
                        stratum_pool_address,
                        peg_in_address,
                        ring_buffer_size,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x8129fc1c) function
        pub fn initialize(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 41, 252, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xfe4b84df) function
        pub fn initialize_with_buffer_size(
            &self,
            buffer_size: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([254, 75, 132, 223], buffer_size)
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
        ///Calls the contract's `setChainTip` (0x2cf82120) function
        pub fn set_chain_tip(
            &self,
            chain_tip: ChainTip,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([44, 248, 33, 32], (chain_tip,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setQBTCContract` (0xb76a223c) function
        pub fn set_qbtc_contract(
            &self,
            qbtc_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([183, 106, 34, 60], qbtc_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setShareContract` (0xa220e272) function
        pub fn set_share_contract(
            &self,
            pool_shares_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 32, 226, 114], pool_shares_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `spvProof` (0x9257f4df) function
        pub fn spv_proof(
            &self,
            merkle_path: ::std::vec::Vec<[u8; 32]>,
            block_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([146, 87, 244, 223], (merkle_path, block_hash))
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
        ///Calls the contract's `submitBlock` (0xfa4c7ad2) function
        pub fn submit_block(
            &self,
            block: BitcoinBlock,
            merkle_path: ::std::vec::Vec<[u8; 32]>,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 76, 122, 210], (block, merkle_path, account))
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
        pub merkle_root_hash: [u8; 32],
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
    ///Container type for all input parameters for the `distributeRewards` function with signature `distributeRewards(((uint32,bytes32,bytes32,uint32,uint32,uint32),bytes32,bytes8))` and selector `0xed723cdf`
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
        name = "distributeRewards",
        abi = "distributeRewards(((uint32,bytes32,bytes32,uint32,uint32,uint32),bytes32,bytes8))"
    )]
    pub struct DistributeRewardsCall {
        pub block: BitcoinBlock,
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
    ///Container type for all input parameters for the `getOneHundred` function with signature `getOneHundred()` and selector `0xffbe5d5a`
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
    #[ethcall(name = "getOneHundred", abi = "getOneHundred()")]
    pub struct GetOneHundredCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,bytes32,uint256)` and selector `0x277a2389`
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
    #[ethcall(name = "initialize", abi = "initialize(address,address,bytes32,uint256)")]
    pub struct InitializeWithOracleAddressAndStratumPoolAddressAndPegInAddressAndRingBufferSizeCall {
        pub oracle_address: ::ethers::core::types::Address,
        pub stratum_pool_address: ::ethers::core::types::Address,
        pub peg_in_address: [u8; 32],
        pub ring_buffer_size: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize()` and selector `0x8129fc1c`
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
    #[ethcall(name = "initialize", abi = "initialize()")]
    pub struct InitializeCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(uint256)` and selector `0xfe4b84df`
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
    #[ethcall(name = "initialize", abi = "initialize(uint256)")]
    pub struct InitializeWithBufferSizeCall {
        pub buffer_size: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `setChainTip` function with signature `setChainTip((bytes32,bytes32))` and selector `0x2cf82120`
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
    #[ethcall(name = "setChainTip", abi = "setChainTip((bytes32,bytes32))")]
    pub struct SetChainTipCall {
        pub chain_tip: ChainTip,
    }
    ///Container type for all input parameters for the `setQBTCContract` function with signature `setQBTCContract(address)` and selector `0xb76a223c`
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
    #[ethcall(name = "setQBTCContract", abi = "setQBTCContract(address)")]
    pub struct SetQBTCContractCall {
        pub qbtc_address: ::ethers::core::types::Address,
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
        pub pool_shares_address: ::ethers::core::types::Address,
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
        pub block_hash: [u8; 32],
    }
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
    ///Container type for all input parameters for the `submitBlock` function with signature `submitBlock(((uint32,bytes32,bytes32,uint32,uint32,uint32),bytes32,bytes8),bytes32[],address)` and selector `0xfa4c7ad2`
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
        abi = "submitBlock(((uint32,bytes32,bytes32,uint32,uint32,uint32),bytes32,bytes8),bytes32[],address)"
    )]
    pub struct SubmitBlockCall {
        pub block: BitcoinBlock,
        pub merkle_path: ::std::vec::Vec<[u8; 32]>,
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
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PoolCalls {
        CalculateDifficulty(CalculateDifficultyCall),
        Buffer(BufferCall),
        BufferSize(BufferSizeCall),
        ChainTip(ChainTipCall),
        Commits(CommitsCall),
        Confirmations(ConfirmationsCall),
        CurrSize(CurrSizeCall),
        DistributeRewards(DistributeRewardsCall),
        End(EndCall),
        GetAddressForSubmittedHash(GetAddressForSubmittedHashCall),
        GetChainTip(GetChainTipCall),
        GetOneHundred(GetOneHundredCall),
        InitializeWithOracleAddressAndStratumPoolAddressAndPegInAddressAndRingBufferSize(
            InitializeWithOracleAddressAndStratumPoolAddressAndPegInAddressAndRingBufferSizeCall,
        ),
        Initialize(InitializeCall),
        InitializeWithBufferSize(InitializeWithBufferSizeCall),
        NumSharesInRingBuffer(NumSharesInRingBufferCall),
        Owner(OwnerCall),
        PopFromRingBuffer(PopFromRingBufferCall),
        PushToRingBuffer(PushToRingBufferCall),
        RenounceOwnership(RenounceOwnershipCall),
        RingBufferIsEmpty(RingBufferIsEmptyCall),
        RingBufferIsFull(RingBufferIsFullCall),
        SetChainTip(SetChainTipCall),
        SetQBTCContract(SetQBTCContractCall),
        SetShareContract(SetShareContractCall),
        SpvProof(SpvProofCall),
        Start(StartCall),
        SubmitBlock(SubmitBlockCall),
        SubmitHash(SubmitHashCall),
        TransferOwnership(TransferOwnershipCall),
        UsedBlockHashes(UsedBlockHashesCall),
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
            if let Ok(decoded) = <GetOneHundredCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOneHundred(decoded));
            }
            if let Ok(decoded) = <InitializeWithOracleAddressAndStratumPoolAddressAndPegInAddressAndRingBufferSizeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::InitializeWithOracleAddressAndStratumPoolAddressAndPegInAddressAndRingBufferSize(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <InitializeWithBufferSizeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InitializeWithBufferSize(decoded));
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
            if let Ok(decoded) = <SetQBTCContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetQBTCContract(decoded));
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
                Self::End(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAddressForSubmittedHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChainTip(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOneHundred(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitializeWithOracleAddressAndStratumPoolAddressAndPegInAddressAndRingBufferSize(
                    element,
                ) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitializeWithBufferSize(element) => {
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
                Self::RingBufferIsEmpty(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RingBufferIsFull(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetChainTip(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetQBTCContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetShareContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SpvProof(element) => {
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
                Self::ChainTip(element) => ::core::fmt::Display::fmt(element, f),
                Self::Commits(element) => ::core::fmt::Display::fmt(element, f),
                Self::Confirmations(element) => ::core::fmt::Display::fmt(element, f),
                Self::CurrSize(element) => ::core::fmt::Display::fmt(element, f),
                Self::DistributeRewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::End(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAddressForSubmittedHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetChainTip(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOneHundred(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializeWithOracleAddressAndStratumPoolAddressAndPegInAddressAndRingBufferSize(
                    element,
                ) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializeWithBufferSize(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NumSharesInRingBuffer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PopFromRingBuffer(element) => ::core::fmt::Display::fmt(element, f),
                Self::PushToRingBuffer(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::RingBufferIsEmpty(element) => ::core::fmt::Display::fmt(element, f),
                Self::RingBufferIsFull(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetChainTip(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetQBTCContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetShareContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::SpvProof(element) => ::core::fmt::Display::fmt(element, f),
                Self::Start(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitBlock(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UsedBlockHashes(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<GetOneHundredCall> for PoolCalls {
        fn from(value: GetOneHundredCall) -> Self {
            Self::GetOneHundred(value)
        }
    }
    impl ::core::convert::From<
        InitializeWithOracleAddressAndStratumPoolAddressAndPegInAddressAndRingBufferSizeCall,
    > for PoolCalls {
        fn from(
            value: InitializeWithOracleAddressAndStratumPoolAddressAndPegInAddressAndRingBufferSizeCall,
        ) -> Self {
            Self::InitializeWithOracleAddressAndStratumPoolAddressAndPegInAddressAndRingBufferSize(
                value,
            )
        }
    }
    impl ::core::convert::From<InitializeCall> for PoolCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<InitializeWithBufferSizeCall> for PoolCalls {
        fn from(value: InitializeWithBufferSizeCall) -> Self {
            Self::InitializeWithBufferSize(value)
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
    impl ::core::convert::From<SetQBTCContractCall> for PoolCalls {
        fn from(value: SetQBTCContractCall) -> Self {
            Self::SetQBTCContract(value)
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
        pub previous_block_hash: [u8; 32],
        pub merkle_root_hash: [u8; 32],
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
    pub struct ConfirmationsReturn(pub u8);
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
    ///Container type for all return fields from the `distributeRewards` function with signature `distributeRewards(((uint32,bytes32,bytes32,uint32,uint32,uint32),bytes32,bytes8))` and selector `0xed723cdf`
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
        pub tip: ChainTip,
    }
    ///Container type for all return fields from the `getOneHundred` function with signature `getOneHundred()` and selector `0xffbe5d5a`
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
    pub struct GetOneHundredReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `submitBlock` function with signature `submitBlock(((uint32,bytes32,bytes32,uint32,uint32,uint32),bytes32,bytes8),bytes32[],address)` and selector `0xfa4c7ad2`
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
}
