# Problem 1: Debugging – corrected code
def sum_of_evens(numbers):
    total = 0
    for num in numbers:
        if num % 2 == 0:
            total += num
    return total


# Problem 2: Refactoring – refactored code
def get_names_of_adults(users):
    return [user["name"] for user in users if user["age"] >= 18]


# Problem 3: Documenting – documented code
def calculate_area(length, width):
    """Calculate the area of a rectangle.

    Parameters
    ----------
    length : float or int
        The rectangle's length. Must be positive.
    width : float or int
        The rectangle's width. Must be positive.

    Returns
    -------
    float or int
        The area of the rectangle.

    Raises
    ------
    ValueError
        If length or width is less than or equal to 0.

    Examples
    --------
    >>> calculate_area(5, 2)
    10
    """
    if length <= 0 or width <= 0:
        raise ValueError("Length and width must be positive numbers.")
    return length * width
