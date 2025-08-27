//! Example calculations.
//! As much as possible, these should work with primitives, slices of primitives,
//! and iterators over primitives so that they don't force too many opinions
//! on upstream operations.
use core::num::NonZeroUsize;

use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    slice::{ParallelSlice, ParallelSliceMut},
};

/// Chunk size for parallelism
#[inline]
pub(crate) fn chunksize(nelem: usize) -> usize {
    let ncores = std::thread::available_parallelism()
        .unwrap_or(NonZeroUsize::MIN)
        .get();

    (nelem / ncores).max(1)
}

/// Gnielinski correlation for heat transfer in turbulent pipes.
///
/// Valid region:
///     Re: in [3e3, 5e6]
///     Pr: in [0.5, inf]
///     f: anywhere the source correlation is valid
///
/// Note that this will produce unhelpful values below its valid range of Re,
/// and should be blended into the constant laminar Nusselt number,
/// which is typically given as Nu_laminar = 3.66.
#[inline]
pub fn nusselt_turbulent_smooth_duct(re: f64, pr: f64, f: f64) -> f64 {
    let num = (f / 8.0) * (re - 1000.0) * pr;
    let denom = 1.0 + 12.7 * (f / 8.0).sqrt() * (pr.powf(2.0 / 3.0) - 1.0);
    let nu = num / denom;
    return nu; // [dimensionless] Nusselt number
}

/// Gnielinski correlation for heat transfer in turbulent pipes.
/// See [nusselt_turbulent_smooth_duct] for more info.
#[inline]
pub fn nusselt_turbulent_smooth_duct_par(
    re: &[f64],
    pr: &[f64],
    f: &[f64],
    out: &mut [f64],
) -> Result<(), &'static str> {
    // Choose size of chunks,
    // using a hand-tuned heuristic to decide when to switch
    // from serial to threaded.
    let n = if re.len() > 10_000 {
        chunksize(re.len())
    } else {
        re.len()
    };
    // let n = re.len();  // uncomment to test singlethreaded

    // Chunk inputs, shadowing original names
    let re = re.par_chunks(n);
    let pr = pr.par_chunks(n);
    let f = f.par_chunks(n);
    let out = out.par_chunks_mut(n);

    // Iterate over each chunk in parallel, shadowing original names again
    (out, re, pr, f)
        .into_par_iter()
        .try_for_each(|(out, re, pr, f)| {
            // Check lengths
            let m = out.len();
            if re.len() != m || pr.len() != m || f.len() != m {
                return Err("Length mismatch");
            }

            // Run scalar loop
            for i in 0..m {
                out[i] = nusselt_turbulent_smooth_duct(re[i], pr[i], f[i]);
            }

            Ok(())
        })?;

    // Single-threaded variant
    // // Check lengths
    // let m = out.len();
    // if re.len() != m || pr.len() != m || f.len() != m {
    //     return Err("Length mismatch");
    // }

    // // Run scalar loop
    // for i in 0..m {
    //     out[i] = nusselt_turbulent_smooth_duct(re[i], pr[i], f[i]);
    // }

    Ok(())
}
