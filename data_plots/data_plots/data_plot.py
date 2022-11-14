import pandas as pd
from dataclasses import dataclass
from statistics import mean, stdev
from typing import Dict, List
import seaborn as sns
from load_data import load_dist
from matplotlib import pyplot

methods = ["exp", "sigmoid", "cos"]
insts = [51, 100]
temps = [1, 10]


@dataclass(frozen=True)
class Config:
    inst: int
    temp: int
    method: str


plot_data_100: Dict[str, List[float]] = dict()
plot_data_51: Dict[str, List[float]] = dict()

for inst in insts:
    for method in methods:
        for temp in temps:
            data = load_dist(
                f"../data/runs/inst_{inst}_{method}_on_temp_{temp}.txt")
            # print(data)
            dist_avg = mean(data)
            dist_stdev = stdev(data)

            m = "2" if method == "sigmoid" else "1" if method == "exp" else "5"
            name = f"Cooling {m} - {temp}"
            print(f"{name:<30}| {dist_avg:>25}| {dist_stdev:>25}")
            if inst == 100:
                plot_data_100[name] = data
            else:
                plot_data_51[name] = data

fig, axs = pyplot.subplots(ncols=2) 
df_100 = pd.DataFrame(data=plot_data_100)
df_51 = pd.DataFrame(data=plot_data_51)
sns.boxplot(x="variable", y="value", data=pd.melt(df_100), ax=axs[0])
sns.boxplot(x="variable", y="value", data=pd.melt(df_51), ax=axs[1])

axs[0].set_ylabel("Distância")
axs[0].set_xlabel("Método Resfriamento | Equilíbrio Térmico")
axs[0].set_title("TSP SA 100")

axs[1].set_ylabel("Distância")
axs[1].set_xlabel("Método Resfriamento | Equilíbrio Térmico")
axs[1].set_title("TSP SA 51")

fig.autofmt_xdate()

pyplot.tight_layout()
pyplot.show()
