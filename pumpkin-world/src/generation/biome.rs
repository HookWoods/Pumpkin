use pumpkin_util::math::{floor_mod, vector3::Vector3};

use super::biome_coords;

// This blends biome boundaries, returning which biome to populate the surface on based on the seed
pub fn get_biome_blend(
    bottom_y: i8,
    height: u16,
    seed: u64,
    global_block_pos: &Vector3<i32>,
) -> Vector3<i32> {
    // This is the "left" side of the biome boundary
    let offset_x = global_block_pos.x - 2;
    let offset_y = global_block_pos.y - 2;
    let offset_z = global_block_pos.z - 2;
    let biome_x = biome_coords::from_block(offset_x);
    let biome_y = biome_coords::from_block(offset_y);
    let biome_z = biome_coords::from_block(offset_z);

    // This is effectively "quarters" into the biome - compute once and reuse
    let biome_x_quarters = (offset_x & 0b11) as f64 / 4.0;
    let biome_y_quarters = (offset_y & 0b11) as f64 / 4.0;
    let biome_z_quarters = (offset_z & 0b11) as f64 / 4.0;

    // Precompute the shifted values for all permutations
    let shifted_biome_x_quarters = biome_x_quarters - 1.0;
    let shifted_biome_y_quarters = biome_y_quarters - 1.0;
    let shifted_biome_z_quarters = biome_z_quarters - 1.0;

    let mut best_permutation = 0;
    let mut best_score = f64::INFINITY;

    // Use a single seed value for all permutations to avoid recalculating
    let seed_i64 = seed as i64;

    // Unroll the permutation loop for better performance
    // Permutation 0: maintain all (x, y, z)
    {
        let score = score_permutation_fast(
            seed_i64,
            biome_x, biome_y, biome_z,
            biome_x_quarters, biome_y_quarters, biome_z_quarters,
        );
        if score < best_score {
            best_score = score;
            best_permutation = 0;
        }
    }

    // Permutation 1: maintain x and y, shift z
    {
        let score = score_permutation_fast(
            seed_i64,
            biome_x, biome_y, biome_z + 1,
            biome_x_quarters, biome_y_quarters, shifted_biome_z_quarters,
        );
        if score < best_score {
            best_score = score;
            best_permutation = 1;
        }
    }

    // Permutation 2: maintain x and z, shift y
    {
        let score = score_permutation_fast(
            seed_i64,
            biome_x, biome_y + 1, biome_z,
            biome_x_quarters, shifted_biome_y_quarters, biome_z_quarters,
        );
        if score < best_score {
            best_score = score;
            best_permutation = 2;
        }
    }

    // Permutation 3: maintain x, shift y and z
    {
        let score = score_permutation_fast(
            seed_i64,
            biome_x, biome_y + 1, biome_z + 1,
            biome_x_quarters, shifted_biome_y_quarters, shifted_biome_z_quarters,
        );
        if score < best_score {
            best_score = score;
            best_permutation = 3;
        }
    }

    // Permutation 4: maintain y and z, shift x
    {
        let score = score_permutation_fast(
            seed_i64,
            biome_x + 1, biome_y, biome_z,
            shifted_biome_x_quarters, biome_y_quarters, biome_z_quarters,
        );
        if score < best_score {
            best_score = score;
            best_permutation = 4;
        }
    }

    // Permutation 5: maintain y, shift x and z
    {
        let score = score_permutation_fast(
            seed_i64,
            biome_x + 1, biome_y, biome_z + 1,
            shifted_biome_x_quarters, biome_y_quarters, shifted_biome_z_quarters,
        );
        if score < best_score {
            best_score = score;
            best_permutation = 5;
        }
    }

    // Permutation 6: maintain z, shift x and y
    {
        let score = score_permutation_fast(
            seed_i64,
            biome_x + 1, biome_y + 1, biome_z,
            shifted_biome_x_quarters, shifted_biome_y_quarters, biome_z_quarters,
        );
        if score < best_score {
            best_score = score;
            best_permutation = 6;
        }
    }

    // Permutation 7: shift all (x, y, z)
    {
        let score = score_permutation_fast(
            seed_i64,
            biome_x + 1, biome_y + 1, biome_z + 1,
            shifted_biome_x_quarters, shifted_biome_y_quarters, shifted_biome_z_quarters,
        );
        if score < best_score {
            best_permutation = 7;
        }
    }

    // Now check if we want to use the "left" side or the "right" side
    // Use bit operations to determine best coordinates
    let final_biome_x = biome_x + ((best_permutation & 0b100) >> 2);
    let final_biome_y = biome_y + ((best_permutation & 0b010) >> 1);
    let final_biome_z = biome_z + (best_permutation & 0b001);

    // Java's `getBiomeForNoiseGen` clamping logic
    let bottom_y = bottom_y as i32;
    let biome_bottom = biome_coords::from_block(bottom_y);
    let biome_top = biome_bottom + biome_coords::from_block(height as i32) - 1;
    let final_biome_y = final_biome_y.clamp(biome_bottom, biome_top);

    Vector3::new(final_biome_x, final_biome_y, final_biome_z)
}

#[inline]
fn score_permutation_fast(
    seed: i64,
    x: i32,
    y: i32,
    z: i32,
    x_part: f64,
    y_part: f64,
    z_part: f64,
) -> f64 {
    // Get the mix through a chain of mixes to avoid intermediates
    let mix1 = salt_mix(seed, x as i64);
    let mix2 = salt_mix(mix1, y as i64);
    let mix3 = salt_mix(mix2, z as i64);
    let mix4 = salt_mix(mix3, x as i64);
    let mix5 = salt_mix(mix4, y as i64);
    let mix6 = salt_mix(mix5, z as i64);

    // Get offset_x
    let offset_x = scale_mix(mix6);

    // Get offset_y
    let mix7 = salt_mix(mix6, seed);
    let offset_y = scale_mix(mix7);

    // Get offset_z
    let mix8 = salt_mix(mix7, seed);
    let offset_z = scale_mix(mix8);

    // Calculate the squared sum directly
    let dx = x_part + offset_x;
    let dy = y_part + offset_y;
    let dz = z_part + offset_z;

    dx * dx + dy * dy + dz * dz
}

#[inline]
fn scale_mix(l: i64) -> f64 {
    let d = floor_mod(l >> 24, 1024) as f64 / 1024.0;
    (d - 0.5) * 0.9
}

#[inline]
fn salt_mix(seed: i64, salt: i64) -> i64 {
    let mixed_seed = seed
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
        .wrapping_mul(seed);
    mixed_seed.wrapping_add(salt)
}

#[cfg(test)]
mod test {
    use pumpkin_util::math::vector3::Vector3;

    use crate::generation::biome::{get_biome_blend, scale_mix, score_permutation_fast};

    use super::salt_mix;

    #[test]
    fn test_mix_seed() {
        let seed = salt_mix(12345678, 12345678);
        assert_eq!(seed, 2937271135939595220);
    }

    #[test]
    fn test_permutation() {
        let seed = score_permutation_fast(123, 123, 456, 456, 5.5, 5.5, 5.5);
        assert_eq!(seed, 84.45165515899657);
    }

    #[test]
    fn test_biome_blend() {
        let biome_pos = get_biome_blend(-64, 384, 1234567890, &Vector3::new(123, 123, 123));
        assert_eq!(biome_pos, Vector3::new(31, 30, 30));
    }

    #[test]
    fn test_scale() {
        let seed = scale_mix(12345678);
        assert_eq!(seed, -0.45);
    }
}
