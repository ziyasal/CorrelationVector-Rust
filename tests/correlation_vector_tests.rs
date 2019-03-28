use correlation_vector;
use correlation_vector::CorrelationVector;

#[test]
fn it_extends_cv() {
    let cv = CorrelationVector::extend("PmvzQKgYek6Sdk/T5sWaqw.1", true);
    assert_eq!("PmvzQKgYek6Sdk/T5sWaqw.1.0", *cv.value());
}

#[test]
fn it_validates_cv_during_extend() {
    let cv = CorrelationVector::extend("PmvzQKgYek6Sdk/T5sWaqw.1!", true);
    assert_eq!("PmvzQKgYek6Sdk/T5sWaqw.1!", *cv.value());
}