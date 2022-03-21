use adder;

//集成测试
#[test]
fn add_two_test(){
    assert_eq!(4, adder::add_two(2));
}