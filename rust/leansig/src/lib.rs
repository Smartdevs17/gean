use leansig::signature::SignatureScheme;
use leansig::signature::generalized_xmss::instantiations_poseidon_top_level::lifetime_2_to_the_32::hashing_optimized::SIGTopLevelTargetSumLifetime32Dim64Base8;


pub type LeanSignatureScheme = SIGTopLevelTargetSumLifetime32Dim64Base8;
pub type LeanPublicKey = <LeanSignatureScheme as SignatureScheme>::PublicKey;
pub type LeanSecretKey = <LeanSignatureScheme as SignatureScheme>::SecretKey;

#[repr(C)]
pub struct SecretKey {
    pub inner: LeanSecretKey,
}

#[repr(C)]
pub struct PublicKey {
    pub inner: LeanPublicKey,
}

pub struct Keypair {
    pub public_key: PublicKey,
    pub secret_key: SecretKey,
}


