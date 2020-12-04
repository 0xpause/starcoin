script {
use 0x1::Vector;
fun main() {
    let vec = Vector::empty<u64>();

    Vector::push_back<u64>(&mut vec, 0);
    Vector::push_back<u64>(&mut vec, 1);
    Vector::push_back<u64>(&mut vec, 2);
    Vector::push_back<u64>(&mut vec, 3);
    Vector::push_back<u64>(&mut vec, 4);

    let removed = Vector::remove<u64>(&mut vec, 2);
    assert(removed == 2, 1000);
    assert(*Vector::borrow<u64>(&vec, 0) == 0, 1001);
    assert(*Vector::borrow<u64>(&vec, 1) == 1, 1002);
    assert(*Vector::borrow<u64>(&vec, 2) == 3, 1003);
    assert(*Vector::borrow<u64>(&vec, 3) == 4, 1004);
}
}

//! new-transaction
script {
    use 0x1::Vector;
    fun main() {
        let vec = Vector::empty<u64>();

        Vector::push_back<u64>(&mut vec, 0);
        Vector::push_back<u64>(&mut vec, 1);
        Vector::push_back<u64>(&mut vec, 2);
        Vector::push_back<u64>(&mut vec, 3);
        Vector::push_back<u64>(&mut vec, 4);

        let removed = Vector::remove<u64>(&mut vec, 5);
        assert(removed == 4, 1000);
    }
}
// check: "Keep(ABORTED { code: 0"