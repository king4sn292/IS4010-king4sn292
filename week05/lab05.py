from typing import List, Dict, Any

# User data
users = [
    {"name": "alice", "age": 30, "is_active": True, "email": "alice@example.com"},
    {"name": "bob", "age": 25, "is_active": False},
    {"name": "charlie", "age": 35, "is_active": True, "email": "charlie@example.com"},
    {"name": "david", "age": "unknown", "is_active": False},
]


def calculate_average_age(users: List[Dict[str, Any]]) -> float:
    """Calculate average age from a list of user dicts.

    - Only considers numeric ages (int/float).
    - Returns 0.0 for empty input or if no valid ages found.
    """
    if not users:
        return 0.0

    total = 0.0
    count = 0
    for u in users:
        age = u.get("age")
        if isinstance(age, (int, float)):
            total += float(age)
            count += 1

    if count == 0:
        return 0.0

    return total / count


def get_active_user_emails(users: List[Dict[str, Any]]) -> List[str]:
    """Return a list of email addresses for users with `is_active` True.

    - If a user is active but has no `email` key, it is skipped.
    - Returns empty list for no input or no active users.
    """
    if not users:
        return []

    emails = []
    for u in users:
        if u.get("is_active"):
            email = u.get("email")
            if isinstance(email, str):
                emails.append(email)

    return emails


if __name__ == '__main__':
    avg_age = calculate_average_age(users)
    print(f"average user age: {avg_age:.2f}")

    active_emails = get_active_user_emails(users)
    print(f"active user emails: {active_emails}")
