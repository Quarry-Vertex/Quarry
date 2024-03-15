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
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x16Wa\x18)\x90\x81a\0\x1C\x829\xF3[`\0\x80\xFD\xFE`@`\x80\x81R`\x04\x90\x816\x10\x15a\0\x15W`\0\x80\xFD[`\0\x805`\xE0\x1C\x80b\xEAw\x08\x14a\x10\x80W\x80c\"3\xAA\xAF\x14a\n]W\x80c+\xBB\x06C\x14a\x0F\xF3W\x80c,\xF8! \x14a\x0E1W\x80cG\x88W\x81\x14a\t!W\x80c[}\x7F\xA6\x14a\x0E\rW\x80chE`\xA2\x14a\x0B\x81W\x80cqP\x18\xA6\x14a\x0B\x17W\x80c\x8D\xA5\xCB[\x14a\n\xE1W\x80c\x92W\xF4\xDF\x14a\n\x99W\x80c\x9C,w\x0B\x14a\n|W\x80c\x9EK\x0F\x8E\x14a\n]W\x80c\x9FD\x1DT\x14a\n/W\x80c\xA2 \xE2r\x14a\t\xEFW\x80c\xA5g\xC9\xEC\x14a\t\xCBW\x80c\xB7j\"<\x14a\t\x8BW\x80c\xB8\x14n\x87\x14a\tTW\x80c\xBB\xB4\x82\xB6\x14a\t!W\x80c\xBE\x9AeU\x14a\t\x02W\x80c\xC8\t\xB4[\x14a\x08\xE2W\x80c\xD5R6`\x14a\x08\xB5W\x80c\xD9\xC9\x86\x1D\x14a\x08\x93W\x80c\xEC\x95\xBF\xE7\x14a\x08hW\x80c\xEDr<\xDF\x14a\x063W\x80c\xEF\xBE\x1C\x1C\x14a\x06\x14W\x80c\xF2\xFD\xE3\x8B\x14a\x05\xE7W\x80c\xF4Fh}\x14a\x05\x86Wc\xFALz\xD2\x14a\x01SW`\0\x80\xFD[4a\x05\x83Wa\x01@6`\x03\x19\x01\x12a\x05\x83Wa\x01n6a\x128V[\x90a\x01\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x05\x7FWa\x01\x8F\x906\x90\x86\x01a\x11TV[`\x01`\x01`\xA0\x1B\x03a\x01$5\x81\x81\x16\x96\x92\x94\x90\x93\x90\x92\x87\x85\x03a\x05\x7FW\x86\x84Q\x01Q\x95\x86\x83R` \x98`\x0E\x8AR\x84\x89\x85 T\x16\x14a\x05<W\x86\x83R`\x0F\x89R`\xFF\x88\x84 T\x16a\x04\xEBWa\x01\xEDc\xFF\xFF\xFF\xFF`\x80\x87Q\x01Q\x16a\x15\xD6V[a\x01\xFC`\x05T`\x06T\x90a\x14!V[\x11\x15a\x04\xA8W\x88\x85Q\x01Q`\x11T\x03a\x04eW\x88\x85\x01Q`\x08T\x03a\x03\xFCW\x90a\x02)\x87a\x02\x8D\x93a\x14_V[P\x86\x83R`\x0F\x89R\x87\x83 \x93`\xFF\x19\x94`\x01\x86\x82T\x16\x17\x90U\x89\x81`\tT\x16`\nT\x90a\x02U\x82a\x13\x13V[`\nU\x8BQc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x16\x86\x82\x01\x90\x81R` \x81\x01\x93\x90\x93R\x95\x86\x92\x83\x91\x89\x91\x83\x91`@\x90\x91\x01\x90V[\x03\x92Z\xF1\x92\x83\x15a\x03\xF2W\x90\x84\x93\x92\x91\x8A\x97\x96\x95\x93a\x03\xB2W[P`\x01T\x84T\x14a\x03\x15W[PP\x90\x7F=j\xC0\xE1\x17\xAAG\xA8.\x86\x9D.\xC9\xAF$\xBA\xDFS\xD2\xBF\x1Dm\t\xA0\xEBH\x9E\xEC\xC82\x0Cx\x96a\x02\xE2\x86\x93a\x138V[\x81R`\x0C\x89R \x80T\x91\x90\x91\x16\x90UQ\x01Q\x83Q\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16` \x82\x01R`@\x90\xA1Q`\x01\x81R\xF3[\x90\x91\x92\x94\x96\x97\x93\x95Pa\x03&a\x16=V[\x90`\tT\x16\x91\x82;\x15a\x03\xAEW\x90`$\x86\x92\x83\x8CQ\x95\x86\x94\x85\x93c\x08R\xCD\x8D`\xE3\x1B\x85R\x84\x01RZ\xF1\x80\x15a\x03\xA4W\x92a\x02\xE2\x89\x96\x94\x93\x7F=j\xC0\xE1\x17\xAAG\xA8.\x86\x9D.\xC9\xAF$\xBA\xDFS\xD2\xBF\x1Dm\t\xA0\xEBH\x9E\xEC\xC82\x0Cx\x99\x98\x96\x93\x88\x95a\x03\x95W[P\x92\x93P\x81\x98Pa\x02\xB3V[a\x03\x9E\x90a\x10\xEAV[8a\x03\x89V[\x88Q=\x86\x82>=\x90\xFD[\x85\x80\xFD[\x8B\x80\x92\x95\x96\x97\x98P\x81\x93\x94P=\x83\x11a\x03\xEBW[a\x03\xD0\x81\x83a\x11\x1AV[\x81\x01\x03\x12a\x03\xE7W\x90\x88\x95\x94\x93\x92\x91Q\x918a\x02\xA7V[\x83\x80\xFD[P=a\x03\xC6V[\x89Q=\x86\x82>=\x90\xFD[\x87QbF\x1B\xCD`\xE5\x1B\x81R\x80\x83\x01\x8A\x90R`<`$\x82\x01R\x7FCoinbase transaction does not po`D\x82\x01R\x7Fint to quarry peg in address\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x87QbF\x1B\xCD`\xE5\x1B\x81R\x80\x83\x01\x8A\x90R`\x18`$\x82\x01R\x7FSubmitted block is stale\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x87QbF\x1B\xCD`\xE5\x1B\x81R\x80\x83\x01\x8A\x90R`\x17`$\x82\x01R\x7FPool difficulty not met\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x87QbF\x1B\xCD`\xE5\x1B\x81R\x80\x83\x01\x8A\x90R`%`$\x82\x01R\x7FBlock hash has already been subm`D\x82\x01Rd\x1A]\x1D\x19Y`\xDA\x1B`d\x82\x01R`\x84\x90\xFD[\x87QbF\x1B\xCD`\xE5\x1B\x81R\x80\x83\x01\x8A\x90R`\x1D`$\x82\x01R\x7FAddress doesn't match account\0\0\0`D\x82\x01R`d\x90\xFD[P\x80\xFD[\x80\xFD[P4a\x05\x83W\x80`\x03\x196\x01\x12a\x05\x83W` \x82Qa\x05\xA4\x81a\x10\xB8V[\x82\x81R\x01Ra\x05\xBE`\x01\x80`\xA0\x1B\x03`\x07T\x163\x14a\x13\xC0V[\x80Qa\x05\xC9\x81a\x10\xB8V[`\x10T\x90\x81\x81R` `\x11T\x91\x01\x90\x81R\x82Q\x91\x82RQ` \x82\x01R\xF3[P4a\x05\x83W` 6`\x03\x19\x01\x12a\x05\x83Wa\x06\x11a\x06\x04a\x10\x9DV[a\x06\x0Ca\x17YV[a\x16\xE5V[\x80\xF3[P\x904a\x05\x7FW\x81`\x03\x196\x01\x12a\x05\x7FW` \x90`\x03T\x90Q\x90\x81R\xF3[P\x914a\x08dWa\x01\x006`\x03\x19\x01\x12a\x08dWa\x06P6a\x128V[\x90\x82\x82Q\x01Q\x84R` \x93`\x0C\x85R`\x06`\xFF\x85\x83 T\x16\x10\x15a\x08!W\x84\x91\x84\x91`\x01\x80a\x06\x87`\x01T\x86\x89\x01Q`\xC0\x1Ca\x15\xCCV[\x91[a\x06\xC4W[PPPP\x7FV\xAD\xDA\xAAi\xE1\xD6P\xAF\xE8J\x8E'\x8D|\xE6h5z\x05\xF6?iFKr\xEE\xA8r\xFD\x80\x10\x92Q\x01Q\x83Q\x90\x81R\xA1Q`\x01\x81R\xF3[\x91\x93\x82\x95\x91\x93\x95T\x15a\x08\x18WPPa\x06\xDBa\x16=V[\x87`\x01\x80`\xA0\x1B\x03`\t\x81\x81T\x16\x8AQ\x93\x84\x80\x92c1\xA9\x10\x8F`\xE1\x1B\x82R\x87\x8C\x83\x01R`$\x96\x87\x91Z\xFA\x90\x81\x15a\x07\xD0W\x88\x91a\x07\xDEW[P\x82`\x0BT\x16\x90\x81;\x15a\x07\xDAW\x8BQc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81\x8B\x01\x90\x81R` \x81\x01\x89\x90R\x90\x91\x89\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x07\xD0Wa\x07\xC1W[PT\x16\x91\x82;\x15a\x03\xAEW\x88Qc\x08R\xCD\x8D`\xE3\x1B\x81R\x87\x81\x01\x91\x90\x91R\x91\x85\x91\x83\x91\x82\x90\x84\x90Z\xF1\x80\x15a\x07\xB7W\x90\x82\x91a\x07\xA8W[P\x91\x87\x94\x92\x87\x94\x92a\x06\x89V[a\x07\xB1\x90a\x10\xEAV[8a\x07\x9BV[\x87Q=\x86\x82>=\x90\xFD[a\x07\xCA\x90a\x10\xEAV[8a\x07dV[\x8BQ=\x8A\x82>=\x90\xFD[\x88\x80\xFD[\x90P\x8B\x81\x81=\x83\x11a\x08\x11W[a\x07\xF5\x81\x83a\x11\x1AV[\x81\x01\x03\x12a\x08\rWQ\x82\x81\x16\x81\x03a\x08\rW8a\x07\x13V[\x87\x80\xFD[P=a\x07\xEBV[\x93\x81\x95Pa\x06\x8EV[\x83QbF\x1B\xCD`\xE5\x1B\x81R\x80\x83\x01\x86\x90R`\x1C`$\x82\x01R\x7FDo not have 6+ confirmations\0\0\0\0`D\x82\x01R`d\x90\xFD[\x82\x80\xFD[P\x914a\x08dW` 6`\x03\x19\x01\x12a\x08dW\x81` \x93`\xFF\x925\x81R`\x0C\x85R T\x16\x90Q\x90\x81R\xF3[P\x904a\x05\x7FW\x81`\x03\x196\x01\x12a\x05\x7FW` \x91`\x01T\x90T\x14\x90Q\x90\x81R\xF3[P\x914a\x08dW` 6`\x03\x19\x01\x12a\x08dW\x81` \x93`\xFF\x925\x81R`\x0F\x85R T\x16\x90Q\x90\x15\x15\x81R\xF3[P\x904a\x05\x7FW\x81`\x03\x196\x01\x12a\x05\x7FW` \x90`\x01T\x15\x90Q\x90\x81R\xF3[P\x904a\x05\x7FW\x81`\x03\x196\x01\x12a\x05\x7FW` \x90`\x02T\x90Q\x90\x81R\xF3[P\x914a\x08dW` 6`\x03\x19\x01\x12a\x08dW5\x82R`\x0E` \x90\x81R\x91\x81\x90 T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\xF3[P4a\x05\x83W` 6`\x03\x19\x01\x12a\x05\x83W\x825\x92T\x83\x10\x15a\x05\x83WPa\t}` \x92a\x12\x01V[\x91\x90T\x90Q\x91`\x03\x1B\x1C\x81R\xF3[P4a\x05\x83W` 6`\x03\x19\x01\x12a\x05\x83Wa\t\xA5a\x10\x9DV[a\t\xADa\x17YV[`\x01\x80`\xA0\x1B\x03\x16`\x01`\x01``\x1B\x03`\xA0\x1B`\x0BT\x16\x17`\x0BU\x80\xF3[P\x904a\x05\x7FW\x81`\x03\x196\x01\x12a\x05\x7FW` \x90a\t\xE8a\x16=V[\x90Q\x90\x81R\xF3[P4a\x05\x83W` 6`\x03\x19\x01\x12a\x05\x83Wa\n\ta\x10\x9DV[a\n\x11a\x17YV[`\x01\x80`\xA0\x1B\x03\x16`\x01`\x01``\x1B\x03`\xA0\x1B`\tT\x16\x17`\tU\x80\xF3[P\x914a\x08dW` 6`\x03\x19\x01\x12a\x08dW5\x91c\xFF\xFF\xFF\xFF\x83\x16\x83\x03a\x05\x83WPa\t\xE8` \x92a\x15\xD6V[P\x904a\x05\x7FW\x81`\x03\x196\x01\x12a\x05\x7FW` \x90`\x01T\x90Q\x90\x81R\xF3[P\x904a\x05\x7FW\x81`\x03\x196\x01\x12a\x05\x7FW` \x91T\x90Q\x90\x81R\xF3[P4a\x05\x83W\x81`\x03\x196\x01\x12a\x05\x83W\x825\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x05\x83WPa\n\xCF` \x93a\n\xD8\x926\x91\x01a\x11TV[`$5\x90a\x14_V[\x90Q\x90\x15\x15\x81R\xF3[P\x904a\x05\x7FW\x81`\x03\x196\x01\x12a\x05\x7FW`\0\x80Q` a\x17\xD4\x839\x81Q\x91RT\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x05\x83W\x80`\x03\x196\x01\x12a\x05\x83Wa\x0B0a\x17YV[`\0\x80Q` a\x17\xD4\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[P\x914a\x08dW``6`\x03\x19\x01\x12a\x08dWa\x0B\x9Ca\x10\x9DV[\x90`D5\x90\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x92\x83T\x92`\xFF\x84\x87\x1C\x16\x15\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x86\x16\x95\x86\x15\x80a\x0E\x06W[`\x01\x80\x98\x14\x90\x81a\r\xFCW[\x15\x90\x81a\r\xF3W[Pa\r\xE3Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x87\x17\x88U\x85a\r\xC4W[Pa\x0C\x1Ca\x17\x92V[a\x0C$a\x17\x92V[a\x0C-3a\x16\xE5V[d\x02T\x0B\xE4\0`\x06Ui\x01z\xA5\xBF\xB9.\xE0\x1D4\0`\x05U\x80\x89Ua\x0Cha\x0CS\x82a\x11<V[\x91a\x0C`\x8AQ\x93\x84a\x11\x1AV[\x80\x83Ra\x11<V[` \x91\x80\x83\x01\x91`\x1F\x19\x016\x837Q\x93\x84\x11a\r\xB1W`\x01`@\x1B\x84\x11a\r\xB1W\x82T\x84\x84U\x80\x85\x10a\roW[P\x91\x89R\x85\x89[\x84\x81\x10a\r=WP\x91PP\x87\x91PU\x85`\x02U\x85`\x03U`\x01\x80`\xA0\x1B\x03\x16`\x01`\x01``\x1B\x03`\xA0\x1B`\x07T\x16\x17`\x07U`$5`\x08U\x84` \x85Qa\x0C\xE3\x81a\x10\xB8V[\x82\x81R\x01R\x84`\x10U\x84`\x11U\x84`\nUa\x0C\xFCW\x83\x80\xF3[\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x92` \x92h\xFF\0\0\0\0\0\0\0\0\x19\x81T\x16\x90UQ\x90\x81R\xA18\x80\x80\x83\x80\xF3[\x82\x84Q\x94\x01\x93\x81\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x01U\x01\x86\x90a\x0C\x9DV[\x87\x85\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x92\x83\x01\x92\x01[\x82\x81\x10a\r\xA6WPPa\x0C\x96V[\x8C\x81U\x01\x88\x90a\r\x98V[cNH{q`\xE0\x1B\x8AR`A\x83R`$\x8A\xFD[h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16h\x01\0\0\0\0\0\0\0\x01\x17\x87U8a\x0C\x13V[\x88Qc\xF9.\xE8\xA9`\xE0\x1B\x81R\x83\x90\xFD[\x90P\x158a\x0B\xF8V[0;\x15\x91Pa\x0B\xF0V[P\x85a\x0B\xE4V[P4a\x05\x83W\x80`\x03\x196\x01\x12a\x05\x83WP`\x10T`\x11T\x82Q\x91\x82R` \x82\x01R\xF3[P4a\x05\x83W\x81`\x03\x196\x01\x12a\x05\x83W\x81Q\x90a\x0EN\x82a\x10\xB8V[\x835\x82R`$\x91` \x93` \x82\x01\x94`$5\x86Ra\x0Ew`\x01\x80`\xA0\x1B\x03`\x07T\x163\x14a\x13\xC0V[`\x11\x92`\x11T\x80a\x0FkW[PQ`\x10U\x7FG\x81\x1A\xEB\xD2\x8C\xBA\x7F\x02U\xE7\xF5U\x8C]\x8F\x15o\x87\x98H\x11\xE4B\x8B\x95IH\x0B\xE4\xBE$` \x87Q\x80`\x11U\x84Q\x90\x81R\xA1\x83[`\rT\x81\x10\x15a\x0F\x13Wa\x0E\xCC\x81a\x11\xB4V[\x90T\x90`\x03\x1B\x1C\x85R`\x0C\x82R\x82\x85 \x80T`\xFF\x80\x82\x16\x81\x81\x14a\x0F\x02W`\xFF\x19\x90\x92\x16`\x01\x92\x83\x01\x90\x91\x16\x17\x90\x91U\x01a\x0E\xB9V[cNH{q`\xE0\x1B\x89R\x87\x8CR\x89\x89\xFD[\x84\x88\x88Q\x90`\rT\x90`\x01`@\x1B\x82\x10\x15a\x0FXWPa\x0F<\x81`\x01a\x0FS\x93\x01`\rUa\x11\xB4V[\x81\x93\x91T\x90`\x03\x1B\x91\x82\x1B\x91`\0\x19\x90\x1B\x19\x16\x17\x90V[\x90U\x80\xF3[cNH{q`\xE0\x1B\x84R`A\x90R`$\x83\xFD[\x81Q\x03a\x0FxW8a\x0E\x83V[\x82QbF\x1B\xCD`\xE5\x1B\x81R` \x81\x8A\x01R`I`$\x82\x01R\x7FNew chain tip prev block hash do`D\x82\x01R\x7Fes not match current chain tip b`d\x82\x01Rh\r\x8D\xECmd\r\x0C.m`\xBB\x1B`\x84\x82\x01R`\xA4\x90\xFD[P\x914a\x08dW\x81`\x03\x196\x01\x12a\x08dW`$5\x905`\x01`\x01`\xA0\x1B\x03\x82\x16\x80\x83\x03a\x10|W\x81\x85R`\x0E` \x90\x81R\x84\x86 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x90\x92\x17\x90\x91U\x92Q\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x91\x81\x01\x91\x90\x91R\x7FP\xD2\x7F\xCE\xED\xD1M\xADe\x8A\xD3\xE3\x94\x160Y\xB4\xB8\xCE\x97\x1E\xD4\x86\t\xF5F\x9Es\xB6\xD3\xFA\x84\x90`@\x90\xA1\x80\xF3[\x84\x80\xFD[P\x824a\x05\x7FW` 6`\x03\x19\x01\x12a\x05\x7FWa\x06\x11\x905a\x138V[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x10\xB3WV[`\0\x80\xFD[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x10\xD4W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x10\xD4W`@RV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x10\xD4W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x10\xD4W`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x10\xD4W`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x10\xB3W` \x90\x825a\x11n\x81a\x11<V[\x93a\x11|`@Q\x95\x86a\x11\x1AV[\x81\x85R` \x80\x86\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x10\xB3W` \x01\x90[\x82\x82\x10a\x11\xA5WPPPP\x90V[\x815\x81R\x90\x83\x01\x90\x83\x01a\x11\x97V[`\rT\x81\x10\x15a\x11\xEBW`\r`\0R\x7F\xD7\xB6\x99\x01\x05q\x91\x01\xDA\xBE\xB7qD\xF2\xA38\\\x803\xAC\xD3\xAF\x97\xE9B:i^\x81\xAD\x1E\xB5\x01\x90`\0\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\x04T\x81\x10\x15a\x11\xEBW`\x04`\0R\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x01\x90`\0\x90V[`\x03\x19\x01\x90a\x01\0\x82\x12a\x10\xB3W`@Qa\x12R\x81a\x10\xFEV[`\xC0\x81\x93\x12a\x10\xB3W`@Q`\xC0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x10\xD4W`@Rc\xFF\xFF\xFF\xFF`\x045\x81\x81\x16\x81\x03a\x10\xB3W\x82R`$5` \x83\x01R`D5`@\x83\x01R`d5\x81\x81\x16\x81\x03a\x10\xB3W``\x83\x01R`\x845\x81\x81\x16\x81\x03a\x10\xB3W`\x80\x83\x01R`\xA45\x90\x81\x16\x81\x03a\x10\xB3W`\xA0\x82\x01R\x81R`\xC45` \x82\x01R`\xE45\x90`\x01`\x01`\xC0\x1B\x03\x19\x82\x16\x82\x03a\x10\xB3W`@\x01RV[\x81\x15a\x12\xFDW\x06\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x19\x81\x14a\x13\"W`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01T`\0T\x14a\x13\xB2W[\x80a\x13Sa\x0F<`\x03Ta\x12\x01V[\x90U\x7Fs\xCB\0<j\xB2L\xDE\xC6\x1C\xA1\xF33S7\xAB\xD9'\xCFg\tv\x99,\xEC\xB2\x8D\xED\x13q\xB7\x94`@`\x03T\x92\x81Q\x90\x81R\x83` \x82\x01R\xA1`\x01\x81\x01\x80\x91\x11a\x13\"W`\0Ta\x13\x9F\x91a\x12\xF3V[`\x03Ua\x13\xAD`\x01Ta\x13\x13V[`\x01UV[a\x13\xBAa\x16=V[Pa\x13DV[\x15a\x13\xC7WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FOnly the chainTipOracle can call`D\x82\x01Rk\x08\x1D\x1A\x1A\\\xC8\x1BY]\x1A\x1B\xD9`\xA2\x1B`d\x82\x01R`\x84\x90\xFD[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x13\"WV[\x90\x81Q\x91`\0[\x83\x81\x10a\x14LWPP\x01`\0\x81R\x90V[\x80` \x80\x92\x84\x01\x01Q\x81\x85\x01R\x01a\x14;V[\x80Q\x15a\x11\xEBW` \x90\x81\x81\x01Q\x93`\x01\x94[\x82Q\x86\x10\x15a\x15\x83W`\x05\x86\x90\x1B\x83\x01\x84\x01Q\x84\x91\x81\x81\x10\x15a\x15\x1AW`@\x91\x82Q\x91\x84\x83\x01R\x82\x82\x01R\x81\x81Ra\x14\xA9\x81a\x10\xFEV[\x81Q\x92\x83\x91\x82a\x14\xBC`\0\x96\x87\x93a\x144V[\x03`\x02\x93\x84Z\xFA\x15a\x15\x0FW\x82\x86\x91a\x14\xEE\x82Q\x85Q\x90\x85\x82\x01R\x84\x81Ra\x14\xE3\x81a\x10\xB8V[\x85Q\x91\x82\x80\x92a\x144V[\x03\x91Z\xFA\x15a\x15\x05WP`\x01\x90Q\x95[\x01\x94a\x14rV[Q\x90=\x90\x82>=\x90\xFD[PQ\x90=\x90\x82>=\x90\xFD[\x90`@\x91\x82Q\x91\x84\x83\x01R\x82\x82\x01R\x81\x81Ra\x155\x81a\x10\xFEV[\x81Q\x92\x83\x91\x82a\x15H`\0\x96\x87\x93a\x144V[\x03`\x02\x93\x84Z\xFA\x15a\x15\x0FW\x82\x86\x91a\x15o\x82Q\x85Q\x90\x85\x82\x01R\x84\x81Ra\x14\xE3\x81a\x10\xB8V[\x03\x91Z\xFA\x15a\x15\x05WP`\x01\x90Q\x95a\x14\xFEV[\x91P\x93P\x91\x90\x91\x03a\x15\x95WP`\x01\x90V[`d\x90`@Q\x90bF\x1B\xCD`\xE5\x1B\x82R`\x04\x82\x01R`\x10`$\x82\x01Ro\x14\xD4\x15\x88\x1C\x1C\x9B\xDB\xD9\x88\x19\x98Z[\x19Y`\x82\x1B`D\x82\x01R\xFD[\x81\x15a\x12\xFDW\x04\x90V[`\xFF\x81`\x18\x1C\x16`\x1D\x03c\xFF\xFF\xFF\xFF\x81\x11a\x13\"W`\x03\x1B\x90d\x07\xFF\xFF\xFF\xF8c\xFF\xFF\xFF\xF8\x83\x16\x92\x16\x82\x03a\x13\"W`\x06Ta\xFF\xFF\x90\x80\x82\x02\x91\x82\x04\x03a\x13\"Wb\xFF\xFF\xFFa\x16%\x92\x16\x90a\x15\xCCV[`\xFF\x82\x11a\x13\"W`\x01a\x16:\x92\x1B\x90a\x14!V[\x90V[`\x01T\x80\x15a\x16\xAEW`\x02T\x90a\x16S\x82a\x12\x01V[\x90T\x90`\x03\x1B\x1C\x91\x7F\xC0\x81\x7F\xF9\xCE!\xEC\xED?kg\x0E5\x13\xACj\x0C\xDF\x83\xD1\x96\xCAk\x1C\xFF\x9F\n\x0E\xC1\xD2\xDF\xD5`@\x80Q\x85\x81R\x83` \x82\x01R\xA1`\x01\x81\x01\x80\x91\x11a\x13\"W`\0Ta\x16\xA1\x91a\x12\xF3V[`\x02U`\0\x19\x01`\x01U\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01RnBuffer is empty`\x88\x1B`D\x82\x01R`d\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x81\x15a\x17@W`\0\x80Q` a\x17\xD4\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x84\x17\x90\x91U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`\0\x80\xA3V[`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x90\xFD[`\0\x80Q` a\x17\xD4\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x163\x03a\x17zWV[`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x90\xFD[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15a\x17\xC1WV[`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x90\xFD\xFE\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\xA2dipfsX\"\x12 \xAEd\xE1/\x91#\xFA\x04\xEE\x11`\xB9R\xD3W\xC9\xFAf\tg\xE6\x1C\xCD\xD2\xEE\x85\x1E\xB3\xA0\xE7B]dsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static POOL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`@`\x80\x81R`\x04\x90\x816\x10\x15a\0\x15W`\0\x80\xFD[`\0\x805`\xE0\x1C\x80b\xEAw\x08\x14a\x10\x80W\x80c\"3\xAA\xAF\x14a\n]W\x80c+\xBB\x06C\x14a\x0F\xF3W\x80c,\xF8! \x14a\x0E1W\x80cG\x88W\x81\x14a\t!W\x80c[}\x7F\xA6\x14a\x0E\rW\x80chE`\xA2\x14a\x0B\x81W\x80cqP\x18\xA6\x14a\x0B\x17W\x80c\x8D\xA5\xCB[\x14a\n\xE1W\x80c\x92W\xF4\xDF\x14a\n\x99W\x80c\x9C,w\x0B\x14a\n|W\x80c\x9EK\x0F\x8E\x14a\n]W\x80c\x9FD\x1DT\x14a\n/W\x80c\xA2 \xE2r\x14a\t\xEFW\x80c\xA5g\xC9\xEC\x14a\t\xCBW\x80c\xB7j\"<\x14a\t\x8BW\x80c\xB8\x14n\x87\x14a\tTW\x80c\xBB\xB4\x82\xB6\x14a\t!W\x80c\xBE\x9AeU\x14a\t\x02W\x80c\xC8\t\xB4[\x14a\x08\xE2W\x80c\xD5R6`\x14a\x08\xB5W\x80c\xD9\xC9\x86\x1D\x14a\x08\x93W\x80c\xEC\x95\xBF\xE7\x14a\x08hW\x80c\xEDr<\xDF\x14a\x063W\x80c\xEF\xBE\x1C\x1C\x14a\x06\x14W\x80c\xF2\xFD\xE3\x8B\x14a\x05\xE7W\x80c\xF4Fh}\x14a\x05\x86Wc\xFALz\xD2\x14a\x01SW`\0\x80\xFD[4a\x05\x83Wa\x01@6`\x03\x19\x01\x12a\x05\x83Wa\x01n6a\x128V[\x90a\x01\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x05\x7FWa\x01\x8F\x906\x90\x86\x01a\x11TV[`\x01`\x01`\xA0\x1B\x03a\x01$5\x81\x81\x16\x96\x92\x94\x90\x93\x90\x92\x87\x85\x03a\x05\x7FW\x86\x84Q\x01Q\x95\x86\x83R` \x98`\x0E\x8AR\x84\x89\x85 T\x16\x14a\x05<W\x86\x83R`\x0F\x89R`\xFF\x88\x84 T\x16a\x04\xEBWa\x01\xEDc\xFF\xFF\xFF\xFF`\x80\x87Q\x01Q\x16a\x15\xD6V[a\x01\xFC`\x05T`\x06T\x90a\x14!V[\x11\x15a\x04\xA8W\x88\x85Q\x01Q`\x11T\x03a\x04eW\x88\x85\x01Q`\x08T\x03a\x03\xFCW\x90a\x02)\x87a\x02\x8D\x93a\x14_V[P\x86\x83R`\x0F\x89R\x87\x83 \x93`\xFF\x19\x94`\x01\x86\x82T\x16\x17\x90U\x89\x81`\tT\x16`\nT\x90a\x02U\x82a\x13\x13V[`\nU\x8BQc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x16\x86\x82\x01\x90\x81R` \x81\x01\x93\x90\x93R\x95\x86\x92\x83\x91\x89\x91\x83\x91`@\x90\x91\x01\x90V[\x03\x92Z\xF1\x92\x83\x15a\x03\xF2W\x90\x84\x93\x92\x91\x8A\x97\x96\x95\x93a\x03\xB2W[P`\x01T\x84T\x14a\x03\x15W[PP\x90\x7F=j\xC0\xE1\x17\xAAG\xA8.\x86\x9D.\xC9\xAF$\xBA\xDFS\xD2\xBF\x1Dm\t\xA0\xEBH\x9E\xEC\xC82\x0Cx\x96a\x02\xE2\x86\x93a\x138V[\x81R`\x0C\x89R \x80T\x91\x90\x91\x16\x90UQ\x01Q\x83Q\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16` \x82\x01R`@\x90\xA1Q`\x01\x81R\xF3[\x90\x91\x92\x94\x96\x97\x93\x95Pa\x03&a\x16=V[\x90`\tT\x16\x91\x82;\x15a\x03\xAEW\x90`$\x86\x92\x83\x8CQ\x95\x86\x94\x85\x93c\x08R\xCD\x8D`\xE3\x1B\x85R\x84\x01RZ\xF1\x80\x15a\x03\xA4W\x92a\x02\xE2\x89\x96\x94\x93\x7F=j\xC0\xE1\x17\xAAG\xA8.\x86\x9D.\xC9\xAF$\xBA\xDFS\xD2\xBF\x1Dm\t\xA0\xEBH\x9E\xEC\xC82\x0Cx\x99\x98\x96\x93\x88\x95a\x03\x95W[P\x92\x93P\x81\x98Pa\x02\xB3V[a\x03\x9E\x90a\x10\xEAV[8a\x03\x89V[\x88Q=\x86\x82>=\x90\xFD[\x85\x80\xFD[\x8B\x80\x92\x95\x96\x97\x98P\x81\x93\x94P=\x83\x11a\x03\xEBW[a\x03\xD0\x81\x83a\x11\x1AV[\x81\x01\x03\x12a\x03\xE7W\x90\x88\x95\x94\x93\x92\x91Q\x918a\x02\xA7V[\x83\x80\xFD[P=a\x03\xC6V[\x89Q=\x86\x82>=\x90\xFD[\x87QbF\x1B\xCD`\xE5\x1B\x81R\x80\x83\x01\x8A\x90R`<`$\x82\x01R\x7FCoinbase transaction does not po`D\x82\x01R\x7Fint to quarry peg in address\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x87QbF\x1B\xCD`\xE5\x1B\x81R\x80\x83\x01\x8A\x90R`\x18`$\x82\x01R\x7FSubmitted block is stale\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x87QbF\x1B\xCD`\xE5\x1B\x81R\x80\x83\x01\x8A\x90R`\x17`$\x82\x01R\x7FPool difficulty not met\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x87QbF\x1B\xCD`\xE5\x1B\x81R\x80\x83\x01\x8A\x90R`%`$\x82\x01R\x7FBlock hash has already been subm`D\x82\x01Rd\x1A]\x1D\x19Y`\xDA\x1B`d\x82\x01R`\x84\x90\xFD[\x87QbF\x1B\xCD`\xE5\x1B\x81R\x80\x83\x01\x8A\x90R`\x1D`$\x82\x01R\x7FAddress doesn't match account\0\0\0`D\x82\x01R`d\x90\xFD[P\x80\xFD[\x80\xFD[P4a\x05\x83W\x80`\x03\x196\x01\x12a\x05\x83W` \x82Qa\x05\xA4\x81a\x10\xB8V[\x82\x81R\x01Ra\x05\xBE`\x01\x80`\xA0\x1B\x03`\x07T\x163\x14a\x13\xC0V[\x80Qa\x05\xC9\x81a\x10\xB8V[`\x10T\x90\x81\x81R` `\x11T\x91\x01\x90\x81R\x82Q\x91\x82RQ` \x82\x01R\xF3[P4a\x05\x83W` 6`\x03\x19\x01\x12a\x05\x83Wa\x06\x11a\x06\x04a\x10\x9DV[a\x06\x0Ca\x17YV[a\x16\xE5V[\x80\xF3[P\x904a\x05\x7FW\x81`\x03\x196\x01\x12a\x05\x7FW` \x90`\x03T\x90Q\x90\x81R\xF3[P\x914a\x08dWa\x01\x006`\x03\x19\x01\x12a\x08dWa\x06P6a\x128V[\x90\x82\x82Q\x01Q\x84R` \x93`\x0C\x85R`\x06`\xFF\x85\x83 T\x16\x10\x15a\x08!W\x84\x91\x84\x91`\x01\x80a\x06\x87`\x01T\x86\x89\x01Q`\xC0\x1Ca\x15\xCCV[\x91[a\x06\xC4W[PPPP\x7FV\xAD\xDA\xAAi\xE1\xD6P\xAF\xE8J\x8E'\x8D|\xE6h5z\x05\xF6?iFKr\xEE\xA8r\xFD\x80\x10\x92Q\x01Q\x83Q\x90\x81R\xA1Q`\x01\x81R\xF3[\x91\x93\x82\x95\x91\x93\x95T\x15a\x08\x18WPPa\x06\xDBa\x16=V[\x87`\x01\x80`\xA0\x1B\x03`\t\x81\x81T\x16\x8AQ\x93\x84\x80\x92c1\xA9\x10\x8F`\xE1\x1B\x82R\x87\x8C\x83\x01R`$\x96\x87\x91Z\xFA\x90\x81\x15a\x07\xD0W\x88\x91a\x07\xDEW[P\x82`\x0BT\x16\x90\x81;\x15a\x07\xDAW\x8BQc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81\x8B\x01\x90\x81R` \x81\x01\x89\x90R\x90\x91\x89\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x07\xD0Wa\x07\xC1W[PT\x16\x91\x82;\x15a\x03\xAEW\x88Qc\x08R\xCD\x8D`\xE3\x1B\x81R\x87\x81\x01\x91\x90\x91R\x91\x85\x91\x83\x91\x82\x90\x84\x90Z\xF1\x80\x15a\x07\xB7W\x90\x82\x91a\x07\xA8W[P\x91\x87\x94\x92\x87\x94\x92a\x06\x89V[a\x07\xB1\x90a\x10\xEAV[8a\x07\x9BV[\x87Q=\x86\x82>=\x90\xFD[a\x07\xCA\x90a\x10\xEAV[8a\x07dV[\x8BQ=\x8A\x82>=\x90\xFD[\x88\x80\xFD[\x90P\x8B\x81\x81=\x83\x11a\x08\x11W[a\x07\xF5\x81\x83a\x11\x1AV[\x81\x01\x03\x12a\x08\rWQ\x82\x81\x16\x81\x03a\x08\rW8a\x07\x13V[\x87\x80\xFD[P=a\x07\xEBV[\x93\x81\x95Pa\x06\x8EV[\x83QbF\x1B\xCD`\xE5\x1B\x81R\x80\x83\x01\x86\x90R`\x1C`$\x82\x01R\x7FDo not have 6+ confirmations\0\0\0\0`D\x82\x01R`d\x90\xFD[\x82\x80\xFD[P\x914a\x08dW` 6`\x03\x19\x01\x12a\x08dW\x81` \x93`\xFF\x925\x81R`\x0C\x85R T\x16\x90Q\x90\x81R\xF3[P\x904a\x05\x7FW\x81`\x03\x196\x01\x12a\x05\x7FW` \x91`\x01T\x90T\x14\x90Q\x90\x81R\xF3[P\x914a\x08dW` 6`\x03\x19\x01\x12a\x08dW\x81` \x93`\xFF\x925\x81R`\x0F\x85R T\x16\x90Q\x90\x15\x15\x81R\xF3[P\x904a\x05\x7FW\x81`\x03\x196\x01\x12a\x05\x7FW` \x90`\x01T\x15\x90Q\x90\x81R\xF3[P\x904a\x05\x7FW\x81`\x03\x196\x01\x12a\x05\x7FW` \x90`\x02T\x90Q\x90\x81R\xF3[P\x914a\x08dW` 6`\x03\x19\x01\x12a\x08dW5\x82R`\x0E` \x90\x81R\x91\x81\x90 T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\xF3[P4a\x05\x83W` 6`\x03\x19\x01\x12a\x05\x83W\x825\x92T\x83\x10\x15a\x05\x83WPa\t}` \x92a\x12\x01V[\x91\x90T\x90Q\x91`\x03\x1B\x1C\x81R\xF3[P4a\x05\x83W` 6`\x03\x19\x01\x12a\x05\x83Wa\t\xA5a\x10\x9DV[a\t\xADa\x17YV[`\x01\x80`\xA0\x1B\x03\x16`\x01`\x01``\x1B\x03`\xA0\x1B`\x0BT\x16\x17`\x0BU\x80\xF3[P\x904a\x05\x7FW\x81`\x03\x196\x01\x12a\x05\x7FW` \x90a\t\xE8a\x16=V[\x90Q\x90\x81R\xF3[P4a\x05\x83W` 6`\x03\x19\x01\x12a\x05\x83Wa\n\ta\x10\x9DV[a\n\x11a\x17YV[`\x01\x80`\xA0\x1B\x03\x16`\x01`\x01``\x1B\x03`\xA0\x1B`\tT\x16\x17`\tU\x80\xF3[P\x914a\x08dW` 6`\x03\x19\x01\x12a\x08dW5\x91c\xFF\xFF\xFF\xFF\x83\x16\x83\x03a\x05\x83WPa\t\xE8` \x92a\x15\xD6V[P\x904a\x05\x7FW\x81`\x03\x196\x01\x12a\x05\x7FW` \x90`\x01T\x90Q\x90\x81R\xF3[P\x904a\x05\x7FW\x81`\x03\x196\x01\x12a\x05\x7FW` \x91T\x90Q\x90\x81R\xF3[P4a\x05\x83W\x81`\x03\x196\x01\x12a\x05\x83W\x825\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x05\x83WPa\n\xCF` \x93a\n\xD8\x926\x91\x01a\x11TV[`$5\x90a\x14_V[\x90Q\x90\x15\x15\x81R\xF3[P\x904a\x05\x7FW\x81`\x03\x196\x01\x12a\x05\x7FW`\0\x80Q` a\x17\xD4\x839\x81Q\x91RT\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x05\x83W\x80`\x03\x196\x01\x12a\x05\x83Wa\x0B0a\x17YV[`\0\x80Q` a\x17\xD4\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[P\x914a\x08dW``6`\x03\x19\x01\x12a\x08dWa\x0B\x9Ca\x10\x9DV[\x90`D5\x90\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x92\x83T\x92`\xFF\x84\x87\x1C\x16\x15\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x86\x16\x95\x86\x15\x80a\x0E\x06W[`\x01\x80\x98\x14\x90\x81a\r\xFCW[\x15\x90\x81a\r\xF3W[Pa\r\xE3Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x87\x17\x88U\x85a\r\xC4W[Pa\x0C\x1Ca\x17\x92V[a\x0C$a\x17\x92V[a\x0C-3a\x16\xE5V[d\x02T\x0B\xE4\0`\x06Ui\x01z\xA5\xBF\xB9.\xE0\x1D4\0`\x05U\x80\x89Ua\x0Cha\x0CS\x82a\x11<V[\x91a\x0C`\x8AQ\x93\x84a\x11\x1AV[\x80\x83Ra\x11<V[` \x91\x80\x83\x01\x91`\x1F\x19\x016\x837Q\x93\x84\x11a\r\xB1W`\x01`@\x1B\x84\x11a\r\xB1W\x82T\x84\x84U\x80\x85\x10a\roW[P\x91\x89R\x85\x89[\x84\x81\x10a\r=WP\x91PP\x87\x91PU\x85`\x02U\x85`\x03U`\x01\x80`\xA0\x1B\x03\x16`\x01`\x01``\x1B\x03`\xA0\x1B`\x07T\x16\x17`\x07U`$5`\x08U\x84` \x85Qa\x0C\xE3\x81a\x10\xB8V[\x82\x81R\x01R\x84`\x10U\x84`\x11U\x84`\nUa\x0C\xFCW\x83\x80\xF3[\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x92` \x92h\xFF\0\0\0\0\0\0\0\0\x19\x81T\x16\x90UQ\x90\x81R\xA18\x80\x80\x83\x80\xF3[\x82\x84Q\x94\x01\x93\x81\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x01U\x01\x86\x90a\x0C\x9DV[\x87\x85\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x92\x83\x01\x92\x01[\x82\x81\x10a\r\xA6WPPa\x0C\x96V[\x8C\x81U\x01\x88\x90a\r\x98V[cNH{q`\xE0\x1B\x8AR`A\x83R`$\x8A\xFD[h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16h\x01\0\0\0\0\0\0\0\x01\x17\x87U8a\x0C\x13V[\x88Qc\xF9.\xE8\xA9`\xE0\x1B\x81R\x83\x90\xFD[\x90P\x158a\x0B\xF8V[0;\x15\x91Pa\x0B\xF0V[P\x85a\x0B\xE4V[P4a\x05\x83W\x80`\x03\x196\x01\x12a\x05\x83WP`\x10T`\x11T\x82Q\x91\x82R` \x82\x01R\xF3[P4a\x05\x83W\x81`\x03\x196\x01\x12a\x05\x83W\x81Q\x90a\x0EN\x82a\x10\xB8V[\x835\x82R`$\x91` \x93` \x82\x01\x94`$5\x86Ra\x0Ew`\x01\x80`\xA0\x1B\x03`\x07T\x163\x14a\x13\xC0V[`\x11\x92`\x11T\x80a\x0FkW[PQ`\x10U\x7FG\x81\x1A\xEB\xD2\x8C\xBA\x7F\x02U\xE7\xF5U\x8C]\x8F\x15o\x87\x98H\x11\xE4B\x8B\x95IH\x0B\xE4\xBE$` \x87Q\x80`\x11U\x84Q\x90\x81R\xA1\x83[`\rT\x81\x10\x15a\x0F\x13Wa\x0E\xCC\x81a\x11\xB4V[\x90T\x90`\x03\x1B\x1C\x85R`\x0C\x82R\x82\x85 \x80T`\xFF\x80\x82\x16\x81\x81\x14a\x0F\x02W`\xFF\x19\x90\x92\x16`\x01\x92\x83\x01\x90\x91\x16\x17\x90\x91U\x01a\x0E\xB9V[cNH{q`\xE0\x1B\x89R\x87\x8CR\x89\x89\xFD[\x84\x88\x88Q\x90`\rT\x90`\x01`@\x1B\x82\x10\x15a\x0FXWPa\x0F<\x81`\x01a\x0FS\x93\x01`\rUa\x11\xB4V[\x81\x93\x91T\x90`\x03\x1B\x91\x82\x1B\x91`\0\x19\x90\x1B\x19\x16\x17\x90V[\x90U\x80\xF3[cNH{q`\xE0\x1B\x84R`A\x90R`$\x83\xFD[\x81Q\x03a\x0FxW8a\x0E\x83V[\x82QbF\x1B\xCD`\xE5\x1B\x81R` \x81\x8A\x01R`I`$\x82\x01R\x7FNew chain tip prev block hash do`D\x82\x01R\x7Fes not match current chain tip b`d\x82\x01Rh\r\x8D\xECmd\r\x0C.m`\xBB\x1B`\x84\x82\x01R`\xA4\x90\xFD[P\x914a\x08dW\x81`\x03\x196\x01\x12a\x08dW`$5\x905`\x01`\x01`\xA0\x1B\x03\x82\x16\x80\x83\x03a\x10|W\x81\x85R`\x0E` \x90\x81R\x84\x86 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x90\x92\x17\x90\x91U\x92Q\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x91\x81\x01\x91\x90\x91R\x7FP\xD2\x7F\xCE\xED\xD1M\xADe\x8A\xD3\xE3\x94\x160Y\xB4\xB8\xCE\x97\x1E\xD4\x86\t\xF5F\x9Es\xB6\xD3\xFA\x84\x90`@\x90\xA1\x80\xF3[\x84\x80\xFD[P\x824a\x05\x7FW` 6`\x03\x19\x01\x12a\x05\x7FWa\x06\x11\x905a\x138V[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x10\xB3WV[`\0\x80\xFD[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x10\xD4W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x10\xD4W`@RV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x10\xD4W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x10\xD4W`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x10\xD4W`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x10\xB3W` \x90\x825a\x11n\x81a\x11<V[\x93a\x11|`@Q\x95\x86a\x11\x1AV[\x81\x85R` \x80\x86\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x10\xB3W` \x01\x90[\x82\x82\x10a\x11\xA5WPPPP\x90V[\x815\x81R\x90\x83\x01\x90\x83\x01a\x11\x97V[`\rT\x81\x10\x15a\x11\xEBW`\r`\0R\x7F\xD7\xB6\x99\x01\x05q\x91\x01\xDA\xBE\xB7qD\xF2\xA38\\\x803\xAC\xD3\xAF\x97\xE9B:i^\x81\xAD\x1E\xB5\x01\x90`\0\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\x04T\x81\x10\x15a\x11\xEBW`\x04`\0R\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x01\x90`\0\x90V[`\x03\x19\x01\x90a\x01\0\x82\x12a\x10\xB3W`@Qa\x12R\x81a\x10\xFEV[`\xC0\x81\x93\x12a\x10\xB3W`@Q`\xC0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x10\xD4W`@Rc\xFF\xFF\xFF\xFF`\x045\x81\x81\x16\x81\x03a\x10\xB3W\x82R`$5` \x83\x01R`D5`@\x83\x01R`d5\x81\x81\x16\x81\x03a\x10\xB3W``\x83\x01R`\x845\x81\x81\x16\x81\x03a\x10\xB3W`\x80\x83\x01R`\xA45\x90\x81\x16\x81\x03a\x10\xB3W`\xA0\x82\x01R\x81R`\xC45` \x82\x01R`\xE45\x90`\x01`\x01`\xC0\x1B\x03\x19\x82\x16\x82\x03a\x10\xB3W`@\x01RV[\x81\x15a\x12\xFDW\x06\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x19\x81\x14a\x13\"W`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01T`\0T\x14a\x13\xB2W[\x80a\x13Sa\x0F<`\x03Ta\x12\x01V[\x90U\x7Fs\xCB\0<j\xB2L\xDE\xC6\x1C\xA1\xF33S7\xAB\xD9'\xCFg\tv\x99,\xEC\xB2\x8D\xED\x13q\xB7\x94`@`\x03T\x92\x81Q\x90\x81R\x83` \x82\x01R\xA1`\x01\x81\x01\x80\x91\x11a\x13\"W`\0Ta\x13\x9F\x91a\x12\xF3V[`\x03Ua\x13\xAD`\x01Ta\x13\x13V[`\x01UV[a\x13\xBAa\x16=V[Pa\x13DV[\x15a\x13\xC7WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FOnly the chainTipOracle can call`D\x82\x01Rk\x08\x1D\x1A\x1A\\\xC8\x1BY]\x1A\x1B\xD9`\xA2\x1B`d\x82\x01R`\x84\x90\xFD[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x13\"WV[\x90\x81Q\x91`\0[\x83\x81\x10a\x14LWPP\x01`\0\x81R\x90V[\x80` \x80\x92\x84\x01\x01Q\x81\x85\x01R\x01a\x14;V[\x80Q\x15a\x11\xEBW` \x90\x81\x81\x01Q\x93`\x01\x94[\x82Q\x86\x10\x15a\x15\x83W`\x05\x86\x90\x1B\x83\x01\x84\x01Q\x84\x91\x81\x81\x10\x15a\x15\x1AW`@\x91\x82Q\x91\x84\x83\x01R\x82\x82\x01R\x81\x81Ra\x14\xA9\x81a\x10\xFEV[\x81Q\x92\x83\x91\x82a\x14\xBC`\0\x96\x87\x93a\x144V[\x03`\x02\x93\x84Z\xFA\x15a\x15\x0FW\x82\x86\x91a\x14\xEE\x82Q\x85Q\x90\x85\x82\x01R\x84\x81Ra\x14\xE3\x81a\x10\xB8V[\x85Q\x91\x82\x80\x92a\x144V[\x03\x91Z\xFA\x15a\x15\x05WP`\x01\x90Q\x95[\x01\x94a\x14rV[Q\x90=\x90\x82>=\x90\xFD[PQ\x90=\x90\x82>=\x90\xFD[\x90`@\x91\x82Q\x91\x84\x83\x01R\x82\x82\x01R\x81\x81Ra\x155\x81a\x10\xFEV[\x81Q\x92\x83\x91\x82a\x15H`\0\x96\x87\x93a\x144V[\x03`\x02\x93\x84Z\xFA\x15a\x15\x0FW\x82\x86\x91a\x15o\x82Q\x85Q\x90\x85\x82\x01R\x84\x81Ra\x14\xE3\x81a\x10\xB8V[\x03\x91Z\xFA\x15a\x15\x05WP`\x01\x90Q\x95a\x14\xFEV[\x91P\x93P\x91\x90\x91\x03a\x15\x95WP`\x01\x90V[`d\x90`@Q\x90bF\x1B\xCD`\xE5\x1B\x82R`\x04\x82\x01R`\x10`$\x82\x01Ro\x14\xD4\x15\x88\x1C\x1C\x9B\xDB\xD9\x88\x19\x98Z[\x19Y`\x82\x1B`D\x82\x01R\xFD[\x81\x15a\x12\xFDW\x04\x90V[`\xFF\x81`\x18\x1C\x16`\x1D\x03c\xFF\xFF\xFF\xFF\x81\x11a\x13\"W`\x03\x1B\x90d\x07\xFF\xFF\xFF\xF8c\xFF\xFF\xFF\xF8\x83\x16\x92\x16\x82\x03a\x13\"W`\x06Ta\xFF\xFF\x90\x80\x82\x02\x91\x82\x04\x03a\x13\"Wb\xFF\xFF\xFFa\x16%\x92\x16\x90a\x15\xCCV[`\xFF\x82\x11a\x13\"W`\x01a\x16:\x92\x1B\x90a\x14!V[\x90V[`\x01T\x80\x15a\x16\xAEW`\x02T\x90a\x16S\x82a\x12\x01V[\x90T\x90`\x03\x1B\x1C\x91\x7F\xC0\x81\x7F\xF9\xCE!\xEC\xED?kg\x0E5\x13\xACj\x0C\xDF\x83\xD1\x96\xCAk\x1C\xFF\x9F\n\x0E\xC1\xD2\xDF\xD5`@\x80Q\x85\x81R\x83` \x82\x01R\xA1`\x01\x81\x01\x80\x91\x11a\x13\"W`\0Ta\x16\xA1\x91a\x12\xF3V[`\x02U`\0\x19\x01`\x01U\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01RnBuffer is empty`\x88\x1B`D\x82\x01R`d\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x81\x15a\x17@W`\0\x80Q` a\x17\xD4\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x84\x17\x90\x91U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`\0\x80\xA3V[`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x90\xFD[`\0\x80Q` a\x17\xD4\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x163\x03a\x17zWV[`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x90\xFD[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15a\x17\xC1WV[`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x90\xFD\xFE\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\xA2dipfsX\"\x12 \xAEd\xE1/\x91#\xFA\x04\xEE\x11`\xB9R\xD3W\xC9\xFAf\tg\xE6\x1C\xCD\xD2\xEE\x85\x1E\xB3\xA0\xE7B]dsolcC\0\x08\x18\x003";
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
        Initialize(InitializeCall),
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
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
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
    ///`BitcoinBlock((uint32,bytes32,bytes32,uint32,uint32,uint32),bytes32,bytes8)`
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
        pub block_reward: [u8; 8],
    }
    ///`BlockHeader(uint32,bytes32,bytes32,uint32,uint32,uint32)`
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
        pub version: u32,
        pub previous_block_hash: [u8; 32],
        pub merkle_root_hash: [u8; 32],
        pub timestamp: u32,
        pub bits: u32,
        pub nonce: u32,
    }
    ///`ChainTip(bytes32,bytes32)`
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
    pub struct ChainTip {
        pub previous_block_hash: [u8; 32],
        pub merkle_root_hash: [u8; 32],
    }
}
