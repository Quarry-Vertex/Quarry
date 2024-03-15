pub use share::*;
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
pub mod share {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
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
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                    ::std::borrow::ToOwned::to_owned("getApproved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getApproved"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                                    name: ::std::borrow::ToOwned::to_owned("poolAddress"),
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
                    ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                    ::std::borrow::ToOwned::to_owned("ownerOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ownerOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                    ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
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
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
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
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("approved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
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
                    ::std::borrow::ToOwned::to_owned("tokenExists"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenExists"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenURI"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenURI"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                                    name: ::std::borrow::ToOwned::to_owned("approved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("approved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
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
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
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
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
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
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ERC721IncorrectOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC721IncorrectOwner",
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
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("ERC721InsufficientApproval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC721InsufficientApproval",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                    ::std::borrow::ToOwned::to_owned("ERC721InvalidApprover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC721InvalidApprover",
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
                    ::std::borrow::ToOwned::to_owned("ERC721InvalidOperator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC721InvalidOperator",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
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
                    ::std::borrow::ToOwned::to_owned("ERC721InvalidOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ERC721InvalidOwner"),
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
                    ::std::borrow::ToOwned::to_owned("ERC721InvalidReceiver"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC721InvalidReceiver",
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
                    ::std::borrow::ToOwned::to_owned("ERC721InvalidSender"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC721InvalidSender",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("ERC721NonexistentToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC721NonexistentToken",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
    pub static SHARE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x16Wa\x17V\x90\x81a\0\x1C\x829\xF3[`\0\x80\xFD\xFE`@`\x80\x81R`\x04\x806\x10\x15a\0\x14W`\0\x80\xFD[`\0\x91\x825`\xE0\x1C\x91\x82b\x92?\x9E\x14a\x10\x06W\x82c\x01\xFF\xC9\xA7\x14a\x0F\x97W\x82c\x06\xFD\xDE\x03\x14a\x0E\xC5W\x82c\x07\x7F\"J\x14a\nZW\x82c\x08\x18\x12\xFC\x14a\n\x0FW\x82c\t^\xA7\xB3\x14a\t,W\x82c#\xB8r\xDD\x14a\t\x14W\x82c@\xC1\x0F\x19\x14a\x06\x94W\x82cB\x84.\x0E\x14a\x06eW\x82cB\x96lh\x14a\x05\x13W\x82ccR!\x1E\x14a\x04\xE2W\x82cp\xA0\x821\x14a\x04\x8EW\x82cqP\x18\xA6\x14a\x04$W\x82c\x8D\xA5\xCB[\x14a\x03\xEEW\x82c\x95\xD8\x9BA\x14a\x02\xE2W\x82c\xA2,\xB4e\x14a\x02?W\x82c\xB8\x8DO\xDE\x14a\x01\xCFW\x82c\xC8{V\xDD\x14a\x01}WP\x81c\xE9\x85\xE9\xC5\x14a\x01-WPc\xF2\xFD\xE3\x8B\x14a\0\xFEW`\0\x80\xFD[4a\x01*W` 6`\x03\x19\x01\x12a\x01*Wa\x01'a\x01\x1Aa\x11\\V[a\x01\"a\x14\xCAV[a\x14\x0EV[\x80\xF3[\x80\xFD[\x90P4a\x01yW\x80`\x03\x196\x01\x12a\x01yW`\xFF\x81` \x93a\x01Ma\x11\\V[a\x01^a\x01Xa\x11rV[\x91a\x13\xD5V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x82R\x85R T\x91Q\x91\x16\x15\x15\x81R\xF3[P\x80\xFD[\x83\x824a\x01yW` 6`\x03\x19\x01\x12a\x01yWa\x01\x9Da\x01\xCB\x935a\x14\x82V[P\x81\x81Qa\x01\xAA\x81a\x10\x92V[R\x80Q\x91a\x01\xB7\x83a\x10\x92V[\x82RQ\x91\x82\x91` \x83R` \x83\x01\x90a\x10RV[\x03\x90\xF3[\x83\x904a\x01yW`\x806`\x03\x19\x01\x12a\x01yWa\x01\xEAa\x11\\V[a\x01\xF2a\x11rV[`D5\x90`d5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02;W6`#\x82\x01\x12\x15a\x02;Wa\x01'\x94\x81`$a\x02)\x936\x93\x015\x91\x01a\x11\x02V[\x92a\x025\x83\x83\x83a\x11\xF7V[3a\x15\x03V[\x85\x80\xFD[\x91P4a\x02\xDEW\x80`\x03\x196\x01\x12a\x02\xDEWa\x02Ya\x11\\V[\x90`$5\x91\x82\x15\x15\x80\x93\x03a\x02\xDAW`\x01`\x01`\xA0\x1B\x03\x16\x92\x83\x15a\x02\xC5WPa\x02\x823a\x13\xD5V[\x83\x85R` R\x80\x84 `\xFF\x19\x81T\x16`\xFF\x84\x16\x17\x90UQ\x90\x81R\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1` 3\x92\xA3\x80\xF3[\x83`$\x92Q\x91c\x0Ba\x17C`\xE3\x1B\x83R\x82\x01R\xFD[\x84\x80\xFD[\x82\x80\xFD[P\x824a\x01*W\x80`\x03\x196\x01\x12a\x01*W\x81Q\x91\x82\x82\x7F\x80\xBB+c\x8C\xC2\x0B\xC4\xD0\xA6\rf\x94\x0F:\xB4\xA0\x0C\x1D{14\x97\xCA\x82\xFB\x0BJ\xB0\x07\x93\x01\x93\x84Ta\x03&\x81a\x11\xBDV[\x91\x82\x85R` \x96`\x01\x92\x88`\x01\x82\x16\x91\x82`\0\x14a\x03\xC4WPP`\x01\x14a\x03iW[\x85\x88a\x01\xCB\x89a\x03Z\x84\x8A\x03\x85a\x10\xC4V[Q\x92\x82\x84\x93\x84R\x83\x01\x90a\x10RV[\x81R\x86\x93P\x91\x90\x7F\xF4\xBA\xD0\xA6\x92H\xF5\x96\x80\xA4\xF2\xB3\0\x03(\xCE\xC7\x1AA4G\xC9g\x81\xCF\xE5\x99m\xAA\x8CEn[\x82\x84\x10a\x03\xACWPPP\x82\x01\x01\x81a\x03Za\x01\xCB\x88a\x03HV[\x80T\x84\x8A\x01\x86\x01R\x88\x95P\x87\x94\x90\x93\x01\x92\x81\x01a\x03\x92V[`\xFF\x19\x16\x88\x82\x01R\x94\x15\x15`\x05\x1B\x87\x01\x90\x94\x01\x94P\x85\x93Pa\x03Z\x92Pa\x01\xCB\x91P\x89\x90Pa\x03HV[\x83\x824a\x01yW\x81`\x03\x196\x01\x12a\x01yW`\0\x80Q` a\x16\xC1\x839\x81Q\x91RT\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x834a\x01*W\x80`\x03\x196\x01\x12a\x01*Wa\x04=a\x14\xCAV[`\0\x80Q` a\x16\xC1\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[\x90\x91P4a\x02\xDEW` 6`\x03\x19\x01\x12a\x02\xDEWa\x04\xAAa\x11\\V[\x92`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x04\xCEW` \x83a\x04\xC6\x86a\x13\x9CV[T\x90Q\x90\x81R\xF3[`$\x92Q\x91c\"q\x8A\xD9`\xE2\x1B\x83R\x82\x01R\xFD[\x90\x834a\x01*W` 6`\x03\x19\x01\x12a\x01*WPa\x05\x02` \x925a\x14\x82V[\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\xF3[\x90\x91P4a\x02\xDEW` 6`\x03\x19\x01\x12a\x02\xDEW\x805\x91a\x052a\x14\xCAV[`\0\x83\x81R`\0\x80Q` a\x16\xE1\x839\x81Q\x91R` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x06+W\x82\x84R`\0\x80Q` a\x16\xE1\x839\x81Q\x91R` \x81\x90R\x81\x85 T`\x01`\x01`\xA0\x1B\x03\x16\x80\x15\x91\x85\x91\x87\x91\x84\x15a\x06\x0CW[\x83\x83R` R\x84\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x82\x80\xA4a\x05\xF7WPP3\x7F\xCC\x16\xF5\xDB\xB4\x872\x80\x81\\\x1E\xE0\x9D\xBD\x06sl\xFF\xCC\x18D\x12\xCFzq\xA0\xFD\xB7]9|\xA5\x83\x80\xA3\x80\xF3[\x91`$\x92Q\x91c~'2\x89`\xE0\x1B\x83R\x82\x01R\xFD[a\x06\x15\x84a\x16\x8FV[a\x06\x1E\x82a\x13\x9CV[\x80T`\0\x19\x01\x90Ua\x05\x8EV[\x90` `d\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\x14`$\x82\x01Rs\x15\x1B\xDA\xD9[\x88\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`b\x1B`D\x82\x01R\xFD[\x83\x824a\x01yWa\x01'\x90a\x06y6a\x11\x88V[\x91\x92Q\x92a\x06\x86\x84a\x10\x92V[\x85\x84Ra\x025\x83\x83\x83a\x11\xF7V[\x83\x824a\x01yW\x80`\x03\x196\x01\x12a\x01yWa\x06\xAEa\x11\\V[`$\x91\x825\x92a\x06\xBCa\x14\xCAV[\x81Q\x95a\x06\xC8\x87a\x10\x92V[\x85\x87R`\x01`\x01`\xA0\x1B\x03\x96\x84\x88\x16\x94\x85\x15a\x08\xFEW\x86\x88R`\0\x80Q` a\x16\xE1\x839\x81Q\x91R\x87\x87` \x9B\x83\x8DR\x88\x8C T\x16\x80\x15\x15\x93\x84a\x08\xDFW[a\x07\x10\x86a\x13\x9CV[\x80T`\x01\x01\x90U\x83\x8DR\x8DR\x88\x8C \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x83\x17\x90U\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x8C\x80\xA4a\x08\xC9W;a\x07\x8CW[PPP\x90\x7F\x0Fg\x98\xA5`y:T\xC3\xBC\xFE\x86\xA9<\xDE\x1Es\x08}\x94L\x0E\xA2\x05D\x13}A!9h\x85\x83\x92Q\x94\x80\xA3\x81R\xF3[\x90\x83\x94\x95\x91\x88\x88\x85a\x07\xCD\x85\x9B\x97\x9C\x98Q\x94\x85\x93\x84\x93c\n\x85\xBD\x01`\xE1\x1B\x98\x89\x86R3\x90\x86\x01R\x84\x01R\x87`D\x84\x01R`\x80`d\x84\x01R`\x84\x83\x01\x90a\x10RV[\x03\x81\x87\x8BZ\xF1\x84\x91\x81a\x08\x89W[Pa\x083WPPP=`\0\x14a\x08+W=a\x07\xF5\x81a\x10\xE6V[\x90a\x08\x02\x85Q\x92\x83a\x10\xC4V[\x81R\x80\x91\x83=\x92\x01>[\x80Q\x91\x82a\x08(WPPPQ\x91c2PWI`\xE1\x1B\x83R\x82\x01R\xFD[\x01\xFD[P``a\x08\x0CV[\x93\x97\x92\x96\x91\x95\x94\x93`\x01`\x01`\xE0\x1B\x03\x19\x16\x03a\x08uWP\x7F\x0Fg\x98\xA5`y:T\xC3\xBC\xFE\x86\xA9<\xDE\x1Es\x08}\x94L\x0E\xA2\x05D\x13}A!9h\x85\x90P\x83\x87a\x07]V[\x82Qc2PWI`\xE1\x1B\x81R\x90\x81\x01\x84\x90R\xFD[\x90\x91P\x85\x81\x81=\x83\x11a\x08\xC2W[a\x08\xA1\x81\x83a\x10\xC4V[\x81\x01\x03\x12a\x02\xDAWQ`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\x02\xDAW\x90\x8Aa\x07\xDBV[P=a\x08\x97V[PP\x82Qc9\xE3V7`\xE1\x1B\x81R\x90\x81\x01\x86\x90R\xFD[a\x08\xE8\x84a\x16\x8FV[a\x08\xF1\x82a\x13\x9CV[\x80T`\0\x19\x01\x90Ua\x07\x07V[PP\x82Qc2PWI`\xE1\x1B\x81R\x90\x81\x01\x86\x90R\xFD[\x834a\x01*Wa\x01'a\t&6a\x11\x88V[\x91a\x11\xF7V[\x91P4a\x02\xDEW\x80`\x03\x196\x01\x12a\x02\xDEWa\tFa\x11\\V[\x91`$5\x90a\tT\x82a\x14\x82V[\x903\x15\x15\x80a\t\xFCW[\x80a\t\xDFW[a\t\xC8WP`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x93\x82\x91\x85\x91\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x87\x80\xA4\x83R`\0\x80Q` a\x17\x01\x839\x81Q\x91R` R\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90\x91\x17\x90U\x80\xF3[`$\x90\x84Q\x90c\xA9\xFB\xF5\x1F`\xE0\x1B\x82R3\x90\x82\x01R\xFD[Pa\t\xE9\x82a\x13\xD5V[3\x87R` R`\xFF\x84\x87 T\x16\x15a\tdV[P`\x01`\x01`\xA0\x1B\x03\x82\x163\x14\x15a\t^V[\x90\x91P4a\x02\xDEW` 6`\x03\x19\x01\x12a\x02\xDEW\x91` \x925a\n1\x81a\x14\x82V[P\x81R`\0\x80Q` a\x17\x01\x839\x81Q\x91R\x83R\x81\x90 T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\xF3[\x90\x91P4a\x02\xDEW``6`\x03\x19\x01\x12a\x02\xDEWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x805\x82\x81\x11a\x02\xDAWa\n\x8E\x906\x90\x83\x01a\x11>V[`$5\x83\x81\x11a\x02;Wa\n\xA5\x906\x90\x84\x01a\x11>V[`D5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x0E\xC1W\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x94\x85T\x94`\xFF\x86\x89\x1C\x16\x15\x94\x82\x87\x16\x96\x87\x15\x80a\x0E\xBAW[`\x01\x80\x99\x14\x90\x81a\x0E\xB0W[\x15\x90\x81a\x0E\xA7W[Pa\x0E\x97Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x88\x17\x89U\x86a\x0ExW[Pa\x0B*a\x16NV[a\x0B2a\x16NV[\x80Q\x83\x81\x11a\x0EeW\x80\x7F\x80\xBB+c\x8C\xC2\x0B\xC4\xD0\xA6\rf\x94\x0F:\xB4\xA0\x0C\x1D{14\x97\xCA\x82\xFB\x0BJ\xB0\x07\x93\0\x92a\x0Bh\x84Ta\x11\xBDV[`\x1F\x81\x11a\r\xF3W[P` \x90\x8D`\x1F\x84\x11`\x01\x14a\rvW\x92a\rkW[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x88\x1B\x17\x90U[\x82Q\x91\x82\x11a\rXWP\x7F\x80\xBB+c\x8C\xC2\x0B\xC4\xD0\xA6\rf\x94\x0F:\xB4\xA0\x0C\x1D{14\x97\xCA\x82\xFB\x0BJ\xB0\x07\x93\x01\x91a\x0B\xD1\x83Ta\x11\xBDV[`\x1F\x81\x11a\x0C\xF3W[P` \x90`\x1F\x83\x11`\x01\x14a\x0CpWa\x0C\x1C\x94\x93\x92\x91\x8A\x91\x83a\x0CeW[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x86\x1B\x17\x90U[a\x0C\x14a\x16NV[a\x01\"a\x16NV[a\x0C$W\x83\x80\xF3[\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x92` \x92h\xFF\0\0\0\0\0\0\0\0\x19\x81T\x16\x90UQ\x90\x81R\xA18\x80\x80\x83\x80\xF3[\x01Q\x90P8\x80a\x0B\xF8V[\x83\x8AR\x93\x92\x91\x86\x91\x7F\xF4\xBA\xD0\xA6\x92H\xF5\x96\x80\xA4\xF2\xB3\0\x03(\xCE\xC7\x1AA4G\xC9g\x81\xCF\xE5\x99m\xAA\x8CEn\x90`\x1F\x19\x83\x16\x8C[\x81\x81\x10a\x0C\xDBWP\x96\x83a\x0C\x1C\x98\x10a\x0C\xC2W[PPP\x81\x1B\x01\x90Ua\x0C\x0CV[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80a\x0C\xB5V[\x82\x89\x01Q\x84U\x8A\x95\x90\x93\x01\x92` \x92\x83\x01\x92\x01a\x0C\xA1V[\x83\x8AR\x7F\xF4\xBA\xD0\xA6\x92H\xF5\x96\x80\xA4\xF2\xB3\0\x03(\xCE\xC7\x1AA4G\xC9g\x81\xCF\xE5\x99m\xAA\x8CEn`\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\rNW[`\x1F\x01`\x05\x1C\x01\x90\x87\x90[\x82\x81\x10a\rCWPPa\x0B\xDAV[\x8B\x81U\x01\x87\x90a\r5V[\x90\x91P\x81\x90a\r*V[cNH{q`\xE0\x1B\x89R`A\x90R`$\x88\xFD[\x01Q\x90P8\x80a\x0B\x87V[\x91\x90\x8B\x94P`\x1F\x19\x84\x16\x86\x84R\x7F7\xC5\x8Cy\x9Bf\t#K\x94^\x88)\x12\xEE\x9A\xD3IH\xA1\xDF\xAA \xA9t\x85\xE1\xA7u+\xBF\x81\x93[\x81\x81\x10a\r\xDBWP\x84\x11a\r\xC2W[PPP\x81\x1B\x01\x90Ua\x0B\x9BV[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80a\r\xB5V[\x82\x84\x01Q\x85U\x8D\x96\x90\x94\x01\x93` \x93\x84\x01\x93\x01a\r\xA6V[\x90\x91P\x83\x8DR\x7F7\xC5\x8Cy\x9Bf\t#K\x94^\x88)\x12\xEE\x9A\xD3IH\xA1\xDF\xAA \xA9t\x85\xE1\xA7u+\xBF\x81`\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\x0E[W[\x8E\x85\x94\x93\x92`\x1F\x8E\x93\x01`\x05\x1C\x01\x92\x90[\x83\x82\x10a\x0EMWPPPa\x0BqV[\x81U\x85\x94P\x8C\x91\x01\x8Fa\x0E>V[\x90\x91P\x81\x90a\x0E-V[cNH{q`\xE0\x1B\x8BR`A\x83R`$\x8B\xFD[h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16h\x01\0\0\0\0\0\0\0\x01\x17\x88U8a\x0B!V[\x89Qc\xF9.\xE8\xA9`\xE0\x1B\x81R\x83\x90\xFD[\x90P\x158a\x0B\x06V[0;\x15\x91Pa\n\xFEV[P\x86a\n\xF2V[\x86\x80\xFD[P\x824a\x01*W\x80`\x03\x196\x01\x12a\x01*W\x81Q\x91\x82\x82\x7F\x80\xBB+c\x8C\xC2\x0B\xC4\xD0\xA6\rf\x94\x0F:\xB4\xA0\x0C\x1D{14\x97\xCA\x82\xFB\x0BJ\xB0\x07\x93\0\x93\x84Ta\x0F\t\x81a\x11\xBDV[\x91\x82\x85R` \x96`\x01\x92\x88`\x01\x82\x16\x91\x82`\0\x14a\x03\xC4WPP`\x01\x14a\x0F<W\x85\x88a\x01\xCB\x89a\x03Z\x84\x8A\x03\x85a\x10\xC4V[\x81R\x86\x93P\x91\x90\x7F7\xC5\x8Cy\x9Bf\t#K\x94^\x88)\x12\xEE\x9A\xD3IH\xA1\xDF\xAA \xA9t\x85\xE1\xA7u+\xBF\x81[\x82\x84\x10a\x0F\x7FWPPP\x82\x01\x01\x81a\x03Za\x01\xCB\x88a\x03HV[\x80T\x84\x8A\x01\x86\x01R\x88\x95P\x87\x94\x90\x93\x01\x92\x81\x01a\x0FeV[\x90\x91P4a\x02\xDEW` 6`\x03\x19\x01\x12a\x02\xDEW5\x90c\xFF\xFF\xFF\xFF`\xE0\x1B\x82\x16\x80\x92\x03a\x02\xDEW` \x92Pc\x80\xACX\xCD`\xE0\x1B\x82\x14\x91\x82\x15a\x0F\xF5W[\x82\x15a\x0F\xE4W[PQ\x90\x15\x15\x81R\xF3[c\x01\xFF\xC9\xA7`\xE0\x1B\x14\x91P8a\x0F\xDBV[c[^\x13\x9F`\xE0\x1B\x81\x14\x92Pa\x0F\xD4V[\x90\x834a\x01*W` 6`\x03\x19\x01\x12a\x01*WPa\x10I` \x925`\0R`\0\x80Q` a\x16\xE1\x839\x81Q\x91R` R`\x01\x80`\xA0\x1B\x03`@`\0 T\x16\x15\x15\x90V[\x90Q\x90\x15\x15\x81R\xF3[\x91\x90\x82Q\x92\x83\x82R`\0[\x84\x81\x10a\x10~WPP\x82`\0` \x80\x94\x95\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[` \x81\x83\x01\x81\x01Q\x84\x83\x01\x82\x01R\x01a\x10]V[` \x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x10\xAEW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x10\xAEW`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x10\xAEW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x11\x0E\x82a\x10\xE6V[\x91a\x11\x1C`@Q\x93\x84a\x10\xC4V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x119W\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[`\0\x80\xFD[\x90\x80`\x1F\x83\x01\x12\x15a\x119W\x81` a\x11Y\x935\x91\x01a\x11\x02V[\x90V[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x119WV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x119WV[``\x90`\x03\x19\x01\x12a\x119W`\x01`\x01`\xA0\x1B\x03\x90`\x045\x82\x81\x16\x81\x03a\x119W\x91`$5\x90\x81\x16\x81\x03a\x119W\x90`D5\x90V[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x11\xEDW[` \x83\x10\x14a\x11\xD7WV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\x11\xCCV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x93\x91\x84\x15a\x13\x83W\x82`\0\x95\x81\x87R`\0\x80Q` a\x16\xE1\x839\x81Q\x91R\x95\x86` R`@\x97\x85\x89\x82 T\x16\x97\x88\x923\x15\x15\x80a\x12\xDDW[P\x90a\x12l\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93\x92\x85a\x12\xBEWa\x13\x9CV[\x80T`\x01\x01\x90U\x85\x82R` R\x89\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x85\x17\x90U\x80\xA4\x16\x92\x83\x83\x03a\x12\x9DWPPPPV[`d\x94PQ\x92cd(={`\xE0\x1B\x84R`\x04\x84\x01R`$\x83\x01R`D\x82\x01R\xFD[a\x12\xC7\x88a\x16\x8FV[a\x12\xD0\x86a\x13\x9CV[\x80T`\0\x19\x01\x90Ua\x13\x9CV[\x91\x93P\x91\x93\x94P\x80a\x134W[\x15a\x12\xFBW\x91\x87\x91\x87\x94\x938a\x12:V[\x88\x87\x89a\x13\x18W`$\x91Q\x90c~'2\x89`\xE0\x1B\x82R`\x04\x82\x01R\xFD[`D\x91Q\x90c\x17~\x80/`\xE0\x1B\x82R3`\x04\x83\x01R`$\x82\x01R\xFD[P3\x88\x14\x80\x15a\x13gW[\x80a\x12\xEAWP\x86\x83R`\0\x80Q` a\x17\x01\x839\x81Q\x91R` R3\x86\x8A\x85 T\x16\x14a\x12\xEAV[Pa\x13q\x88a\x13\xD5V[3\x84R` R`\xFF\x89\x84 T\x16a\x13?V[`@Qc2PWI`\xE1\x1B\x81R`\0`\x04\x82\x01R`$\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R\x7F\x80\xBB+c\x8C\xC2\x0B\xC4\xD0\xA6\rf\x94\x0F:\xB4\xA0\x0C\x1D{14\x97\xCA\x82\xFB\x0BJ\xB0\x07\x93\x03` R`@\x90 \x90V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R\x7F\x80\xBB+c\x8C\xC2\x0B\xC4\xD0\xA6\rf\x94\x0F:\xB4\xA0\x0C\x1D{14\x97\xCA\x82\xFB\x0BJ\xB0\x07\x93\x05` R`@\x90 \x90V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x81\x15a\x14iW`\0\x80Q` a\x16\xC1\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x84\x17\x90\x91U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`\0\x80\xA3V[`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x90\xFD[`\0\x81\x81R`\0\x80Q` a\x16\xE1\x839\x81Q\x91R` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a\x14\xB2WP\x90V[`$\x90`@Q\x90c~'2\x89`\xE0\x1B\x82R`\x04\x82\x01R\xFD[`\0\x80Q` a\x16\xC1\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x163\x03a\x14\xEBWV[`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x90\xFD[\x90\x91\x92\x83;a\x15\x14W[PPPPPV[`@Qc\n\x85\xBD\x01`\xE1\x1B\x80\x82R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x83\x01R\x93\x83\x16`$\x82\x01R`D\x81\x01\x91\x90\x91R`\x80`d\x82\x01R` \x95\x91\x90\x93\x16\x93\x90\x91\x90\x83\x90a\x15d\x90`\x84\x83\x01\x90a\x10RV[\x03\x92\x85\x81`\0\x95\x81\x87\x89Z\xF1\x84\x91\x81a\x16\x0EW[Pa\x15\xD8WPPP=`\0\x14a\x15\xD0W=a\x15\x92\x81a\x10\xE6V[\x90a\x15\xA0`@Q\x92\x83a\x10\xC4V[\x81R\x80\x91\x84=\x92\x01>[\x80Q\x92\x83a\x15\xCBW`@Qc2PWI`\xE1\x1B\x81R`\x04\x81\x01\x84\x90R`$\x90\xFD[\x01\x90P\xFD[P``a\x15\xAAV[\x91\x94P\x91Pc\xFF\xFF\xFF\xFF`\xE0\x1B\x16\x03a\x15\xF6WP8\x80\x80\x80\x80a\x15\rV[`$\x90`@Q\x90c2PWI`\xE1\x1B\x82R`\x04\x82\x01R\xFD[\x90\x91P\x86\x81\x81=\x83\x11a\x16GW[a\x16&\x81\x83a\x10\xC4V[\x81\x01\x03\x12a\x02\xDAWQ`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\x02\xDAW\x908a\x15xV[P=a\x16\x1CV[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15a\x16}WV[`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x90\xFD[`\0R`\0\x80Q` a\x17\x01\x839\x81Q\x91R` R`@`\0 k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x81T\x16\x90UV\xFE\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x80\xBB+c\x8C\xC2\x0B\xC4\xD0\xA6\rf\x94\x0F:\xB4\xA0\x0C\x1D{14\x97\xCA\x82\xFB\x0BJ\xB0\x07\x93\x02\x80\xBB+c\x8C\xC2\x0B\xC4\xD0\xA6\rf\x94\x0F:\xB4\xA0\x0C\x1D{14\x97\xCA\x82\xFB\x0BJ\xB0\x07\x93\x04\xA2dipfsX\"\x12 ?\n\x08\x0F\x16\x96\xF19xMP\x93\xA3\x84\x86\x86\xC9\xBC'1Pk/\xEC\x1D\xF9\xE2Gh$Y?dsolcC\0\x08\x17\x003";
    /// The bytecode of the contract.
    pub static SHARE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`@`\x80\x81R`\x04\x806\x10\x15a\0\x14W`\0\x80\xFD[`\0\x91\x825`\xE0\x1C\x91\x82b\x92?\x9E\x14a\x10\x06W\x82c\x01\xFF\xC9\xA7\x14a\x0F\x97W\x82c\x06\xFD\xDE\x03\x14a\x0E\xC5W\x82c\x07\x7F\"J\x14a\nZW\x82c\x08\x18\x12\xFC\x14a\n\x0FW\x82c\t^\xA7\xB3\x14a\t,W\x82c#\xB8r\xDD\x14a\t\x14W\x82c@\xC1\x0F\x19\x14a\x06\x94W\x82cB\x84.\x0E\x14a\x06eW\x82cB\x96lh\x14a\x05\x13W\x82ccR!\x1E\x14a\x04\xE2W\x82cp\xA0\x821\x14a\x04\x8EW\x82cqP\x18\xA6\x14a\x04$W\x82c\x8D\xA5\xCB[\x14a\x03\xEEW\x82c\x95\xD8\x9BA\x14a\x02\xE2W\x82c\xA2,\xB4e\x14a\x02?W\x82c\xB8\x8DO\xDE\x14a\x01\xCFW\x82c\xC8{V\xDD\x14a\x01}WP\x81c\xE9\x85\xE9\xC5\x14a\x01-WPc\xF2\xFD\xE3\x8B\x14a\0\xFEW`\0\x80\xFD[4a\x01*W` 6`\x03\x19\x01\x12a\x01*Wa\x01'a\x01\x1Aa\x11\\V[a\x01\"a\x14\xCAV[a\x14\x0EV[\x80\xF3[\x80\xFD[\x90P4a\x01yW\x80`\x03\x196\x01\x12a\x01yW`\xFF\x81` \x93a\x01Ma\x11\\V[a\x01^a\x01Xa\x11rV[\x91a\x13\xD5V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x82R\x85R T\x91Q\x91\x16\x15\x15\x81R\xF3[P\x80\xFD[\x83\x824a\x01yW` 6`\x03\x19\x01\x12a\x01yWa\x01\x9Da\x01\xCB\x935a\x14\x82V[P\x81\x81Qa\x01\xAA\x81a\x10\x92V[R\x80Q\x91a\x01\xB7\x83a\x10\x92V[\x82RQ\x91\x82\x91` \x83R` \x83\x01\x90a\x10RV[\x03\x90\xF3[\x83\x904a\x01yW`\x806`\x03\x19\x01\x12a\x01yWa\x01\xEAa\x11\\V[a\x01\xF2a\x11rV[`D5\x90`d5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02;W6`#\x82\x01\x12\x15a\x02;Wa\x01'\x94\x81`$a\x02)\x936\x93\x015\x91\x01a\x11\x02V[\x92a\x025\x83\x83\x83a\x11\xF7V[3a\x15\x03V[\x85\x80\xFD[\x91P4a\x02\xDEW\x80`\x03\x196\x01\x12a\x02\xDEWa\x02Ya\x11\\V[\x90`$5\x91\x82\x15\x15\x80\x93\x03a\x02\xDAW`\x01`\x01`\xA0\x1B\x03\x16\x92\x83\x15a\x02\xC5WPa\x02\x823a\x13\xD5V[\x83\x85R` R\x80\x84 `\xFF\x19\x81T\x16`\xFF\x84\x16\x17\x90UQ\x90\x81R\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1` 3\x92\xA3\x80\xF3[\x83`$\x92Q\x91c\x0Ba\x17C`\xE3\x1B\x83R\x82\x01R\xFD[\x84\x80\xFD[\x82\x80\xFD[P\x824a\x01*W\x80`\x03\x196\x01\x12a\x01*W\x81Q\x91\x82\x82\x7F\x80\xBB+c\x8C\xC2\x0B\xC4\xD0\xA6\rf\x94\x0F:\xB4\xA0\x0C\x1D{14\x97\xCA\x82\xFB\x0BJ\xB0\x07\x93\x01\x93\x84Ta\x03&\x81a\x11\xBDV[\x91\x82\x85R` \x96`\x01\x92\x88`\x01\x82\x16\x91\x82`\0\x14a\x03\xC4WPP`\x01\x14a\x03iW[\x85\x88a\x01\xCB\x89a\x03Z\x84\x8A\x03\x85a\x10\xC4V[Q\x92\x82\x84\x93\x84R\x83\x01\x90a\x10RV[\x81R\x86\x93P\x91\x90\x7F\xF4\xBA\xD0\xA6\x92H\xF5\x96\x80\xA4\xF2\xB3\0\x03(\xCE\xC7\x1AA4G\xC9g\x81\xCF\xE5\x99m\xAA\x8CEn[\x82\x84\x10a\x03\xACWPPP\x82\x01\x01\x81a\x03Za\x01\xCB\x88a\x03HV[\x80T\x84\x8A\x01\x86\x01R\x88\x95P\x87\x94\x90\x93\x01\x92\x81\x01a\x03\x92V[`\xFF\x19\x16\x88\x82\x01R\x94\x15\x15`\x05\x1B\x87\x01\x90\x94\x01\x94P\x85\x93Pa\x03Z\x92Pa\x01\xCB\x91P\x89\x90Pa\x03HV[\x83\x824a\x01yW\x81`\x03\x196\x01\x12a\x01yW`\0\x80Q` a\x16\xC1\x839\x81Q\x91RT\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x834a\x01*W\x80`\x03\x196\x01\x12a\x01*Wa\x04=a\x14\xCAV[`\0\x80Q` a\x16\xC1\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[\x90\x91P4a\x02\xDEW` 6`\x03\x19\x01\x12a\x02\xDEWa\x04\xAAa\x11\\V[\x92`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x04\xCEW` \x83a\x04\xC6\x86a\x13\x9CV[T\x90Q\x90\x81R\xF3[`$\x92Q\x91c\"q\x8A\xD9`\xE2\x1B\x83R\x82\x01R\xFD[\x90\x834a\x01*W` 6`\x03\x19\x01\x12a\x01*WPa\x05\x02` \x925a\x14\x82V[\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\xF3[\x90\x91P4a\x02\xDEW` 6`\x03\x19\x01\x12a\x02\xDEW\x805\x91a\x052a\x14\xCAV[`\0\x83\x81R`\0\x80Q` a\x16\xE1\x839\x81Q\x91R` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x06+W\x82\x84R`\0\x80Q` a\x16\xE1\x839\x81Q\x91R` \x81\x90R\x81\x85 T`\x01`\x01`\xA0\x1B\x03\x16\x80\x15\x91\x85\x91\x87\x91\x84\x15a\x06\x0CW[\x83\x83R` R\x84\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x82\x80\xA4a\x05\xF7WPP3\x7F\xCC\x16\xF5\xDB\xB4\x872\x80\x81\\\x1E\xE0\x9D\xBD\x06sl\xFF\xCC\x18D\x12\xCFzq\xA0\xFD\xB7]9|\xA5\x83\x80\xA3\x80\xF3[\x91`$\x92Q\x91c~'2\x89`\xE0\x1B\x83R\x82\x01R\xFD[a\x06\x15\x84a\x16\x8FV[a\x06\x1E\x82a\x13\x9CV[\x80T`\0\x19\x01\x90Ua\x05\x8EV[\x90` `d\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\x14`$\x82\x01Rs\x15\x1B\xDA\xD9[\x88\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`b\x1B`D\x82\x01R\xFD[\x83\x824a\x01yWa\x01'\x90a\x06y6a\x11\x88V[\x91\x92Q\x92a\x06\x86\x84a\x10\x92V[\x85\x84Ra\x025\x83\x83\x83a\x11\xF7V[\x83\x824a\x01yW\x80`\x03\x196\x01\x12a\x01yWa\x06\xAEa\x11\\V[`$\x91\x825\x92a\x06\xBCa\x14\xCAV[\x81Q\x95a\x06\xC8\x87a\x10\x92V[\x85\x87R`\x01`\x01`\xA0\x1B\x03\x96\x84\x88\x16\x94\x85\x15a\x08\xFEW\x86\x88R`\0\x80Q` a\x16\xE1\x839\x81Q\x91R\x87\x87` \x9B\x83\x8DR\x88\x8C T\x16\x80\x15\x15\x93\x84a\x08\xDFW[a\x07\x10\x86a\x13\x9CV[\x80T`\x01\x01\x90U\x83\x8DR\x8DR\x88\x8C \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x83\x17\x90U\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x8C\x80\xA4a\x08\xC9W;a\x07\x8CW[PPP\x90\x7F\x0Fg\x98\xA5`y:T\xC3\xBC\xFE\x86\xA9<\xDE\x1Es\x08}\x94L\x0E\xA2\x05D\x13}A!9h\x85\x83\x92Q\x94\x80\xA3\x81R\xF3[\x90\x83\x94\x95\x91\x88\x88\x85a\x07\xCD\x85\x9B\x97\x9C\x98Q\x94\x85\x93\x84\x93c\n\x85\xBD\x01`\xE1\x1B\x98\x89\x86R3\x90\x86\x01R\x84\x01R\x87`D\x84\x01R`\x80`d\x84\x01R`\x84\x83\x01\x90a\x10RV[\x03\x81\x87\x8BZ\xF1\x84\x91\x81a\x08\x89W[Pa\x083WPPP=`\0\x14a\x08+W=a\x07\xF5\x81a\x10\xE6V[\x90a\x08\x02\x85Q\x92\x83a\x10\xC4V[\x81R\x80\x91\x83=\x92\x01>[\x80Q\x91\x82a\x08(WPPPQ\x91c2PWI`\xE1\x1B\x83R\x82\x01R\xFD[\x01\xFD[P``a\x08\x0CV[\x93\x97\x92\x96\x91\x95\x94\x93`\x01`\x01`\xE0\x1B\x03\x19\x16\x03a\x08uWP\x7F\x0Fg\x98\xA5`y:T\xC3\xBC\xFE\x86\xA9<\xDE\x1Es\x08}\x94L\x0E\xA2\x05D\x13}A!9h\x85\x90P\x83\x87a\x07]V[\x82Qc2PWI`\xE1\x1B\x81R\x90\x81\x01\x84\x90R\xFD[\x90\x91P\x85\x81\x81=\x83\x11a\x08\xC2W[a\x08\xA1\x81\x83a\x10\xC4V[\x81\x01\x03\x12a\x02\xDAWQ`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\x02\xDAW\x90\x8Aa\x07\xDBV[P=a\x08\x97V[PP\x82Qc9\xE3V7`\xE1\x1B\x81R\x90\x81\x01\x86\x90R\xFD[a\x08\xE8\x84a\x16\x8FV[a\x08\xF1\x82a\x13\x9CV[\x80T`\0\x19\x01\x90Ua\x07\x07V[PP\x82Qc2PWI`\xE1\x1B\x81R\x90\x81\x01\x86\x90R\xFD[\x834a\x01*Wa\x01'a\t&6a\x11\x88V[\x91a\x11\xF7V[\x91P4a\x02\xDEW\x80`\x03\x196\x01\x12a\x02\xDEWa\tFa\x11\\V[\x91`$5\x90a\tT\x82a\x14\x82V[\x903\x15\x15\x80a\t\xFCW[\x80a\t\xDFW[a\t\xC8WP`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x93\x82\x91\x85\x91\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x87\x80\xA4\x83R`\0\x80Q` a\x17\x01\x839\x81Q\x91R` R\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90\x91\x17\x90U\x80\xF3[`$\x90\x84Q\x90c\xA9\xFB\xF5\x1F`\xE0\x1B\x82R3\x90\x82\x01R\xFD[Pa\t\xE9\x82a\x13\xD5V[3\x87R` R`\xFF\x84\x87 T\x16\x15a\tdV[P`\x01`\x01`\xA0\x1B\x03\x82\x163\x14\x15a\t^V[\x90\x91P4a\x02\xDEW` 6`\x03\x19\x01\x12a\x02\xDEW\x91` \x925a\n1\x81a\x14\x82V[P\x81R`\0\x80Q` a\x17\x01\x839\x81Q\x91R\x83R\x81\x90 T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\xF3[\x90\x91P4a\x02\xDEW``6`\x03\x19\x01\x12a\x02\xDEWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x805\x82\x81\x11a\x02\xDAWa\n\x8E\x906\x90\x83\x01a\x11>V[`$5\x83\x81\x11a\x02;Wa\n\xA5\x906\x90\x84\x01a\x11>V[`D5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x0E\xC1W\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x94\x85T\x94`\xFF\x86\x89\x1C\x16\x15\x94\x82\x87\x16\x96\x87\x15\x80a\x0E\xBAW[`\x01\x80\x99\x14\x90\x81a\x0E\xB0W[\x15\x90\x81a\x0E\xA7W[Pa\x0E\x97Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x88\x17\x89U\x86a\x0ExW[Pa\x0B*a\x16NV[a\x0B2a\x16NV[\x80Q\x83\x81\x11a\x0EeW\x80\x7F\x80\xBB+c\x8C\xC2\x0B\xC4\xD0\xA6\rf\x94\x0F:\xB4\xA0\x0C\x1D{14\x97\xCA\x82\xFB\x0BJ\xB0\x07\x93\0\x92a\x0Bh\x84Ta\x11\xBDV[`\x1F\x81\x11a\r\xF3W[P` \x90\x8D`\x1F\x84\x11`\x01\x14a\rvW\x92a\rkW[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x88\x1B\x17\x90U[\x82Q\x91\x82\x11a\rXWP\x7F\x80\xBB+c\x8C\xC2\x0B\xC4\xD0\xA6\rf\x94\x0F:\xB4\xA0\x0C\x1D{14\x97\xCA\x82\xFB\x0BJ\xB0\x07\x93\x01\x91a\x0B\xD1\x83Ta\x11\xBDV[`\x1F\x81\x11a\x0C\xF3W[P` \x90`\x1F\x83\x11`\x01\x14a\x0CpWa\x0C\x1C\x94\x93\x92\x91\x8A\x91\x83a\x0CeW[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x86\x1B\x17\x90U[a\x0C\x14a\x16NV[a\x01\"a\x16NV[a\x0C$W\x83\x80\xF3[\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x92` \x92h\xFF\0\0\0\0\0\0\0\0\x19\x81T\x16\x90UQ\x90\x81R\xA18\x80\x80\x83\x80\xF3[\x01Q\x90P8\x80a\x0B\xF8V[\x83\x8AR\x93\x92\x91\x86\x91\x7F\xF4\xBA\xD0\xA6\x92H\xF5\x96\x80\xA4\xF2\xB3\0\x03(\xCE\xC7\x1AA4G\xC9g\x81\xCF\xE5\x99m\xAA\x8CEn\x90`\x1F\x19\x83\x16\x8C[\x81\x81\x10a\x0C\xDBWP\x96\x83a\x0C\x1C\x98\x10a\x0C\xC2W[PPP\x81\x1B\x01\x90Ua\x0C\x0CV[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80a\x0C\xB5V[\x82\x89\x01Q\x84U\x8A\x95\x90\x93\x01\x92` \x92\x83\x01\x92\x01a\x0C\xA1V[\x83\x8AR\x7F\xF4\xBA\xD0\xA6\x92H\xF5\x96\x80\xA4\xF2\xB3\0\x03(\xCE\xC7\x1AA4G\xC9g\x81\xCF\xE5\x99m\xAA\x8CEn`\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\rNW[`\x1F\x01`\x05\x1C\x01\x90\x87\x90[\x82\x81\x10a\rCWPPa\x0B\xDAV[\x8B\x81U\x01\x87\x90a\r5V[\x90\x91P\x81\x90a\r*V[cNH{q`\xE0\x1B\x89R`A\x90R`$\x88\xFD[\x01Q\x90P8\x80a\x0B\x87V[\x91\x90\x8B\x94P`\x1F\x19\x84\x16\x86\x84R\x7F7\xC5\x8Cy\x9Bf\t#K\x94^\x88)\x12\xEE\x9A\xD3IH\xA1\xDF\xAA \xA9t\x85\xE1\xA7u+\xBF\x81\x93[\x81\x81\x10a\r\xDBWP\x84\x11a\r\xC2W[PPP\x81\x1B\x01\x90Ua\x0B\x9BV[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80a\r\xB5V[\x82\x84\x01Q\x85U\x8D\x96\x90\x94\x01\x93` \x93\x84\x01\x93\x01a\r\xA6V[\x90\x91P\x83\x8DR\x7F7\xC5\x8Cy\x9Bf\t#K\x94^\x88)\x12\xEE\x9A\xD3IH\xA1\xDF\xAA \xA9t\x85\xE1\xA7u+\xBF\x81`\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\x0E[W[\x8E\x85\x94\x93\x92`\x1F\x8E\x93\x01`\x05\x1C\x01\x92\x90[\x83\x82\x10a\x0EMWPPPa\x0BqV[\x81U\x85\x94P\x8C\x91\x01\x8Fa\x0E>V[\x90\x91P\x81\x90a\x0E-V[cNH{q`\xE0\x1B\x8BR`A\x83R`$\x8B\xFD[h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16h\x01\0\0\0\0\0\0\0\x01\x17\x88U8a\x0B!V[\x89Qc\xF9.\xE8\xA9`\xE0\x1B\x81R\x83\x90\xFD[\x90P\x158a\x0B\x06V[0;\x15\x91Pa\n\xFEV[P\x86a\n\xF2V[\x86\x80\xFD[P\x824a\x01*W\x80`\x03\x196\x01\x12a\x01*W\x81Q\x91\x82\x82\x7F\x80\xBB+c\x8C\xC2\x0B\xC4\xD0\xA6\rf\x94\x0F:\xB4\xA0\x0C\x1D{14\x97\xCA\x82\xFB\x0BJ\xB0\x07\x93\0\x93\x84Ta\x0F\t\x81a\x11\xBDV[\x91\x82\x85R` \x96`\x01\x92\x88`\x01\x82\x16\x91\x82`\0\x14a\x03\xC4WPP`\x01\x14a\x0F<W\x85\x88a\x01\xCB\x89a\x03Z\x84\x8A\x03\x85a\x10\xC4V[\x81R\x86\x93P\x91\x90\x7F7\xC5\x8Cy\x9Bf\t#K\x94^\x88)\x12\xEE\x9A\xD3IH\xA1\xDF\xAA \xA9t\x85\xE1\xA7u+\xBF\x81[\x82\x84\x10a\x0F\x7FWPPP\x82\x01\x01\x81a\x03Za\x01\xCB\x88a\x03HV[\x80T\x84\x8A\x01\x86\x01R\x88\x95P\x87\x94\x90\x93\x01\x92\x81\x01a\x0FeV[\x90\x91P4a\x02\xDEW` 6`\x03\x19\x01\x12a\x02\xDEW5\x90c\xFF\xFF\xFF\xFF`\xE0\x1B\x82\x16\x80\x92\x03a\x02\xDEW` \x92Pc\x80\xACX\xCD`\xE0\x1B\x82\x14\x91\x82\x15a\x0F\xF5W[\x82\x15a\x0F\xE4W[PQ\x90\x15\x15\x81R\xF3[c\x01\xFF\xC9\xA7`\xE0\x1B\x14\x91P8a\x0F\xDBV[c[^\x13\x9F`\xE0\x1B\x81\x14\x92Pa\x0F\xD4V[\x90\x834a\x01*W` 6`\x03\x19\x01\x12a\x01*WPa\x10I` \x925`\0R`\0\x80Q` a\x16\xE1\x839\x81Q\x91R` R`\x01\x80`\xA0\x1B\x03`@`\0 T\x16\x15\x15\x90V[\x90Q\x90\x15\x15\x81R\xF3[\x91\x90\x82Q\x92\x83\x82R`\0[\x84\x81\x10a\x10~WPP\x82`\0` \x80\x94\x95\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[` \x81\x83\x01\x81\x01Q\x84\x83\x01\x82\x01R\x01a\x10]V[` \x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x10\xAEW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x10\xAEW`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x10\xAEW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x11\x0E\x82a\x10\xE6V[\x91a\x11\x1C`@Q\x93\x84a\x10\xC4V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x119W\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[`\0\x80\xFD[\x90\x80`\x1F\x83\x01\x12\x15a\x119W\x81` a\x11Y\x935\x91\x01a\x11\x02V[\x90V[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x119WV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x119WV[``\x90`\x03\x19\x01\x12a\x119W`\x01`\x01`\xA0\x1B\x03\x90`\x045\x82\x81\x16\x81\x03a\x119W\x91`$5\x90\x81\x16\x81\x03a\x119W\x90`D5\x90V[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x11\xEDW[` \x83\x10\x14a\x11\xD7WV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\x11\xCCV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x93\x91\x84\x15a\x13\x83W\x82`\0\x95\x81\x87R`\0\x80Q` a\x16\xE1\x839\x81Q\x91R\x95\x86` R`@\x97\x85\x89\x82 T\x16\x97\x88\x923\x15\x15\x80a\x12\xDDW[P\x90a\x12l\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93\x92\x85a\x12\xBEWa\x13\x9CV[\x80T`\x01\x01\x90U\x85\x82R` R\x89\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x85\x17\x90U\x80\xA4\x16\x92\x83\x83\x03a\x12\x9DWPPPPV[`d\x94PQ\x92cd(={`\xE0\x1B\x84R`\x04\x84\x01R`$\x83\x01R`D\x82\x01R\xFD[a\x12\xC7\x88a\x16\x8FV[a\x12\xD0\x86a\x13\x9CV[\x80T`\0\x19\x01\x90Ua\x13\x9CV[\x91\x93P\x91\x93\x94P\x80a\x134W[\x15a\x12\xFBW\x91\x87\x91\x87\x94\x938a\x12:V[\x88\x87\x89a\x13\x18W`$\x91Q\x90c~'2\x89`\xE0\x1B\x82R`\x04\x82\x01R\xFD[`D\x91Q\x90c\x17~\x80/`\xE0\x1B\x82R3`\x04\x83\x01R`$\x82\x01R\xFD[P3\x88\x14\x80\x15a\x13gW[\x80a\x12\xEAWP\x86\x83R`\0\x80Q` a\x17\x01\x839\x81Q\x91R` R3\x86\x8A\x85 T\x16\x14a\x12\xEAV[Pa\x13q\x88a\x13\xD5V[3\x84R` R`\xFF\x89\x84 T\x16a\x13?V[`@Qc2PWI`\xE1\x1B\x81R`\0`\x04\x82\x01R`$\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R\x7F\x80\xBB+c\x8C\xC2\x0B\xC4\xD0\xA6\rf\x94\x0F:\xB4\xA0\x0C\x1D{14\x97\xCA\x82\xFB\x0BJ\xB0\x07\x93\x03` R`@\x90 \x90V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R\x7F\x80\xBB+c\x8C\xC2\x0B\xC4\xD0\xA6\rf\x94\x0F:\xB4\xA0\x0C\x1D{14\x97\xCA\x82\xFB\x0BJ\xB0\x07\x93\x05` R`@\x90 \x90V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x81\x15a\x14iW`\0\x80Q` a\x16\xC1\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x84\x17\x90\x91U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`\0\x80\xA3V[`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x90\xFD[`\0\x81\x81R`\0\x80Q` a\x16\xE1\x839\x81Q\x91R` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a\x14\xB2WP\x90V[`$\x90`@Q\x90c~'2\x89`\xE0\x1B\x82R`\x04\x82\x01R\xFD[`\0\x80Q` a\x16\xC1\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x163\x03a\x14\xEBWV[`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x90\xFD[\x90\x91\x92\x83;a\x15\x14W[PPPPPV[`@Qc\n\x85\xBD\x01`\xE1\x1B\x80\x82R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x83\x01R\x93\x83\x16`$\x82\x01R`D\x81\x01\x91\x90\x91R`\x80`d\x82\x01R` \x95\x91\x90\x93\x16\x93\x90\x91\x90\x83\x90a\x15d\x90`\x84\x83\x01\x90a\x10RV[\x03\x92\x85\x81`\0\x95\x81\x87\x89Z\xF1\x84\x91\x81a\x16\x0EW[Pa\x15\xD8WPPP=`\0\x14a\x15\xD0W=a\x15\x92\x81a\x10\xE6V[\x90a\x15\xA0`@Q\x92\x83a\x10\xC4V[\x81R\x80\x91\x84=\x92\x01>[\x80Q\x92\x83a\x15\xCBW`@Qc2PWI`\xE1\x1B\x81R`\x04\x81\x01\x84\x90R`$\x90\xFD[\x01\x90P\xFD[P``a\x15\xAAV[\x91\x94P\x91Pc\xFF\xFF\xFF\xFF`\xE0\x1B\x16\x03a\x15\xF6WP8\x80\x80\x80\x80a\x15\rV[`$\x90`@Q\x90c2PWI`\xE1\x1B\x82R`\x04\x82\x01R\xFD[\x90\x91P\x86\x81\x81=\x83\x11a\x16GW[a\x16&\x81\x83a\x10\xC4V[\x81\x01\x03\x12a\x02\xDAWQ`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\x02\xDAW\x908a\x15xV[P=a\x16\x1CV[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15a\x16}WV[`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x90\xFD[`\0R`\0\x80Q` a\x17\x01\x839\x81Q\x91R` R`@`\0 k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x81T\x16\x90UV\xFE\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x80\xBB+c\x8C\xC2\x0B\xC4\xD0\xA6\rf\x94\x0F:\xB4\xA0\x0C\x1D{14\x97\xCA\x82\xFB\x0BJ\xB0\x07\x93\x02\x80\xBB+c\x8C\xC2\x0B\xC4\xD0\xA6\rf\x94\x0F:\xB4\xA0\x0C\x1D{14\x97\xCA\x82\xFB\x0BJ\xB0\x07\x93\x04\xA2dipfsX\"\x12 ?\n\x08\x0F\x16\x96\xF19xMP\x93\xA3\x84\x86\x86\xC9\xBC'1Pk/\xEC\x1D\xF9\xE2Gh$Y?dsolcC\0\x08\x17\x003";
    /// The deployed bytecode of the contract.
    pub static SHARE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Share<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Share<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Share<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Share<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Share<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Share)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Share<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SHARE_ABI.clone(),
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
                SHARE_ABI.clone(),
                SHARE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 94, 167, 179], (to, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burn` (0x42966c68) function
        pub fn burn(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 150, 108, 104], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getApproved` (0x081812fc) function
        pub fn get_approved(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([8, 24, 18, 252], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x077f224a) function
        pub fn initialize(
            &self,
            name: ::std::string::String,
            symbol: ::std::string::String,
            pool_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 127, 34, 74], (name, symbol, pool_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isApprovedForAll` (0xe985e9c5) function
        pub fn is_approved_for_all(
            &self,
            owner: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (owner, operator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0x40c10f19) function
        pub fn mint(
            &self,
            user: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([64, 193, 15, 25], (user, token_id))
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
        ///Calls the contract's `ownerOf` (0x6352211e) function
        pub fn owner_of(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([99, 82, 33, 30], token_id)
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
        ///Calls the contract's `safeTransferFrom` (0x42842e0e) function
        pub fn safe_transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 132, 46, 14], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0xb88d4fde) function
        pub fn safe_transfer_from_with_from_and_to_and_data(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 141, 79, 222], (from, to, token_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setApprovalForAll` (0xa22cb465) function
        pub fn set_approval_for_all(
            &self,
            operator: ::ethers::core::types::Address,
            approved: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
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
        ///Calls the contract's `tokenExists` (0x00923f9e) function
        pub fn token_exists(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([0, 146, 63, 158], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenURI` (0xc87b56dd) function
        pub fn token_uri(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([200, 123, 86, 221], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, token_id))
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
        ///Gets the contract's `ApprovalForAll` event
        pub fn approval_for_all_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalForAllFilter,
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ShareEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Share<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ERC721IncorrectOwner` with signature `ERC721IncorrectOwner(address,uint256,address)` and selector `0x64283d7b`
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
        name = "ERC721IncorrectOwner",
        abi = "ERC721IncorrectOwner(address,uint256,address)"
    )]
    pub struct ERC721IncorrectOwner {
        pub sender: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub owner: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC721InsufficientApproval` with signature `ERC721InsufficientApproval(address,uint256)` and selector `0x177e802f`
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
        name = "ERC721InsufficientApproval",
        abi = "ERC721InsufficientApproval(address,uint256)"
    )]
    pub struct ERC721InsufficientApproval {
        pub operator: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC721InvalidApprover` with signature `ERC721InvalidApprover(address)` and selector `0xa9fbf51f`
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
    #[etherror(name = "ERC721InvalidApprover", abi = "ERC721InvalidApprover(address)")]
    pub struct ERC721InvalidApprover {
        pub approver: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC721InvalidOperator` with signature `ERC721InvalidOperator(address)` and selector `0x5b08ba18`
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
    #[etherror(name = "ERC721InvalidOperator", abi = "ERC721InvalidOperator(address)")]
    pub struct ERC721InvalidOperator {
        pub operator: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC721InvalidOwner` with signature `ERC721InvalidOwner(address)` and selector `0x89c62b64`
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
    #[etherror(name = "ERC721InvalidOwner", abi = "ERC721InvalidOwner(address)")]
    pub struct ERC721InvalidOwner {
        pub owner: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC721InvalidReceiver` with signature `ERC721InvalidReceiver(address)` and selector `0x64a0ae92`
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
    #[etherror(name = "ERC721InvalidReceiver", abi = "ERC721InvalidReceiver(address)")]
    pub struct ERC721InvalidReceiver {
        pub receiver: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC721InvalidSender` with signature `ERC721InvalidSender(address)` and selector `0x73c6ac6e`
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
    #[etherror(name = "ERC721InvalidSender", abi = "ERC721InvalidSender(address)")]
    pub struct ERC721InvalidSender {
        pub sender: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC721NonexistentToken` with signature `ERC721NonexistentToken(uint256)` and selector `0x7e273289`
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
    #[etherror(name = "ERC721NonexistentToken", abi = "ERC721NonexistentToken(uint256)")]
    pub struct ERC721NonexistentToken {
        pub token_id: ::ethers::core::types::U256,
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
    pub enum ShareErrors {
        ERC721IncorrectOwner(ERC721IncorrectOwner),
        ERC721InsufficientApproval(ERC721InsufficientApproval),
        ERC721InvalidApprover(ERC721InvalidApprover),
        ERC721InvalidOperator(ERC721InvalidOperator),
        ERC721InvalidOwner(ERC721InvalidOwner),
        ERC721InvalidReceiver(ERC721InvalidReceiver),
        ERC721InvalidSender(ERC721InvalidSender),
        ERC721NonexistentToken(ERC721NonexistentToken),
        InvalidInitialization(InvalidInitialization),
        NotInitializing(NotInitializing),
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ShareErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <ERC721IncorrectOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC721IncorrectOwner(decoded));
            }
            if let Ok(decoded) = <ERC721InsufficientApproval as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC721InsufficientApproval(decoded));
            }
            if let Ok(decoded) = <ERC721InvalidApprover as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC721InvalidApprover(decoded));
            }
            if let Ok(decoded) = <ERC721InvalidOperator as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC721InvalidOperator(decoded));
            }
            if let Ok(decoded) = <ERC721InvalidOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC721InvalidOwner(decoded));
            }
            if let Ok(decoded) = <ERC721InvalidReceiver as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC721InvalidReceiver(decoded));
            }
            if let Ok(decoded) = <ERC721InvalidSender as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC721InvalidSender(decoded));
            }
            if let Ok(decoded) = <ERC721NonexistentToken as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC721NonexistentToken(decoded));
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
    impl ::ethers::core::abi::AbiEncode for ShareErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ERC721IncorrectOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC721InsufficientApproval(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC721InvalidApprover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC721InvalidOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC721InvalidOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC721InvalidReceiver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC721InvalidSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC721NonexistentToken(element) => {
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
    impl ::ethers::contract::ContractRevert for ShareErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ERC721IncorrectOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC721InsufficientApproval as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC721InvalidApprover as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC721InvalidOperator as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC721InvalidOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC721InvalidReceiver as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC721InvalidSender as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC721NonexistentToken as ::ethers::contract::EthError>::selector() => {
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
    impl ::core::fmt::Display for ShareErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ERC721IncorrectOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC721InsufficientApproval(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC721InvalidApprover(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC721InvalidOperator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC721InvalidOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC721InvalidReceiver(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC721InvalidSender(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC721NonexistentToken(element) => {
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
    impl ::core::convert::From<::std::string::String> for ShareErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ERC721IncorrectOwner> for ShareErrors {
        fn from(value: ERC721IncorrectOwner) -> Self {
            Self::ERC721IncorrectOwner(value)
        }
    }
    impl ::core::convert::From<ERC721InsufficientApproval> for ShareErrors {
        fn from(value: ERC721InsufficientApproval) -> Self {
            Self::ERC721InsufficientApproval(value)
        }
    }
    impl ::core::convert::From<ERC721InvalidApprover> for ShareErrors {
        fn from(value: ERC721InvalidApprover) -> Self {
            Self::ERC721InvalidApprover(value)
        }
    }
    impl ::core::convert::From<ERC721InvalidOperator> for ShareErrors {
        fn from(value: ERC721InvalidOperator) -> Self {
            Self::ERC721InvalidOperator(value)
        }
    }
    impl ::core::convert::From<ERC721InvalidOwner> for ShareErrors {
        fn from(value: ERC721InvalidOwner) -> Self {
            Self::ERC721InvalidOwner(value)
        }
    }
    impl ::core::convert::From<ERC721InvalidReceiver> for ShareErrors {
        fn from(value: ERC721InvalidReceiver) -> Self {
            Self::ERC721InvalidReceiver(value)
        }
    }
    impl ::core::convert::From<ERC721InvalidSender> for ShareErrors {
        fn from(value: ERC721InvalidSender) -> Self {
            Self::ERC721InvalidSender(value)
        }
    }
    impl ::core::convert::From<ERC721NonexistentToken> for ShareErrors {
        fn from(value: ERC721NonexistentToken) -> Self {
            Self::ERC721NonexistentToken(value)
        }
    }
    impl ::core::convert::From<InvalidInitialization> for ShareErrors {
        fn from(value: InvalidInitialization) -> Self {
            Self::InvalidInitialization(value)
        }
    }
    impl ::core::convert::From<NotInitializing> for ShareErrors {
        fn from(value: NotInitializing) -> Self {
            Self::NotInitializing(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner> for ShareErrors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount> for ShareErrors {
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
        pub approved: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
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
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
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
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
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
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
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
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ShareEvents {
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        BurnFilter(BurnFilter),
        InitializedFilter(InitializedFilter),
        MintFilter(MintFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for ShareEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(ShareEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(ShareEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = BurnFilter::decode_log(log) {
                return Ok(ShareEvents::BurnFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(ShareEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(ShareEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(ShareEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(ShareEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ShareEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovalForAllFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
    impl ::core::convert::From<ApprovalFilter> for ShareEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalForAllFilter> for ShareEvents {
        fn from(value: ApprovalForAllFilter) -> Self {
            Self::ApprovalForAllFilter(value)
        }
    }
    impl ::core::convert::From<BurnFilter> for ShareEvents {
        fn from(value: BurnFilter) -> Self {
            Self::BurnFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for ShareEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<MintFilter> for ShareEvents {
        fn from(value: MintFilter) -> Self {
            Self::MintFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for ShareEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for ShareEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
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
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
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
        pub owner: ::ethers::core::types::Address,
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
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getApproved` function with signature `getApproved(uint256)` and selector `0x081812fc`
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
    #[ethcall(name = "getApproved", abi = "getApproved(uint256)")]
    pub struct GetApprovedCall {
        pub token_id: ::ethers::core::types::U256,
    }
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
        pub pool_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
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
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall {
        pub owner: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
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
        pub user: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `ownerOf` function with signature `ownerOf(uint256)` and selector `0x6352211e`
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
    #[ethcall(name = "ownerOf", abi = "ownerOf(uint256)")]
    pub struct OwnerOfCall {
        pub token_id: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256)` and selector `0x42842e0e`
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256,bytes)` and selector `0xb88d4fde`
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,bytes)"
    )]
    pub struct SafeTransferFromWithFromAndToAndDataCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setApprovalForAll` function with signature `setApprovalForAll(address,bool)` and selector `0xa22cb465`
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
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
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
    ///Container type for all input parameters for the `tokenExists` function with signature `tokenExists(uint256)` and selector `0x00923f9e`
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
    #[ethcall(name = "tokenExists", abi = "tokenExists(uint256)")]
    pub struct TokenExistsCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `tokenURI` function with signature `tokenURI(uint256)` and selector `0xc87b56dd`
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
    #[ethcall(name = "tokenURI", abi = "tokenURI(uint256)")]
    pub struct TokenURICall {
        pub token_id: ::ethers::core::types::U256,
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
        pub token_id: ::ethers::core::types::U256,
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
    pub enum ShareCalls {
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        GetApproved(GetApprovedCall),
        Initialize(InitializeCall),
        IsApprovedForAll(IsApprovedForAllCall),
        Mint(MintCall),
        Name(NameCall),
        Owner(OwnerCall),
        OwnerOf(OwnerOfCall),
        RenounceOwnership(RenounceOwnershipCall),
        SafeTransferFrom(SafeTransferFromCall),
        SafeTransferFromWithFromAndToAndData(SafeTransferFromWithFromAndToAndDataCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        TokenExists(TokenExistsCall),
        TokenURI(TokenURICall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for ShareCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
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
            if let Ok(decoded) = <GetApprovedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetApproved(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <IsApprovedForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsApprovedForAll(decoded));
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
            if let Ok(decoded) = <OwnerOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnerOf(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SafeTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) = <SafeTransferFromWithFromAndToAndDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeTransferFromWithFromAndToAndData(decoded));
            }
            if let Ok(decoded) = <SetApprovalForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) = <TokenExistsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenExists(decoded));
            }
            if let Ok(decoded) = <TokenURICall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenURI(decoded));
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
    impl ::ethers::core::abi::AbiEncode for ShareCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Burn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetApproved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsApprovedForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OwnerOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeTransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetApprovalForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenURI(element) => {
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
    impl ::core::fmt::Display for ShareCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetApproved(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsApprovedForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnerOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetApprovalForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenExists(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenURI(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApproveCall> for ShareCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for ShareCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BurnCall> for ShareCalls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<GetApprovedCall> for ShareCalls {
        fn from(value: GetApprovedCall) -> Self {
            Self::GetApproved(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for ShareCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsApprovedForAllCall> for ShareCalls {
        fn from(value: IsApprovedForAllCall) -> Self {
            Self::IsApprovedForAll(value)
        }
    }
    impl ::core::convert::From<MintCall> for ShareCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<NameCall> for ShareCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for ShareCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<OwnerOfCall> for ShareCalls {
        fn from(value: OwnerOfCall) -> Self {
            Self::OwnerOf(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for ShareCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromCall> for ShareCalls {
        fn from(value: SafeTransferFromCall) -> Self {
            Self::SafeTransferFrom(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromWithFromAndToAndDataCall> for ShareCalls {
        fn from(value: SafeTransferFromWithFromAndToAndDataCall) -> Self {
            Self::SafeTransferFromWithFromAndToAndData(value)
        }
    }
    impl ::core::convert::From<SetApprovalForAllCall> for ShareCalls {
        fn from(value: SetApprovalForAllCall) -> Self {
            Self::SetApprovalForAll(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for ShareCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for ShareCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TokenExistsCall> for ShareCalls {
        fn from(value: TokenExistsCall) -> Self {
            Self::TokenExists(value)
        }
    }
    impl ::core::convert::From<TokenURICall> for ShareCalls {
        fn from(value: TokenURICall) -> Self {
            Self::TokenURI(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for ShareCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for ShareCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
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
    ///Container type for all return fields from the `getApproved` function with signature `getApproved(uint256)` and selector `0x081812fc`
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
    pub struct GetApprovedReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
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
    pub struct IsApprovedForAllReturn(pub bool);
    ///Container type for all return fields from the `mint` function with signature `mint(address,uint256)` and selector `0x40c10f19`
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
    pub struct MintReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `ownerOf` function with signature `ownerOf(uint256)` and selector `0x6352211e`
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
    pub struct OwnerOfReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
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
    ///Container type for all return fields from the `tokenExists` function with signature `tokenExists(uint256)` and selector `0x00923f9e`
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
    pub struct TokenExistsReturn(pub bool);
    ///Container type for all return fields from the `tokenURI` function with signature `tokenURI(uint256)` and selector `0xc87b56dd`
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
    pub struct TokenURIReturn(pub ::std::string::String);
}
