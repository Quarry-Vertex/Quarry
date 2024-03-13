#!/usr/bin/sh
echo "Contract to Test"
# List contract dirs
contract_dirs=(../contracts/*)
for i in "${!contract_dirs[@]}"; do
    dir=${contract_dirs[$i]}
    echo "$((i+1))) ${dir##*/}"
done

# choose contract by index
echo "Choose a contract by number:"
read choice

# ensure valid index selected
if [[ $choice =~ ^[0-9]+$ ]] && [ "$choice" -ge 1 ] && [ "$choice" -le "${#contract_dirs[@]}" ]; then
    # test the chosen contract
    chosen_contract=${contract_dirs[$((choice-1))]}
    cd "$chosen_contract"
    echo "Testing $chosen_contract"
    while true; do
        read -p "> optimize build and test? (y/n) " optimize 
        case "$optimize" in
            [Yy]* ) break;;
            [Nn]* ) break;;
            * ) echo "Please answer yes or no";;
        esac
    done
    case "$optimize" in
      [Yy]* )
          forge clean && forge build --via-ir && forge test --ffi -vv --via-ir;;
      [Nn]* )
          forge clean && forge build && forge test;;
    esac
else
    # invalid index
    echo "Invalid choice"
fi

