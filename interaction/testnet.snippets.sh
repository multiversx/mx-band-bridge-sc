# To deploy to testnet
# 1. add your feeder_key.json and feeder_password.txt to this directory
# 2. source devnet.snippets.sh
# 3. deploy_lvl2

ADDRESS=$(erdpy data load --key=address-testnet)
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction-testnet)
DEPLOY_ARGUMENTS="12 42000000"
DEPLOY_GAS="80000000"
PROXY=https://testnet-api.elrond.com

deploy_lvl2() {
    erdpy --verbose contract deploy --recall-nonce \
          --keyfile="feeder_key.json" --passfile="feeder_password.txt" \
          --gas-limit=${DEPLOY_GAS} \
          --proxy=${PROXY} --chain=T \
          --bytecode="../lvl2/output/band-bridge-lvl2.wasm" \
          --outfile="deploy-devnet.interaction.json" --send || return

    TRANSACTION=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['address']")

    erdpy data store --key=address-devnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}
