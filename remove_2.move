// Remove validator node from testnet validatorSet
script {
    use aptos_framework::aptos_governance;
    use aptos_framework::stake;

    fun main(account: &signer){
        let framework_signer = aptos_governance::get_signer_testnet_only(account, @0000000000000000000000000000000000000000000000000000000000000001);
        stake::remove_validators(&framework_signer, &vector[
	  @0xc0bc5fe1cf2749394cc5c36ae84bfeac90f25229275900fc4357f1aef2335e35,
	  @0x23f422eb4212100c47cdc28c242cab052b65ace49d1429c1197f4a1b153b178e,
	  @0xe9018e9290ecf240b6600465ae877b60210659c8c2da8ddd4a230409856dde33,
	  @0xc21e278d285adea8a9a0ceafdff12eb3b4dea55f682de6948a4051829016a0c5,
	  @0xa9e136449be1ace0702284b34da3ce1742231cb844c1848c2ea95e2fc9b18ecd,
	  @0x8ad1fa9a18169bd01afb0c402e9063bd580d58f0201abd62a4941dc9dcaf8c2,
	  @0xec4a90b7161abc9110a1bc35eff7bf8e7928f4270259140c22cf16a2ed2a2aee
        ]);
        aptos_governance::reconfigure(&framework_signer);
    }
}

