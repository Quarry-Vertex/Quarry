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
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x16Wa\x08\xEE\x90\x81a\0\x1C\x829\xF3[`\0\x80\xFD\xFE`\x80`@\x90\x80\x82R`\x04\x806\x10\x15a\0\x16W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x92\x83cH\\\xC9U\x14a\x05\xCAWP\x82cqP\x18\xA6\x14a\x05`W\x82c\x8D\xA5\xCB[\x14a\x05*W\x82c\xBAp\x90\xC9\x14a\x03\x01W\x82c\xD7\x0E\\\x08\x14a\x02\xBCW\x82c\xE4\xBAC\xF6\x14a\0\xA2WPPc\xF2\xFD\xE3\x8B\x14a\0sW`\0\x80\xFD[4a\0\x9FW` 6`\x03\x19\x01\x12a\0\x9FWa\0\x9Ca\0\x8Fa\x07?V[a\0\x97a\x08\x1EV[a\x07\xAAV[\x80\xF3[\x80\xFD[\x83\x904a\x02\xB8W\x80`\x03\x196\x01\x12a\x02\xB8W`$5\x92`\x01\x80`\xA0\x1B\x03`\x02T\x16\x93\x82Q\x94cp\xA0\x821`\xE0\x1B\x86R3\x83\x87\x01R` \x95\x86\x81`$\x81\x85Z\xFA\x90\x81\x15a\x02\xAEW\x90\x83\x91\x87\x91a\x02yW[P\x10a\x02\x18W\x85\x85\x91`d\x86Q\x80\x94\x81\x93c#\xB8r\xDD`\xE0\x1B\x83R3\x89\x84\x01R0`$\x84\x01R\x87`D\x84\x01RZ\xF1\x90\x81\x15a\x02\x0EW\x85\x91a\x01\xE1W[P\x15a\x01dW\x7F\xBFg}\xBBJ\xD8\xE5a\xB4\xDE\xE5\x18/\x82\xB8\xC6\xE5\x84PE`\xE3\x82\x1C\xFA\xBA\x98EK\xB0\x13b\x93\x94\x83Q\x925\x83R\x82\x01R\xA1\x80\xF3[P\x83`\xA4\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`L`$\x82\x01R\x7FToken transfer failed. Please en`D\x82\x01R\x7Fsure QSAT to be pegged out has b`d\x82\x01Rk\x19Y[\x88\x18\\\x1C\x1C\x9B\xDD\x99Y`\xA2\x1B`\x84\x82\x01R\xFD[a\x02\x01\x91P\x86=\x88\x11a\x02\x07W[a\x01\xF9\x81\x83a\x07ZV[\x81\x01\x90a\x07\x92V[\x86a\x01.V[P=a\x01\xEFV[\x84Q=\x87\x82>=\x90\xFD[\x83QbF\x1B\xCD`\xE5\x1B\x81R\x80\x84\x01\x87\x90R`5`$\x82\x01R\x7FAddress has insufficient QSATs t`D\x82\x01Rt\x1B\xC8\x1C\x19Y\xC8\x1B\xDD]\x08\x1D\x1A\x1A\\\xC8\x18[[\xDD[\x9D`Z\x1B`d\x82\x01R`\x84\x90\xFD[\x80\x92P\x88\x80\x92P=\x83\x11a\x02\xA7W[a\x02\x92\x81\x83a\x07ZV[\x81\x01\x03\x12a\x02\xA3W\x82\x90Q\x88a\0\xF2V[\x85\x80\xFD[P=a\x02\x88V[\x85Q=\x88\x82>=\x90\xFD[P\x80\xFD[\x834a\0\x9FW` 6`\x03\x19\x01\x12a\0\x9FWa\x02\xD6a\x07?V[a\x02\xDEa\x08\x1EV[`\x01\x80`\xA0\x1B\x03\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B`\x02T\x16\x17`\x02U\x80\xF3[\x91P4a\x05&W\x81`\x03\x196\x01\x12a\x05&Wa\x03\x1Ba\x07?V[\x83T`$5\x92\x90`\x01`\x01`\xA0\x1B\x03\x90\x81\x163\x14\x80\x15a\x05\x19W[\x15a\x04\xB0W`\x02T\x16\x84Q\x91cp\xA0\x821`\xE0\x1B\x83R0\x81\x84\x01R` \x92\x83\x81`$\x81\x86Z\xFA\x80\x15a\x04\xA6W\x86\x91\x89\x91a\x04qW[P\x10a\x04 W\x85Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x91\x81\x01\x91\x82R` \x82\x01\x86\x90R\x91\x83\x91\x83\x91\x90\x82\x90\x8A\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x02\xAEW\x91\x7F\x11i\xB6\x99\x8Ep\"\xF7[z\"\xEE\x99\0\x98\xDB\x1FE\xB3\x1C\xC5#\x92_\x90\xA38#&\x18\xBDG\x95\x91a\x03\xFC\x93a\x04\x02W[PPQ`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R` \x82\x01\x92\x90\x92R\x90\x81\x90`@\x82\x01\x90V[\x03\x90\xA1\x80\xF3[\x81a\x04\x18\x92\x90=\x10a\x02\x07Wa\x01\xF9\x81\x83a\x07ZV[P8\x80a\x03\xDAV[\x85QbF\x1B\xCD`\xE5\x1B\x81R\x90\x81\x01\x83\x90R`%`$\x82\x01R\x7FBridge contract has insufficient`D\x82\x01Rd\x08\x14T\xD0U`\xDA\x1B`d\x82\x01R`\x84\x90\xFD[\x80\x92P\x85\x80\x92P=\x83\x11a\x04\x9FW[a\x04\x8A\x81\x83a\x07ZV[\x81\x01\x03\x12a\x04\x9BW\x85\x90Q8a\x03kV[\x87\x80\xFD[P=a\x04\x80V[\x87Q=\x8A\x82>=\x90\xFD[\x84QbF\x1B\xCD`\xE5\x1B\x81R` \x81\x84\x01R`$\x81\x01\x86\x90R\x7FOnly the oracleAddress or shares`D\x82\x01R\x7FPoolAddress can call this method`d\x82\x01R`\x84\x90\xFD[P\x80`\x01T\x163\x14a\x036V[\x82\x80\xFD[\x83\x904a\x02\xB8W\x81`\x03\x196\x01\x12a\x02\xB8W`\0\x80Q` a\x08\x99\x839\x81Q\x91RT\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x834a\0\x9FW\x80`\x03\x196\x01\x12a\0\x9FWa\x05ya\x08\x1EV[`\0\x80Q` a\x08\x99\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[\x90\x92P4a\x07;W\x82`\x03\x196\x01\x12a\x07;Wa\x05\xE5a\x07?V[`$5`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x92\x91\x83\x90\x03a\x077W\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x94\x85T\x90`\xFF\x82\x89\x1C\x16\x15\x95g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x80\x15\x90\x81a\x07/W[`\x01\x14\x90\x81a\x07%W[\x15\x90\x81a\x07\x1CW[Pa\x07\x10WPPg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16`\x01\x17\x86U\x84a\x06\xF1W[Pa\x06va\x08WV[a\x06~a\x08WV[a\x06\x873a\x07\xAAV[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x91\x16\x81\x87T\x16\x17\x86U`\x01T\x16\x17`\x01Ua\x06\xB0W\x82\x80\xF3[\x80Th\xFF\0\0\0\0\0\0\0\0\x19\x16\x90UQ`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x90\xA18\x80\x82\x80\xF3[h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16h\x01\0\0\0\0\0\0\0\x01\x17\x85U8a\x06mV[c\xF9.\xE8\xA9`\xE0\x1B\x81R\xFD[\x90P\x158a\x06OV[0;\x15\x91Pa\x06GV[\x88\x91Pa\x06=V[\x86\x80\xFD[\x83\x80\xFD[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x07UWV[`\0\x80\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x07|W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90\x81` \x91\x03\x12a\x07UWQ\x80\x15\x15\x81\x03a\x07UW\x90V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x81\x15a\x08\x05W`\0\x80Q` a\x08\x99\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x84\x17\x90\x91U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`\0\x80\xA3V[`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x90\xFD[`\0\x80Q` a\x08\x99\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x163\x03a\x08?WV[`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x90\xFD[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15a\x08\x86WV[`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x90\xFD\xFE\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\xA2dipfsX\"\x12 \x1F\x8A\xCF\xE1\xFEs\xBCe\x93\x0CX\xDDI\x07\xA3T\xC9a~>\xD1s\x80\xAE\x86\x1B\xE3Y\x95\xD0B\x82dsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static QSATBRIDGE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@\x90\x80\x82R`\x04\x806\x10\x15a\0\x16W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x92\x83cH\\\xC9U\x14a\x05\xCAWP\x82cqP\x18\xA6\x14a\x05`W\x82c\x8D\xA5\xCB[\x14a\x05*W\x82c\xBAp\x90\xC9\x14a\x03\x01W\x82c\xD7\x0E\\\x08\x14a\x02\xBCW\x82c\xE4\xBAC\xF6\x14a\0\xA2WPPc\xF2\xFD\xE3\x8B\x14a\0sW`\0\x80\xFD[4a\0\x9FW` 6`\x03\x19\x01\x12a\0\x9FWa\0\x9Ca\0\x8Fa\x07?V[a\0\x97a\x08\x1EV[a\x07\xAAV[\x80\xF3[\x80\xFD[\x83\x904a\x02\xB8W\x80`\x03\x196\x01\x12a\x02\xB8W`$5\x92`\x01\x80`\xA0\x1B\x03`\x02T\x16\x93\x82Q\x94cp\xA0\x821`\xE0\x1B\x86R3\x83\x87\x01R` \x95\x86\x81`$\x81\x85Z\xFA\x90\x81\x15a\x02\xAEW\x90\x83\x91\x87\x91a\x02yW[P\x10a\x02\x18W\x85\x85\x91`d\x86Q\x80\x94\x81\x93c#\xB8r\xDD`\xE0\x1B\x83R3\x89\x84\x01R0`$\x84\x01R\x87`D\x84\x01RZ\xF1\x90\x81\x15a\x02\x0EW\x85\x91a\x01\xE1W[P\x15a\x01dW\x7F\xBFg}\xBBJ\xD8\xE5a\xB4\xDE\xE5\x18/\x82\xB8\xC6\xE5\x84PE`\xE3\x82\x1C\xFA\xBA\x98EK\xB0\x13b\x93\x94\x83Q\x925\x83R\x82\x01R\xA1\x80\xF3[P\x83`\xA4\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`L`$\x82\x01R\x7FToken transfer failed. Please en`D\x82\x01R\x7Fsure QSAT to be pegged out has b`d\x82\x01Rk\x19Y[\x88\x18\\\x1C\x1C\x9B\xDD\x99Y`\xA2\x1B`\x84\x82\x01R\xFD[a\x02\x01\x91P\x86=\x88\x11a\x02\x07W[a\x01\xF9\x81\x83a\x07ZV[\x81\x01\x90a\x07\x92V[\x86a\x01.V[P=a\x01\xEFV[\x84Q=\x87\x82>=\x90\xFD[\x83QbF\x1B\xCD`\xE5\x1B\x81R\x80\x84\x01\x87\x90R`5`$\x82\x01R\x7FAddress has insufficient QSATs t`D\x82\x01Rt\x1B\xC8\x1C\x19Y\xC8\x1B\xDD]\x08\x1D\x1A\x1A\\\xC8\x18[[\xDD[\x9D`Z\x1B`d\x82\x01R`\x84\x90\xFD[\x80\x92P\x88\x80\x92P=\x83\x11a\x02\xA7W[a\x02\x92\x81\x83a\x07ZV[\x81\x01\x03\x12a\x02\xA3W\x82\x90Q\x88a\0\xF2V[\x85\x80\xFD[P=a\x02\x88V[\x85Q=\x88\x82>=\x90\xFD[P\x80\xFD[\x834a\0\x9FW` 6`\x03\x19\x01\x12a\0\x9FWa\x02\xD6a\x07?V[a\x02\xDEa\x08\x1EV[`\x01\x80`\xA0\x1B\x03\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B`\x02T\x16\x17`\x02U\x80\xF3[\x91P4a\x05&W\x81`\x03\x196\x01\x12a\x05&Wa\x03\x1Ba\x07?V[\x83T`$5\x92\x90`\x01`\x01`\xA0\x1B\x03\x90\x81\x163\x14\x80\x15a\x05\x19W[\x15a\x04\xB0W`\x02T\x16\x84Q\x91cp\xA0\x821`\xE0\x1B\x83R0\x81\x84\x01R` \x92\x83\x81`$\x81\x86Z\xFA\x80\x15a\x04\xA6W\x86\x91\x89\x91a\x04qW[P\x10a\x04 W\x85Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x91\x81\x01\x91\x82R` \x82\x01\x86\x90R\x91\x83\x91\x83\x91\x90\x82\x90\x8A\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x02\xAEW\x91\x7F\x11i\xB6\x99\x8Ep\"\xF7[z\"\xEE\x99\0\x98\xDB\x1FE\xB3\x1C\xC5#\x92_\x90\xA38#&\x18\xBDG\x95\x91a\x03\xFC\x93a\x04\x02W[PPQ`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R` \x82\x01\x92\x90\x92R\x90\x81\x90`@\x82\x01\x90V[\x03\x90\xA1\x80\xF3[\x81a\x04\x18\x92\x90=\x10a\x02\x07Wa\x01\xF9\x81\x83a\x07ZV[P8\x80a\x03\xDAV[\x85QbF\x1B\xCD`\xE5\x1B\x81R\x90\x81\x01\x83\x90R`%`$\x82\x01R\x7FBridge contract has insufficient`D\x82\x01Rd\x08\x14T\xD0U`\xDA\x1B`d\x82\x01R`\x84\x90\xFD[\x80\x92P\x85\x80\x92P=\x83\x11a\x04\x9FW[a\x04\x8A\x81\x83a\x07ZV[\x81\x01\x03\x12a\x04\x9BW\x85\x90Q8a\x03kV[\x87\x80\xFD[P=a\x04\x80V[\x87Q=\x8A\x82>=\x90\xFD[\x84QbF\x1B\xCD`\xE5\x1B\x81R` \x81\x84\x01R`$\x81\x01\x86\x90R\x7FOnly the oracleAddress or shares`D\x82\x01R\x7FPoolAddress can call this method`d\x82\x01R`\x84\x90\xFD[P\x80`\x01T\x163\x14a\x036V[\x82\x80\xFD[\x83\x904a\x02\xB8W\x81`\x03\x196\x01\x12a\x02\xB8W`\0\x80Q` a\x08\x99\x839\x81Q\x91RT\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x834a\0\x9FW\x80`\x03\x196\x01\x12a\0\x9FWa\x05ya\x08\x1EV[`\0\x80Q` a\x08\x99\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[\x90\x92P4a\x07;W\x82`\x03\x196\x01\x12a\x07;Wa\x05\xE5a\x07?V[`$5`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x92\x91\x83\x90\x03a\x077W\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x94\x85T\x90`\xFF\x82\x89\x1C\x16\x15\x95g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x80\x15\x90\x81a\x07/W[`\x01\x14\x90\x81a\x07%W[\x15\x90\x81a\x07\x1CW[Pa\x07\x10WPPg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16`\x01\x17\x86U\x84a\x06\xF1W[Pa\x06va\x08WV[a\x06~a\x08WV[a\x06\x873a\x07\xAAV[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x91\x16\x81\x87T\x16\x17\x86U`\x01T\x16\x17`\x01Ua\x06\xB0W\x82\x80\xF3[\x80Th\xFF\0\0\0\0\0\0\0\0\x19\x16\x90UQ`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x90\xA18\x80\x82\x80\xF3[h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16h\x01\0\0\0\0\0\0\0\x01\x17\x85U8a\x06mV[c\xF9.\xE8\xA9`\xE0\x1B\x81R\xFD[\x90P\x158a\x06OV[0;\x15\x91Pa\x06GV[\x88\x91Pa\x06=V[\x86\x80\xFD[\x83\x80\xFD[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x07UWV[`\0\x80\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x07|W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90\x81` \x91\x03\x12a\x07UWQ\x80\x15\x15\x81\x03a\x07UW\x90V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x81\x15a\x08\x05W`\0\x80Q` a\x08\x99\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x84\x17\x90\x91U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`\0\x80\xA3V[`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x90\xFD[`\0\x80Q` a\x08\x99\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x163\x03a\x08?WV[`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x90\xFD[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15a\x08\x86WV[`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x90\xFD\xFE\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\xA2dipfsX\"\x12 \x1F\x8A\xCF\xE1\xFEs\xBCe\x93\x0CX\xDDI\x07\xA3T\xC9a~>\xD1s\x80\xAE\x86\x1B\xE3Y\x95\xD0B\x82dsolcC\0\x08\x18\x003";
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
        Initialize(InitializeCall),
        Owner(OwnerCall),
        PegInQSAT(PegInQSATCall),
        PegOutQSAT(PegOutQSATCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetQSATContract(SetQSATContractCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for QSATBridgeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
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
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PegInQSAT(element) => ::core::fmt::Display::fmt(element, f),
                Self::PegOutQSAT(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetQSATContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
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
}
