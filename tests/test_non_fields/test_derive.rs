use non_none_fields::*;

fn main (){
    it_should_check_non_none_fields_length();
}

fn it_should_check_non_none_fields_length() {
    #[derive(NonNoneFields)]
    struct SampleTest {
        first: String,
        sec: Option<u8>,
        thr: Option<u8>
    }
    
    let samp = SampleTest{first: "hello".to_string(), sec: Some(1), thr: None, };
    let nn_flds = samp.non_none_fields();
    assert_eq!(nn_flds.len(), 2);
}
