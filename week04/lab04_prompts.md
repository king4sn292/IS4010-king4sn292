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
