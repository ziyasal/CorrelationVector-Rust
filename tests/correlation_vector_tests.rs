use correlation_vector;
use correlation_vector::CorrelationVector;
use correlation_vector::enums::CorrelationVectorVersion;

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

#[test]
fn it_validates_cv_increment() {
    let mut cv = CorrelationVector::new(String::from("PmvzQKgYek6Sdk/T5sWaqw"),
                                        0, CorrelationVectorVersion::V1, false);

    cv.increment();
    let x = *cv.value();
    let a: Vec<&str> = x.split('.').collect();

    assert_eq!("1", a[1]);
}