use ndarray::Array;
use smartcore::dataset::iris::load_dataset;

use smartcore::linear::logistic_regression::LogisticRegression;
use smartcore::metrics::accuracy;

fn main() {
    // Load Iris dataset
    let iris_data = load_dataset();
    // Turn Iris dataset into NxM matrix
    let x = Array::from_shape_vec(
        (iris_data.num_samples, iris_data.num_features),
        iris_data.data,
    )
    .unwrap();
    // These are our target class labels
    let y = Array::from_shape_vec(iris_data.num_samples, iris_data.target).unwrap();
    // Fit Logistic Regression to Iris dataset
    let lr = LogisticRegression::fit(&x, &y, Default::default()).unwrap();
    let y_hat = lr.predict(&x).unwrap(); // Predict class labels

    // Calculate training error
    println!("accuracy: {}", accuracy(&y, &y_hat)); // Prints 0.98
}
