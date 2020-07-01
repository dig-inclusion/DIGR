
#[cfg(test)]
use non_none_fields;

#[derive(NotNoneFields)]
struct SampleTest {
    first: String,
    sec: Option<u8>,
    thr: Option<u8>
}

#[test]
fn it_should_check_non_none_fields_length() {
    let samp = SampleTest{first: "hello DIGR".to_string(), sec: Some(1), thr: None};
    let nn_flds = samp::non_none_fields();
    assert_eq!(nn_flds.len, 3);
}

#[test]
fn it_should_non_none_fields_is_vec() {

}

