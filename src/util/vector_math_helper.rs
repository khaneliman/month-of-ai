pub struct VectorMathHelper;

impl VectorMathHelper {
    /**
     * Validates the vectors for non-emptiness and equal length, throwing an error if the validation fails.
     *
     * @param vecA - The first vector to be validated.
     * @param vecB - The second vector to be validated.
     * @throws Will throw an error if either vector is empty or if the vectors are not of equal length.
     */
    pub fn validate_vectors(vec_a: &[f32], vec_b: &[f32]) {
        if vec_a.is_empty() || vec_b.is_empty() {
            panic!("Both vectors must be non-empty");
        }
        if vec_a.len() != vec_b.len() {
            panic!("Vectors must be of the same length");
        }
    }

    /**
     * Calculates the cosine similarity between two vectors.
     *
     * @param vectorA - The first vector for the calculation.
     * @param vectorB - The second vector for the calculation.
     * @returns The cosine similarity between vectorA and vectorB.
     * @throws Will throw an error if either vector is a zero vector or if vectors are not the same lenth.
     */
    pub fn cosine_similarity(vector_a: &[f32], vector_b: &[f32]) -> f32 {
        Self::validate_vectors(vector_a, vector_b);

        let mut dot_product = 0.0;
        let mut magnitude_a = 0.0;
        let mut magnitude_b = 0.0;

        for i in 0..vector_a.len() {
            dot_product += vector_a[i] * vector_b[i];
            magnitude_a += vector_a[i] * vector_a[i];
            magnitude_b += vector_b[i] * vector_b[i];
        }

        magnitude_a = magnitude_a.sqrt();
        magnitude_b = magnitude_b.sqrt();

        if magnitude_a == 0.0 || magnitude_b == 0.0 {
            panic!("Cannot calculate cosine similarity for a zero vector");
        }

        dot_product / (magnitude_a * magnitude_b)
    }
}
