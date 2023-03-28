// Remove validator node from testnet validatorSet
script {
    use aptos_framework::aptos_governance;
    use aptos_framework::stake;

    fun main(account: &signer){
        let framework_signer = aptos_governance::get_signer_testnet_only(account, @0000000000000000000000000000000000000000000000000000000000000001);
        stake::remove_validators(&framework_signer, &vector[
          @0x6ff9d7b06e7f41dd9782fda9f034d40af7dd106f2c438e65a508d0e42efe2495
        ]);
        aptos_governance::reconfigure(&framework_signer);
    }
}

