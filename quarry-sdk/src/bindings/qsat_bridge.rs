pub use qsat_bridge::*;
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
pub mod qsat_bridge {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getPegOutRequest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPegOutRequest"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_index"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct QSATBridge.PegOutRequest",
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
                    ::std::borrow::ToOwned::to_owned("getTotalPegOutRequests"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTotalPegOutRequests",
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
                                        "_sharesPoolAddress",
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
                    ::std::borrow::ToOwned::to_owned("pegInQSAT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pegInQSAT"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ethAddress"),
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
                    ::std::borrow::ToOwned::to_owned("pegOutQSAT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pegOutQSAT"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("btcAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("pegOutRequests"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pegOutRequests"),
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
                                    name: ::std::borrow::ToOwned::to_owned("btcAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("processedTransaction"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "processedTransaction",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("setQSATContract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setQSATContract"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_qsatAddress"),
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
            ]),
            events: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("PegInQSATEvent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PegInQSATEvent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("ethAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
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
                    ::std::borrow::ToOwned::to_owned("PegOutQSATEvent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PegOutQSATEvent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("btcAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
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
    pub static QSATBRIDGE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4`\x15Wa\x0B9\x90\x81a\0\x1B\x829\xF3[`\0\x80\xFD\xFE`@`\x80\x81R`\x04\x90\x816\x10\x15a\0\x15W`\0\x80\xFD[`\0\x91\x825`\xE0\x1C\x91\x82c1%\xD5\xFC\x14a\x08\xE2W\x82cH\\\xC9U\x14a\x07pW\x82cqP\x18\xA6\x14a\x07\x06W\x82c\x8C\x1A\xACF\x14a\x06\xE8W\x82c\x8D\xA5\xCB[\x14a\x06\xB2W\x82c\x8E\x86q\xF6\x14a\x06\nW\x82c\xA5\xE86s\x14a\x05\xBCW\x82c\xBAp\x90\xC9\x14a\x03\x97W\x82c\xD7\x0E\\\x08\x14a\x03RW\x82c\xE4\xBAC\xF6\x14a\0\xCCWPPc\xF2\xFD\xE3\x8B\x14a\0\x9DW`\0\x80\xFD[4a\0\xC9W` 6`\x03\x19\x01\x12a\0\xC9Wa\0\xC6a\0\xB9a\tnV[a\0\xC1a\niV[a\t\xF5V[\x80\xF3[\x80\xFD[\x83\x904a\x03NW\x80`\x03\x196\x01\x12a\x03NW`\x02T\x81Qcp\xA0\x821`\xE0\x1B\x81R3\x81\x86\x01R`$\x94\x805\x92\x865\x92` \x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90\x83\x81\x8A\x81\x85Z\xFA\x90\x81\x15a\x03DW\x90\x85\x91\x89\x91a\x03\x0FW[P\x10a\x02\xB0W\x82\x87\x91`d\x88Q\x80\x94\x81\x93c#\xB8r\xDD`\xE0\x1B\x83R3\x88\x84\x01R\x8D0\x90\x84\x01R\x89`D\x84\x01RZ\xF1\x90\x81\x15a\x02\xA6W\x87\x91a\x02yW[P\x15a\x01\xFEW\x84Qa\x01n\x81a\t\x89V[\x84\x81R\x82\x81\x01\x97\x84\x89R\x82Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x01\xECW\x80`\x01a\x01\x9A\x92\x01\x85Ua\t\x1EV[\x93\x90\x93a\x01\xDCWPP\x7F\xBFg}\xBBJ\xD8\xE5a\xB4\xDE\xE5\x18/\x82\xB8\xC6\xE5\x84PE`\xE3\x82\x1C\xFA\xBA\x98EK\xB0\x13b\x96\x97`\x01\x91Q\x83UQ\x91\x01U\x83Q\x92\x83R\x82\x01R\xA1\x80\xF3[cNH{q`\xE0\x1B\x89R\x88\x90R\x87\xFD[PcNH{q`\xE0\x1B\x88R`A\x83R\x87\xFD[`L\x87`\xA4\x93\x87Q\x93bF\x1B\xCD`\xE5\x1B\x85R\x84\x01R\x82\x01R\x7FToken transfer failed. Please en`D\x82\x01R\x7Fsure QSAT to be pegged out has b`d\x82\x01Rk\x19Y[\x88\x18\\\x1C\x1C\x9B\xDD\x99Y`\xA2\x1B`\x84\x82\x01R\xFD[a\x02\x99\x91P\x83=\x85\x11a\x02\x9FW[a\x02\x91\x81\x83a\t\xBBV[\x81\x01\x90a\t\xDDV[\x88a\x01]V[P=a\x02\x87V[\x86Q=\x89\x82>=\x90\xFD[P`5\x87`\x84\x93\x87Q\x93bF\x1B\xCD`\xE5\x1B\x85R\x84\x01R\x82\x01R\x7FAddress has insufficient QSATs t`D\x82\x01Rt\x1B\xC8\x1C\x19Y\xC8\x1B\xDD]\x08\x1D\x1A\x1A\\\xC8\x18[[\xDD[\x9D`Z\x1B`d\x82\x01R\xFD[\x80\x92P\x85\x80\x92P=\x83\x11a\x03=W[a\x03(\x81\x83a\t\xBBV[\x81\x01\x03\x12a\x039W\x84\x90Q\x8Aa\x01!V[\x87\x80\xFD[P=a\x03\x1EV[\x87Q=\x8A\x82>=\x90\xFD[P\x80\xFD[\x834a\0\xC9W` 6`\x03\x19\x01\x12a\0\xC9Wa\x03la\tnV[a\x03ta\niV[`\x01\x80`\xA0\x1B\x03\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B`\x02T\x16\x17`\x02U\x80\xF3[\x91P4a\x05\xB8W\x81`\x03\x196\x01\x12a\x05\xB8Wa\x03\xB1a\tnV[\x83T`$5\x92\x90`\x01`\x01`\xA0\x1B\x03\x90\x81\x163\x14\x80\x15a\x05\xABW[\x15a\x05BW`\x02T\x16\x84Q\x91cp\xA0\x821`\xE0\x1B\x83R0\x81\x84\x01R` \x92\x83\x81`$\x81\x86Z\xFA\x80\x15a\x03DW\x86\x91\x89\x91a\x05\x11W[P\x10a\x04\xC0W\x85Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x91\x81\x01\x91\x82R` \x82\x01\x86\x90R\x91\x83\x91\x83\x91\x90\x82\x90\x8A\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x04\xB6W\x91\x7F\x11i\xB6\x99\x8Ep\"\xF7[z\"\xEE\x99\0\x98\xDB\x1FE\xB3\x1C\xC5#\x92_\x90\xA38#&\x18\xBDG\x95\x91a\x04\x92\x93a\x04\x98W[PPQ`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R` \x82\x01\x92\x90\x92R\x90\x81\x90`@\x82\x01\x90V[\x03\x90\xA1\x80\xF3[\x81a\x04\xAE\x92\x90=\x10a\x02\x9FWa\x02\x91\x81\x83a\t\xBBV[P8\x80a\x04pV[\x85Q=\x88\x82>=\x90\xFD[\x85QbF\x1B\xCD`\xE5\x1B\x81R\x90\x81\x01\x83\x90R`%`$\x82\x01R\x7FBridge contract has insufficient`D\x82\x01Rd\x08\x14T\xD0U`\xDA\x1B`d\x82\x01R`\x84\x90\xFD[\x80\x92P\x85\x80\x92P=\x83\x11a\x05;W[a\x05*\x81\x83a\t\xBBV[\x81\x01\x03\x12a\x039W\x85\x90Q8a\x04\x01V[P=a\x05 V[\x84QbF\x1B\xCD`\xE5\x1B\x81R` \x81\x84\x01R`$\x81\x01\x86\x90R\x7FOnly the oracleAddress or shares`D\x82\x01R\x7FPoolAddress can call this method`d\x82\x01R`\x84\x90\xFD[P\x80`\x01T\x163\x14a\x03\xCCV[\x82\x80\xFD[\x91P4a\x05\xB8W` 6`\x03\x19\x01\x12a\x05\xB8W5`\x03T\x81\x10\x15a\x05\xB8W`\x03` \x93R\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[\x01T\x90Q\x90\x81R\xF3[\x83\x824a\x03NW` 6`\x03\x19\x01\x12a\x03NW\x805\x91` \x84Qa\x06-\x81a\t\x89V[\x82\x81R\x01R\x80T\x82\x10\x15a\x06oWPa\x06E\x90a\t\x1EV[P\x81Qa\x06Q\x81a\t\x89V[` `\x01\x83T\x93\x84\x84R\x01T\x91\x01\x90\x81R\x82Q\x91\x82RQ` \x82\x01R\xF3[`d\x90` \x84Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\x1C`$\x82\x01R\x7FBurn request does not exist.\0\0\0\0`D\x82\x01R\xFD[\x83\x904a\x03NW\x81`\x03\x196\x01\x12a\x03NW`\0\x80Q` a\n\xE4\x839\x81Q\x91RT\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x91P4a\x05\xB8W\x82`\x03\x196\x01\x12a\x05\xB8W` \x92PT\x90Q\x90\x81R\xF3[\x834a\0\xC9W\x80`\x03\x196\x01\x12a\0\xC9Wa\x07\x1Fa\niV[`\0\x80Q` a\n\xE4\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[\x91P4a\x05\xB8W\x81`\x03\x196\x01\x12a\x05\xB8Wa\x07\x8Aa\tnV[`\x01`\x01`\xA0\x1B\x03\x91\x90`$5\x83\x81\x16\x91\x90\x82\x90\x03a\x08\xDEW\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x93\x84T`\xFF\x81\x88\x1C\x16\x15\x94g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80\x15\x90\x81a\x08\xD6W[`\x01\x14\x90\x81a\x08\xCCW[\x15\x90\x81a\x08\xC3W[Pa\x08\xB5WPg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16`\x01\x17\x86U\x84a\x08\x96W[Pa\x08\x1Ba\n\xA2V[a\x08#a\n\xA2V[a\x08,3a\t\xF5V[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x91\x16\x81\x87T\x16\x17\x86U`\x01T\x16\x17`\x01Ua\x08UW\x82\x80\xF3[\x80Th\xFF\0\0\0\0\0\0\0\0\x19\x16\x90UQ`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x90\xA18\x80\x82\x80\xF3[h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16h\x01\0\0\0\0\0\0\0\x01\x17\x85U8a\x08\x12V[\x87Qc\xF9.\xE8\xA9`\xE0\x1B\x81R\xFD[\x90P\x158a\x07\xF5V[0;\x15\x91Pa\x07\xEDV[\x87\x91Pa\x07\xE3V[\x85\x80\xFD[\x90\x834a\0\xC9W` 6`\x03\x19\x01\x12a\0\xC9W\x815\x91T\x82\x10\x15a\0\xC9WPa\t\n\x90a\t\x1EV[P`\x01\x81T\x91\x01T\x82Q\x91\x82R` \x82\x01R\xF3[`\x04T\x81\x10\x15a\tXW`\x04`\0R`\x01\x1B\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x01\x90`\0\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\t\x84WV[`\0\x80\xFD[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\t\xA5W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\t\xA5W`@RV[\x90\x81` \x91\x03\x12a\t\x84WQ\x80\x15\x15\x81\x03a\t\x84W\x90V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x81\x15a\nPW`\0\x80Q` a\n\xE4\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x84\x17\x90\x91U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`\0\x80\xA3V[`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x90\xFD[`\0\x80Q` a\n\xE4\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x163\x03a\n\x8AWV[`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x90\xFD[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15a\n\xD1WV[`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x90\xFD\xFE\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\xA2dipfsX\"\x12 \x8C\xB1\xA5\x96\xC9\xF5\xAA\xB0\xF5O\xAC\xEAh\xC1\xCE\x0F\x12\x07U\x18\xB7\xD5Rg\x8A\xAA\x06V\xA5<\x96\x93dsolcC\0\x08\x19\x003";
    /// The bytecode of the contract.
    pub static QSATBRIDGE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`@`\x80\x81R`\x04\x90\x816\x10\x15a\0\x15W`\0\x80\xFD[`\0\x91\x825`\xE0\x1C\x91\x82c1%\xD5\xFC\x14a\x08\xE2W\x82cH\\\xC9U\x14a\x07pW\x82cqP\x18\xA6\x14a\x07\x06W\x82c\x8C\x1A\xACF\x14a\x06\xE8W\x82c\x8D\xA5\xCB[\x14a\x06\xB2W\x82c\x8E\x86q\xF6\x14a\x06\nW\x82c\xA5\xE86s\x14a\x05\xBCW\x82c\xBAp\x90\xC9\x14a\x03\x97W\x82c\xD7\x0E\\\x08\x14a\x03RW\x82c\xE4\xBAC\xF6\x14a\0\xCCWPPc\xF2\xFD\xE3\x8B\x14a\0\x9DW`\0\x80\xFD[4a\0\xC9W` 6`\x03\x19\x01\x12a\0\xC9Wa\0\xC6a\0\xB9a\tnV[a\0\xC1a\niV[a\t\xF5V[\x80\xF3[\x80\xFD[\x83\x904a\x03NW\x80`\x03\x196\x01\x12a\x03NW`\x02T\x81Qcp\xA0\x821`\xE0\x1B\x81R3\x81\x86\x01R`$\x94\x805\x92\x865\x92` \x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90\x83\x81\x8A\x81\x85Z\xFA\x90\x81\x15a\x03DW\x90\x85\x91\x89\x91a\x03\x0FW[P\x10a\x02\xB0W\x82\x87\x91`d\x88Q\x80\x94\x81\x93c#\xB8r\xDD`\xE0\x1B\x83R3\x88\x84\x01R\x8D0\x90\x84\x01R\x89`D\x84\x01RZ\xF1\x90\x81\x15a\x02\xA6W\x87\x91a\x02yW[P\x15a\x01\xFEW\x84Qa\x01n\x81a\t\x89V[\x84\x81R\x82\x81\x01\x97\x84\x89R\x82Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x01\xECW\x80`\x01a\x01\x9A\x92\x01\x85Ua\t\x1EV[\x93\x90\x93a\x01\xDCWPP\x7F\xBFg}\xBBJ\xD8\xE5a\xB4\xDE\xE5\x18/\x82\xB8\xC6\xE5\x84PE`\xE3\x82\x1C\xFA\xBA\x98EK\xB0\x13b\x96\x97`\x01\x91Q\x83UQ\x91\x01U\x83Q\x92\x83R\x82\x01R\xA1\x80\xF3[cNH{q`\xE0\x1B\x89R\x88\x90R\x87\xFD[PcNH{q`\xE0\x1B\x88R`A\x83R\x87\xFD[`L\x87`\xA4\x93\x87Q\x93bF\x1B\xCD`\xE5\x1B\x85R\x84\x01R\x82\x01R\x7FToken transfer failed. Please en`D\x82\x01R\x7Fsure QSAT to be pegged out has b`d\x82\x01Rk\x19Y[\x88\x18\\\x1C\x1C\x9B\xDD\x99Y`\xA2\x1B`\x84\x82\x01R\xFD[a\x02\x99\x91P\x83=\x85\x11a\x02\x9FW[a\x02\x91\x81\x83a\t\xBBV[\x81\x01\x90a\t\xDDV[\x88a\x01]V[P=a\x02\x87V[\x86Q=\x89\x82>=\x90\xFD[P`5\x87`\x84\x93\x87Q\x93bF\x1B\xCD`\xE5\x1B\x85R\x84\x01R\x82\x01R\x7FAddress has insufficient QSATs t`D\x82\x01Rt\x1B\xC8\x1C\x19Y\xC8\x1B\xDD]\x08\x1D\x1A\x1A\\\xC8\x18[[\xDD[\x9D`Z\x1B`d\x82\x01R\xFD[\x80\x92P\x85\x80\x92P=\x83\x11a\x03=W[a\x03(\x81\x83a\t\xBBV[\x81\x01\x03\x12a\x039W\x84\x90Q\x8Aa\x01!V[\x87\x80\xFD[P=a\x03\x1EV[\x87Q=\x8A\x82>=\x90\xFD[P\x80\xFD[\x834a\0\xC9W` 6`\x03\x19\x01\x12a\0\xC9Wa\x03la\tnV[a\x03ta\niV[`\x01\x80`\xA0\x1B\x03\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B`\x02T\x16\x17`\x02U\x80\xF3[\x91P4a\x05\xB8W\x81`\x03\x196\x01\x12a\x05\xB8Wa\x03\xB1a\tnV[\x83T`$5\x92\x90`\x01`\x01`\xA0\x1B\x03\x90\x81\x163\x14\x80\x15a\x05\xABW[\x15a\x05BW`\x02T\x16\x84Q\x91cp\xA0\x821`\xE0\x1B\x83R0\x81\x84\x01R` \x92\x83\x81`$\x81\x86Z\xFA\x80\x15a\x03DW\x86\x91\x89\x91a\x05\x11W[P\x10a\x04\xC0W\x85Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x91\x81\x01\x91\x82R` \x82\x01\x86\x90R\x91\x83\x91\x83\x91\x90\x82\x90\x8A\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x04\xB6W\x91\x7F\x11i\xB6\x99\x8Ep\"\xF7[z\"\xEE\x99\0\x98\xDB\x1FE\xB3\x1C\xC5#\x92_\x90\xA38#&\x18\xBDG\x95\x91a\x04\x92\x93a\x04\x98W[PPQ`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R` \x82\x01\x92\x90\x92R\x90\x81\x90`@\x82\x01\x90V[\x03\x90\xA1\x80\xF3[\x81a\x04\xAE\x92\x90=\x10a\x02\x9FWa\x02\x91\x81\x83a\t\xBBV[P8\x80a\x04pV[\x85Q=\x88\x82>=\x90\xFD[\x85QbF\x1B\xCD`\xE5\x1B\x81R\x90\x81\x01\x83\x90R`%`$\x82\x01R\x7FBridge contract has insufficient`D\x82\x01Rd\x08\x14T\xD0U`\xDA\x1B`d\x82\x01R`\x84\x90\xFD[\x80\x92P\x85\x80\x92P=\x83\x11a\x05;W[a\x05*\x81\x83a\t\xBBV[\x81\x01\x03\x12a\x039W\x85\x90Q8a\x04\x01V[P=a\x05 V[\x84QbF\x1B\xCD`\xE5\x1B\x81R` \x81\x84\x01R`$\x81\x01\x86\x90R\x7FOnly the oracleAddress or shares`D\x82\x01R\x7FPoolAddress can call this method`d\x82\x01R`\x84\x90\xFD[P\x80`\x01T\x163\x14a\x03\xCCV[\x82\x80\xFD[\x91P4a\x05\xB8W` 6`\x03\x19\x01\x12a\x05\xB8W5`\x03T\x81\x10\x15a\x05\xB8W`\x03` \x93R\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[\x01T\x90Q\x90\x81R\xF3[\x83\x824a\x03NW` 6`\x03\x19\x01\x12a\x03NW\x805\x91` \x84Qa\x06-\x81a\t\x89V[\x82\x81R\x01R\x80T\x82\x10\x15a\x06oWPa\x06E\x90a\t\x1EV[P\x81Qa\x06Q\x81a\t\x89V[` `\x01\x83T\x93\x84\x84R\x01T\x91\x01\x90\x81R\x82Q\x91\x82RQ` \x82\x01R\xF3[`d\x90` \x84Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\x1C`$\x82\x01R\x7FBurn request does not exist.\0\0\0\0`D\x82\x01R\xFD[\x83\x904a\x03NW\x81`\x03\x196\x01\x12a\x03NW`\0\x80Q` a\n\xE4\x839\x81Q\x91RT\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x91P4a\x05\xB8W\x82`\x03\x196\x01\x12a\x05\xB8W` \x92PT\x90Q\x90\x81R\xF3[\x834a\0\xC9W\x80`\x03\x196\x01\x12a\0\xC9Wa\x07\x1Fa\niV[`\0\x80Q` a\n\xE4\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[\x91P4a\x05\xB8W\x81`\x03\x196\x01\x12a\x05\xB8Wa\x07\x8Aa\tnV[`\x01`\x01`\xA0\x1B\x03\x91\x90`$5\x83\x81\x16\x91\x90\x82\x90\x03a\x08\xDEW\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x93\x84T`\xFF\x81\x88\x1C\x16\x15\x94g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80\x15\x90\x81a\x08\xD6W[`\x01\x14\x90\x81a\x08\xCCW[\x15\x90\x81a\x08\xC3W[Pa\x08\xB5WPg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16`\x01\x17\x86U\x84a\x08\x96W[Pa\x08\x1Ba\n\xA2V[a\x08#a\n\xA2V[a\x08,3a\t\xF5V[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x91\x16\x81\x87T\x16\x17\x86U`\x01T\x16\x17`\x01Ua\x08UW\x82\x80\xF3[\x80Th\xFF\0\0\0\0\0\0\0\0\x19\x16\x90UQ`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x90\xA18\x80\x82\x80\xF3[h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16h\x01\0\0\0\0\0\0\0\x01\x17\x85U8a\x08\x12V[\x87Qc\xF9.\xE8\xA9`\xE0\x1B\x81R\xFD[\x90P\x158a\x07\xF5V[0;\x15\x91Pa\x07\xEDV[\x87\x91Pa\x07\xE3V[\x85\x80\xFD[\x90\x834a\0\xC9W` 6`\x03\x19\x01\x12a\0\xC9W\x815\x91T\x82\x10\x15a\0\xC9WPa\t\n\x90a\t\x1EV[P`\x01\x81T\x91\x01T\x82Q\x91\x82R` \x82\x01R\xF3[`\x04T\x81\x10\x15a\tXW`\x04`\0R`\x01\x1B\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x01\x90`\0\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\t\x84WV[`\0\x80\xFD[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\t\xA5W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\t\xA5W`@RV[\x90\x81` \x91\x03\x12a\t\x84WQ\x80\x15\x15\x81\x03a\t\x84W\x90V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x81\x15a\nPW`\0\x80Q` a\n\xE4\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x84\x17\x90\x91U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`\0\x80\xA3V[`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x90\xFD[`\0\x80Q` a\n\xE4\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x163\x03a\n\x8AWV[`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x90\xFD[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15a\n\xD1WV[`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x90\xFD\xFE\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\xA2dipfsX\"\x12 \x8C\xB1\xA5\x96\xC9\xF5\xAA\xB0\xF5O\xAC\xEAh\xC1\xCE\x0F\x12\x07U\x18\xB7\xD5Rg\x8A\xAA\x06V\xA5<\x96\x93dsolcC\0\x08\x19\x003";
    /// The deployed bytecode of the contract.
    pub static QSATBRIDGE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct QSATBridge<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for QSATBridge<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for QSATBridge<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for QSATBridge<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for QSATBridge<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(QSATBridge)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> QSATBridge<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    QSATBRIDGE_ABI.clone(),
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
                QSATBRIDGE_ABI.clone(),
                QSATBRIDGE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getPegOutRequest` (0x8e8671f6) function
        pub fn get_peg_out_request(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, PegOutRequest> {
            self.0
                .method_hash([142, 134, 113, 246], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalPegOutRequests` (0x8c1aac46) function
        pub fn get_total_peg_out_requests(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([140, 26, 172, 70], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x485cc955) function
        pub fn initialize(
            &self,
            oracle_address: ::ethers::core::types::Address,
            shares_pool_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 92, 201, 85], (oracle_address, shares_pool_address))
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
        ///Calls the contract's `pegInQSAT` (0xba7090c9) function
        pub fn peg_in_qsat(
            &self,
            eth_address: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([186, 112, 144, 201], (eth_address, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pegOutQSAT` (0xe4ba43f6) function
        pub fn peg_out_qsat(
            &self,
            btc_address: [u8; 32],
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([228, 186, 67, 246], (btc_address, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pegOutRequests` (0x3125d5fc) function
        pub fn peg_out_requests(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ([u8; 32], ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([49, 37, 213, 252], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `processedTransaction` (0xa5e83673) function
        pub fn processed_transaction(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([165, 232, 54, 115], p0)
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
        ///Calls the contract's `setQSATContract` (0xd70e5c08) function
        pub fn set_qsat_contract(
            &self,
            qsat_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([215, 14, 92, 8], qsat_address)
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
        ///Gets the contract's `PegInQSATEvent` event
        pub fn peg_in_qsat_event_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PegInQSATEventFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PegOutQSATEvent` event
        pub fn peg_out_qsat_event_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PegOutQSATEventFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            QSATBridgeEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for QSATBridge<M> {
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
    pub enum QSATBridgeErrors {
        InvalidInitialization(InvalidInitialization),
        NotInitializing(NotInitializing),
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for QSATBridgeErrors {
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
    impl ::ethers::core::abi::AbiEncode for QSATBridgeErrors {
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
    impl ::ethers::contract::ContractRevert for QSATBridgeErrors {
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
    impl ::core::fmt::Display for QSATBridgeErrors {
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
    impl ::core::convert::From<::std::string::String> for QSATBridgeErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidInitialization> for QSATBridgeErrors {
        fn from(value: InvalidInitialization) -> Self {
            Self::InvalidInitialization(value)
        }
    }
    impl ::core::convert::From<NotInitializing> for QSATBridgeErrors {
        fn from(value: NotInitializing) -> Self {
            Self::NotInitializing(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner> for QSATBridgeErrors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount> for QSATBridgeErrors {
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
    #[ethevent(name = "PegInQSATEvent", abi = "PegInQSATEvent(address,uint256)")]
    pub struct PegInQSATEventFilter {
        pub eth_address: ::ethers::core::types::Address,
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
    #[ethevent(name = "PegOutQSATEvent", abi = "PegOutQSATEvent(bytes32,uint256)")]
    pub struct PegOutQSATEventFilter {
        pub btc_address: [u8; 32],
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum QSATBridgeEvents {
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PegInQSATEventFilter(PegInQSATEventFilter),
        PegOutQSATEventFilter(PegOutQSATEventFilter),
    }
    impl ::ethers::contract::EthLogDecode for QSATBridgeEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(QSATBridgeEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(QSATBridgeEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PegInQSATEventFilter::decode_log(log) {
                return Ok(QSATBridgeEvents::PegInQSATEventFilter(decoded));
            }
            if let Ok(decoded) = PegOutQSATEventFilter::decode_log(log) {
                return Ok(QSATBridgeEvents::PegOutQSATEventFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for QSATBridgeEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PegInQSATEventFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PegOutQSATEventFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<InitializedFilter> for QSATBridgeEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for QSATBridgeEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PegInQSATEventFilter> for QSATBridgeEvents {
        fn from(value: PegInQSATEventFilter) -> Self {
            Self::PegInQSATEventFilter(value)
        }
    }
    impl ::core::convert::From<PegOutQSATEventFilter> for QSATBridgeEvents {
        fn from(value: PegOutQSATEventFilter) -> Self {
            Self::PegOutQSATEventFilter(value)
        }
    }
    ///Container type for all input parameters for the `getPegOutRequest` function with signature `getPegOutRequest(uint256)` and selector `0x8e8671f6`
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
    #[ethcall(name = "getPegOutRequest", abi = "getPegOutRequest(uint256)")]
    pub struct GetPegOutRequestCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getTotalPegOutRequests` function with signature `getTotalPegOutRequests()` and selector `0x8c1aac46`
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
    #[ethcall(name = "getTotalPegOutRequests", abi = "getTotalPegOutRequests()")]
    pub struct GetTotalPegOutRequestsCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address)` and selector `0x485cc955`
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
    #[ethcall(name = "initialize", abi = "initialize(address,address)")]
    pub struct InitializeCall {
        pub oracle_address: ::ethers::core::types::Address,
        pub shares_pool_address: ::ethers::core::types::Address,
    }
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
    ///Container type for all input parameters for the `pegInQSAT` function with signature `pegInQSAT(address,uint256)` and selector `0xba7090c9`
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
    #[ethcall(name = "pegInQSAT", abi = "pegInQSAT(address,uint256)")]
    pub struct PegInQSATCall {
        pub eth_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pegOutQSAT` function with signature `pegOutQSAT(bytes32,uint256)` and selector `0xe4ba43f6`
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
    #[ethcall(name = "pegOutQSAT", abi = "pegOutQSAT(bytes32,uint256)")]
    pub struct PegOutQSATCall {
        pub btc_address: [u8; 32],
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pegOutRequests` function with signature `pegOutRequests(uint256)` and selector `0x3125d5fc`
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
    #[ethcall(name = "pegOutRequests", abi = "pegOutRequests(uint256)")]
    pub struct PegOutRequestsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `processedTransaction` function with signature `processedTransaction(uint256)` and selector `0xa5e83673`
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
    #[ethcall(name = "processedTransaction", abi = "processedTransaction(uint256)")]
    pub struct ProcessedTransactionCall(pub ::ethers::core::types::U256);
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
    ///Container type for all input parameters for the `setQSATContract` function with signature `setQSATContract(address)` and selector `0xd70e5c08`
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
    #[ethcall(name = "setQSATContract", abi = "setQSATContract(address)")]
    pub struct SetQSATContractCall {
        pub qsat_address: ::ethers::core::types::Address,
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
    pub enum QSATBridgeCalls {
        GetPegOutRequest(GetPegOutRequestCall),
        GetTotalPegOutRequests(GetTotalPegOutRequestsCall),
        Initialize(InitializeCall),
        Owner(OwnerCall),
        PegInQSAT(PegInQSATCall),
        PegOutQSAT(PegOutQSATCall),
        PegOutRequests(PegOutRequestsCall),
        ProcessedTransaction(ProcessedTransactionCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetQSATContract(SetQSATContractCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for QSATBridgeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetPegOutRequestCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPegOutRequest(decoded));
            }
            if let Ok(decoded) = <GetTotalPegOutRequestsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTotalPegOutRequests(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PegInQSATCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PegInQSAT(decoded));
            }
            if let Ok(decoded) = <PegOutQSATCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PegOutQSAT(decoded));
            }
            if let Ok(decoded) = <PegOutRequestsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PegOutRequests(decoded));
            }
            if let Ok(decoded) = <ProcessedTransactionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProcessedTransaction(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetQSATContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetQSATContract(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for QSATBridgeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetPegOutRequest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalPegOutRequests(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PegInQSAT(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PegOutQSAT(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PegOutRequests(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProcessedTransaction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetQSATContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for QSATBridgeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetPegOutRequest(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTotalPegOutRequests(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PegInQSAT(element) => ::core::fmt::Display::fmt(element, f),
                Self::PegOutQSAT(element) => ::core::fmt::Display::fmt(element, f),
                Self::PegOutRequests(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProcessedTransaction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetQSATContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetPegOutRequestCall> for QSATBridgeCalls {
        fn from(value: GetPegOutRequestCall) -> Self {
            Self::GetPegOutRequest(value)
        }
    }
    impl ::core::convert::From<GetTotalPegOutRequestsCall> for QSATBridgeCalls {
        fn from(value: GetTotalPegOutRequestsCall) -> Self {
            Self::GetTotalPegOutRequests(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for QSATBridgeCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for QSATBridgeCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PegInQSATCall> for QSATBridgeCalls {
        fn from(value: PegInQSATCall) -> Self {
            Self::PegInQSAT(value)
        }
    }
    impl ::core::convert::From<PegOutQSATCall> for QSATBridgeCalls {
        fn from(value: PegOutQSATCall) -> Self {
            Self::PegOutQSAT(value)
        }
    }
    impl ::core::convert::From<PegOutRequestsCall> for QSATBridgeCalls {
        fn from(value: PegOutRequestsCall) -> Self {
            Self::PegOutRequests(value)
        }
    }
    impl ::core::convert::From<ProcessedTransactionCall> for QSATBridgeCalls {
        fn from(value: ProcessedTransactionCall) -> Self {
            Self::ProcessedTransaction(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for QSATBridgeCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetQSATContractCall> for QSATBridgeCalls {
        fn from(value: SetQSATContractCall) -> Self {
            Self::SetQSATContract(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for QSATBridgeCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the `getPegOutRequest` function with signature `getPegOutRequest(uint256)` and selector `0x8e8671f6`
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
    pub struct GetPegOutRequestReturn(pub PegOutRequest);
    ///Container type for all return fields from the `getTotalPegOutRequests` function with signature `getTotalPegOutRequests()` and selector `0x8c1aac46`
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
    pub struct GetTotalPegOutRequestsReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `pegOutRequests` function with signature `pegOutRequests(uint256)` and selector `0x3125d5fc`
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
    pub struct PegOutRequestsReturn {
        pub btc_address: [u8; 32],
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `processedTransaction` function with signature `processedTransaction(uint256)` and selector `0xa5e83673`
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
    pub struct ProcessedTransactionReturn(pub [u8; 32]);
    ///`PegOutRequest(bytes32,uint256)`
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
    pub struct PegOutRequest {
        pub btc_address: [u8; 32],
        pub amount: ::ethers::core::types::U256,
    }
}
