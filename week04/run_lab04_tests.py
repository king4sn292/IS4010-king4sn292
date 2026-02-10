from lab04 import find_common_elements, find_user_by_name, get_list_of_even_numbers


def assert_equal(a, b):
    if a != b:
        raise AssertionError(f"Assertion failed: {a!r} != {b!r}")


def assert_set_equal(a, b):
    if set(a) != set(b):
        raise AssertionError(f"Assertion failed: set({a!r}) != {b!r}")


def run_tests():
    # find_common_elements
    l1 = [1, 2, 3, 4, 5]
    l2 = [4, 5, 6, 7, 8]
    assert_set_equal(find_common_elements(l1, l2), {4, 5})

    assert_equal(find_common_elements([1,2,3], [4,5,6]), [])
    assert_equal(find_common_elements([], []), [])
    assert_equal(find_common_elements([1,2,3], []), [])
    l1 = [1,1,2,3,3]
    l2 = [1,2,2,4,5]
    assert_set_equal(find_common_elements(l1, l2), {1,2})

    # find_user_by_name
    sample_users = [
        {"name": "alice", "age": 30, "email": "alice@example.com"},
        {"name": "bob", "age": 25, "email": "bob@example.com"},
        {"name": "charlie", "age": 35, "email": "charlie@example.com"},
    ]
    assert_equal(find_user_by_name(sample_users, "alice"), sample_users[0])
    assert find_user_by_name(sample_users, "david") is None
    assert find_user_by_name([], "alice") is None
    assert find_user_by_name(sample_users, "Alice") is None
    assert find_user_by_name(sample_users, "alice") is not None

    # get_list_of_even_numbers
    assert_equal(get_list_of_even_numbers([1,2,3,4,5,6]), [2,4,6])
    assert_equal(get_list_of_even_numbers([1,3,5,7]), [])
    assert_equal(get_list_of_even_numbers([2,4,6,8]), [2,4,6,8])
    assert_equal(get_list_of_even_numbers([]), [])
    assert_equal(get_list_of_even_numbers([0,1,2,3]), [0,2])
    assert_equal(get_list_of_even_numbers([10,1,8,3,6,5,4]), [10,8,6,4])
    assert_equal(get_list_of_even_numbers([-4,-3,-2,-1,0,1,2]), [-4,-2,0,2])

    print("All lab04 tests passed (runner).")


if __name__ == '__main__':
    run_tests()
