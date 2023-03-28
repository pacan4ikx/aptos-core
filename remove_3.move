// Remove validator node from testnet validatorSet
script {
    use aptos_framework::aptos_governance;
    use aptos_framework::stake;

    fun main(account: &signer){
        let framework_signer = aptos_governance::get_signer_testnet_only(account, @0000000000000000000000000000000000000000000000000000000000000001);
        stake::remove_validators(&framework_signer, &vector[
	  @0x09ade6d94d26da38214cab90ad990dd3714743c5f9144971e737245ad34663e7,
	  @0x1bb8459cbf7099d6a5e7f85e1b8fd12a8eaf16b8b7481b1a63655224f97a6a32,
	  @0x3556cdadc9697e7e416b90866f22fd32cd5281acdd669d6c07d24efbecc7ae7d

        ]);
        aptos_governance::reconfigure(&framework_signer);
    }
}

