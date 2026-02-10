# Lab 04 AI prompts and responses

This file records the exact prompts used with an AI assistant and the assistant's recommendation/reasoning for each problem in Lab 04.

---

## Problem 1 — Finding common items

Prompt:

"I have two very large lists of product IDs and need to find which IDs appear in both lists. The order of the final list does not matter and performance is important. Which Python data structure should I use and why?"

AI recommendation:

- Use `set` for both lists and compute the intersection. Sets provide O(1) average-time membership tests and intersection is efficient for large collections. Convert the resulting set back to a list if a list return type is required.

Reasoning:

- Sets eliminate duplicates automatically and support fast membership operations and intersection, making them appropriate when order does not matter and performance is important.

---

## Problem 2 — User profile lookup

Prompt:

"I have a list of user profile dictionaries where each user has a unique username. I need to look up a user's full profile by username frequently and performance is important. Which data structure should I use and why?"

AI recommendation:

- Convert the list to a dictionary keyed by username: `{user['name']: user for user in users}`. Dictionary lookups by key are O(1) on average, which is ideal for frequent profile lookups.

Reasoning:

- Dictionaries provide constant-time key-based lookup and let you retrieve the full profile quickly without scanning the list.

---

## Problem 3 — Listing even numbers in order

Prompt:

"Given a list of integer sensor readings, I need to produce a list containing only the even readings and preserve the original order. Which data structure or approach should I use and why?"

AI recommendation:

- Use a list comprehension to filter and preserve order: `[n for n in numbers if n % 2 == 0]`.

Reasoning:

- Lists preserve order and a list comprehension is an idiomatic, efficient, and readable way to filter while preserving the input sequence.

---

End of prompts.
# Lab 04 Prompts

## Problem 1: Finding common items
**Prompt I used:**
I have two very large lists of product IDs. I need the IDs that appear in both lists. Order does not matter. What Python data structure should I use and why?

**AI recommendation and reasoning:**
Use a `set` because sets allow fast membership checks and support intersection operations. Converting both lists to sets and intersecting them efficiently returns the unique common IDs.

---

## Problem 2: User profile lookup
**Prompt I used:**
I have a list of user dictionaries with keys name, age, and email. I need to look up a user by username frequently, and performance is critical. What data structure should I use?

**AI recommendation and reasoning:**
Use a `dictionary` that maps each username to its full user profile. This allows constant-time (O(1)) lookups using `dict.get()`.

---

## Problem 3: Listing even numbers in order
**Prompt I used:**
I have a list of integers and need to return only the even values while preserving the original order. What data structure or approach should I use?

**AI recommendation and reasoning:**
Use a `list` with a list comprehension. Lists preserve order naturally, and list comprehensions provide a clean way to filter values.
