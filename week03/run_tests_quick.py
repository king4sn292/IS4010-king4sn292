import builtins
import lab03


def test_generate_mad_lib():
    s = lab03.generate_mad_lib('silly', 'cat', 'jumped')
    assert isinstance(s, str)
    sl = s.lower()
    assert 'silly' in sl and 'cat' in sl and 'jumped' in sl


def run_guess_sim(target, inputs):
    outputs = []

    # patch randint
    original_randint = lab03.random.randint
    lab03.random.randint = lambda a, b: target

    # patch input
    it = iter(inputs)

    def fake_input(prompt=''):
        try:
            return next(it)
        except StopIteration:
            return ''

    original_input = builtins.input
    builtins.input = fake_input

    # patch print
    original_print = builtins.print

    def fake_print(*args, **kwargs):
        outputs.append(' '.join(str(a) for a in args))

    builtins.print = fake_print

    try:
        lab03.guessing_game()
    finally:
        # restore
        lab03.random.randint = original_randint
        builtins.input = original_input
        builtins.print = original_print

    return outputs


def test_guessing_game():
    # correct on first try
    out = run_guess_sim(50, ['50'])
    joined = '\n'.join(out).lower()
    assert 'congrat' in joined or 'correct' in joined or 'right' in joined

    # multiple guesses
    out2 = run_guess_sim(42, ['30', '50', '42'])
    assert len(out2) >= 3


if __name__ == '__main__':
    test_generate_mad_lib()
    print('generate_mad_lib: OK')
    test_guessing_game()
    print('guessing_game: OK')
