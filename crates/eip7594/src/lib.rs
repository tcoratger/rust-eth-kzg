#[cfg(all(feature = "singlethreaded", feature = "multithreaded"))]
compile_error!("`singlethreaded` and `multithreaded` cannot be enabled simultaneously");

pub mod constants;
mod errors;
mod prover;
mod recovery;
mod serialization;
mod trusted_setup;
mod verifier;

pub use bls12_381::fixed_base_msm::UsePrecomp;
// Exported types
//
pub use errors::Error;
/// TrustedSetup contains the Structured Reference String(SRS)
/// needed to make and verify proofs.
pub use trusted_setup::TrustedSetup;

/// `BlobRef` denotes a references to an opaque Blob.
///
/// Note: This library never returns a Blob, which is why we
/// do not have a Blob type.
pub type BlobRef<'a> = &'a [u8; BYTES_PER_BLOB];

/// `Bytes48Ref` denotes a reference to an untrusted cryptographic type
/// that can be represented in 48 bytes. This will be either a
/// purported `KZGProof` or a purported `KZGCommitment`.
pub type Bytes48Ref<'a> = &'a [u8; 48];

/// Cell contains a group of evaluations on a coset that one would like to
/// make and verify opening proofs about.
///
/// Note: These are heap allocated.
pub type Cell = Box<[u8; BYTES_PER_CELL]>;

/// `CellRef` contains a reference to a Cell.
///
/// Note: Similar to Blob, the library takes in references
/// to Cell and returns heap allocated instances as return types.
pub type CellRef<'a> = &'a [u8; BYTES_PER_CELL];

/// `KZGProof` denotes a 48 byte commitment to a polynomial
/// that one can use to prove that a polynomial f(x) was
/// correctly evaluated on a coset `H` and returned a set of points.
pub type KZGProof = [u8; BYTES_PER_COMMITMENT];

/// `KZGCommitment` denotes a 48 byte commitment to a polynomial f(x)
/// that we would like to make and verify opening proofs about.
pub type KZGCommitment = [u8; BYTES_PER_COMMITMENT];

/// `CellIndex` is reference to the coset/set of points that were used to create that Cell,
/// on a particular polynomial, f(x).
///
/// Note: Since the verifier and prover both know what cosets will be used
/// to evaluate the polynomials being used in opening proofs, the protocol
/// only requires an index to reference them.
pub type CellIndex = kzg_multi_open::CosetIndex;

use constants::{BYTES_PER_BLOB, BYTES_PER_CELL, BYTES_PER_COMMITMENT};
use prover::ProverContext;
use verifier::VerifierContext;

/// The context that will be used to create and verify opening proofs.
#[derive(Debug)]
pub struct DASContext {
    pub prover_ctx: ProverContext,
    pub verifier_ctx: VerifierContext,
}

impl Default for DASContext {
    fn default() -> Self {
        let trusted_setup = TrustedSetup::default();

        Self::new(&trusted_setup, UsePrecomp::No)
    }
}

impl DASContext {
    pub fn new(
        trusted_setup: &TrustedSetup,
        // This parameter indicates whether we should allocate memory
        // in order to speed up proof creation. Heuristics show that
        // if pre-computations are desired, one should set the
        // width value to `8` for optimal storage and performance tradeoffs.
        use_precomp: UsePrecomp,
    ) -> Self {
        Self {
            prover_ctx: ProverContext::new(trusted_setup, use_precomp),
            verifier_ctx: VerifierContext::new(trusted_setup),
        }
    }
}
