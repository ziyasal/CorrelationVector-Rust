use correlation_vector;
use correlation_vector::CorrelationVector;

#[test]
fn it_sets_entropy() {

    let cv = CorrelationVector::default();
    assert_eq!(23, cv.create());
}