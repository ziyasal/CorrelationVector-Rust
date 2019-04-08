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

    for c in 1..10 {
        cv.increment();
        let increased_value = *cv.value();
        let vector_parts: Vec<&str> = increased_value.split('.').collect();

        assert_eq!(c.to_string(), vector_parts[1]);
    }
}