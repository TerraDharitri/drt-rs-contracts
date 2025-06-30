WALLET="${PWD}/wallet.pem"
PROJECT="${PWD}"
PROXY=https://devnet-gateway.dharitri.com
CHAINID=D

DEPLOY_GAS="25000000"
SFT_IDENTIFIER=0x585354525245504149522d653162363733 #XSTRREPAIR-e1b673

CONTRACT_ADDRESS="drt1qqqqqqqqqqqqqpgqkm3wla3wk0yqk7lk725wee8yh0e2zeru76lsv55vsv"

deploy() {
    drtpy --verbose contract deploy \
          --bytecode="output/on-chain-claim.drtsc.json" \
          --arguments ${SFT_IDENTIFIER} \
          --pem=${WALLET} \
          --gas-limit=${DEPLOY_GAS} \
          --proxy=${PROXY} \
          --chain=${CHAINID} \
          --recall-nonce \
          --send \
          --outfile="deploy-devnet.interaction.json" || return

    TRANSACTION=$(drtpy data parse --file="deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(drtpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    drtpy data store --key=address-devnet --value=${ADDRESS}
    drtpy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

upgrade() {
    drtpy --verbose contract upgrade ${CONTRACT_ADDRESS} \
          --bytecode="output/on-chain-claim.drtsc.json" \
          --pem=${WALLET} \
          --gas-limit=${DEPLOY_GAS} \
          --proxy=${PROXY} \
          --chain=${CHAINID} \
          --recall-nonce \
          --send \
          --outfile="upgrade-devnet.interaction.json" || return

    TRANSACTION=$(drtpy data parse --file="upgrade-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(drtpy data parse --file="upgrade-devnet.interaction.json" --expression="data['contractAddress']")

    drtpy data store --key=address-devnet --value=${ADDRESS}
    drtpy data store --key=upgradeTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}
