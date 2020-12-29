ALICE="../testnet/wallets/users/alice.pem"
ADDRESS=$(erdpy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction-devnet)
DEPLOY_GAS="80000000"

deploy_lvl2() {
    erdpy --verbose contract deploy --recall-nonce --pem=${ALICE} \
          --gas-limit=${DEPLOY_GAS} \
          --proxy="http://localhost:7950" \
          --chain="local-testnet" \
          --bytecode="../lvl2/output/band-bridge-lvl2.wasm" \
          --outfile="deploy-devnet.interaction.json" --send || return

    TRANSACTION=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['address']")

    erdpy data store --key=address-devnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}
