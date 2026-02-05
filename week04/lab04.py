def find_common_elements(list1, list2):
    """Find the common elements between two lists."""
    return list(set(list1) & set(list2))


def find_user_by_name(users, name):
    """Find a user's profile by name from a list of user data."""
    user_map = {user["name"]: user for user in users}
    return user_map.get(name)


def get_list_of_even_numbers(numbers):
    """Return a new list containing only the even numbers from the input list."""
    return [n for n in numbers if n % 2 == 0]
