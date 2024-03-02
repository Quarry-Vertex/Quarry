#!/usr/bin/python3

"""
run.py is a utility for managing the varying components of the Quarry project.
The project is composed of several pieces of software that have to interact
to result in the end product, a merge mining pool for free market hash rate 
sales.

| Name          | Purpose                                                      |
|---------------+--------------------------------------------------------------|
| oracleService | sends current block chain information to contracts           |
| contracts     | foundry project for managing contracts to distribute rewards |
| stratum       | using stratumv2 to handle mining and direct hashes to        |

This script allows for interacting with the varying components of the Quarry
project from the main directory.
"""

import argparse
import subprocess
import os
import json


# run the orcale by running serverConnect.js
def run_oracle(args):
    os.chdir("oracleService")
    subprocess.run(["node", "serverConnect.js"])
    os.chdir("..")


# build the smart contracts in the foundry project
def forge_build(args):
    # move to the foundry project
    os.chdir("contracts")
    subprocess.run(["forge", "build"])
    os.chdir("../")


# test a smart contract
def forge_test(args):
    pass


# deploy the smart contract with presets
def contract_default_deploy(args):
    # open the json file with default parameters
    with open("run.json", "r") as f:
        data = json.load(f)
    # move to the foundry project
    os.chdir("contracts")
    # build forge command
    deploy_command = [
        "forge",
        "create",
        "--rpc-url",
        data["rpc"],
        "--private-key",
        data["privkey"],
        data["contract"],
    ]
    # surround constructor paramets in ""
    deploy_command.extend(
        ["--constructor-args"] + ['"' + arg + '"' for arg in data["constructor"]]
    )
    subprocess.run(deploy_command)
    os.chdir("../")


# deploy the smart contract, passing in required params
def forge_deploy(args):
    # move to the foundry project
    os.chdir("SharesPool/sharespool_foundry")
    # build forge command
    deploy_command = [
        "forge",
        "create",
        "--rpc-url",
        args.rpc_url,
        "--private-key",
        args.private_key,
        args.contract,
    ]
    # surround constructor paramets in ""
    deploy_command.extend(
        ["--constructor-args"] + ['"' + arg + '"' for arg in args.constructor]
    )
    subprocess.run(deploy_command)
    os.chdir("../..")


def main():
    # basic parser
    parser = argparse.ArgumentParser(
        description="Deployment and Testing Manager for Quarry",
    )
    subparsers = parser.add_subparsers(title="commands", dest="command")

    # run the oracle
    oracle_parser = subparsers.add_parser("oracle", help="run the oracle js script")
    oracle_parser.set_defaults(func=run_oracle)

    # run forge
    forge_parser = subparsers.add_parser(
        "forge", help="interface with the forge smart contracts"
    )
    forge_subparsers = forge_parser.add_subparsers(
        title="forge_commands", dest="forge_command", required=True
    )

    # forge build
    forge_build_parser = forge_subparsers.add_parser(
        "build", help="build the smart contracts"
    )
    forge_build_parser.set_defaults(func=forge_build)

    # forge default deploy
    forge_default_deploy = forge_subparsers.add_parser(
        "default-deploy", help="deploy smart contract based on values in run.json"
    )
    forge_default_deploy.set_defaults(func=contract_default_deploy)

    # forge deploy with inputs
    forge_deploy_parser = forge_subparsers.add_parser(
        "deploy", help="deploy the smart contracts"
    )
    forge_deploy_parser.add_argument(
        "--rpc-url", required=True, help="RPC URL for deployment"
    )
    forge_deploy_parser.add_argument(
        "--private-key", required=True, help="Private key for deployment"
    )
    forge_deploy_parser.add_argument(
        "--contract",
        required=True,
        help="Path to the contract file and contract name",
    )
    forge_deploy_parser.add_argument(
        "--constructor",
        required=True,
        nargs="+",
        help="Constructor arguments for the contract",
    )
    forge_deploy_parser.set_defaults(func=forge_deploy)

    # parse the provided arguments
    args = parser.parse_args()
    if args.command is None:
        parser.print_help()
    elif hasattr(args, "func"):
        # run the command
        args.func(args)
    else:
        print(f"Error: '{args.command}' requires a subcommand.")
        parser.parse_args([args.command, "--help"])


if __name__ == "__main__":
    main()


"""
./run.py forge deploy --rpc-url testrpc.url.com --private-key 0xprivkey --contract sharesPool/contract:Contract --constructor "Quarry" "QRY" ""
"""
