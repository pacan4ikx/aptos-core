// Remove validator node from testnet validatorSet
script {
    use aptos_framework::aptos_governance;
    use aptos_framework::stake;

    fun main(account: &signer){
        let framework_signer = aptos_governance::get_signer_testnet_only(account, @0000000000000000000000000000000000000000000000000000000000000001);
        stake::remove_validators(&framework_signer, &vector[
          @0x188a5ac6a2c1caf0ffdabeb0a2146be18383df71ab4f99b7131ee507a30120f4,
	  @0x21832491b4f3e73b49e0cbbbfb0c99a3e325d41343561a802763f05397fee8b0,
	  @0xe92b865f5fcfc3621471a196f0256243e05e70c3fbfceba30a0110d381136109,
	  @0xfe8af4b771bae665a880a11c983063959644f4c6b08d9e320dfe28b334e705a8,
	  @0x353476334d3c4999d6f9c1ee54341d535035f6e0e346063e747d76b0e1f7a9d7,
	  @0x52c10f15d1ca206b221c8a607be09f58e648c9769a42ff2fc8ffd1803015c7ff
	]);
        aptos_governance::reconfigure(&framework_signer);
    }
}

