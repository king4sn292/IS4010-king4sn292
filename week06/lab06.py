class Book:
    """A class representing a printed book."""

    def __init__(self, title, author, year):
        """Initialize a Book object.

        Parameters
        ----------
        title : str
            The title of the book.
        author : str
            The author of the book.
        year : int
            The publication year of the book.
        """
        self.title = title
        self.author = author
        self.year = year

    def __str__(self):
        """Return a user-friendly string representation of the book.

        Returns
        -------
        str
            A formatted string containing the book's title, author, and year.
        """
        return f'"{self.title}" by {self.author} ({self.year})'

    def get_age(self):
        """Calculate the age of the book.

        Assumes the current year is 2025.

        Returns
        -------
        int
            The age of the book in years.
        """
        return 2025 - self.year


class EBook(Book):
    """A class representing a digital book that inherits from Book."""

    def __init__(self, title, author, year, file_size):
        """Initialize an EBook object.

        Parameters
        ----------
        title : str
            The title of the book.
        author : str
            The author of the book.
        year : int
            The publication year of the book.
        file_size : int
            The size of the file in megabytes.
        """
        super().__init__(title, author, year)
        self.file_size = file_size

    def __str__(self):
        """Return a user-friendly string representation of the ebook.

        Returns
        -------
        str
            A formatted string containing the book's title, author, year, and file size.
        """
        return f'{super().__str__()} ({self.file_size} MB)'


if __name__ == '__main__':
    # Test the Book class
    book = Book("The Hobbit", "J.R.R. Tolkien", 1937)
    print(book)
    print(f"Age: {book.get_age()} years")

    # Test the EBook class
    ebook = EBook("Dune", "Frank Herbert", 1965, 5)
    print(ebook)
    print(f"Age: {ebook.get_age()} years")
