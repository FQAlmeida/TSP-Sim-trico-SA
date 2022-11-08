from data_plots.load_data import load

def test_can_load():
    data = load("../data/output.txt")
    assert len(data) > 0

def test_iter_is_ordered():
    data = load("../data/output.txt")
    iters = list(map(lambda d: d.iteraction, data))
    assert sorted(iters) == iters
    
