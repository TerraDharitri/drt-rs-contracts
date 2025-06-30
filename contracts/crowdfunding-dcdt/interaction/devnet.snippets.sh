ALICE="./interaction/alice.pem"
PROJECT="${PWD}"
PROXY=https://devnet-gateway.dharitri.com
CHAINID=D

BOB="${USERS}/bob.pem"

ADDRESS=$(drtpy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(drtpy data load --key=deployTransaction-devnet)

DEPLOY_GAS="25000000"
TARGET=10

DEADLINE_UNIX_TIMESTAMP=$(date -d '2100-05-12 00:00:01' +"%s")
REWA_TOKEN_ID=0x52455741 # "REWA"

deploy() {
    drtpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} \
          --gas-limit=${DEPLOY_GAS} \
          --arguments ${TARGET} ${DEADLINE_UNIX_TIMESTAMP} ${REWA_TOKEN_ID} \
          --proxy=${PROXY} --chain=${CHAINID} --send
          --outfile="deploy-devnet.interaction.json" || return

    TRANSACTION=$(drtpy data parse --file="deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(drtpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    drtpy data store --key=address-devnet --value=${ADDRESS}
    drtpy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

deploySimulate() {
    drtpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} \
          --gas-limit=${DEPLOY_GAS} \
          --arguments ${TARGET} ${DEADLINE_UNIX_TIMESTAMP} ${REWA_TOKEN_ID} \
          --outfile="simulate-devnet.interaction.json" --simulate || return

    TRANSACTION=$(drtpy data parse --file="simulate-devnet.interaction.json" --expression="data['result']['hash']")
    ADDRESS=$(drtpy data parse --file="simulate-devnet.interaction.json" --expression="data['contractAddress']")
    RETCODE=$(drtpy data parse --file="simulate-devnet.interaction.json" --expression="data['result']['returnCode']")
    RETMSG=$(drtpy data parse --file="simulate-devnet.interaction.json" --expression="data['result']['returnMessage']")

    echo ""
    echo "Simulated transaction: ${TRANSACTION}"
    echo "Smart contract address: ${ADDRESS}"
    echo "Deployment return code: ${RETCODE}"
    echo "Deployment return message: ${RETMSG}"
}

checkDeployment() {
    drtpy tx get --hash=$DEPLOY_TRANSACTION --omit-fields="['data', 'signature']"
    drtpy account get --address=$ADDRESS --omit-fields="['code']"
}

# BOB sends funds
sendFunds() {
    drtpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${BOB} --gas-limit=10000000 \
        --function="fund" --value=5 \
        --send
}

# ALICE claims
claimFunds() {
    drtpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=10000000 \
        --function="claim" \
        --send
}

# 0 - Funding Period
# 1 - Successful
# 2 - Failed
status() {
    drtpy --verbose contract query ${ADDRESS} --function="status"
}

getCurrentFunds() {
    drtpy --verbose contract query ${ADDRESS} --function="getCurrentFunds"
}

getTarget() {
    drtpy --verbose contract query ${ADDRESS} --function="getTarget"
}

getDeadline() {
    drtpy --verbose contract query ${ADDRESS} --function="getDeadline"
}

# BOB's deposit
getDeposit() {
    local BOB_ADDRESS_BECH32=drt1spyavw0956vq68xj8y4tenjpq2wd5a9p2c6j8gsz7ztyrnpxrruqlqde3c
    local BOB_ADDRESS_HEX=0x$(drtpy wallet bech32 --decode ${BOB_ADDRESS_BECH32})

    drtpy --verbose contract query ${ADDRESS} --function="getDeposit" --arguments ${BOB_ADDRESS_HEX}
}

getCrowdfundingTokenName() {
    drtpy --verbose contract query ${ADDRESS} --function="getCrowdfundingTokenName"
}
